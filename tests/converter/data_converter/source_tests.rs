// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
// =============================================================================
//! Source construction and public wrapper tests.

use std::collections::HashMap;
use std::time::Duration;

use bigdecimal::BigDecimal;
use chrono::{
    DateTime,
    NaiveDate,
    NaiveDateTime,
    NaiveTime,
    Utc,
};
use num_bigint::BigInt;
use qubit_datatype::{
    DataConversionError,
    DataConverter,
    DataType,
};
use url::Url;

/// Asserts that a converter reports the expected data type.
fn assert_data_type(converter: DataConverter<'_>, expected: DataType) {
    assert_eq!(converter.data_type(), expected);
}

/// Test every supported `From` implementation and all `data_type()` branches.
#[test]
fn test_data_converter_from_impls_cover_all_sources() {
    let bool_value = true;
    assert_data_type(DataConverter::from(bool_value), DataType::Bool);
    assert_data_type(DataConverter::from(&bool_value), DataType::Bool);

    let char_value = 'A';
    assert_data_type(DataConverter::from(char_value), DataType::Char);
    assert_data_type(DataConverter::from(&char_value), DataType::Char);

    let int8_value = 1i8;
    let int16_value = 2i16;
    let int32_value = 3i32;
    let int64_value = 4i64;
    let int128_value = 5i128;
    assert_data_type(DataConverter::from(int8_value), DataType::Int8);
    assert_data_type(DataConverter::from(&int8_value), DataType::Int8);
    assert_data_type(DataConverter::from(int16_value), DataType::Int16);
    assert_data_type(DataConverter::from(&int16_value), DataType::Int16);
    assert_data_type(DataConverter::from(int32_value), DataType::Int32);
    assert_data_type(DataConverter::from(&int32_value), DataType::Int32);
    assert_data_type(DataConverter::from(int64_value), DataType::Int64);
    assert_data_type(DataConverter::from(&int64_value), DataType::Int64);
    assert_data_type(DataConverter::from(int128_value), DataType::Int128);
    assert_data_type(DataConverter::from(&int128_value), DataType::Int128);

    let uint8_value = 6u8;
    let uint16_value = 7u16;
    let uint32_value = 8u32;
    let uint64_value = 9u64;
    let uint128_value = 10u128;
    assert_data_type(DataConverter::from(uint8_value), DataType::UInt8);
    assert_data_type(DataConverter::from(&uint8_value), DataType::UInt8);
    assert_data_type(DataConverter::from(uint16_value), DataType::UInt16);
    assert_data_type(DataConverter::from(&uint16_value), DataType::UInt16);
    assert_data_type(DataConverter::from(uint32_value), DataType::UInt32);
    assert_data_type(DataConverter::from(&uint32_value), DataType::UInt32);
    assert_data_type(DataConverter::from(uint64_value), DataType::UInt64);
    assert_data_type(DataConverter::from(&uint64_value), DataType::UInt64);
    assert_data_type(DataConverter::from(uint128_value), DataType::UInt128);
    assert_data_type(DataConverter::from(&uint128_value), DataType::UInt128);

    let intsize_value = 11isize;
    let uintsize_value = 12usize;
    let float32_value = 13.0f32;
    let float64_value = 14.0f64;
    assert_data_type(DataConverter::from(intsize_value), DataType::IntSize);
    assert_data_type(DataConverter::from(&intsize_value), DataType::IntSize);
    assert_data_type(DataConverter::from(uintsize_value), DataType::UIntSize);
    assert_data_type(DataConverter::from(&uintsize_value), DataType::UIntSize);
    assert_data_type(DataConverter::from(float32_value), DataType::Float32);
    assert_data_type(DataConverter::from(&float32_value), DataType::Float32);
    assert_data_type(DataConverter::from(float64_value), DataType::Float64);
    assert_data_type(DataConverter::from(&float64_value), DataType::Float64);

    let date =
        NaiveDate::from_ymd_opt(2026, 5, 1).expect("test date should be valid");
    let time =
        NaiveTime::from_hms_opt(12, 30, 45).expect("test time should be valid");
    let datetime = NaiveDateTime::new(date, time);
    let instant = DateTime::<Utc>::from_naive_utc_and_offset(datetime, Utc);
    let duration = Duration::new(1, 2);
    assert_data_type(DataConverter::from(date), DataType::Date);
    assert_data_type(DataConverter::from(&date), DataType::Date);
    assert_data_type(DataConverter::from(time), DataType::Time);
    assert_data_type(DataConverter::from(&time), DataType::Time);
    assert_data_type(DataConverter::from(datetime), DataType::DateTime);
    assert_data_type(DataConverter::from(&datetime), DataType::DateTime);
    assert_data_type(DataConverter::from(instant), DataType::Instant);
    assert_data_type(DataConverter::from(&instant), DataType::Instant);
    assert_data_type(DataConverter::from(duration), DataType::Duration);
    assert_data_type(DataConverter::from(&duration), DataType::Duration);

    let big_int = BigInt::from(15);
    let big_decimal = BigDecimal::from(16);
    let url = Url::parse("https://example.com").expect("test URL should parse");
    let mut map = HashMap::new();
    map.insert("k".to_string(), "v".to_string());
    let json = serde_json::json!({"k": "v"});
    assert_data_type(
        DataConverter::from(big_int.clone()),
        DataType::BigInteger,
    );
    assert_data_type(DataConverter::from(&big_int), DataType::BigInteger);
    assert_data_type(
        DataConverter::from(big_decimal.clone()),
        DataType::BigDecimal,
    );
    assert_data_type(DataConverter::from(&big_decimal), DataType::BigDecimal);
    assert_data_type(DataConverter::from(url.clone()), DataType::Url);
    assert_data_type(DataConverter::from(&url), DataType::Url);
    assert_data_type(DataConverter::from(map.clone()), DataType::StringMap);
    assert_data_type(DataConverter::from(&map), DataType::StringMap);
    assert_data_type(DataConverter::from(json.clone()), DataType::Json);
    assert_data_type(DataConverter::from(&json), DataType::Json);

    let string = String::from("17");
    assert_data_type(DataConverter::from("17"), DataType::String);
    assert_data_type(DataConverter::from(&string), DataType::String);
    assert_data_type(DataConverter::from(string), DataType::String);

    assert_data_type(DataConverter::Empty(DataType::Json), DataType::Json);
}

