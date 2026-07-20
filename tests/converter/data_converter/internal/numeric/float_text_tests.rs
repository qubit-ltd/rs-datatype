// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Floating-point text conversion regression tests.

use qubit_datatype::{
    DataConversionOptions, DataConverter, FloatRoundingPolicy, InvalidValueReason,
    NumericConversionOptions,
};

/// Returns the decimal digits of five raised to `exponent`.
///
/// # Parameters
///
/// * `exponent` - Non-negative exponent applied to five.
///
/// # Returns
///
/// The exact base-ten representation of the resulting integer.
fn decimal_power_of_five(exponent: usize) -> String {
    let mut digits = vec![1_u8];
    for _ in 0..exponent {
        let mut carry = 0_u8;
        for digit in &mut digits {
            let product = *digit * 5 + carry;
            *digit = product % 10;
            carry = product / 10;
        }
        if carry != 0 {
            digits.push(carry);
        }
    }
    digits
        .iter()
        .rev()
        .map(|digit| char::from(b'0' + digit))
        .collect()
}

/// Verifies that lossy text-to-f32 conversion rounds directly at target width.
#[test]
fn test_lossy_text_to_f32_avoids_double_rounding() {
    let options = DataConversionOptions::lossy();
    let converted = DataConverter::from("1.0000000596046448")
        .to_with::<f32>(&options)
        .expect("finite decimal text should convert lossily to f32");
    assert_eq!(converted.to_bits(), 0x3f80_0001);
}

/// Verifies text-to-float rounding can be enabled independently.
#[test]
fn test_text_to_float_rounding_is_independent() {
    let options = DataConversionOptions::strict().with_numeric_options(
        NumericConversionOptions::strict().with_text_to_float(FloatRoundingPolicy::NearestEven),
    );

    assert_eq!(
        DataConverter::from("0.1").to_with::<f32>(&options),
        Ok(0.1_f32),
    );
    assert!(
        DataConverter::from(16_777_217_u32)
            .to_with::<f32>(&options)
            .is_err()
    );
    assert!(DataConverter::from("3.9").to_with::<i32>(&options).is_err());
}

/// Verifies the environment profile relaxes only textual float parsing.
#[test]
fn test_env_friendly_numeric_profile_relaxes_only_text_float() {
    let options = DataConversionOptions::env_friendly();

    assert_eq!(
        DataConverter::from("0.1").to_with::<f32>(&options),
        Ok(0.1_f32),
    );
    assert!(
        DataConverter::from(16_777_217_u32)
            .to_with::<f32>(&options)
            .is_err()
    );
    assert!(DataConverter::from("3.9").to_with::<i32>(&options).is_err());
}

/// Verifies that redundant decimal zeros do not defeat exact conversion.
#[test]
fn test_exact_float_text_normalizes_redundant_zeros() {
    let source = DataConverter::from("1.000000000000000000000000000000000000000");
    assert_eq!(
        source
            .to::<f32>()
            .expect("redundant zeros should remain exact for f32"),
        1.0,
    );
    assert_eq!(
        source
            .to::<f64>()
            .expect("redundant zeros should remain exact for f64"),
        1.0,
    );
}

/// Verifies exact f64 text conversion is not limited by `u128` coefficients.
#[test]
fn test_exact_float_text_accepts_power_of_two_beyond_u128() {
    let value = "340282366920938463463374607431768211456";

    assert_eq!(DataConverter::from(value).to::<f64>(), Ok(2_f64.powi(128)));
}

/// Verifies the exact decimal expansion of the minimum f32 subnormal converts.
#[test]
fn test_exact_float_text_accepts_minimum_f32_subnormal() {
    let value = format!("{}e-149", decimal_power_of_five(149));

    assert_eq!(
        DataConverter::from(value.as_str()).to::<f32>(),
        Ok(f32::from_bits(1)),
    );
}

/// Verifies the exact decimal expansion of the minimum f64 subnormal converts.
#[test]
fn test_exact_float_text_accepts_minimum_f64_subnormal() {
    let value = format!("{}e-1074", decimal_power_of_five(1074));

    assert_eq!(
        DataConverter::from(value.as_str()).to::<f64>(),
        Ok(f64::from_bits(1)),
    );
}

/// Verifies a half-minimum f64 subnormal remains a precision failure.
#[test]
fn test_exact_float_text_rejects_half_minimum_f64_subnormal() {
    let value = format!("{}e-1075", decimal_power_of_five(1075));

    assert!(matches!(
        DataConverter::from(value.as_str()).to::<f64>(),
        Err(conversion_error) if matches!(conversion_error.reason(), Some(InvalidValueReason::PrecisionLoss))
    ));
}

/// Verifies zero remains exact even when its exponent exceeds integer bounds.
#[test]
fn test_exact_float_text_accepts_zero_with_unbounded_exponent() {
    assert_eq!(
        DataConverter::from("0e999999999999999999999999").to::<f64>(),
        Ok(0.0),
    );
    assert_eq!(
        DataConverter::from("-0e-999999999999999999999999").to::<f32>(),
        Ok(-0.0),
    );
}

/// Verifies f32 text conversion distinguishes range and precision failures.
#[test]
fn test_text_to_f32_classifies_range_and_precision_failures() {
    assert!(matches!(
        DataConverter::from("1e1000").to::<f32>(),
        Err(conversion_error) if matches!(conversion_error.reason(), Some(InvalidValueReason::OutOfRange)
    )));
    assert!(matches!(
        DataConverter::from("0.1").to::<f32>(),
        Err(conversion_error) if matches!(conversion_error.reason(), Some(InvalidValueReason::PrecisionLoss)
    )));
}
