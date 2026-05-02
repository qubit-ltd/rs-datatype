/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # DataConverter Unit Tests
//!
//! Tests for reusable data conversion behavior.
//!
//! # Author
//!
//! Haixing Hu

use std::collections::HashMap;
use std::str::FromStr;
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
use qubit_datatype::DataType;
use qubit_datatype::converter::{
    DataConversionError,
    DataConversionOptions,
    DataConverter,
    DurationConversionOptions,
    DurationUnit,
};
use url::Url;

/// Asserts that a converter reports the expected data type.
fn assert_data_type(converter: DataConverter<'_>, expected: DataType) {
    assert_eq!(converter.data_type(), expected);
}

/// Creates a very large BigInt for overflow-oriented tests.
fn create_huge_bigint() -> BigInt {
    BigInt::parse_bytes(b"10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000", 10)
        .expect("huge BigInt literal should parse")
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

    let date = NaiveDate::from_ymd_opt(2026, 5, 1).expect("test date should be valid");
    let time = NaiveTime::from_hms_opt(12, 30, 45).expect("test time should be valid");
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
    assert_data_type(DataConverter::from(big_int.clone()), DataType::BigInteger);
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

/// Test conversions from borrowed and owned string values.
#[test]
fn test_data_converter_from_string_converts_to_numbers_and_bool() {
    let port: u16 = DataConverter::from("8080")
        .to()
        .expect("string port should convert to u16");
    assert_eq!(port, 8080);

    let enabled: bool = DataConverter::from(" TRUE ")
        .to()
        .expect("string true should convert to bool");
    assert!(enabled);

    let disabled: bool = DataConverter::from(String::from("0"))
        .to()
        .expect("owned string zero should convert to bool");
    assert!(!disabled);

    let negative: i32 = DataConverter::from("-42")
        .to()
        .expect("negative string should convert to i32");
    assert_eq!(negative, -42);
}

/// Test bool target conversions for all supported source variants.
#[test]
fn test_data_converter_bool_target_accepts_supported_sources() {
    assert!(
        DataConverter::from(true)
            .to::<bool>()
            .expect("bool should convert to bool")
    );
    assert!(
        DataConverter::from(1i8)
            .to::<bool>()
            .expect("i8 should convert to bool")
    );
    assert!(
        !DataConverter::from(0i16)
            .to::<bool>()
            .expect("i16 should convert to bool")
    );
    assert!(
        DataConverter::from(1i32)
            .to::<bool>()
            .expect("i32 should convert to bool")
    );
    assert!(
        !DataConverter::from(0i64)
            .to::<bool>()
            .expect("i64 should convert to bool")
    );
    assert!(
        DataConverter::from(1i128)
            .to::<bool>()
            .expect("i128 should convert to bool")
    );
    assert!(
        DataConverter::from(1u8)
            .to::<bool>()
            .expect("u8 should convert to bool")
    );
    assert!(
        !DataConverter::from(0u16)
            .to::<bool>()
            .expect("u16 should convert to bool")
    );
    assert!(
        DataConverter::from(1u32)
            .to::<bool>()
            .expect("u32 should convert to bool")
    );
    assert!(
        !DataConverter::from(0u64)
            .to::<bool>()
            .expect("u64 should convert to bool")
    );
    assert!(
        DataConverter::from(1u128)
            .to::<bool>()
            .expect("u128 should convert to bool")
    );
    assert!(
        !DataConverter::from("false")
            .to::<bool>()
            .expect("false string should convert to bool")
    );

    assert!(matches!(
        DataConverter::from("maybe").to::<bool>(),
        Err(DataConversionError::ConversionError(_))
    ));
    assert!(matches!(
        DataConverter::Empty(DataType::Bool).to::<bool>(),
        Err(DataConversionError::NoValue)
    ));
}

/// Test char target conversion behavior.
#[test]
fn test_data_converter_char_target_conversions() {
    assert_eq!(
        DataConverter::from('z')
            .to::<char>()
            .expect("char should convert to char"),
        'z'
    );
    assert!(matches!(
        DataConverter::Empty(DataType::Char).to::<char>(),
        Err(DataConversionError::NoValue)
    ));
    assert!(matches!(
        DataConverter::from("z").to::<char>(),
        Err(DataConversionError::ConversionFailed { .. })
    ));
}

/// Test signed integer targets across all supported source variants.
#[test]
fn test_data_converter_signed_integer_targets_accept_supported_sources() {
    let big_int = BigInt::from(20);
    let big_decimal = BigDecimal::from(21);

    let cases = [
        (DataConverter::from(true), 1i128),
        (DataConverter::from('A'), 65),
        (DataConverter::from(-1i8), -1),
        (DataConverter::from(2i16), 2),
        (DataConverter::from(3i32), 3),
        (DataConverter::from(4i64), 4),
        (DataConverter::from(5i128), 5),
        (DataConverter::from(6isize), 6),
        (DataConverter::from(7u8), 7),
        (DataConverter::from(8u16), 8),
        (DataConverter::from(9u32), 9),
        (DataConverter::from(10u64), 10),
        (DataConverter::from(11u128), 11),
        (DataConverter::from(12usize), 12),
        (DataConverter::from(13.9f32), 13),
        (DataConverter::from(14.9f64), 14),
        (DataConverter::from("15"), 15),
        (DataConverter::from(&big_int), 20),
        (DataConverter::from(&big_decimal), 21),
    ];
    for (source, expected) in cases {
        assert_eq!(
            source.to::<i128>().expect("source should convert to i128"),
            expected
        );
    }

    assert_eq!(
        DataConverter::from(120i16)
            .to::<i8>()
            .expect("i16 should convert to i8"),
        120
    );
    assert_eq!(
        DataConverter::from(32_000i32)
            .to::<i16>()
            .expect("i32 should convert to i16"),
        32_000
    );
    assert_eq!(
        DataConverter::from(1_000_000i64)
            .to::<i32>()
            .expect("i64 should convert to i32"),
        1_000_000
    );
    assert_eq!(
        DataConverter::from(1_000_000i128)
            .to::<isize>()
            .expect("i128 should convert to isize"),
        1_000_000isize
    );

    assert!(matches!(
        DataConverter::Empty(DataType::Int128).to::<i128>(),
        Err(DataConversionError::NoValue)
    ));
    assert!(matches!(
        DataConverter::from("bad").to::<i128>(),
        Err(DataConversionError::ConversionError(_))
    ));
    assert!(matches!(
        DataConverter::from(f64::MAX).to::<i128>(),
        Err(DataConversionError::ConversionError(_))
    ));
    let huge = create_huge_bigint();
    let huge_decimal = BigDecimal::from(huge.clone());
    assert!(matches!(
        DataConverter::from(&huge).to::<i128>(),
        Err(DataConversionError::ConversionError(_))
    ));
    assert!(matches!(
        DataConverter::from(&huge_decimal).to::<i128>(),
        Err(DataConversionError::ConversionError(_))
    ));
    assert!(matches!(
        DataConverter::from(NaiveDate::from_ymd_opt(2026, 5, 1).expect("test date")).to::<i128>(),
        Err(DataConversionError::ConversionFailed { .. })
    ));
}

/// Test unsigned integer targets across all supported source variants.
#[test]
fn test_data_converter_unsigned_integer_targets_accept_supported_sources() {
    let cases = [
        (DataConverter::from(true), 1u128),
        (DataConverter::from('A'), 65),
        (DataConverter::from(1i8), 1),
        (DataConverter::from(2i16), 2),
        (DataConverter::from(3i32), 3),
        (DataConverter::from(4i64), 4),
        (DataConverter::from(5i128), 5),
        (DataConverter::from(6isize), 6),
        (DataConverter::from(7u8), 7),
        (DataConverter::from(8u16), 8),
        (DataConverter::from(9u32), 9),
        (DataConverter::from(10u64), 10),
        (DataConverter::from(11u128), 11),
        (DataConverter::from(12usize), 12),
        (DataConverter::from("13"), 13),
    ];
    for (source, expected) in cases {
        assert_eq!(
            source.to::<u128>().expect("source should convert to u128"),
            expected
        );
    }

    assert_eq!(
        DataConverter::from(250u16)
            .to::<u8>()
            .expect("u16 should convert to u8"),
        250
    );
    assert_eq!(
        DataConverter::from(65_000u32)
            .to::<u16>()
            .expect("u32 should convert to u16"),
        65_000
    );
    assert_eq!(
        DataConverter::from(1_000_000u64)
            .to::<u32>()
            .expect("u64 should convert to u32"),
        1_000_000
    );
    assert_eq!(
        DataConverter::from(1_000_000u128)
            .to::<u64>()
            .expect("u128 should convert to u64"),
        1_000_000
    );
    assert_eq!(
        DataConverter::from(1_000usize)
            .to::<usize>()
            .expect("usize should convert to usize"),
        1_000
    );

    assert!(matches!(
        DataConverter::Empty(DataType::UInt128).to::<u128>(),
        Err(DataConversionError::NoValue)
    ));
    assert!(matches!(
        DataConverter::from("bad").to::<u128>(),
        Err(DataConversionError::ConversionError(_))
    ));
    assert!(matches!(
        DataConverter::from(1.0f64).to::<u128>(),
        Err(DataConversionError::ConversionFailed { .. })
    ));
}

/// Test string target conversion for every supported source variant.
#[test]
fn test_data_converter_string_target_accepts_all_value_sources() {
    let date = NaiveDate::from_ymd_opt(2026, 5, 1).expect("test date should be valid");
    let time = NaiveTime::from_hms_opt(12, 30, 45).expect("test time should be valid");
    let datetime = NaiveDateTime::new(date, time);
    let instant = DateTime::<Utc>::from_naive_utc_and_offset(datetime, Utc);
    let big_int = BigInt::from(18);
    let big_decimal = BigDecimal::from_str("19.5").expect("test BigDecimal should parse");
    let url = Url::parse("https://example.com").expect("test URL should parse");
    let mut map = HashMap::new();
    map.insert("k".to_string(), "v".to_string());
    let json = serde_json::json!({"k": "v"});

    let sources = [
        DataConverter::from("text"),
        DataConverter::from(true),
        DataConverter::from('A'),
        DataConverter::from(1i8),
        DataConverter::from(2i16),
        DataConverter::from(3i32),
        DataConverter::from(4i64),
        DataConverter::from(5i128),
        DataConverter::from(6u8),
        DataConverter::from(7u16),
        DataConverter::from(8u32),
        DataConverter::from(9u64),
        DataConverter::from(10u128),
        DataConverter::from(11isize),
        DataConverter::from(12usize),
        DataConverter::from(13.5f32),
        DataConverter::from(14.5f64),
        DataConverter::from(&big_int),
        DataConverter::from(&big_decimal),
        DataConverter::from(date),
        DataConverter::from(time),
        DataConverter::from(datetime),
        DataConverter::from(instant),
        DataConverter::from(Duration::new(0, 20)),
        DataConverter::from(&url),
        DataConverter::from(&map),
        DataConverter::from(&json),
    ];

    for source in sources {
        assert!(
            !source
                .to::<String>()
                .expect("source should convert to string")
                .is_empty()
        );
    }

    assert!(matches!(
        DataConverter::Empty(DataType::String).to::<String>(),
        Err(DataConversionError::NoValue)
    ));
}

/// Test floating point target conversions across supported source variants.
#[test]
fn test_data_converter_float_targets_accept_supported_sources() {
    let big_int = BigInt::from(20);
    let big_decimal = BigDecimal::from_str("21.5").expect("test BigDecimal should parse");

    let f32_sources = [
        DataConverter::from(1.0f32),
        DataConverter::from(2.0f64),
        DataConverter::from(true),
        DataConverter::from('A'),
        DataConverter::from(3i8),
        DataConverter::from(4i16),
        DataConverter::from(5i32),
        DataConverter::from(6i64),
        DataConverter::from(7i128),
        DataConverter::from(8isize),
        DataConverter::from(9u8),
        DataConverter::from(10u16),
        DataConverter::from(11u32),
        DataConverter::from(12u64),
        DataConverter::from(13u128),
        DataConverter::from(14usize),
        DataConverter::from("15.5"),
        DataConverter::from(&big_int),
        DataConverter::from(&big_decimal),
    ];
    for source in f32_sources {
        assert!(
            source
                .to::<f32>()
                .expect("source should convert to f32")
                .is_finite()
        );
    }

    let f64_sources = [
        DataConverter::from(1.0f64),
        DataConverter::from(2.0f32),
        DataConverter::from(false),
        DataConverter::from('B'),
        DataConverter::from(3i8),
        DataConverter::from(4i16),
        DataConverter::from(5i32),
        DataConverter::from(6i64),
        DataConverter::from(7i128),
        DataConverter::from(8isize),
        DataConverter::from(9u8),
        DataConverter::from(10u16),
        DataConverter::from(11u32),
        DataConverter::from(12u64),
        DataConverter::from(13u128),
        DataConverter::from(14usize),
        DataConverter::from("15.5"),
        DataConverter::from(&big_int),
        DataConverter::from(&big_decimal),
    ];
    for source in f64_sources {
        assert!(
            source
                .to::<f64>()
                .expect("source should convert to f64")
                .is_finite()
        );
    }

    assert!(
        DataConverter::from(f64::INFINITY)
            .to::<f32>()
            .expect("infinity should cast to f32")
            .is_infinite()
    );
    assert!(matches!(
        DataConverter::from("bad").to::<f32>(),
        Err(DataConversionError::ConversionError(_))
    ));
    assert!(matches!(
        DataConverter::from("bad").to::<f64>(),
        Err(DataConversionError::ConversionError(_))
    ));
    assert!(matches!(
        DataConverter::Empty(DataType::Float32).to::<f32>(),
        Err(DataConversionError::NoValue)
    ));
    assert!(matches!(
        DataConverter::Empty(DataType::Float64).to::<f64>(),
        Err(DataConversionError::NoValue)
    ));
    assert!(matches!(
        DataConverter::from(Duration::new(1, 0)).to::<f32>(),
        Err(DataConversionError::ConversionFailed { .. })
    ));
    assert!(matches!(
        DataConverter::from(Duration::new(1, 0)).to::<f64>(),
        Err(DataConversionError::ConversionFailed { .. })
    ));

    let huge = create_huge_bigint();
    let huge_decimal = BigDecimal::from(huge.clone());
    assert!(DataConverter::from(&huge).to::<f32>().is_err());
    assert!(DataConverter::from(&huge).to::<f64>().is_err());
    assert!(DataConverter::from(&huge_decimal).to::<f32>().is_err());
    assert!(DataConverter::from(&huge_decimal).to::<f64>().is_err());
}

/// Test direct strict conversions for non-numeric target types.
#[test]
fn test_data_converter_strict_targets_cover_success_and_errors() {
    let date = NaiveDate::from_ymd_opt(2026, 5, 1).expect("test date should be valid");
    let time = NaiveTime::from_hms_opt(12, 30, 45).expect("test time should be valid");
    let datetime = NaiveDateTime::new(date, time);
    let instant = DateTime::<Utc>::from_naive_utc_and_offset(datetime, Utc);
    let big_int = BigInt::from(18);
    let big_decimal = BigDecimal::from(19);
    let mut map = HashMap::new();
    map.insert("k".to_string(), "v".to_string());

    assert_eq!(DataConverter::from(date).to::<NaiveDate>().unwrap(), date);
    assert_eq!(DataConverter::from(time).to::<NaiveTime>().unwrap(), time);
    assert_eq!(
        DataConverter::from(datetime).to::<NaiveDateTime>().unwrap(),
        datetime
    );
    assert_eq!(
        DataConverter::from(instant).to::<DateTime<Utc>>().unwrap(),
        instant
    );
    assert_eq!(
        DataConverter::from(&big_int).to::<BigInt>().unwrap(),
        big_int
    );
    assert_eq!(
        DataConverter::from(&big_decimal)
            .to::<BigDecimal>()
            .unwrap(),
        big_decimal
    );
    assert_eq!(
        DataConverter::from(&map)
            .to::<HashMap<String, String>>()
            .unwrap(),
        map
    );

    assert!(matches!(
        DataConverter::Empty(DataType::Date).to::<NaiveDate>(),
        Err(DataConversionError::NoValue)
    ));
    assert!(matches!(
        DataConverter::from("2026-05-01").to::<NaiveDate>(),
        Err(DataConversionError::ConversionFailed { .. })
    ));
    assert!(matches!(
        DataConverter::Empty(DataType::BigInteger).to::<BigInt>(),
        Err(DataConversionError::NoValue)
    ));
    assert!(matches!(
        DataConverter::from(1i32).to::<BigInt>(),
        Err(DataConversionError::ConversionFailed { .. })
    ));
    assert!(matches!(
        DataConverter::Empty(DataType::Time).to::<NaiveTime>(),
        Err(DataConversionError::NoValue)
    ));
    assert!(matches!(
        DataConverter::from(1i32).to::<NaiveTime>(),
        Err(DataConversionError::ConversionFailed { .. })
    ));
    assert!(matches!(
        DataConverter::Empty(DataType::DateTime).to::<NaiveDateTime>(),
        Err(DataConversionError::NoValue)
    ));
    assert!(matches!(
        DataConverter::from(1i32).to::<NaiveDateTime>(),
        Err(DataConversionError::ConversionFailed { .. })
    ));
    assert!(matches!(
        DataConverter::Empty(DataType::Instant).to::<DateTime<Utc>>(),
        Err(DataConversionError::NoValue)
    ));
    assert!(matches!(
        DataConverter::from(1i32).to::<DateTime<Utc>>(),
        Err(DataConversionError::ConversionFailed { .. })
    ));
    assert!(matches!(
        DataConverter::Empty(DataType::BigDecimal).to::<BigDecimal>(),
        Err(DataConversionError::NoValue)
    ));
    assert!(matches!(
        DataConverter::from(1i32).to::<BigDecimal>(),
        Err(DataConversionError::ConversionFailed { .. })
    ));
    assert!(matches!(
        DataConverter::Empty(DataType::StringMap).to::<HashMap<String, String>>(),
        Err(DataConversionError::NoValue)
    ));
    assert!(matches!(
        DataConverter::from(1i32).to::<HashMap<String, String>>(),
        Err(DataConversionError::ConversionFailed { .. })
    ));
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

/// Test integer conversion range checks across signed and unsigned targets.
#[test]
fn test_data_converter_numeric_conversions_check_integer_bounds() {
    let signed: i16 = DataConverter::from(255u8)
        .to()
        .expect("u8 should convert to i16");
    assert_eq!(signed, 255);

    assert!(matches!(
        DataConverter::from(-1i8).to::<u16>(),
        Err(DataConversionError::ConversionError(_))
    ));

    assert!(matches!(
        DataConverter::from(u16::MAX as u32 + 1).to::<u16>(),
        Err(DataConversionError::ConversionError(_))
    ));

    assert!(matches!(
        DataConverter::from(i128::MAX).to::<i64>(),
        Err(DataConversionError::ConversionError(_))
    ));
}

/// Test floating point conversion edge cases for integer and f32 targets.
#[test]
fn test_data_converter_float_conversions_check_non_finite_and_overflow() {
    let truncated: i8 = DataConverter::from(-12.9f64)
        .to()
        .expect("finite f64 should truncate when converting to i8");
    assert_eq!(truncated, -12);

    assert!(matches!(
        DataConverter::from(f64::NAN).to::<i64>(),
        Err(DataConversionError::ConversionError(_))
    ));

    assert!(matches!(
        DataConverter::from(f64::INFINITY).to::<i64>(),
        Err(DataConversionError::ConversionError(_))
    ));

    assert!(matches!(
        DataConverter::from(f64::MAX).to::<f32>(),
        Err(DataConversionError::ConversionError(_))
    ));

    assert!(matches!(
        DataConverter::from(u128::MAX).to::<f32>(),
        Err(DataConversionError::ConversionError(_))
    ));
}

/// Test BigInt and BigDecimal conversions and range failures.
#[test]
fn test_data_converter_big_number_conversions_check_range() {
    let big_int = BigInt::from(i64::MAX);
    let converted: i64 = DataConverter::from(&big_int)
        .to()
        .expect("in-range BigInt should convert to i64");
    assert_eq!(converted, i64::MAX);

    let huge_int = BigInt::parse_bytes(b"999999999999999999999999999999999999", 10)
        .expect("test BigInt literal should parse");
    assert!(matches!(
        DataConverter::from(&huge_int).to::<i64>(),
        Err(DataConversionError::ConversionError(_))
    ));

    let decimal = BigDecimal::from_str("123.75").expect("test BigDecimal should parse");
    let converted: f64 = DataConverter::from(&decimal)
        .to()
        .expect("BigDecimal should convert to f64");
    assert!((converted - 123.75).abs() < f64::EPSILON);
}

/// Test Duration string formatting and parsing.
#[test]
fn test_data_converter_duration_string_conversion() {
    let duration = Duration::new(1, 500);
    let text: String = DataConverter::from(duration)
        .to()
        .expect("Duration should convert to string");
    assert_eq!(text, "1000ms");

    let direct: Duration = DataConverter::from(duration)
        .to()
        .expect("Duration should convert to Duration");
    assert_eq!(direct, duration);

    let parsed: Duration = DataConverter::from("1000000500ns")
        .to()
        .expect("duration string should parse");
    assert_eq!(parsed, duration);

    let millis: Duration = DataConverter::from("10ms")
        .to()
        .expect("duration string with milliseconds should parse");
    assert_eq!(millis, Duration::from_millis(10));

    let seconds: Duration = DataConverter::from("2s")
        .to()
        .expect("duration string with seconds should parse");
    assert_eq!(seconds, Duration::from_secs(2));

    let minutes: Duration = DataConverter::from("2m")
        .to()
        .expect("duration string with minutes should parse");
    assert_eq!(minutes, Duration::from_secs(120));

    let hours: Duration = DataConverter::from("2h")
        .to()
        .expect("duration string with hours should parse");
    assert_eq!(hours, Duration::from_secs(7200));

    let days: Duration = DataConverter::from("2d")
        .to()
        .expect("duration string with days should parse");
    assert_eq!(days, Duration::from_secs(172800));

    for text in ["10us", "10µs", "10μs"] {
        let parsed: Duration = DataConverter::from(text)
            .to()
            .expect("duration string with microseconds should parse");
        assert_eq!(parsed, Duration::from_micros(10));
    }

    let bare_default: Duration = DataConverter::from("10")
        .to()
        .expect("bare duration string should use default milliseconds");
    assert_eq!(bare_default, Duration::from_millis(10));

    let options = DataConversionOptions::default().with_duration_options(
        DurationConversionOptions::default().with_unit(DurationUnit::Seconds),
    );
    let bare_seconds: Duration = DataConverter::from("10")
        .to_with(&options)
        .expect("bare duration string should use configured seconds");
    assert_eq!(bare_seconds, Duration::from_secs(10));

    let no_suffix = DataConversionOptions::default().with_duration_options(
        DurationConversionOptions::default()
            .with_unit(DurationUnit::Seconds)
            .with_append_unit_suffix(false),
    );
    let text: String = DataConverter::from(Duration::from_millis(1500))
        .to_with(&no_suffix)
        .expect("Duration should convert to suffixless rounded seconds");
    assert_eq!(text, "2");

    assert!(matches!(
        DataConverter::from("badns").to::<Duration>(),
        Err(DataConversionError::ConversionError(_))
    ));
    assert!(matches!(
        DataConverter::from("").to::<Duration>(),
        Err(DataConversionError::ConversionError(_))
    ));
    assert!(matches!(
        DataConverter::from("10fortnights").to::<Duration>(),
        Err(DataConversionError::ConversionError(_))
    ));
    let overflowing_duration = format!("{}ns", (u64::MAX as u128 + 1) * 1_000_000_000);
    assert!(matches!(
        DataConverter::from(overflowing_duration).to::<Duration>(),
        Err(DataConversionError::ConversionError(_))
    ));
    let overflowing_days = format!("{}d", u64::MAX / (24 * 60 * 60) + 1);
    assert!(matches!(
        DataConverter::from(overflowing_days).to::<Duration>(),
        Err(DataConversionError::ConversionError(_))
    ));
    assert!(matches!(
        DataConverter::Empty(DataType::Duration).to::<Duration>(),
        Err(DataConversionError::NoValue)
    ));
    assert!(matches!(
        DataConverter::from(1i32).to::<Duration>(),
        Ok(duration) if duration == Duration::from_millis(1)
    ));
}

/// Test Duration conversions with integer sources and targets.
#[test]
fn test_data_converter_duration_integer_conversion_uses_configured_unit() {
    let duration: Duration = DataConverter::from(1500u64)
        .to()
        .expect("integer duration should use default milliseconds");
    assert_eq!(duration, Duration::from_millis(1500));

    let options = DataConversionOptions::default().with_duration_options(
        DurationConversionOptions::default().with_unit(DurationUnit::Seconds),
    );
    let duration: Duration = DataConverter::from(2u64)
        .to_with(&options)
        .expect("integer duration should use configured seconds");
    assert_eq!(duration, Duration::from_secs(2));

    let integer_sources = [
        DataConverter::from(1i8),
        DataConverter::from(1i16),
        DataConverter::from(1i64),
        DataConverter::from(1i128),
        DataConverter::from(1isize),
        DataConverter::from(1u8),
        DataConverter::from(1u16),
        DataConverter::from(1u32),
        DataConverter::from(1u128),
        DataConverter::from(1usize),
    ];
    for source in integer_sources {
        let duration: Duration = source
            .to_with(&options)
            .expect("integer source should convert with configured seconds");
        assert_eq!(duration, Duration::from_secs(1));
    }

    let big_integer = BigInt::from(3u8);
    let duration: Duration = DataConverter::from(&big_integer)
        .to_with(&options)
        .expect("BigInteger should convert to Duration");
    assert_eq!(duration, Duration::from_secs(3));

    let units: u64 = DataConverter::from(Duration::from_millis(1499))
        .to_with(&options)
        .expect("Duration should round to configured integer unit");
    assert_eq!(units, 1);

    let units: u64 = DataConverter::from(Duration::from_millis(1500))
        .to_with(&options)
        .expect("Duration should round half up to configured integer unit");
    assert_eq!(units, 2);

    let signed_units: i64 = DataConverter::from(Duration::from_millis(1500))
        .to_with(&options)
        .expect("Duration should convert to signed integer units");
    assert_eq!(signed_units, 2);

    assert!(matches!(
        DataConverter::from(-1i32).to::<Duration>(),
        Err(DataConversionError::ConversionError(_))
    ));
    assert!(matches!(
        DataConverter::from(u128::from(u64::MAX) + 1).to::<Duration>(),
        Err(DataConversionError::ConversionError(_))
    ));

    let overflowing_options = DataConversionOptions::default()
        .with_duration_options(DurationConversionOptions::default().with_unit(DurationUnit::Days));
    assert!(matches!(
        DataConverter::from(u64::MAX).to_with::<Duration>(&overflowing_options),
        Err(DataConversionError::ConversionError(_))
    ));
    assert!(matches!(
        DataConverter::from(i128::from(u64::MAX)).to_with::<Duration>(&overflowing_options),
        Err(DataConversionError::ConversionError(_))
    ));

    let negative_big_integer = BigInt::from(-1);
    assert!(matches!(
        DataConverter::from(&negative_big_integer).to::<Duration>(),
        Err(DataConversionError::ConversionError(_))
    ));
    let overflowing_big_integer = BigInt::from(u64::MAX);
    assert!(matches!(
        DataConverter::from(&overflowing_big_integer).to_with::<Duration>(&overflowing_options),
        Err(DataConversionError::ConversionError(_))
    ));
    assert!(matches!(
        DataConverter::from(NaiveDate::from_ymd_opt(2026, 5, 1).expect("test date"))
            .to::<Duration>(),
        Err(DataConversionError::ConversionFailed { .. })
    ));
}

/// Test URL and JSON conversion behavior.
#[test]
fn test_data_converter_url_and_json_conversions() {
    let url: Url = DataConverter::from("https://example.com/path")
        .to()
        .expect("URL string should parse");
    assert_eq!(url.as_str(), "https://example.com/path");

    let direct_url: Url = DataConverter::from(&url)
        .to()
        .expect("URL should convert to URL");
    assert_eq!(direct_url, url);

    assert!(matches!(
        DataConverter::from("not a url").to::<Url>(),
        Err(DataConversionError::ConversionError(_))
    ));
    assert!(matches!(
        DataConverter::Empty(DataType::Url).to::<Url>(),
        Err(DataConversionError::NoValue)
    ));
    assert!(matches!(
        DataConverter::from(1i32).to::<Url>(),
        Err(DataConversionError::ConversionFailed { .. })
    ));

    let json: serde_json::Value = DataConverter::from(r#"{"answer":42}"#)
        .to()
        .expect("JSON string should parse");
    assert_eq!(json["answer"], 42);

    let direct_json: serde_json::Value = DataConverter::from(&json)
        .to()
        .expect("JSON should convert to JSON");
    assert_eq!(direct_json["answer"], 42);

    let mut map = HashMap::new();
    map.insert("answer".to_string(), "42".to_string());
    let json: serde_json::Value = DataConverter::from(&map)
        .to()
        .expect("string map should convert to JSON object");
    assert_eq!(json["answer"], "42");

    let text: String = DataConverter::from(json)
        .to()
        .expect("JSON value should serialize to string");
    assert_eq!(text, r#"{"answer":"42"}"#);

    assert!(matches!(
        DataConverter::from("{").to::<serde_json::Value>(),
        Err(DataConversionError::JsonDeserializationError(_))
    ));
    assert!(matches!(
        DataConverter::Empty(DataType::Json).to::<serde_json::Value>(),
        Err(DataConversionError::NoValue)
    ));
    assert!(matches!(
        DataConverter::from(1i32).to::<serde_json::Value>(),
        Err(DataConversionError::ConversionFailed { .. })
    ));
}

/// Test empty values and unsupported source-target pairs.
#[test]
fn test_data_converter_empty_and_unsupported_errors_include_types() {
    assert!(matches!(
        DataConverter::Empty(DataType::Int32).to::<i32>(),
        Err(DataConversionError::NoValue)
    ));

    match DataConverter::from('x').to::<bool>() {
        Err(DataConversionError::ConversionFailed { from, to }) => {
            assert_eq!(from, DataType::Char);
            assert_eq!(to, DataType::Bool);
        }
        other => panic!("expected char to bool conversion failure, got {other:?}"),
    }
}

/// Test temporal and complex conversions with strict target behavior.
#[test]
fn test_data_converter_temporal_and_complex_conversions() {
    let date = NaiveDate::from_ymd_opt(2026, 5, 1).expect("test date should be valid");
    let time = NaiveTime::from_hms_opt(12, 30, 45).expect("test time should be valid");
    let datetime = NaiveDateTime::new(date, time);
    let instant = DateTime::<Utc>::from_naive_utc_and_offset(datetime, Utc);

    assert_eq!(
        DataConverter::from(date)
            .to::<NaiveDate>()
            .expect("date should convert strictly"),
        date
    );
    assert_eq!(
        DataConverter::from(time)
            .to::<NaiveTime>()
            .expect("time should convert strictly"),
        time
    );
    assert_eq!(
        DataConverter::from(datetime)
            .to::<NaiveDateTime>()
            .expect("datetime should convert strictly"),
        datetime
    );
    assert_eq!(
        DataConverter::from(instant)
            .to::<DateTime<Utc>>()
            .expect("instant should convert strictly"),
        instant
    );
    assert_eq!(
        DataConverter::from(date)
            .to::<String>()
            .expect("date should convert to string"),
        "2026-05-01"
    );
}
