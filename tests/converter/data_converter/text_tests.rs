// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Textual and temporal conversion tests.

use std::collections::HashMap;
#[cfg(any(feature = "big-integer", feature = "big-decimal"))]
use std::str::FromStr;
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
use qubit_datatype::ConversionLimits;
use qubit_datatype::ConversionPolicy;
use qubit_datatype::DataConversionError;
use qubit_datatype::DataConverter;
use qubit_datatype::DataType;
use qubit_datatype::InvalidValueReason;
use qubit_datatype::StringConversionPolicy;
use qubit_datatype::converter::DataConversionErrorKind;
#[cfg(feature = "url")]
use url::Url;

/// Assert an invalid-syntax error with exact source and target types.
fn assert_invalid_syntax<T>(result: Result<T, DataConversionError>, to: DataType, expected: &'static str) {
    let matches_expected = matches!(
        &result,
        Err(error) if error.from_type() == Some(DataType::String)
            && error.to_type() == to
            && matches!(
                error.reason(),
                Some(InvalidValueReason::InvalidSyntax {
                expected: actual_expected }) if *actual_expected == expected
    )
    );
    assert!(matches_expected, "unexpected result: {:?}", result.err());
}

/// Asserts that rich-value formatting rejected a non-canonical year.
///
/// # Parameters
///
/// * `result` - String conversion result to inspect.
/// * `from` - Expected temporal source type.
#[cfg(feature = "chrono")]
fn assert_non_canonical_year_rejected(result: Result<String, DataConversionError>, from: DataType) {
    assert_eq!(
        result,
        Err(DataConversionError::invalid(
            from,
            DataType::String,
            InvalidValueReason::OutOfRange,
        )),
    );
}

/// Test the canonical textual formats for rich target types.
#[test]
fn test_data_converter_canonical_char_without_optional_features() {
    assert_eq!(
        DataConverter::from("汉")
            .to::<char>()
            .expect("one Unicode scalar should parse"),
        '汉',
    );
}

