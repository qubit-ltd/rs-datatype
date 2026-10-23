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

use std::collections::HashMap;
use std::time::Duration;

#[cfg(feature = "big-decimal")]
use bigdecimal::BigDecimal;
#[cfg(feature = "chrono")]
use chrono::DateTime;
#[cfg(feature = "chrono")]
use chrono::NaiveDate;
#[cfg(feature = "chrono")]
use chrono::NaiveDateTime;
#[cfg(feature = "chrono")]
use chrono::NaiveTime;
#[cfg(feature = "chrono")]
use chrono::Utc;
#[cfg(feature = "big-integer")]
use num_bigint::BigInt;
use qubit_datatype::DataType;
use qubit_datatype::DataTypeOf;
#[cfg(feature = "json")]
use serde_json::Value;
#[cfg(feature = "url")]
use url::Url;

/// Test primitive and standard-library mappings without optional features.
#[test]
fn test_data_type_of_primitive_and_standard_types() {
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
    assert_eq!(NaiveDate::DATA_TYPE, DataType::Date);
    assert_eq!(NaiveTime::DATA_TYPE, DataType::Time);
    assert_eq!(NaiveDateTime::DATA_TYPE, DataType::DateTime);
    assert_eq!(DateTime::<Utc>::DATA_TYPE, DataType::Instant);
}

/// Test arbitrary-precision integer mapping when the feature is enabled.
#[test]
#[cfg(feature = "big-integer")]
fn test_data_type_of_big_integer_type() {
    assert_eq!(BigInt::DATA_TYPE, DataType::BigInteger);
}

/// Test arbitrary-precision decimal mapping when the feature is enabled.
#[test]
#[cfg(feature = "big-decimal")]
fn test_data_type_of_big_decimal_type() {
    assert_eq!(BigDecimal::DATA_TYPE, DataType::BigDecimal);
}

/// Test JSON mapping when the feature is enabled.
#[test]
#[cfg(feature = "json")]
fn test_data_type_of_json_type() {
    assert_eq!(Value::DATA_TYPE, DataType::Json);
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

    assert_eq!(data_type_of::<Url>(), DataType::Url);
}

#[test]
#[cfg(feature = "url")]
fn test_data_type_of_url_distinct_from_string() {
    assert_ne!(Url::DATA_TYPE, String::DATA_TYPE);
    assert_eq!(String::DATA_TYPE, DataType::String);
}

#[test]
#[cfg(feature = "url")]
fn test_data_type_of_url_inferred_from_value() {
    fn mapping_for<T: DataTypeOf>(_sample: &T) -> DataType {
        T::DATA_TYPE
    }

    let https = Url::parse("https://example.com/path?x=1#frag").expect("HTTPS URL fixture should parse");
    assert_eq!(mapping_for(&https), DataType::Url);

    let file = Url::parse("file:///tmp/a.txt").expect("file URL fixture should parse");
    assert_eq!(mapping_for(&file), DataType::Url);
}
