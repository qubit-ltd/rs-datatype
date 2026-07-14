// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
// =============================================================================
//! Textual and temporal conversion tests.

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
use qubit_datatype::{
    DataConversionError,
    DataConversionErrorKind,
    DataConverter,
    DataType,
};
use url::Url;

/// Assert an invalid-syntax error with exact source and target types.
fn assert_invalid_syntax<T>(
    result: Result<T, DataConversionError>,
    to: DataType,
    expected: &'static str,
) {
    let matches_expected = matches!(
        &result,
        Err(DataConversionError::Invalid {
            from: DataType::String,
            to: actual_to,
            kind: DataConversionErrorKind::InvalidSyntax {
                expected: actual_expected,
            },
        }) if *actual_to == to && *actual_expected == expected
    );
    assert!(matches_expected, "unexpected result: {:?}", result.err());
}

/// Test the canonical textual formats for rich target types.
#[test]
fn test_data_converter_rich_targets_use_canonical_text_formats() {
    assert_eq!(
        DataConverter::from("汉")
            .to::<char>()
            .expect("one Unicode scalar should parse"),
        '汉',
    );
    assert_eq!(
        DataConverter::from("2026-07-12")
            .to::<NaiveDate>()
            .expect("canonical date should parse"),
        NaiveDate::from_ymd_opt(2026, 7, 12)
            .expect("test date should be valid"),
    );
    assert_eq!(
        DataConverter::from("10:11:12.123456789")
            .to::<NaiveTime>()
            .expect("canonical time should parse"),
        NaiveTime::from_hms_nano_opt(10, 11, 12, 123_456_789)
            .expect("test time should be valid"),
    );
    assert_eq!(
        DataConverter::from("2026-07-12T10:11:12")
            .to::<NaiveDateTime>()
            .expect("canonical local datetime should parse"),
        NaiveDate::from_ymd_opt(2026, 7, 12)
            .expect("test date should be valid")
            .and_hms_opt(10, 11, 12)
            .expect("test datetime should be valid"),
    );
    assert_eq!(
        DataConverter::from("2026-07-12T18:11:12+08:00")
            .to::<DateTime<Utc>>()
            .expect("RFC 3339 instant should parse")
            .to_rfc3339(),
        "2026-07-12T10:11:12+00:00",
    );
    assert_eq!(
        DataConverter::from("+12345678901234567890")
            .to::<BigInt>()
            .expect("signed decimal BigInt should parse"),
        BigInt::from_str("12345678901234567890")
            .expect("expected BigInt should parse"),
    );
    assert_eq!(
        DataConverter::from("1.25e3")
            .to::<BigDecimal>()
            .expect("exponent BigDecimal should parse"),
        BigDecimal::from(1_250),
    );
    assert_eq!(
        DataConverter::from("https://example.com/path")
            .to::<Url>()
            .expect("absolute URL should parse"),
        Url::parse("https://example.com/path")
            .expect("expected URL should parse"),
    );
    assert_eq!(
        DataConverter::from("[1,true,null]")
            .to::<serde_json::Value>()
            .expect("any JSON value should parse"),
        serde_json::json!([1, true, null]),
    );
    assert_eq!(
        DataConverter::from(r#"{"key":"value"}"#)
            .to::<HashMap<String, String>>()
            .expect("string map JSON should parse")
            .get("key"),
        Some(&"value".to_string()),
    );
    assert_eq!(
        DataConverter::from("1s")
            .to::<Duration>()
            .expect("duration with suffix should parse"),
        Duration::from_secs(1),
    );
}

/// Test parser-internal rejection branches for canonical temporal formats.
#[test]
fn test_data_converter_temporal_parsers_reject_invalid_components() {
    for value in ["2026-7-12", "2026-13-40"] {
        assert!(DataConverter::from(value).to::<NaiveDate>().is_err());
    }
    for value in ["1:02:03", "01:02:03.", "01:02:03.a", "25:00:00"] {
        assert!(DataConverter::from(value).to::<NaiveTime>().is_err());
    }
    for value in [
        "2026-13-40T01:02:03",
        "2026-07-12T25:00:00",
        "2026-07-12T01:02:03.a",
    ] {
        assert!(DataConverter::from(value).to::<NaiveDateTime>().is_err());
    }
}

/// Test that trim is applied exactly according to string options.
#[test]
fn test_data_converter_default_does_not_trim_any_text_parser() {
    assert!(matches!(
        DataConverter::from(" true ").to::<bool>(),
        Err(DataConversionError::Invalid {
            from: DataType::String,
            to: DataType::Bool,
            kind: DataConversionErrorKind::InvalidBoolean,
        }),
    ));
    assert_invalid_syntax(
        DataConverter::from(" 1 ").to::<i32>(),
        DataType::Int32,
        "integer",
    );
    assert_invalid_syntax(
        DataConverter::from(" 1s ").to::<Duration>(),
        DataType::Duration,
        "[0-9]+(ns|us|ms|s|m|h|d)?",
    );
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
        Err(DataConversionError::Missing {
            from: DataType::Char,
            to: DataType::Char,
        })
    ));
    assert_eq!(DataConverter::from("z").to::<char>(), Ok('z'));
    assert!(matches!(
        DataConverter::from(1i32).to::<char>(),
        Err(DataConversionError::Unsupported { .. })
    ));
}

