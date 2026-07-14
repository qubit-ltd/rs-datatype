// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
// =============================================================================
//! Structured conversion tests.

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
    DataConversionErrorKind,
    DataConverter,
    DataFormat,
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

/// Assert an invalid error with exact source type, target type, and kind.
fn assert_invalid_kind<T>(
    result: Result<T, DataConversionError>,
    to: DataType,
    expected_kind: DataConversionErrorKind,
) {
    assert!(matches!(
        result,
        Err(DataConversionError::Invalid {
            from: DataType::String,
            to: actual_to,
            kind,
        }) if actual_to == to && kind == expected_kind,
    ));
}

/// Test rejection boundaries for every canonical rich textual format.
#[test]
fn test_data_converter_rich_targets_reject_noncanonical_text() {
    for value in ["", "ab"] {
        assert_invalid_syntax(
            DataConverter::from(value).to::<char>(),
            DataType::Char,
            "one Unicode scalar value",
        );
    }
    assert_invalid_syntax(
        DataConverter::from("2026/07/12").to::<NaiveDate>(),
        DataType::Date,
        "YYYY-MM-DD",
    );
    assert_invalid_syntax(
        DataConverter::from("10:11:12.1234567890").to::<NaiveTime>(),
        DataType::Time,
        "HH:MM:SS[.fraction]",
    );
    assert_invalid_syntax(
        DataConverter::from("2026-07-12 10:11:12").to::<NaiveDateTime>(),
        DataType::DateTime,
        "YYYY-MM-DDTHH:MM:SS[.fraction]",
    );
    assert_invalid_syntax(
        DataConverter::from("2026-07-12T10:11:12").to::<DateTime<Utc>>(),
        DataType::Instant,
        "RFC 3339 timestamp with offset",
    );
    assert_invalid_syntax(
        DataConverter::from("1.5").to::<BigInt>(),
        DataType::BigInteger,
        "[+-]?[0-9]+",
    );
    assert_invalid_syntax(
        DataConverter::from("1.2.3").to::<BigDecimal>(),
        DataType::BigDecimal,
        "decimal number with optional exponent",
    );
    assert_invalid_syntax(
        DataConverter::from("relative/path").to::<Url>(),
        DataType::Url,
        "absolute URL",
    );
    assert_invalid_kind(
        DataConverter::from("{").to::<serde_json::Value>(),
        DataType::Json,
        DataConversionErrorKind::Deserialization {
            format: DataFormat::Json,
        },
    );
    for value in [r#"{"key":1}"#, r#"{"key":"one","key":"two"}"#] {
        assert_invalid_kind(
            DataConverter::from(value).to::<HashMap<String, String>>(),
            DataType::StringMap,
            DataConversionErrorKind::Deserialization {
                format: DataFormat::Json,
            },
        );
    }
    assert_invalid_kind(
        DataConverter::from("[]").to::<HashMap<String, String>>(),
        DataType::StringMap,
        DataConversionErrorKind::Deserialization {
            format: DataFormat::Json,
        },
    );
    assert_invalid_kind(
        DataConverter::from(r#"{"key":"value"} []"#)
            .to::<HashMap<String, String>>(),
        DataType::StringMap,
        DataConversionErrorKind::Deserialization {
            format: DataFormat::Json,
        },
    );
    for value in [" 1s", "+1s", "1.5s"] {
        assert_invalid_syntax(
            DataConverter::from(value).to::<Duration>(),
            DataType::Duration,
            "[0-9]+(ns|us|ms|s|m|h|d)?",
        );
    }
    assert_invalid_kind(
        DataConverter::from("1fortnight").to::<Duration>(),
        DataType::Duration,
        DataConversionErrorKind::UnsupportedDurationUnit,
    );
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
        Err(DataConversionError::Invalid { .. })
    ));
    assert!(matches!(
        DataConverter::Empty(DataType::Url).to::<Url>(),
        Err(DataConversionError::Missing { .. })
    ));
    assert!(matches!(
        DataConverter::from(1i32).to::<Url>(),
        Err(DataConversionError::Unsupported { .. })
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
        Err(DataConversionError::Invalid {
            kind: DataConversionErrorKind::Deserialization {
                format: DataFormat::Json,
            },
            ..
        })
    ));
    assert!(matches!(
        DataConverter::Empty(DataType::Json).to::<serde_json::Value>(),
        Err(DataConversionError::Missing { .. })
    ));
    assert!(matches!(
        DataConverter::from(1i32).to::<serde_json::Value>(),
        Err(DataConversionError::Unsupported { .. })
    ));
}
