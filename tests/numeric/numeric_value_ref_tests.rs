// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_datatype::NumericValueRef;

#[cfg(feature = "big-number")]
use bigdecimal::BigDecimal;
#[cfg(feature = "big-number")]
use num_bigint::BigInt;

/// Verifies that borrowed numeric values remain cheap to copy.
#[test]
fn test_numeric_value_ref_is_copy() {
    let value = NumericValueRef::UInt128(u128::MAX);
    let copied = value;
    assert!(matches!(copied, NumericValueRef::UInt128(u128::MAX)));
    assert!(matches!(value, NumericValueRef::UInt128(u128::MAX)));
}

/// Verifies that callers can classify primitive NaN values directly.
#[test]
fn test_numeric_value_ref_reports_nan() {
    assert!(NumericValueRef::Float32(f32::NAN).is_nan());
    assert!(NumericValueRef::Float64(f64::NAN).is_nan());
    assert!(!NumericValueRef::Float64(f64::INFINITY).is_nan());
    assert!(!NumericValueRef::Int32(0).is_nan());
}

/// Verifies conversion from every fixed-width primitive representation.
#[test]
fn test_numeric_value_ref_from_primitive_values() {
    assert!(matches!(
        NumericValueRef::from(1_i8),
        NumericValueRef::Int8(1)
    ));
    assert!(matches!(
        NumericValueRef::from(2_i16),
        NumericValueRef::Int16(2)
    ));
    assert!(matches!(
        NumericValueRef::from(3_i32),
        NumericValueRef::Int32(3)
    ));
    assert!(matches!(
        NumericValueRef::from(4_i64),
        NumericValueRef::Int64(4)
    ));
    assert!(matches!(
        NumericValueRef::from(5_i128),
        NumericValueRef::Int128(5)
    ));
    assert!(matches!(
        NumericValueRef::from(6_u8),
        NumericValueRef::UInt8(6)
    ));
    assert!(matches!(
        NumericValueRef::from(7_u16),
        NumericValueRef::UInt16(7)
    ));
    assert!(matches!(
        NumericValueRef::from(8_u32),
        NumericValueRef::UInt32(8)
    ));
    assert!(matches!(
        NumericValueRef::from(9_u64),
        NumericValueRef::UInt64(9)
    ));
    assert!(matches!(
        NumericValueRef::from(10_u128),
        NumericValueRef::UInt128(10)
    ));
    assert!(matches!(
        NumericValueRef::from(11.5_f32),
        NumericValueRef::Float32(value) if value == 11.5,
    ));
    assert!(matches!(
        NumericValueRef::from(12.5_f64),
        NumericValueRef::Float64(value) if value == 12.5,
    ));
}

/// Verifies conversion from borrowed arbitrary-precision representations.
#[cfg(feature = "big-number")]
#[test]
fn test_numeric_value_ref_from_big_number_references() {
    let integer = BigInt::from(13);
    let decimal = BigDecimal::from(14);

    assert!(matches!(
        NumericValueRef::from(&integer),
        NumericValueRef::BigInteger(value) if value == &integer,
    ));
    assert!(matches!(
        NumericValueRef::from(&decimal),
        NumericValueRef::BigDecimal(value) if value == &decimal,
    ));
}
