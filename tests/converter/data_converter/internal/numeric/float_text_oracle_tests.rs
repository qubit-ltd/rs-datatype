// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Independent rational-oracle tests for strict floating-point text conversion.

use num_bigint::BigInt;
use num_rational::BigRational;
use proptest::prelude::any;
use proptest::proptest;
use qubit_datatype::DataConversionErrorKind;
use qubit_datatype::DataConverter;
use qubit_datatype::DataType;
use qubit_datatype::InvalidValueReason;

/// Decodes a finite IEEE-754 value into its exact rational representation.
///
/// # Parameters
///
/// * `value` - The finite value to decode.
///
/// # Returns
///
/// The exact mathematical value represented by `value`.
///
/// # Panics
///
/// Panics if `value` is NaN or infinite.
fn finite_float_rational(value: f64) -> BigRational {
    assert!(value.is_finite());
    let bits = value.to_bits();
    let encoded_exponent = ((bits >> 52) & 0x7ff) as i32;
    let fraction = bits & ((1_u64 << 52) - 1);
    let (significand, exponent) = if encoded_exponent == 0 {
        (fraction, -1074)
    } else {
        ((1_u64 << 52) | fraction, encoded_exponent - 1075)
    };
    let mut numerator = BigInt::from(significand);
    if bits >> 63 != 0 {
        numerator = -numerator;
    }
    if exponent >= 0 {
        BigRational::from_integer(numerator << (exponent as usize))
    } else {
        BigRational::new(numerator, BigInt::from(1_u8) << ((-exponent) as usize))
    }
}

/// Constructs the exact rational represented by a bounded decimal coefficient
/// and exponent.
///
/// # Parameters
///
/// * `coefficient` - Signed integer coefficient of the decimal value.
/// * `exponent` - Bounded power of ten applied to the coefficient.
///
/// # Returns
///
/// The exact value `coefficient * 10^exponent`.
fn decimal_rational(coefficient: i64, exponent: i32) -> BigRational {
    let coefficient = BigInt::from(coefficient);
    let factor = BigInt::from(10_u8).pow(exponent.unsigned_abs());
    if exponent >= 0 {
        BigRational::from_integer(coefficient * factor)
    } else {
        BigRational::new(coefficient, factor)
    }
}

/// Writes a rational with a power-of-two denominator as exact decimal
/// scientific notation.
///
/// The caller must supply a reduced rational whose denominator is a power of
/// two.
///
/// # Parameters
///
/// * `value` - Exact binary rational to express in decimal notation.
///
/// # Returns
///
/// A decimal string with the same exact mathematical value.
///
/// # Panics
///
/// Panics if the denominator is not one or a power of two, or if its exponent
/// is unbounded.
fn exact_binary_decimal_text(value: &BigRational) -> String {
    let denominator_bits = value.denom().bits();
    assert!(denominator_bits > 0);
    assert_eq!(value.denom(), &(BigInt::from(1_u8) << (denominator_bits as usize - 1)));
    let power = u32::try_from(denominator_bits - 1).expect("bounded IEEE exponent");
    let coefficient = value.numer() * BigInt::from(5_u8).pow(power);
    format!("{coefficient}e-{power}")
}

macro_rules! define_text_check {
    ($name:ident, $float:ty, $target:expr) => {
        /// Verifies strict conversion against the target-width IEEE candidate and
        /// rational oracle.
        fn $name(text: &str, expected: &BigRational) {
            let parsed = text
                .parse::<$float>()
                .expect("generator emits valid decimal syntax");
            let actual = DataConverter::from(text).to::<$float>();
            if parsed.is_finite() && finite_float_rational(f64::from(parsed)) == *expected {
                let value = actual.expect("exact decimal must be accepted");
                assert_eq!(value.to_bits(), parsed.to_bits());
                assert_eq!(finite_float_rational(f64::from(value)), *expected);
            } else {
                let error = actual.expect_err("inexact or overflowing decimal must fail");
                let reason = if parsed.is_finite() {
                    InvalidValueReason::PrecisionLoss
                } else {
                    InvalidValueReason::OutOfRange
                };
                assert_eq!(error.kind(), DataConversionErrorKind::InvalidValue);
                assert_eq!(error.reason(), Some(&reason));
                assert_eq!(error.from_type(), Some(DataType::String));
                assert_eq!(error.to_type(), $target);
            }
        }
    };
}

define_text_check!(check_f32_text, f32, DataType::Float32);
define_text_check!(check_f64_text, f64, DataType::Float64);