#[test]
#[cfg(feature = "chrono")]
fn test_data_converter_canonical_temporal_text_formats() {
    assert_eq!(
        DataConverter::from("2026-07-12")
            .to::<NaiveDate>()
            .expect("canonical date should parse"),
        NaiveDate::from_ymd_opt(2026, 7, 12).expect("test date should be valid"),
    );
    assert_eq!(
        DataConverter::from("10:11:12.123456789")
            .to::<NaiveTime>()
            .expect("canonical time should parse"),
        NaiveTime::from_hms_nano_opt(10, 11, 12, 123_456_789).expect("test time should be valid"),
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
}

#[test]
#[cfg(feature = "chrono")]
fn test_instant_fraction_precision_boundary() {
    let profiles = [
        ConversionPolicy::default(),
        ConversionPolicy::lossy(),
        ConversionPolicy::env_friendly(),
    ];
    let accepted = [
        ("2026-07-12T10:11:12Z", 0),
        ("2026-07-12T10:11:12.1Z", 100_000_000),
        ("2026-07-12T10:11:12.123456789Z", 123_456_789),
        ("2026-07-12T10:11:12.123456789+08:00", 123_456_789),
    ];
    for (text, nanos) in accepted {
        for policy in &profiles {
            for converted in [
                DataConverter::from(text)
                    .to_with::<DateTime<Utc>>(policy, ConversionLimits::default_ref())
                    .expect("up to nine fractional digits should parse"),
                DataConverter::from(text)
                    .into_target_with::<DateTime<Utc>>(policy, ConversionLimits::default_ref())
                    .expect("consuming parse should accept up to nine fractional digits"),
            ] {
                assert_eq!(converted.timestamp_subsec_nanos(), nanos, "{text}");
                assert_eq!(converted.offset(), &Utc, "{text}");
                if text.ends_with("+08:00") {
                    assert_eq!(converted.to_rfc3339(), "2026-07-12T02:11:12.123456789+00:00");
                }
            }
        }
    }

    let rejected = [
        "2026-07-12T10:11:12.1234567899Z",
        "2026-07-12T10:11:12.1234567890Z",
        "2026-07-12T10:11:12.0000000000Z",
        "2026-07-12T10:11:12.Z",
    ];
    for text in rejected {
        for policy in &profiles {
            for result in [
                DataConverter::from(text).to_with::<DateTime<Utc>>(policy, ConversionLimits::default_ref()),
                DataConverter::from(text).into_target_with::<DateTime<Utc>>(policy, ConversionLimits::default_ref()),
            ] {
                assert!(
                    matches!(
                        result,
                        Err(ref error)
                            if error.kind() == DataConversionErrorKind::InvalidValue
                                && error.from_type() == Some(DataType::String)
                                && error.to_type() == DataType::Instant
                                && matches!(
                                    error.reason(),
                                    Some(InvalidValueReason::InvalidSyntax { .. })
                                )
                    ),
                    "unexpected result for {text}: {result:?}"
                );
            }
        }
    }
}

#[test]
#[cfg(feature = "big-integer")]
fn test_data_converter_canonical_big_integer_text() {
    assert_eq!(
        DataConverter::from("+12345678901234567890")
            .to::<BigInt>()
            .expect("signed decimal BigInt should parse"),
        BigInt::from_str("12345678901234567890").expect("expected BigInt should parse"),
    );
}

#[test]
#[cfg(feature = "big-decimal")]
fn test_data_converter_canonical_big_decimal_text() {
    assert_eq!(
        DataConverter::from("1.25e3")
            .to::<BigDecimal>()
            .expect("exponent BigDecimal should parse"),
        BigDecimal::from(1_250),
    );
}

#[test]
#[cfg(feature = "url")]
fn test_data_converter_canonical_url_text() {
    assert_eq!(
        DataConverter::from("https://example.com/path")
            .to::<Url>()
            .expect("absolute URL should parse"),
        Url::parse("https://example.com/path").expect("expected URL should parse"),
    );
}

#[test]
#[cfg(feature = "json")]
fn test_data_converter_canonical_json_text() {
    assert_eq!(
        DataConverter::from("[1,true,null]")
            .to::<serde_json::Value>()
            .expect("any JSON value should parse"),
        serde_json::json!([1, true, null]),
    );
}

#[test]
#[cfg(feature = "json")]
fn test_data_converter_canonical_string_map_json_text() {
    assert_eq!(
        DataConverter::from(r#"{"key":"value"}"#)
            .to::<HashMap<String, String>>()
            .expect("string map JSON should parse")
            .get("key"),
        Some(&"value".to_string()),
    );
}

#[test]
fn test_data_converter_canonical_duration_text() {
    assert_eq!(
        DataConverter::from("1s")
            .to::<Duration>()
            .expect("duration with suffix should parse"),
        Duration::from_secs(1),
    );
}

/// Test parser-internal rejection branches for canonical temporal formats.
#[test]
#[cfg(feature = "chrono")]
fn test_data_converter_temporal_parsers_reject_invalid_components() {
    for value in ["2026-7-12", "2026-13-40"] {
        assert!(DataConverter::from(value).to::<NaiveDate>().is_err());
    }
    for value in ["1:02:03", "01:02:03.", "01:02:03.a", "25:00:00"] {
        assert!(DataConverter::from(value).to::<NaiveTime>().is_err());
    }
    for value in ["2026-13-40T01:02:03", "2026-07-12T25:00:00", "2026-07-12T01:02:03.a"] {
        assert!(DataConverter::from(value).to::<NaiveDateTime>().is_err());
    }
}

/// Verifies fixed-width temporal parsers reject non-canonical byte shapes.
#[test]
#[cfg(feature = "chrono")]
fn test_data_converter_temporal_parsers_reject_noncanonical_byte_shapes() {
    for value in ["2026- 7-12", "+001-01-01"] {
        assert_invalid_syntax(
            DataConverter::from(value).to::<NaiveDate>(),
            DataType::Date,
            "YYYY-MM-DD",
        );
    }
    assert_invalid_syntax(
        DataConverter::from(" 1:02:03").to::<NaiveTime>(),
        DataType::Time,
        "HH:MM:SS[.fraction]",
    );
    assert_invalid_syntax(
        DataConverter::from("2026- 7-12T 1:02:03").to::<NaiveDateTime>(),
        DataType::DateTime,
        "YYYY-MM-DDTHH:MM:SS[.fraction]",
    );
}

/// Test that trim is applied exactly according to string options.
#[test]
fn test_data_converter_default_does_not_trim_any_text_parser() {
    assert!(matches!(
        DataConverter::from(" true ").to::<bool>(),
        Err(ref error) if error == &DataConversionError::invalid(
            DataType::String,
            DataType::Bool,
            InvalidValueReason::InvalidBoolean,
        ),
    ));
    assert_invalid_syntax(DataConverter::from(" 1 ").to::<i32>(), DataType::Int32, "integer");
    assert_invalid_syntax(
        DataConverter::from(" 1s ").to::<Duration>(),
        DataType::Duration,
        "[0-9]+(ns|us|µs|μs|ms|s|min|h|d)",
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
        DataConverter::Unset(DataType::Char).to::<char>(),
        Err(ref error) if error == &DataConversionError::missing(DataType::Char, DataType::Char)
    ));
    assert_eq!(DataConverter::from("z").to::<char>(), Ok('z'));
    assert!(matches!(
        DataConverter::from(1i32).to::<char>(),
        Err(conversion_error) if conversion_error.kind() == DataConversionErrorKind::Unsupported
    ));
}

/// Test string target conversion from core scalar sources.
#[test]
fn test_data_converter_string_target_accepts_core_value_sources() {
    for (source, expected) in [
        (DataConverter::from("text"), "text"),
        (DataConverter::from(true), "true"),
        (DataConverter::from('A'), "A"),
        (DataConverter::from(1i8), "1"),
        (DataConverter::from(2i16), "2"),
        (DataConverter::from(3i32), "3"),
        (DataConverter::from(4i64), "4"),
        (DataConverter::from(5i128), "5"),
        (DataConverter::from(6u8), "6"),
        (DataConverter::from(7u16), "7"),
        (DataConverter::from(8u32), "8"),
        (DataConverter::from(9u64), "9"),
        (DataConverter::from(10u128), "10"),
        (DataConverter::from(Duration::from_millis(1)), "1ms"),
    ] {
        assert_eq!(
            source.to::<String>().expect("source should convert to string"),
            expected,
        );
    }
    assert_eq!(DataConverter::from(13.5_f32).to::<String>().unwrap(), "13.5");
    assert_eq!(DataConverter::from(14.5_f64).to::<String>().unwrap(), "14.5");
    assert_eq!(DataConverter::from(-0.0_f32).to::<String>().unwrap(), "-0");
    assert_eq!(DataConverter::from(-0.0_f64).to::<String>().unwrap(), "-0");
    assert_eq!(DataConverter::from(f64::NAN).to::<String>().unwrap(), "NaN");
    assert_eq!(DataConverter::from(f64::INFINITY).to::<String>().unwrap(), "inf");
    assert_eq!(DataConverter::from(f64::NEG_INFINITY).to::<String>().unwrap(), "-inf");
    assert!(matches!(
        DataConverter::Unset(DataType::String).to::<String>(),
        Err(conversion_error) if conversion_error.kind() == DataConversionErrorKind::Missing
    ));
}

#[test]
#[cfg(feature = "chrono")]
fn test_data_converter_string_target_accepts_chrono_sources() {
    let date = NaiveDate::from_ymd_opt(2026, 5, 1).expect("test date should be valid");
    let time = NaiveTime::from_hms_opt(12, 30, 45).expect("test time should be valid");
    let datetime = NaiveDateTime::new(date, time);
    let instant = DateTime::<Utc>::from_naive_utc_and_offset(datetime, Utc);
    assert_eq!(DataConverter::from(date).to::<String>().unwrap(), "2026-05-01");
    assert_eq!(DataConverter::from(time).to::<String>().unwrap(), "12:30:45");
    assert_eq!(
        DataConverter::from(datetime).to::<String>().unwrap(),
        "2026-05-01T12:30:45",
    );
    assert_eq!(
        DataConverter::from(instant).to::<String>().unwrap(),
        "2026-05-01T12:30:45+00:00",
    );

    let fractional_time = NaiveTime::from_hms_nano_opt(12, 30, 45, 123_456_789).expect("test time should be valid");
    let fractional_datetime = NaiveDateTime::new(date, fractional_time);
    let fractional_instant = DateTime::<Utc>::from_naive_utc_and_offset(fractional_datetime, Utc);
    assert_eq!(
        DataConverter::from(fractional_time).to::<String>().unwrap(),
        "12:30:45.123456789",
    );
    assert_eq!(
        DataConverter::from(fractional_datetime).to::<String>().unwrap(),
        "2026-05-01T12:30:45.123456789",
    );
    assert_eq!(
        DataConverter::from(fractional_instant).to::<String>().unwrap(),
        "2026-05-01T12:30:45.123456789+00:00",
    );

    let millisecond_time = NaiveTime::from_hms_milli_opt(12, 30, 45, 123).expect("test time should be valid");
    assert_eq!(
        DataConverter::from(millisecond_time).to::<String>().unwrap(),
        "12:30:45.123",
    );
}

#[test]
#[cfg(feature = "big-integer")]
fn test_data_converter_string_target_accepts_big_integer_source() {
    assert!(
        !DataConverter::from(BigInt::from(18))
            .to::<String>()
            .expect("BigInt should convert")
            .is_empty()
    );
}

#[test]
#[cfg(feature = "big-decimal")]
fn test_data_converter_string_target_accepts_big_decimal_source() {
    let value = BigDecimal::from_str("19.5").expect("test decimal should parse");
    assert!(
        !DataConverter::from(value)
            .to::<String>()
            .expect("BigDecimal should convert")
            .is_empty()
    );
}

#[test]
#[cfg(feature = "url")]
fn test_data_converter_string_target_accepts_url_source() {
    let value = Url::parse("https://example.com").expect("test URL should parse");
    assert!(
        !DataConverter::from(&value)
            .to::<String>()
            .expect("URL should convert")
            .is_empty()
    );
}

#[test]
#[cfg(feature = "json")]
fn test_data_converter_string_target_accepts_json_and_map_sources() {
    let map = HashMap::from([("k".to_owned(), "v".to_owned())]);
    let json = serde_json::json!({"k": "v"});
    assert!(
        !DataConverter::from(&map)
            .to::<String>()
            .expect("map should convert")
            .is_empty()
    );
    assert!(
        !DataConverter::from(&json)
            .to::<String>()
            .expect("JSON should convert")
            .is_empty()
    );
}

/// Test direct strict conversions for chrono target types.
#[test]
#[cfg(feature = "chrono")]
fn test_data_converter_strict_chrono_targets_cover_success_and_errors() {
    let date = NaiveDate::from_ymd_opt(2026, 5, 1).expect("test date should be valid");
    let time = NaiveTime::from_hms_opt(12, 30, 45).expect("test time should be valid");
    let datetime = NaiveDateTime::new(date, time);
    let instant = DateTime::<Utc>::from_naive_utc_and_offset(datetime, Utc);

    assert_eq!(
        DataConverter::from(date)
            .to::<NaiveDate>()
            .expect("date conversion should succeed"),
        date
    );
    assert_eq!(
        DataConverter::from(time)
            .to::<NaiveTime>()
            .expect("time conversion should succeed"),
        time
    );
    assert_eq!(
        DataConverter::from(datetime)
            .to::<NaiveDateTime>()
            .expect("naive date-time conversion should succeed"),
        datetime
    );
    assert_eq!(
        DataConverter::from(instant)
            .to::<DateTime<Utc>>()
            .expect("UTC date-time conversion should succeed"),
        instant
    );
    assert!(matches!(
        DataConverter::Unset(DataType::Date).to::<NaiveDate>(),
        Err(conversion_error) if conversion_error.kind() == DataConversionErrorKind::Missing
    ));
    assert_eq!(DataConverter::from("2026-05-01").to::<NaiveDate>(), Ok(date));
    assert!(matches!(
        DataConverter::Unset(DataType::Time).to::<NaiveTime>(),
        Err(conversion_error) if conversion_error.kind() == DataConversionErrorKind::Missing
    ));
    assert!(matches!(
        DataConverter::from(1i32).to::<NaiveTime>(),
        Err(conversion_error) if conversion_error.kind() == DataConversionErrorKind::Unsupported
    ));
    assert!(matches!(
        DataConverter::Unset(DataType::DateTime).to::<NaiveDateTime>(),
        Err(conversion_error) if conversion_error.kind() == DataConversionErrorKind::Missing
    ));
    assert!(matches!(
        DataConverter::from(1i32).to::<NaiveDateTime>(),
        Err(conversion_error) if conversion_error.kind() == DataConversionErrorKind::Unsupported
    ));
    assert!(matches!(
        DataConverter::Unset(DataType::Instant).to::<DateTime<Utc>>(),
        Err(conversion_error) if conversion_error.kind() == DataConversionErrorKind::Missing
    ));
    assert!(matches!(
        DataConverter::from(1i32).to::<DateTime<Utc>>(),
        Err(conversion_error) if conversion_error.kind() == DataConversionErrorKind::Unsupported
    ));
}

#[test]
#[cfg(feature = "big-integer")]
fn test_data_converter_strict_big_integer_target() {
    let big_int = BigInt::from(18);
    assert_eq!(DataConverter::from(&big_int).to::<BigInt>(), Ok(big_int));
    assert!(matches!(
        DataConverter::Unset(DataType::BigInteger).to::<BigInt>(),
        Err(error) if error.kind() == DataConversionErrorKind::Missing
    ));
    assert_eq!(DataConverter::from(1i32).to::<BigInt>(), Ok(BigInt::from(1)));
}

#[test]
#[cfg(feature = "big-decimal")]
fn test_data_converter_strict_big_decimal_target() {
    let value = BigDecimal::from(19);
    assert_eq!(DataConverter::from(&value).to::<BigDecimal>(), Ok(value));
    assert!(matches!(
        DataConverter::Unset(DataType::BigDecimal).to::<BigDecimal>(),
        Err(error) if error.kind() == DataConversionErrorKind::Missing
    ));
    assert_eq!(DataConverter::from(1i32).to::<BigDecimal>(), Ok(BigDecimal::from(1)));
}

#[test]
fn test_data_converter_strict_string_map_target() {
    let map = HashMap::from([("k".to_owned(), "v".to_owned())]);
    assert_eq!(DataConverter::from(&map).to::<HashMap<String, String>>(), Ok(map));
    assert!(matches!(
        DataConverter::Unset(DataType::StringMap).to::<HashMap<String, String>>(),
        Err(error) if error.kind() == DataConversionErrorKind::Missing
    ));
    assert!(matches!(
        DataConverter::from(1i32).to::<HashMap<String, String>>(),
        Err(error) if error.kind() == DataConversionErrorKind::Unsupported
    ));
}

/// Test temporal conversions with strict target behavior.
#[test]
#[cfg(feature = "chrono")]
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

/// Verifies date formatting rejects years outside the canonical four digits.
#[test]
#[cfg(feature = "chrono")]
fn test_date_to_string_rejects_non_canonical_years() {
    for year in [-1, 10_000] {
        let date = NaiveDate::from_ymd_opt(year, 1, 1).expect("test date should be valid in Chrono");
        assert_non_canonical_year_rejected(DataConverter::from(date).to::<String>(), DataType::Date);
    }
}

/// Verifies local date-time formatting rejects non-canonical years.
#[test]
#[cfg(feature = "chrono")]
fn test_datetime_to_string_rejects_non_canonical_years() {
    let time = NaiveTime::from_hms_opt(0, 0, 0).expect("test time should be valid");
    for year in [-1, 10_000] {
        let date = NaiveDate::from_ymd_opt(year, 1, 1).expect("test date should be valid in Chrono");
        let datetime = NaiveDateTime::new(date, time);
        assert_non_canonical_year_rejected(DataConverter::from(datetime).to::<String>(), DataType::DateTime);
    }
}

/// Verifies UTC instant formatting rejects non-canonical years.
#[test]
#[cfg(feature = "chrono")]
fn test_instant_to_string_rejects_non_canonical_years() {
    let time = NaiveTime::from_hms_opt(0, 0, 0).expect("test time should be valid");
    for year in [-1, 10_000] {
        let date = NaiveDate::from_ymd_opt(year, 1, 1).expect("test date should be valid in Chrono");
        let datetime = NaiveDateTime::new(date, time);
        let instant = DateTime::<Utc>::from_naive_utc_and_offset(datetime, Utc);
        assert_non_canonical_year_rejected(DataConverter::from(instant).to::<String>(), DataType::Instant);
    }
}

/// Verifies consuming String conversion reuses owned storage and still applies
/// normalization.
#[test]
fn test_data_converter_consuming_string_identity_reuses_owned_storage() {
    let source = String::from("owned payload");
    let source_pointer = source.as_ptr();
    let converted = DataConverter::from(source)
        .into_target::<String>()
        .expect("owned String identity conversion should succeed");

    assert_eq!(converted.as_ptr(), source_pointer);

    let borrowed = String::from("borrowed payload");
    let borrowed_pointer = borrowed.as_ptr();
    let converted = DataConverter::from(&borrowed)
        .into_target::<String>()
        .expect("borrowed String identity conversion should succeed");
    assert_ne!(converted.as_ptr(), borrowed_pointer);

    let options = ConversionPolicy::builder()
        .string_policy(StringConversionPolicy::builder().trim(true).build())
        .build();
    let trimmed = DataConverter::from(String::from("  payload  "))
        .into_target_with::<String>(&options, ConversionLimits::default_ref())
        .expect("consuming String conversion should still trim text");
    assert_eq!(trimmed, "payload");

    assert_eq!(DataConverter::from(7i32).into_target::<String>(), Ok("7".to_owned()),);
}

/// Verifies StringMap text conversion emits JSON members in canonical key
/// order.
#[test]
#[cfg(feature = "json")]
fn test_data_converter_string_map_to_string_orders_keys() {
    let source = HashMap::from([
        ("zeta".to_owned(), "last".to_owned()),
        ("alpha".to_owned(), "first".to_owned()),
        ("omega".to_owned(), "middle".to_owned()),
        ("beta".to_owned(), "second".to_owned()),
    ]);

    let converted = DataConverter::from(&source)
        .to::<String>()
        .expect("string map should convert to canonical JSON text");

    assert_eq!(
        converted,
        r#"{"alpha":"first","beta":"second","omega":"middle","zeta":"last"}"#,
    );
}

/// Verifies consuming URL-to-String conversion reuses the URL text storage.
#[test]
#[cfg(feature = "url")]
fn test_data_converter_consuming_url_to_string_reuses_storage() {
    let source = Url::parse("https://example.com/owned-payload").expect("test URL should parse");
    let source_pointer = source.as_str().as_ptr();

    let converted = DataConverter::from(source)
        .into_target::<String>()
        .expect("owned URL should convert to String");

    assert_eq!(converted, "https://example.com/owned-payload");
    assert_eq!(converted.as_ptr(), source_pointer);
}

/// Verifies consuming URL conversion reuses owned URL storage.
#[test]
#[cfg(feature = "url")]
fn test_data_converter_consuming_url_identity_reuses_owned_storage() {
    let source = Url::parse("https://example.com/owned-payload").expect("test URL should parse");
    let source_pointer = source.as_str().as_ptr();
    let converted = DataConverter::from(source)
        .into_target::<Url>()
        .expect("owned URL identity conversion should succeed");

    assert_eq!(converted.as_str().as_ptr(), source_pointer);

    let converted = DataConverter::from(String::from("https://example.com/non-identity-source"))
        .into_target::<Url>()
        .expect("owned URL text should convert through the borrowed path");
    assert_eq!(converted.as_str(), "https://example.com/non-identity-source",);
}
