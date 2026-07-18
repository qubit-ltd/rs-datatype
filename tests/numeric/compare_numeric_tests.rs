// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use std::cmp::Ordering;
use std::marker::PhantomData;

#[cfg(feature = "big-number")]
use bigdecimal::BigDecimal;
#[cfg(feature = "big-number")]
use num_bigint::BigInt;
#[cfg(feature = "big-number")]
use num_rational::BigRational;
use proptest::arbitrary::any;
use proptest::test_runner::Config as ProptestConfig;
use proptest::{
    prop_assert_eq,
    prop_assert_ne,
    proptest,
};
use qubit_datatype::{
    NumericComparisonPolicy,
    NumericValueRef,
    compare_numeric,
};
#[cfg(feature = "big-number")]
use std::str::FromStr;

/// Asserts an exact ordering and its reverse-direction symmetry.
fn assert_exact(
    left: NumericValueRef<'_>,
    right: NumericValueRef<'_>,
    expected: Option<Ordering>,
) {
    assert_eq!(
        compare_numeric(left, right, NumericComparisonPolicy::Exact),
        expected
    );
    assert_eq!(
        compare_numeric(right, left, NumericComparisonPolicy::Exact),
        expected.map(Ordering::reverse)
    );
}

/// Constructs a finite `f64` from independently generated IEEE-754 fields.
///
/// # Parameters
///
/// * `sign` - Whether to set the sign bit.
/// * `exponent` - An encoded finite exponent below `0x7ff`.
/// * `fraction` - The low 52 bits of the significand.
///
/// # Returns
///
/// The finite value represented by the supplied fields.
///
/// # Panics
///
/// Panics in debug builds if `exponent` encodes a non-finite value.
fn finite_f64(sign: bool, exponent: u16, fraction: u64) -> f64 {
    debug_assert!(exponent < 0x7ff);
    let sign = u64::from(sign) << 63;
    let exponent = u64::from(exponent) << 52;
    let fraction = fraction & ((1_u64 << 52) - 1);
    f64::from_bits(sign | exponent | fraction)
}

/// Decodes a finite IEEE-754 `f64` into an independent exact rational oracle.
///
/// # Parameters
///
/// * `value` - The finite floating-point value to decode.
///
/// # Returns
///
/// A rational with exactly the same mathematical value as `value`.
///
/// # Panics
///
/// Panics if `value` is NaN or infinite.
#[cfg(feature = "big-number")]
fn finite_f64_to_rational(value: f64) -> BigRational {
    assert!(
        value.is_finite(),
        "the rational oracle requires a finite f64"
    );
    let bits = value.to_bits();
    let negative = bits >> 63 != 0;
    let encoded_exponent = ((bits >> 52) & 0x7ff) as i32;
    let fraction = bits & ((1_u64 << 52) - 1);
    let (significand, binary_exponent) = if encoded_exponent == 0 {
        (fraction, -1074)
    } else {
        ((1_u64 << 52) | fraction, encoded_exponent - 1023 - 52)
    };
    let mut numerator = BigInt::from(significand);
    if negative {
        numerator = -numerator;
    }
    if binary_exponent >= 0 {
        BigRational::from_integer(numerator << (binary_exponent as usize))
    } else {
        BigRational::new(
            numerator,
            BigInt::from(1_u8) << ((-binary_exponent) as usize),
        )
    }
}

/// Converts bounded `BigDecimal` parts into an independent rational oracle.
///
/// # Parameters
///
/// * `coefficient` - The signed decimal coefficient.
/// * `scale` - The bounded power-of-ten scale used by `BigDecimal`.
///
/// # Returns
///
/// The rational value represented by `coefficient * 10^-scale`.
#[cfg(feature = "big-number")]
fn decimal_parts_to_rational(coefficient: i64, scale: i64) -> BigRational {
    let coefficient = BigInt::from(coefficient);
    let factor = BigInt::from(10_u8).pow(scale.unsigned_abs() as u32);
    if scale >= 0 {
        BigRational::new(coefficient, factor)
    } else {
        BigRational::from_integer(coefficient * factor)
    }
}

