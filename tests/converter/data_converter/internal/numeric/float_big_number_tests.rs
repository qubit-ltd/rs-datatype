// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Arbitrary-precision source to primitive float tests.

#[cfg(feature = "big-decimal")]
use std::str::FromStr;

#[cfg(feature = "big-decimal")]
use bigdecimal::BigDecimal;
#[cfg(feature = "big-integer")]
use num_bigint::BigInt;
#[cfg(feature = "big-decimal")]
use num_traits::FromPrimitive;
#[cfg(any(feature = "big-integer", feature = "big-decimal"))]
use qubit_datatype::ConversionLimits;
#[cfg(any(feature = "big-integer", feature = "big-decimal"))]
use qubit_datatype::ConversionPolicy;
#[cfg(any(feature = "big-integer", feature = "big-decimal"))]
use qubit_datatype::DataConversionErrorKind;
#[cfg(any(feature = "big-integer", feature = "big-decimal"))]
use qubit_datatype::DataConverter;
#[cfg(feature = "big-decimal")]
use qubit_datatype::DataType;
#[cfg(any(feature = "big-integer", feature = "big-decimal"))]
use qubit_datatype::InvalidValueReason;

/// Verifies that BigInt-to-f32 conversion applies the target precision policy.
#[cfg(feature = "big-integer")]
#[test]
fn test_bigint_to_f32_applies_target_width_precision() {
    let value = BigInt::from(16_777_217_u32);
    let error = DataConverter::from(&value)
        .to::<f32>()
        .expect_err("strict conversion must reject target-width precision loss");
    assert_eq!(error.kind(), DataConversionErrorKind::InvalidValue);
    assert_eq!(error.reason(), Some(&InvalidValueReason::PrecisionLoss));
    assert_eq!(
        DataConverter::from(&value).to_with::<f32>(&ConversionPolicy::lossy(), ConversionLimits::default_ref()),
        Ok(16_777_216.0),
    );
}

/// Verifies exact BigDecimal-to-f32 conversion rejects rounded values.
#[cfg(feature = "big-decimal")]
#[test]
fn test_big_decimal_to_f32_rejects_precision_loss() {
    let value = BigDecimal::from_str("0.1").expect("test decimal");
    let error = DataConverter::from(&value)
        .to::<f32>()
        .expect_err("strict conversion must reject target-width precision loss");
    assert_eq!(error.kind(), DataConversionErrorKind::InvalidValue);
    assert_eq!(error.reason(), Some(&InvalidValueReason::PrecisionLoss));
}

/// Verifies BigDecimal conversion rounds directly to the target float width.
#[cfg(feature = "big-decimal")]
#[test]
fn test_decimal_nearest_even_avoids_intermediate_rounding() {
    let policy = ConversionPolicy::lossy();
    let limits = ConversionLimits::default();
    for (text, expected) in [
        ("1.0000000596046447753906250000000000000001", 0x3f80_0001_u32),
        ("1.000000059604644775390625", 0x3f80_0000_u32),
        ("1.0000000596046447753906249999999999999999", 0x3f80_0000_u32),
        ("-1.0000000596046447753906250000000000000001", 0xbf80_0001_u32),
    ] {
        let value = BigDecimal::from_str(text).expect("valid decimal fixture");
        let actual = DataConverter::from(&value)
            .to_with::<f32>(&policy, &limits)
            .expect("finite rounded result");
        assert_eq!(actual.to_bits(), expected, "input: {text}");
    }

    let value =
        BigDecimal::from_str("1.00000000000000011102230246251565404236316680908203125000000000000000000000000000001")
            .expect("valid decimal fixture");
    let actual = DataConverter::from(&value)
        .to_with::<f64>(&policy, &limits)
        .expect("finite rounded result");
    assert_eq!(actual.to_bits(), 0x3ff0_0000_0000_0001_u64);
}

/// Checks nearest-even ties and their neighbors at normal and subnormal edges.
#[cfg(feature = "big-decimal")]
#[test]
fn test_decimal_f32_nearest_even_boundaries() {
    let policy = ConversionPolicy::lossy();
    let limits = ConversionLimits::default();
    let epsilon = BigDecimal::from_str("1e-40").expect("valid epsilon");

    for (lower_bits, upper_bits, epsilon) in [
        (0x3f80_0000_u32, 0x3f80_0001_u32, epsilon.clone()),
        (0x3f80_0001_u32, 0x3f80_0002_u32, epsilon),
        (0x0000_0000_u32, 0x0000_0001_u32, BigDecimal::from_str("1e-60").unwrap()),
        (0x007f_ffff_u32, 0x0080_0000_u32, BigDecimal::from_str("1e-60").unwrap()),
    ] {
        let lower = BigDecimal::from_f32(f32::from_bits(lower_bits)).expect("finite lower");
        let upper = BigDecimal::from_f32(f32::from_bits(upper_bits)).expect("finite upper");
        let midpoint = (&lower + &upper) * BigDecimal::new(5.into(), 1);
        let tie_bits = if lower_bits & 1 == 0 { lower_bits } else { upper_bits };
        for (value, expected) in [
            (&midpoint - &epsilon, lower_bits),
            (midpoint.clone(), tie_bits),
            (&midpoint + &epsilon, upper_bits),
            (-(&midpoint - &epsilon), lower_bits | 0x8000_0000),
            (-midpoint.clone(), tie_bits | 0x8000_0000),
            (-(&midpoint + &epsilon), upper_bits | 0x8000_0000),
        ] {
            let actual = DataConverter::from(&value)
                .to_with::<f32>(&policy, &limits)
                .expect("finite rounded result");
            assert_eq!(actual.to_bits(), expected, "{value} expected {expected:#x}");
        }
    }
}

