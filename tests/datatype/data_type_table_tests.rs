// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Cross-checks the public consumers of the private data type mapping table.

use std::collections::HashMap;
use std::time::Duration;

use qubit_datatype::{
    DataConverter,
    DataType,
    DataTypeOf,
};

/// Asserts that compile-time and runtime source mappings agree.
fn assert_mapping<T>(value: T, expected: DataType)
where
    T: DataTypeOf,
    DataConverter<'static>: From<T>,
{
    assert_eq!(T::DATA_TYPE, expected);
    assert_eq!(DataConverter::<'static>::from(value).data_type(), expected);
}

/// Tests base mappings shared by `DataTypeOf` and `DataConverter`.
#[test]
fn test_data_type_mapping_base_consumers_agree() {
    assert_mapping(true, DataType::Bool);
    assert_mapping('x', DataType::Char);
    assert_mapping(1_i8, DataType::Int8);
    assert_mapping(1_i16, DataType::Int16);
    assert_mapping(1_i32, DataType::Int32);
    assert_mapping(1_i64, DataType::Int64);
    assert_mapping(1_i128, DataType::Int128);
    assert_mapping(1_u8, DataType::UInt8);
    assert_mapping(1_u16, DataType::UInt16);
    assert_mapping(1_u32, DataType::UInt32);
    assert_mapping(1_u64, DataType::UInt64);
    assert_mapping(1_u128, DataType::UInt128);
    assert_mapping(1.0_f32, DataType::Float32);
    assert_mapping(1.0_f64, DataType::Float64);
    assert_mapping(String::from("value"), DataType::String);
    assert_mapping(Duration::from_secs(1), DataType::Duration);

    let mut map = HashMap::new();
    map.insert(String::from("key"), String::from("value"));
    assert_mapping(map, DataType::StringMap);
}

/// Tests chrono mappings when the feature is enabled.
#[test]
#[cfg(feature = "chrono")]
fn test_data_type_mapping_chrono_consumers_agree() {
    use chrono::{
        DateTime,
        NaiveDate,
        NaiveDateTime,
        NaiveTime,
        Utc,
    };

    let date = NaiveDate::from_ymd_opt(2026, 7, 20)
        .expect("test date should be valid");
    let time =
        NaiveTime::from_hms_opt(12, 0, 0).expect("test time should be valid");
    let date_time = NaiveDateTime::new(date, time);
    let instant = DateTime::<Utc>::from_naive_utc_and_offset(date_time, Utc);

    assert_mapping(date, DataType::Date);
    assert_mapping(time, DataType::Time);
    assert_mapping(date_time, DataType::DateTime);
    assert_mapping(instant, DataType::Instant);
}

/// Tests arbitrary-precision integer mappings when the feature is enabled.
#[test]
#[cfg(feature = "big-integer")]
fn test_data_type_mapping_big_integer_consumers_agree() {
    assert_mapping(num_bigint::BigInt::from(1), DataType::BigInteger);
}

/// Tests arbitrary-precision decimal mappings when the feature is enabled.
#[test]
#[cfg(feature = "big-decimal")]
fn test_data_type_mapping_big_decimal_consumers_agree() {
    assert_mapping(bigdecimal::BigDecimal::from(1), DataType::BigDecimal);
}

/// Tests URL mappings when the feature is enabled.
#[test]
#[cfg(feature = "url")]
fn test_data_type_mapping_url_consumers_agree() {
    let url =
        url::Url::parse("https://example.com").expect("test URL should parse");
    assert_mapping(url, DataType::Url);
}

/// Tests JSON mappings when the feature is enabled.
#[test]
#[cfg(feature = "json")]
fn test_data_type_mapping_json_consumers_agree() {
    assert_mapping(serde_json::json!({"key": "value"}), DataType::Json);
}