/// Test string target conversion for every supported source variant.
#[test]
fn test_data_converter_string_target_accepts_all_value_sources() {
    let date =
        NaiveDate::from_ymd_opt(2026, 5, 1).expect("test date should be valid");
    let time =
        NaiveTime::from_hms_opt(12, 30, 45).expect("test time should be valid");
    let datetime = NaiveDateTime::new(date, time);
    let instant = DateTime::<Utc>::from_naive_utc_and_offset(datetime, Utc);
    let big_int = BigInt::from(18);
    let big_decimal =
        BigDecimal::from_str("19.5").expect("test BigDecimal should parse");
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
        DataConverter::from(Duration::from_millis(1)),
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
        Err(DataConversionError::Missing { .. })
    ));
}

/// Test direct strict conversions for non-numeric target types.
#[test]
fn test_data_converter_strict_targets_cover_success_and_errors() {
    let date =
        NaiveDate::from_ymd_opt(2026, 5, 1).expect("test date should be valid");
    let time =
        NaiveTime::from_hms_opt(12, 30, 45).expect("test time should be valid");
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
        Err(DataConversionError::Missing { .. })
    ));
    assert_eq!(
        DataConverter::from("2026-05-01").to::<NaiveDate>(),
        Ok(date)
    );
    assert!(matches!(
        DataConverter::Empty(DataType::BigInteger).to::<BigInt>(),
        Err(DataConversionError::Missing { .. })
    ));
    assert_eq!(
        DataConverter::from(1i32).to::<BigInt>(),
        Ok(BigInt::from(1))
    );
    assert!(matches!(
        DataConverter::Empty(DataType::Time).to::<NaiveTime>(),
        Err(DataConversionError::Missing { .. })
    ));
    assert!(matches!(
        DataConverter::from(1i32).to::<NaiveTime>(),
        Err(DataConversionError::Unsupported { .. })
    ));
    assert!(matches!(
        DataConverter::Empty(DataType::DateTime).to::<NaiveDateTime>(),
        Err(DataConversionError::Missing { .. })
    ));
    assert!(matches!(
        DataConverter::from(1i32).to::<NaiveDateTime>(),
        Err(DataConversionError::Unsupported { .. })
    ));
    assert!(matches!(
        DataConverter::Empty(DataType::Instant).to::<DateTime<Utc>>(),
        Err(DataConversionError::Missing { .. })
    ));
    assert!(matches!(
        DataConverter::from(1i32).to::<DateTime<Utc>>(),
        Err(DataConversionError::Unsupported { .. })
    ));
    assert!(matches!(
        DataConverter::Empty(DataType::BigDecimal).to::<BigDecimal>(),
        Err(DataConversionError::Missing { .. })
    ));
    assert_eq!(
        DataConverter::from(1i32).to::<BigDecimal>(),
        Ok(BigDecimal::from(1)),
    );
    assert!(matches!(
        DataConverter::Empty(DataType::StringMap)
            .to::<HashMap<String, String>>(),
        Err(DataConversionError::Missing { .. })
    ));
    assert!(matches!(
        DataConverter::from(1i32).to::<HashMap<String, String>>(),
        Err(DataConversionError::Unsupported { .. })
    ));
}

/// Test temporal and complex conversions with strict target behavior.
#[test]
fn test_data_converter_temporal_and_complex_conversions() {
    let date =
        NaiveDate::from_ymd_opt(2026, 5, 1).expect("test date should be valid");
    let time =
        NaiveTime::from_hms_opt(12, 30, 45).expect("test time should be valid");
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