/// Covers exact fixed-width boundaries without integer-to-float projection.
#[test]
fn test_compare_numeric_exact_fixed_boundaries() {
    assert_exact(
        NumericValueRef::Int128(i128::MIN),
        NumericValueRef::UInt128(u128::MAX),
        Some(Ordering::Less),
    );
    assert_exact(
        NumericValueRef::UInt64((1_u64 << 53) + 1),
        NumericValueRef::Float64((1_u64 << 53) as f64),
        Some(Ordering::Greater),
    );
    assert_exact(
        NumericValueRef::UInt32((1_u32 << 24) + 1),
        NumericValueRef::Float32((1_u32 << 24) as f32),
        Some(Ordering::Greater),
    );
    assert_exact(
        NumericValueRef::Int32(1),
        NumericValueRef::Float64(1.5),
        Some(Ordering::Less),
    );
    assert_exact(
        NumericValueRef::Float64(-0.0),
        NumericValueRef::Float64(0.0),
        Some(Ordering::Equal),
    );
    assert_exact(
        NumericValueRef::Float64(f64::NEG_INFINITY),
        NumericValueRef::Int8(i8::MIN),
        Some(Ordering::Less),
    );
    assert_exact(
        NumericValueRef::Float32(f32::INFINITY),
        NumericValueRef::UInt128(u128::MAX),
        Some(Ordering::Greater),
    );
    assert_exact(
        NumericValueRef::Float32(f32::NEG_INFINITY),
        NumericValueRef::Float64(f64::INFINITY),
        Some(Ordering::Less),
    );
    assert_exact(
        NumericValueRef::Float32(f32::INFINITY),
        NumericValueRef::Float64(f64::INFINITY),
        Some(Ordering::Equal),
    );

    for value in [
        NumericValueRef::Int8(1),
        NumericValueRef::Int16(1),
        NumericValueRef::Int32(1),
        NumericValueRef::Int64(1),
        NumericValueRef::Int128(1),
        NumericValueRef::UInt8(1),
        NumericValueRef::UInt16(1),
        NumericValueRef::UInt32(1),
        NumericValueRef::UInt64(1),
        NumericValueRef::UInt128(1),
        NumericValueRef::Float32(1.0),
        NumericValueRef::Float64(1.0),
    ] {
        assert_exact(value, NumericValueRef::Int8(1), Some(Ordering::Equal));
    }
    assert_exact(
        NumericValueRef::Float32(f32::from_bits(1)),
        NumericValueRef::Int8(0),
        Some(Ordering::Greater),
    );
    assert_exact(
        NumericValueRef::Float64(-f64::from_bits(1)),
        NumericValueRef::Int8(0),
        Some(Ordering::Less),
    );
    assert_exact(
        NumericValueRef::Float64(4.0),
        NumericValueRef::Float64(2.0),
        Some(Ordering::Greater),
    );
    assert_exact(
        NumericValueRef::Float64(-4.0),
        NumericValueRef::Float64(-2.0),
        Some(Ordering::Less),
    );
}

/// Verifies that every NaN payload is unordered.
#[test]
fn test_compare_numeric_rejects_nan() {
    for nan in [
        f64::from_bits(0x7ff8_0000_0000_0001),
        f64::from_bits(0x7fff_ffff_ffff_ffff),
    ] {
        assert_exact(
            NumericValueRef::Float64(nan),
            NumericValueRef::Float64(nan),
            None,
        );
        assert_exact(
            NumericValueRef::Float64(nan),
            NumericValueRef::Int8(0),
            None,
        );
    }
}

/// Verifies approximate projection only changes float-participating pairs.
#[test]
fn test_compare_numeric_approximate_policy() {
    assert_eq!(
        compare_numeric(
            NumericValueRef::UInt64((1_u64 << 53) + 1),
            NumericValueRef::Float64((1_u64 << 53) as f64),
            NumericComparisonPolicy::Approximate,
        ),
        Some(Ordering::Equal)
    );
    assert_eq!(
        compare_numeric(
            NumericValueRef::UInt128(u128::MAX - 1),
            NumericValueRef::UInt128(u128::MAX),
            NumericComparisonPolicy::Approximate,
        ),
        Some(Ordering::Less)
    );

    for value in [
        NumericValueRef::Int8(0),
        NumericValueRef::Int16(0),
        NumericValueRef::Int32(0),
        NumericValueRef::Int64(0),
        NumericValueRef::Int128(0),
        NumericValueRef::UInt8(0),
        NumericValueRef::UInt16(0),
        NumericValueRef::UInt32(0),
        NumericValueRef::UInt64(0),
        NumericValueRef::UInt128(0),
        NumericValueRef::Float32(0.0),
        NumericValueRef::Float64(0.0),
    ] {
        assert_eq!(
            compare_numeric(
                value,
                NumericValueRef::Float64(0.0),
                NumericComparisonPolicy::Approximate,
            ),
            Some(Ordering::Equal)
        );
    }
    assert_eq!(
        compare_numeric(
            NumericValueRef::__Lifetime(PhantomData),
            NumericValueRef::Float64(0.0),
            NumericComparisonPolicy::Approximate,
        ),
        None
    );
}

