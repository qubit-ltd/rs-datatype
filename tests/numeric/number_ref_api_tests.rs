// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0 (the "License");
// =============================================================================

use std::cmp::Ordering;

#[cfg(feature = "big-decimal")]
use bigdecimal::BigDecimal;
#[cfg(feature = "big-integer")]
use num_bigint::BigInt;
use qubit_datatype::{
    NumberRef,
    NumericComparisonPolicy,
};

/// Verifies classification and sign properties for fixed-width values.
#[test]
fn test_number_ref_fixed_width_properties() {
    let integer = NumberRef::from(42_i32);
    assert!(integer.is_integer());
    assert!(!integer.is_float());
    assert!(!integer.is_big_integer());
    assert!(!integer.is_big_decimal());
    assert!(!integer.is_nan());
    assert!(!integer.is_infinite());
    assert!(integer.is_finite());
    assert!(!integer.is_zero());
    assert!(integer.is_positive());
    assert!(!integer.is_negative());

    let zero = NumberRef::from(0_i32);
    assert!(zero.is_zero());
    assert!(!zero.is_positive());
    assert!(!zero.is_negative());

    let negative = NumberRef::from(-42_i32);
    assert!(negative.is_negative());
    assert!(!negative.is_positive());
}

/// Exercises every primitive representation through the public property API.
#[test]
fn test_number_ref_all_primitive_property_paths() {
    let values = [
        NumberRef::from(-1_i8),
        NumberRef::from(-1_i16),
        NumberRef::from(-1_i32),
        NumberRef::from(-1_i64),
        NumberRef::from(-1_i128),
        NumberRef::from(1_u8),
        NumberRef::from(1_u16),
        NumberRef::from(1_u32),
        NumberRef::from(1_u64),
        NumberRef::from(1_u128),
        NumberRef::from(1.0_f32),
        NumberRef::from(1.0_f64),
    ];

    for value in values {
        let _ = (
            value.is_integer(),
            value.is_float(),
            value.is_big_integer(),
            value.is_big_decimal(),
            value.is_nan(),
            value.is_infinite(),
            value.is_finite(),
            value.is_zero(),
            value.is_positive(),
            value.is_negative(),
        );
    }

    let zeros = [
        NumberRef::from(0_i8),
        NumberRef::from(0_i16),
        NumberRef::from(0_i32),
        NumberRef::from(0_i64),
        NumberRef::from(0_i128),
        NumberRef::from(0_u8),
        NumberRef::from(0_u16),
        NumberRef::from(0_u32),
        NumberRef::from(0_u64),
        NumberRef::from(0_u128),
        NumberRef::from(0.0_f32),
        NumberRef::from(0.0_f64),
    ];
    for value in zeros {
        assert!(value.is_zero());
        assert!(!value.is_positive());
        assert!(!value.is_negative());
    }
}

/// Verifies floating-point finiteness, sign, zero, and NaN properties.
#[test]
fn test_number_ref_float_properties() {
    let positive_infinity = NumberRef::from(f64::INFINITY);
    assert!(positive_infinity.is_float());
    assert!(positive_infinity.is_infinite());
    assert!(!positive_infinity.is_finite());
    assert!(positive_infinity.is_positive());
    assert!(!positive_infinity.is_negative());

    let negative_infinity = NumberRef::from(f64::NEG_INFINITY);
    assert!(negative_infinity.is_infinite());
    assert!(negative_infinity.is_negative());
    assert!(!negative_infinity.is_positive());

    let negative_zero = NumberRef::from(-0.0_f64);
    assert!(negative_zero.is_zero());
    assert!(!negative_zero.is_positive());
    assert!(!negative_zero.is_negative());

    let nan = NumberRef::from(f64::NAN);
    assert!(nan.is_nan());
    assert!(!nan.is_infinite());
    assert!(!nan.is_finite());
    assert!(!nan.is_positive());
    assert!(!nan.is_negative());
}

/// Verifies comparison is exposed as a method on the left operand.
#[test]
fn test_number_ref_compare() {
    let left = NumberRef::from(1_i32);
    let right = NumberRef::from(2_u64);
    assert_eq!(
        left.compare(right, NumericComparisonPolicy::Exact),
        Some(Ordering::Less),
    );
    assert_eq!(
        NumberRef::from(f64::NAN)
            .compare(right, NumericComparisonPolicy::Exact),
        None,
    );
}

/// Verifies arbitrary-precision integer properties when enabled.
#[cfg(feature = "big-integer")]
#[test]
fn test_number_ref_big_integer_properties() {
    let value = BigInt::from(7_i8);
    let number = NumberRef::from(&value);
    assert!(number.is_integer());
    assert!(number.is_big_integer());
    assert!(!number.is_big_decimal());
    assert!(number.is_finite());
    assert!(number.is_positive());
    assert!(!number.is_negative());
}

/// Verifies arbitrary-precision decimal properties when enabled.
#[cfg(feature = "big-decimal")]
#[test]
fn test_number_ref_big_decimal_properties() {
    let value = BigDecimal::from(-7_i8);
    let number = NumberRef::from(&value);
    assert!(!number.is_integer());
    assert!(!number.is_big_integer());
    assert!(number.is_big_decimal());
    assert!(number.is_finite());
    assert!(!number.is_positive());
    assert!(number.is_negative());
}
