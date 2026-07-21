// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Fixed-width integer conversion tests.

use proptest::{
    prop_assert_eq,
    proptest,
};
use qubit_datatype::{
    DataConversionOptions,
    DataConverter,
    FloatRoundingPolicy,
    InvalidValueReason,
    NumericConversionOptions,
};

/// Verifies exact and lossy integer-to-f32 conversion at the mantissa boundary.
#[test]
fn test_integer_to_f32_checks_target_mantissa() {
    let source = DataConverter::from(16_777_217_u32);
    assert!(source.to::<f32>().is_err());
    assert_eq!(
        source.to_with::<f32>(&DataConversionOptions::lossy()),
        Ok(16_777_216.0),
    );
}

/// Verifies negative integer-to-f32 conversion preserves the sign while
/// applying the target mantissa policy.
#[test]
fn test_negative_integer_to_f32_checks_target_mantissa() {
    let source = DataConverter::from(-16_777_217_i32);
    assert!(matches!(
        source.to::<f32>(),
        Err(error)
            if matches!(
                error.reason(),
                Some(InvalidValueReason::PrecisionLoss)
            )
    ));
    assert_eq!(
        source.to_with::<f32>(&DataConversionOptions::lossy()),
        Ok(-16_777_216.0),
    );
}

/// Verifies integer-to-f32 overflow is rejected before lossy rounding can be
/// applied.
#[test]
fn test_integer_to_f32_rejects_overflow() {
    let error = DataConverter::from(u128::MAX)
        .to_with::<f32>(&DataConversionOptions::lossy())
        .expect_err("u128::MAX must overflow f32");

    assert!(matches!(
        error.reason(),
        Some(InvalidValueReason::OutOfRange)
    ));
}

/// Verifies numeric-to-float rounding can be enabled independently.
#[test]
fn test_numeric_to_float_rounding_is_independent() {
    let options = DataConversionOptions::strict().with_numeric_options(
        NumericConversionOptions::strict()
            .with_numeric_to_float(FloatRoundingPolicy::NearestEven),
    );

    assert_eq!(
        DataConverter::from(16_777_217_u32).to_with::<f32>(&options),
        Ok(16_777_216.0),
    );
    assert!(DataConverter::from("0.1").to_with::<f32>(&options).is_err());
    assert!(DataConverter::from("3.9").to_with::<i32>(&options).is_err());
}

/// Verifies float sources preserve exact representable integer values.
#[test]
fn test_float_to_integer_preserves_exact_binary_integer() {
    assert_eq!(
        DataConverter::from(2_f64.powi(100)).to::<u128>(),
        Ok(1_u128 << 100),
    );
    assert_eq!(
        DataConverter::from(2_f32.powi(100)).to::<u128>(),
        Ok(1_u128 << 100),
    );
    assert_eq!(
        DataConverter::from(-2_f64.powi(100)).to::<i128>(),
        Ok(-(1_i128 << 100)),
    );
    assert_eq!(
        DataConverter::from(-2_f64.powi(127)).to::<i128>(),
        Ok(i128::MIN),
    );
    assert!(DataConverter::from(2_f64.powi(128)).to::<u128>().is_err());
}

proptest! {
    /// Verifies every representable integer power of two reaches `u128` exactly.
    #[test]
    fn test_float_to_integer_preserves_representable_powers_of_two(
        exponent in 0_i32..128,
    ) {
        let expected = 1_u128 << exponent;
        prop_assert_eq!(
            DataConverter::from(2_f64.powi(exponent)).to::<u128>(),
            Ok(expected),
        );
    }
}