/// Characterizes the pair-dependent, non-transitive approximate projection.
#[test]
fn test_compare_numeric_approximate_is_not_transitive() {
    let lower = NumericValueRef::UInt64(1_u64 << 53);
    let projected = NumericValueRef::Float64((1_u64 << 53) as f64);
    let upper = NumericValueRef::UInt64((1_u64 << 53) + 1);
    assert_eq!(
        compare_numeric(lower, projected, NumericComparisonPolicy::Approximate),
        Some(Ordering::Equal)
    );
    assert_eq!(
        compare_numeric(projected, upper, NumericComparisonPolicy::Approximate),
        Some(Ordering::Equal)
    );
    assert_eq!(
        compare_numeric(lower, upper, NumericComparisonPolicy::Approximate),
        Some(Ordering::Less)
    );
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]

    /// Checks exact signed/unsigned ordering against primitive integer ordering.
    #[test]
    fn test_compare_numeric_exact_signed_unsigned_property(
        signed in any::<i128>(),
        unsigned in any::<u128>(),
    ) {
        let expected = if signed < 0 {
            Ordering::Less
        } else {
            (signed as u128).cmp(&unsigned)
        };
        prop_assert_eq!(
            compare_numeric(
                NumericValueRef::Int128(signed),
                NumericValueRef::UInt128(unsigned),
                NumericComparisonPolicy::Exact,
            ),
            Some(expected),
        );
    }

    /// Checks exact comparisons are antisymmetric across integer and float forms.
    #[test]
    fn test_compare_numeric_exact_antisymmetry_property(
        integer in any::<i128>(),
        sign in any::<bool>(),
        exponent in 0_u16..0x7ff,
        fraction in 0_u64..(1_u64 << 52),
    ) {
        let float = finite_f64(sign, exponent, fraction);
        let forward = compare_numeric(
            NumericValueRef::Int128(integer),
            NumericValueRef::Float64(float),
            NumericComparisonPolicy::Exact,
        );
        let reverse = compare_numeric(
            NumericValueRef::Float64(float),
            NumericValueRef::Int128(integer),
            NumericComparisonPolicy::Exact,
        );
        prop_assert_eq!(forward, reverse.map(Ordering::reverse));
    }

    /// Checks exact comparison transitivity across three direct representations.
    #[test]
    fn test_compare_numeric_exact_transitivity_property(
        signed in any::<i128>(),
        unsigned in any::<u128>(),
        sign in any::<bool>(),
        exponent in 0_u16..0x7ff,
        fraction in 0_u64..(1_u64 << 52),
    ) {
        let float = finite_f64(sign, exponent, fraction);
        let left = NumericValueRef::Int128(signed);
        let middle = NumericValueRef::Float64(float);
        let right = NumericValueRef::UInt128(unsigned);
        let left_middle = compare_numeric(left, middle, NumericComparisonPolicy::Exact)
            .expect("generated finite values must be ordered");
        let middle_right = compare_numeric(middle, right, NumericComparisonPolicy::Exact)
            .expect("generated finite values must be ordered");
        if left_middle != Ordering::Greater && middle_right != Ordering::Greater {
            prop_assert_ne!(
                compare_numeric(left, right, NumericComparisonPolicy::Exact),
                Some(Ordering::Greater),
            );
        }
    }

    /// Checks same-representation exact comparisons against native ordering.
    #[test]
    fn test_compare_numeric_exact_native_agreement_property(
        left_signed in any::<i128>(),
        right_signed in any::<i128>(),
        left_unsigned in any::<u128>(),
        right_unsigned in any::<u128>(),
        left_sign in any::<bool>(),
        left_exponent in 0_u16..0x7ff,
        left_fraction in 0_u64..(1_u64 << 52),
        right_sign in any::<bool>(),
        right_exponent in 0_u16..0x7ff,
        right_fraction in 0_u64..(1_u64 << 52),
    ) {
        let left_float = finite_f64(left_sign, left_exponent, left_fraction);
        let right_float = finite_f64(right_sign, right_exponent, right_fraction);
        prop_assert_eq!(
            compare_numeric(
                NumericValueRef::Int128(left_signed),
                NumericValueRef::Int128(right_signed),
                NumericComparisonPolicy::Exact,
            ),
            Some(left_signed.cmp(&right_signed)),
        );
        prop_assert_eq!(
            compare_numeric(
                NumericValueRef::UInt128(left_unsigned),
                NumericValueRef::UInt128(right_unsigned),
                NumericComparisonPolicy::Exact,
            ),
            Some(left_unsigned.cmp(&right_unsigned)),
        );
        prop_assert_eq!(
            compare_numeric(
                NumericValueRef::Float64(left_float),
                NumericValueRef::Float64(right_float),
                NumericComparisonPolicy::Exact,
            ),
            left_float.partial_cmp(&right_float),
        );
    }

    /// Checks integer/finite-float comparisons against an independent rational oracle.
    #[cfg(feature = "big-number")]
    #[test]
    fn test_compare_numeric_exact_integer_float_oracle_property(
        integer in any::<i128>(),
        sign in any::<bool>(),
        exponent in 0_u16..0x7ff,
        fraction in 0_u64..(1_u64 << 52),
    ) {
        let float = finite_f64(sign, exponent, fraction);
        let expected = BigRational::from_integer(BigInt::from(integer))
            .cmp(&finite_f64_to_rational(float));
        prop_assert_eq!(
            compare_numeric(
                NumericValueRef::Int128(integer),
                NumericValueRef::Float64(float),
                NumericComparisonPolicy::Exact,
            ),
            Some(expected),
        );
    }

    /// Checks bounded arbitrary-precision coefficients and decimal scales.
    #[cfg(feature = "big-number")]
    #[test]
    fn test_compare_numeric_exact_big_number_oracle_property(
        integer_coefficient in -1_000_000_i64..=1_000_000,
        decimal_coefficient in -1_000_000_i64..=1_000_000,
        decimal_scale in -32_i64..=32,
    ) {
        let integer = BigInt::from(integer_coefficient);
        let decimal = BigDecimal::new(BigInt::from(decimal_coefficient), decimal_scale);
        let expected = BigRational::from_integer(BigInt::from(integer_coefficient))
            .cmp(&decimal_parts_to_rational(decimal_coefficient, decimal_scale));
        prop_assert_eq!(
            compare_numeric(
                NumericValueRef::BigInteger(&integer),
                NumericValueRef::BigDecimal(&decimal),
                NumericComparisonPolicy::Exact,
            ),
            Some(expected),
        );
    }

    /// Checks every public non-NaN representation pair remains ordered.
    #[cfg(feature = "big-number")]
    #[test]
    fn test_compare_numeric_all_public_representations_are_ordered(
        signed in any::<i128>(),
        unsigned in any::<u128>(),
        float32_numerator in any::<i32>(),
        float64_sign in any::<bool>(),
        float64_exponent in 0_u16..0x7ff,
        float64_fraction in 0_u64..(1_u64 << 52),
        integer_coefficient in -1_000_000_i64..=1_000_000,
        decimal_coefficient in -1_000_000_i64..=1_000_000,
        decimal_scale in -32_i64..=32,
    ) {
        let float32 = float32_numerator as f32 / 17.0;
        let float64 = finite_f64(
            float64_sign,
            float64_exponent,
            float64_fraction,
        );
        let integer = BigInt::from(integer_coefficient);
        let decimal = BigDecimal::new(
            BigInt::from(decimal_coefficient),
            decimal_scale,
        );
        let values = [
            NumericValueRef::Int8(signed as i8),
            NumericValueRef::Int16(signed as i16),
            NumericValueRef::Int32(signed as i32),
            NumericValueRef::Int64(signed as i64),
            NumericValueRef::Int128(signed),
            NumericValueRef::UInt8(unsigned as u8),
            NumericValueRef::UInt16(unsigned as u16),
            NumericValueRef::UInt32(unsigned as u32),
            NumericValueRef::UInt64(unsigned as u64),
            NumericValueRef::UInt128(unsigned),
            NumericValueRef::Float32(float32),
            NumericValueRef::Float64(float64),
            NumericValueRef::BigInteger(&integer),
            NumericValueRef::BigDecimal(&decimal),
        ];

        for policy in [
            NumericComparisonPolicy::Exact,
            NumericComparisonPolicy::Approximate,
        ] {
            for &left in &values {
                for &right in &values {
                    let forward = compare_numeric(left, right, policy);
                    let reverse = compare_numeric(right, left, policy);
                    prop_assert_ne!(forward, None);
                    prop_assert_eq!(reverse, forward.map(Ordering::reverse));
                }
            }
        }
    }
}