/// Checks f64 nearest-even ties around one and the subnormal range.
#[cfg(feature = "big-decimal")]
#[test]
fn test_decimal_f64_nearest_even_boundaries() {
    let policy = ConversionPolicy::lossy();
    let limits = ConversionLimits::default();
    let epsilon = BigDecimal::from_str("1e-100").expect("valid epsilon");
    for (lower_bits, upper_bits, epsilon) in [
        (0x3ff0_0000_0000_0000_u64, 0x3ff0_0000_0000_0001_u64, epsilon.clone()),
        (0x3ff0_0000_0000_0001_u64, 0x3ff0_0000_0000_0002_u64, epsilon),
        (
            0x0000_0000_0000_0000_u64,
            0x0000_0000_0000_0001_u64,
            BigDecimal::from_str("1e-400").unwrap(),
        ),
        (
            0x000f_ffff_ffff_ffff_u64,
            0x0010_0000_0000_0000_u64,
            BigDecimal::from_str("1e-400").unwrap(),
        ),
    ] {
        let lower = BigDecimal::from_f64(f64::from_bits(lower_bits)).expect("finite lower");
        let upper = BigDecimal::from_f64(f64::from_bits(upper_bits)).expect("finite upper");
        let midpoint = (&lower + &upper) * BigDecimal::new(5.into(), 1);
        let tie_bits = if lower_bits & 1 == 0 { lower_bits } else { upper_bits };
        for (value, expected) in [
            (&midpoint - &epsilon, lower_bits),
            (midpoint.clone(), tie_bits),
            (&midpoint + &epsilon, upper_bits),
            (-(&midpoint - &epsilon), lower_bits | 0x8000_0000_0000_0000),
            (-midpoint.clone(), tie_bits | 0x8000_0000_0000_0000),
            (-(&midpoint + &epsilon), upper_bits | 0x8000_0000_0000_0000),
        ] {
            let actual = DataConverter::from(&value)
                .to_with::<f64>(&policy, &limits)
                .expect("finite rounded result");
            assert_eq!(actual.to_bits(), expected, "{value} expected {expected:#x}");
        }
    }
}

/// Checks BigDecimal exactness, range errors, contexts, and owned conversion.
#[cfg(feature = "big-decimal")]
#[test]
fn test_decimal_float_conversion_preserves_error_contracts() {
    let limits = ConversionLimits::default();
    let exact = BigDecimal::from_str("0.5").expect("valid exact decimal");
    assert_eq!(DataConverter::from(&exact).to::<f32>(), Ok(0.5));
    assert_eq!(DataConverter::from(exact.clone()).to::<f64>(), Ok(0.5));

    let inexact = BigDecimal::from_str("0.1").expect("valid inexact decimal");
    let error = DataConverter::from(&inexact)
        .to::<f32>()
        .expect_err("strict conversion rejects precision loss");
    assert_eq!(error.reason(), Some(&InvalidValueReason::PrecisionLoss));
    assert_eq!(error.from_type(), Some(DataType::BigDecimal));
    assert_eq!(error.to_type(), DataType::Float32);

    let overflow = BigDecimal::from_str("1e1000").expect("valid overflow decimal");
    let error = DataConverter::from(&overflow)
        .to_with::<f64>(&ConversionPolicy::lossy(), &limits)
        .expect_err("overflow is out of range");
    assert_eq!(error.reason(), Some(&InvalidValueReason::OutOfRange));
    assert_eq!(error.from_type(), Some(DataType::BigDecimal));
    assert_eq!(error.to_type(), DataType::Float64);

    for scale in [i64::MIN, i64::MAX] {
        let zero = BigDecimal::new(0.into(), scale);
        assert_eq!(DataConverter::from(&zero).to::<f64>(), Ok(0.0));
    }

    let extreme_overflow = BigDecimal::new(1.into(), i64::MIN);
    let error = DataConverter::from(&extreme_overflow)
        .to_with::<f64>(&ConversionPolicy::lossy(), &limits)
        .expect_err("extreme positive exponent is out of range");
    assert_eq!(error.reason(), Some(&InvalidValueReason::OutOfRange));

    let extreme_underflow = BigDecimal::new(1.into(), i64::MAX);
    assert_eq!(
        DataConverter::from(&extreme_underflow).to_with::<f64>(&ConversionPolicy::lossy(), &limits),
        Ok(0.0),
    );
    let error = DataConverter::from(&extreme_underflow)
        .to::<f64>()
        .expect_err("strict conversion rejects underflow precision loss");
    assert_eq!(error.reason(), Some(&InvalidValueReason::PrecisionLoss));
}
