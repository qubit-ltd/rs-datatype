// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # DataTypeOf Unit Tests
//!
//! Tests for compile-time Rust type to `DataType` mapping.

use qubit_datatype::{DataType, DataTypeOf};

/// Test primitive and standard-library mappings without optional features.
#[test]
fn test_data_type_of_primitive_and_standard_types() {
    use std::collections::HashMap;
    use std::time::Duration;

    assert_eq!(bool::DATA_TYPE, DataType::Bool);
    assert_eq!(char::DATA_TYPE, DataType::Char);
    assert_eq!(i8::DATA_TYPE, DataType::Int8);
    assert_eq!(i16::DATA_TYPE, DataType::Int16);
    assert_eq!(i32::DATA_TYPE, DataType::Int32);
    assert_eq!(i64::DATA_TYPE, DataType::Int64);
    assert_eq!(i128::DATA_TYPE, DataType::Int128);
    assert_eq!(u8::DATA_TYPE, DataType::UInt8);
    assert_eq!(u16::DATA_TYPE, DataType::UInt16);
    assert_eq!(u32::DATA_TYPE, DataType::UInt32);
    assert_eq!(u64::DATA_TYPE, DataType::UInt64);
    assert_eq!(u128::DATA_TYPE, DataType::UInt128);
    assert_eq!(f32::DATA_TYPE, DataType::Float32);
    assert_eq!(f64::DATA_TYPE, DataType::Float64);
    assert_eq!(String::DATA_TYPE, DataType::String);
    assert_eq!(Duration::DATA_TYPE, DataType::Duration);
    assert_eq!(HashMap::<String, String>::DATA_TYPE, DataType::StringMap,);
}

/// Test chrono mappings when the feature is enabled.
#[test]
#[cfg(feature = "chrono")]
fn test_data_type_of_chrono_types() {
    use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};

    assert_eq!(NaiveDate::DATA_TYPE, DataType::Date);
    assert_eq!(NaiveTime::DATA_TYPE, DataType::Time);
    assert_eq!(NaiveDateTime::DATA_TYPE, DataType::DateTime);
    assert_eq!(DateTime::<Utc>::DATA_TYPE, DataType::Instant);
}

/// Test arbitrary-precision integer mapping when the feature is enabled.
#[test]
#[cfg(feature = "big-integer")]
fn test_data_type_of_big_integer_type() {
    assert_eq!(num_bigint::BigInt::DATA_TYPE, DataType::BigInteger);
}

/// Test arbitrary-precision decimal mapping when the feature is enabled.
#[test]
#[cfg(feature = "big-decimal")]
fn test_data_type_of_big_decimal_type() {
    assert_eq!(bigdecimal::BigDecimal::DATA_TYPE, DataType::BigDecimal);
}

/// Test DataTypeOf for v0.4.0 new types
#[test]
#[cfg(all(feature = "url", feature = "json"))]
fn test_data_type_of_v040_types() {
    use std::time::Duration;

    assert_eq!(Duration::DATA_TYPE, DataType::Duration);
    assert_eq!(url::Url::DATA_TYPE, DataType::Url);
    assert_eq!(serde_json::Value::DATA_TYPE, DataType::Json);
}

// ============================================================================
// url::Url + DataTypeOf（crate 内绑定约定）
// ============================================================================

#[test]
#[cfg(feature = "url")]
fn test_data_type_of_url_generic() {
    fn data_type_of<T: DataTypeOf>() -> DataType {
        T::DATA_TYPE
    }

    assert_eq!(data_type_of::<url::Url>(), DataType::Url);
}

#[test]
#[cfg(feature = "url")]
fn test_data_type_of_url_distinct_from_string() {
    assert_ne!(url::Url::DATA_TYPE, String::DATA_TYPE);
    assert_eq!(String::DATA_TYPE, DataType::String);
}

#[test]
#[cfg(feature = "url")]
fn test_data_type_of_url_inferred_from_value() {
    fn mapping_for<T: DataTypeOf>(_sample: &T) -> DataType {
        T::DATA_TYPE
    }

    let https = url::Url::parse("https://example.com/path?x=1#frag").unwrap();
    assert_eq!(mapping_for(&https), DataType::Url);

    let file = url::Url::parse("file:///tmp/a.txt").unwrap();
    assert_eq!(mapping_for(&file), DataType::Url);
}