/// Covers exact and approximate arbitrary-precision comparisons.
#[cfg(feature = "big-number")]
#[test]
fn test_compare_numeric_big_number_paths() {
    let integer = BigInt::from(u128::MAX) + BigInt::from(1_u8);
    assert_exact(
        NumericValueRef::BigInteger(&integer),
        NumericValueRef::UInt128(u128::MAX),
        Some(Ordering::Greater),
    );

    let decimal = BigDecimal::from_str("0.1").unwrap();
    assert_eq!(
        compare_numeric(
            NumericValueRef::BigDecimal(&decimal),
            NumericValueRef::Float64(0.1),
            NumericComparisonPolicy::Exact,
        ),
        Some(Ordering::Less)
    );
    assert_eq!(
        compare_numeric(
            NumericValueRef::BigDecimal(&decimal),
            NumericValueRef::Float64(0.1),
            NumericComparisonPolicy::Approximate,
        ),
        Some(Ordering::Equal)
    );

    for value in [
        NumericValueRef::Int8(1),
        NumericValueRef::Int16(1),
        NumericValueRef::Int32(1),
        NumericValueRef::Int64(1),
        NumericValueRef::Int128(1),
        NumericValueRef::UInt8(1),
        NumericValueRef::UInt16(1),
        NumericValueRef::UInt32(1),
        NumericValueRef::UInt64(1),
        NumericValueRef::UInt128(1),
        NumericValueRef::Float32(1.0),
        NumericValueRef::Float64(1.0),
    ] {
        assert_exact(
            NumericValueRef::BigInteger(&BigInt::from(1)),
            value,
            Some(Ordering::Equal),
        );
    }

    assert_exact(
        NumericValueRef::BigInteger(&BigInt::from(0)),
        NumericValueRef::Float32(-f32::from_bits(1)),
        Some(Ordering::Greater),
    );
    for value in [
        NumericValueRef::Float64(f64::from_bits(1)),
        NumericValueRef::Float64(f64::MAX),
    ] {
        assert_exact(
            NumericValueRef::BigInteger(&BigInt::from(0)),
            value,
            Some(Ordering::Less),
        );
    }

    let negative_scale = BigDecimal::new(BigInt::from(12), -2);
    assert_exact(
        NumericValueRef::BigDecimal(&negative_scale),
        NumericValueRef::BigInteger(&BigInt::from(1_200)),
        Some(Ordering::Equal),
    );

    let extreme_scale =
        BigDecimal::new(BigInt::from(0), i64::from(u32::MAX) + 1);
    let extreme_negative_scale =
        BigDecimal::new(BigInt::from(0), -(i64::from(u32::MAX) + 1));
    assert_exact(
        NumericValueRef::BigDecimal(&extreme_negative_scale),
        NumericValueRef::BigDecimal(&extreme_scale),
        Some(Ordering::Equal),
    );
    let large_scale = BigDecimal::new(BigInt::from(1), 1_000_000);
    let large_negative_scale = BigDecimal::new(BigInt::from(1), -1_000_000);
    assert_exact(
        NumericValueRef::BigDecimal(&large_scale),
        NumericValueRef::BigInteger(&BigInt::from(0)),
        Some(Ordering::Greater),
    );
    assert_exact(
        NumericValueRef::BigDecimal(&large_negative_scale),
        NumericValueRef::BigInteger(&BigInt::from(0)),
        Some(Ordering::Greater),
    );
    for value in [
        NumericValueRef::Int8(0),
        NumericValueRef::Int16(0),
        NumericValueRef::Int32(0),
        NumericValueRef::Int64(0),
        NumericValueRef::Int128(0),
        NumericValueRef::UInt8(0),
        NumericValueRef::UInt16(0),
        NumericValueRef::UInt32(0),
        NumericValueRef::UInt64(0),
        NumericValueRef::UInt128(0),
        NumericValueRef::Float32(0.0),
        NumericValueRef::Float64(0.0),
        NumericValueRef::BigInteger(&integer),
        NumericValueRef::BigDecimal(&decimal),
    ] {
        let expected = if matches!(
            value,
            NumericValueRef::BigInteger(_) | NumericValueRef::BigDecimal(_)
        ) {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Equal)
        };
        assert_exact(
            NumericValueRef::BigDecimal(&extreme_scale),
            value,
            expected,
        );
    }
    assert_exact(
        NumericValueRef::BigInteger(&BigInt::from(0)),
        NumericValueRef::__Lifetime(PhantomData),
        None,
    );

    let too_large_for_f64 = BigInt::from(1_u8) << 20_000;
    assert_eq!(
        compare_numeric(
            NumericValueRef::BigInteger(&too_large_for_f64),
            NumericValueRef::Float64(1.0),
            NumericComparisonPolicy::Approximate,
        ),
        Some(Ordering::Greater)
    );
}
