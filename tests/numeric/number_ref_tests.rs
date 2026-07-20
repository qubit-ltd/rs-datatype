// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

mod api_tests;
mod comparison_tests;
mod exact_decimal_tests;

use std::cmp::Ordering;

use qubit_datatype::{NumberRef, NumericComparisonPolicy};

#[cfg(feature = "big-number")]
use bigdecimal::BigDecimal;
#[cfg(feature = "big-number")]
use num_bigint::BigInt;

/// Asserts that two number references have the same exact value.
fn assert_exactly_equal(left: NumberRef<'_>, right: NumberRef<'_>) {
    assert_eq!(
        left.compare(right, NumericComparisonPolicy::Exact),
        Some(Ordering::Equal),
    );
}

/// Verifies that borrowed numeric values remain cheap to copy.
#[test]
fn test_number_ref_is_copy() {
    let value = NumberRef::from(u128::MAX);
    let copied = value;

    assert_exactly_equal(copied, NumberRef::from(u128::MAX));
    assert_exactly_equal(value, NumberRef::from(u128::MAX));
}

/// Verifies that callers can classify primitive NaN values directly.
#[test]
fn test_number_ref_reports_nan() {
    assert!(NumberRef::from(f32::NAN).is_nan());
    assert!(NumberRef::from(f64::NAN).is_nan());
    assert!(!NumberRef::from(f64::INFINITY).is_nan());
    assert!(!NumberRef::from(0_i32).is_nan());
}

/// Verifies conversion from every fixed-width primitive representation.
#[test]
fn test_number_ref_from_primitive_values() {
    assert_exactly_equal(NumberRef::from(1_i8), NumberRef::from(1_i128));
    assert_exactly_equal(NumberRef::from(2_i16), NumberRef::from(2_i128));
    assert_exactly_equal(NumberRef::from(3_i32), NumberRef::from(3_i128));
    assert_exactly_equal(NumberRef::from(4_i64), NumberRef::from(4_i128));
    assert_exactly_equal(NumberRef::from(5_i128), NumberRef::from(5_i128));
    assert_exactly_equal(NumberRef::from(6_u8), NumberRef::from(6_u128));
    assert_exactly_equal(NumberRef::from(7_u16), NumberRef::from(7_u128));
    assert_exactly_equal(NumberRef::from(8_u32), NumberRef::from(8_u128));
    assert_exactly_equal(NumberRef::from(9_u64), NumberRef::from(9_u128));
    assert_exactly_equal(NumberRef::from(10_u128), NumberRef::from(10_u128));
    assert_exactly_equal(NumberRef::from(11.5_f32), NumberRef::from(11.5_f64));
    assert_exactly_equal(NumberRef::from(12.5_f64), NumberRef::from(12.5_f64));
}

/// Verifies conversion from borrowed arbitrary-precision representations.
#[cfg(feature = "big-number")]
#[test]
fn test_number_ref_from_big_number_references() {
    let integer = BigInt::from(13);
    let decimal = BigDecimal::from(14);

    assert_exactly_equal(NumberRef::from(&integer), NumberRef::from(13_i8));
    assert_exactly_equal(NumberRef::from(&decimal), NumberRef::from(14_i8));
}