/// Test derived converter traits used by callers and assertions.
#[test]
fn test_data_converter_derived_traits() {
    let converter = DataConverter::from("42");
    let cloned = converter.clone();
    assert_eq!(cloned, converter);
    assert!(format!("{:?}", cloned).contains("String"));
}

/// Test that borrowed and owned strings preserve string data type metadata.
#[test]
fn test_data_converter_string_sources_report_string_data_type() {
    let owned = DataConverter::from(String::from("15"));
    assert_eq!(owned.data_type(), DataType::String);
    assert_eq!(
        owned.to::<u8>().expect("owned string should convert to u8"),
        15
    );

    let borrowed_source = String::from("16");
    let borrowed = DataConverter::from(&borrowed_source);
    assert_eq!(borrowed.data_type(), DataType::String);
    assert_eq!(
        borrowed
            .to::<u8>()
            .expect("borrowed String should convert to u8"),
        16
    );
}

/// Test empty values and unsupported source-target pairs.
#[test]
fn test_data_converter_empty_and_unsupported_errors_include_types() {
    assert!(matches!(
        DataConverter::Empty(DataType::Int32).to::<i32>(),
        Err(DataConversionError::Missing {
            from: DataType::Int32,
            to: DataType::Int32,
        })
    ));

    match DataConverter::from('x').to::<bool>() {
        Err(DataConversionError::Unsupported { from, to }) => {
            assert_eq!(from, DataType::Char);
            assert_eq!(to, DataType::Bool);
        }
        other => {
            panic!("expected char to bool conversion failure, got {other:?}")
        }
    }
}
