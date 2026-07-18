// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use std::cmp::Ordering;

use qubit_datatype::{
    NumericComparisonPolicy,
    NumericValueRef,
    compare_numeric,
};

#[cfg(feature = "big-number")]
use bigdecimal::BigDecimal;
#[cfg(feature = "big-number")]
use num_bigint::BigInt;

/// Asserts that two numeric references have the same exact value.
fn assert_exactly_equal(left: NumericValueRef<'_>, right: NumericValueRef<'_>) {
    assert_eq!(
        compare_numeric(left, right, NumericComparisonPolicy::Exact),
        Some(Ordering::Equal),
    );
}

/// Verifies that borrowed numeric values remain cheap to copy.
#[test]
fn test_numeric_value_ref_is_copy() {
    let value = NumericValueRef::from(u128::MAX);
    let copied = value;

    assert_exactly_equal(copied, NumericValueRef::from(u128::MAX));
    assert_exactly_equal(value, NumericValueRef::from(u128::MAX));
}

/// Verifies that callers can classify primitive NaN values directly.
#[test]
fn test_numeric_value_ref_reports_nan() {
    assert!(NumericValueRef::from(f32::NAN).is_nan());
    assert!(NumericValueRef::from(f64::NAN).is_nan());
    assert!(!NumericValueRef::from(f64::INFINITY).is_nan());
    assert!(!NumericValueRef::from(0_i32).is_nan());
}

/// Verifies conversion from every fixed-width primitive representation.
#[test]
fn test_numeric_value_ref_from_primitive_values() {
    assert_exactly_equal(
        NumericValueRef::from(1_i8),
        NumericValueRef::from(1_i128),
    );
    assert_exactly_equal(
        NumericValueRef::from(2_i16),
        NumericValueRef::from(2_i128),
    );
    assert_exactly_equal(
        NumericValueRef::from(3_i32),
        NumericValueRef::from(3_i128),
    );
    assert_exactly_equal(
        NumericValueRef::from(4_i64),
        NumericValueRef::from(4_i128),
    );
    assert_exactly_equal(
        NumericValueRef::from(5_i128),
        NumericValueRef::from(5_i128),
    );
    assert_exactly_equal(
        NumericValueRef::from(6_u8),
        NumericValueRef::from(6_u128),
    );
    assert_exactly_equal(
        NumericValueRef::from(7_u16),
        NumericValueRef::from(7_u128),
    );
    assert_exactly_equal(
        NumericValueRef::from(8_u32),
        NumericValueRef::from(8_u128),
    );
    assert_exactly_equal(
        NumericValueRef::from(9_u64),
        NumericValueRef::from(9_u128),
    );
    assert_exactly_equal(
        NumericValueRef::from(10_u128),
        NumericValueRef::from(10_u128),
    );
    assert_exactly_equal(
        NumericValueRef::from(11.5_f32),
        NumericValueRef::from(11.5_f64),
    );
    assert_exactly_equal(
        NumericValueRef::from(12.5_f64),
        NumericValueRef::from(12.5_f64),
    );
}

/// Verifies conversion from borrowed arbitrary-precision representations.
#[cfg(feature = "big-number")]
#[test]
fn test_numeric_value_ref_from_big_number_references() {
    let integer = BigInt::from(13);
    let decimal = BigDecimal::from(14);

    assert_exactly_equal(
        NumericValueRef::from(&integer),
        NumericValueRef::from(13_i8),
    );
    assert_exactly_equal(
        NumericValueRef::from(&decimal),
        NumericValueRef::from(14_i8),
    );
}