proptest! {
    #![proptest_config(proptest::test_runner::Config::with_cases(256))]

    #[test]
    fn test_strict_decimal_text_matches_rational(
        coefficient in any::<i64>(), exponent in -350_i32..=350,
    ) {
        let expected = decimal_rational(coefficient, exponent);
        for text in [
            format!("{coefficient}e{exponent}"),
            format!("{coefficient}.0e{exponent}"),
            format!("{coefficient}0e{}", exponent - 1),
        ] {
            check_f32_text(&text, &expected);
            check_f64_text(&text, &expected);
        }
    }

    #[test]
    fn test_strict_f64_exact_text_and_midpoint(
        bits in 1_u64..0x7fef_ffff_ffff_ffff_u64,
    ) {
        let value = f64::from_bits(bits);
        let rational = finite_float_rational(value);
        let next = finite_float_rational(f64::from_bits(bits + 1));
        for exact in [rational.clone(), -rational.clone()] {
            let text = exact_binary_decimal_text(&exact);
            check_f64_text(&text, &exact);
            assert_eq!(
                finite_float_rational(DataConverter::from(text.as_str()).to::<f64>().unwrap()),
                exact,
            );
        }
        let midpoint = (rational + next) / BigRational::from_integer(BigInt::from(2_u8));
        let text = exact_binary_decimal_text(&midpoint);
        check_f64_text(&text, &midpoint);
        assert_eq!(
            DataConverter::from(text.as_str()).to::<f64>().unwrap_err().reason(),
            Some(&InvalidValueReason::PrecisionLoss),
        );
    }

    #[test]
    fn test_strict_f32_exact_text_and_midpoint(
        bits in 1_u32..0x7f7f_ffff_u32,
    ) {
        let value = f32::from_bits(bits);
        let rational = finite_float_rational(f64::from(value));
        let next = finite_float_rational(f64::from(f32::from_bits(bits + 1)));
        for exact in [rational.clone(), -rational.clone()] {
            let text = exact_binary_decimal_text(&exact);
            check_f32_text(&text, &exact);
            assert_eq!(
                finite_float_rational(f64::from(DataConverter::from(text.as_str()).to::<f32>().unwrap())),
                exact,
            );
        }
        let midpoint = (rational + next) / BigRational::from_integer(BigInt::from(2_u8));
        let text = exact_binary_decimal_text(&midpoint);
        check_f32_text(&text, &midpoint);
        assert_eq!(
            DataConverter::from(text.as_str()).to::<f32>().unwrap_err().reason(),
            Some(&InvalidValueReason::PrecisionLoss),
        );
    }
}

#[test]
fn test_strict_float_oracle_known_boundaries() {
    assert_eq!(finite_float_rational(0.5), BigRational::new(1.into(), 2.into()));
    assert_eq!(decimal_rational(1, -1), BigRational::new(1.into(), 10.into()));
    check_f32_text("0.1", &decimal_rational(1, -1));
    check_f64_text("0.1", &decimal_rational(1, -1));
    check_f32_text("1e39", &decimal_rational(1, 39));
    check_f64_text("1e309", &decimal_rational(1, 309));
    for bits in [
        1_u64,
        0x000f_ffff_ffff_ffff,
        0x0010_0000_0000_0000,
        0x7fef_ffff_ffff_ffff,
    ] {
        for value in [f64::from_bits(bits), -f64::from_bits(bits)] {
            let rational = finite_float_rational(value);
            let text = exact_binary_decimal_text(&rational);
            check_f64_text(&text, &rational);
            assert_eq!(
                DataConverter::from(text.as_str()).to::<f64>().unwrap().to_bits(),
                value.to_bits()
            );
        }
    }
    for bits in [1_u32, 0x007f_ffff, 0x0080_0000, 0x7f7f_ffff] {
        for value in [f32::from_bits(bits), -f32::from_bits(bits)] {
            let rational = finite_float_rational(f64::from(value));
            let text = exact_binary_decimal_text(&rational);
            check_f32_text(&text, &rational);
            assert_eq!(
                DataConverter::from(text.as_str()).to::<f32>().unwrap().to_bits(),
                value.to_bits()
            );
        }
    }
    for (text, negative) in [("0.000e-9", false), ("-0.000e-9", true)] {
        assert_eq!(
            DataConverter::from(text).to::<f32>().unwrap().is_sign_negative(),
            negative
        );
        assert_eq!(
            DataConverter::from(text).to::<f64>().unwrap().is_sign_negative(),
            negative
        );
    }
}
