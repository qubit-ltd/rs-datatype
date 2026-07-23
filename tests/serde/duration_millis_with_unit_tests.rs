// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests for the fixed-millisecond unit-suffixed duration serde adapter.

use std::time::Duration;

use qubit_datatype::DurationParseError;
use qubit_datatype::serde::duration_millis_with_unit;

use super::internal::DurationMillisWithUnitHolder;

/// Verifies Duration serializes as fixed millisecond text.
#[test]
fn test_duration_millis_with_unit_serialize_as_millisecond_string() {
    let holder = DurationMillisWithUnitHolder {
        duration: Duration::from_millis(1500),
    };

    let json =
        serde_json::to_string(&holder).expect("duration should serialize");

    assert_eq!(json, r#"{"duration":"1500ms"}"#);
}

/// Verifies fixed millisecond text uses half-up rounding.
#[test]
fn test_duration_millis_with_unit_serialize_uses_half_up_rounding() {
    let cases = [
        (Duration::from_micros(499), "0ms"),
        (Duration::from_micros(500), "1ms"),
        (Duration::from_micros(1499), "1ms"),
        (Duration::from_micros(1500), "2ms"),
    ];

    for (duration, expected) in cases {
        let holder = DurationMillisWithUnitHolder { duration };
        let json =
            serde_json::to_value(holder).expect("duration should serialize");
        assert_eq!(json["duration"], expected);
    }
}

/// Verifies formatting always emits the ms suffix.
#[test]
fn test_duration_millis_with_unit_format_keeps_millisecond_unit() {
    let text = duration_millis_with_unit::format(&Duration::from_millis(2500));

    assert_eq!(text, "2500ms");
}

/// Verifies millisecond formatting saturates at the largest parseable value.
#[test]
fn test_duration_millis_with_unit_format_saturates_at_duration_max() {
    let maximum_text = format!("{}ms", Duration::MAX.as_millis());
    let maximum_millis = Duration::new(u64::MAX, 999_000_000);
    let cases = [
        Duration::new(u64::MAX, 999_499_999),
        Duration::new(u64::MAX, 999_500_000),
        Duration::MAX,
    ];

    for duration in cases {
        let text = duration_millis_with_unit::format(&duration);
        assert_eq!(text, maximum_text);
        assert_eq!(duration_millis_with_unit::parse(&text), Ok(maximum_millis));
    }
}

/// Verifies canonical fixed millisecond text deserializes into Duration.
#[test]
fn test_duration_millis_with_unit_deserialize_supported_input() {
    let holder: DurationMillisWithUnitHolder =
        serde_json::from_str(r#"{"duration":"42ms"}"#)
            .expect("duration should deserialize");

    assert_eq!(holder.duration, Duration::from_millis(42));
}

/// Verifies fixed millisecond text rejects other unit suffixes.
#[test]
fn test_duration_millis_with_unit_rejects_non_millisecond_input() {
    assert!(
        serde_json::from_str::<DurationMillisWithUnitHolder>(
            r#"{"duration":"42ns"}"#
        )
        .is_err()
    );
    assert!(duration_millis_with_unit::parse("42s").is_err());
}

/// Verifies fixed millisecond parsing exposes structured errors.
#[test]
fn test_duration_millis_with_unit_parse_returns_structured_error() {
    assert_eq!(
        duration_millis_with_unit::parse("12fortnights"),
        Err(DurationParseError::InvalidSyntax)
    );
}

/// Verifies the direct serializer emits fixed millisecond text.
#[test]
fn test_duration_millis_with_unit_serialize_function() {
    let mut buffer = Vec::new();
    let mut serializer = serde_json::Serializer::new(&mut buffer);
    duration_millis_with_unit::serialize(
        &Duration::from_micros(1500),
        &mut serializer,
    )
    .expect("duration should serialize");

    assert_eq!(
        String::from_utf8(buffer).expect("serialized text should be UTF-8"),
        r#""2ms""#
    );
}

/// Verifies parsing accepts the largest whole-millisecond Duration.
#[test]
fn test_duration_millis_with_unit_parse_accepts_maximum_milliseconds() {
    let text = format!("{}ms", Duration::MAX.as_millis());

    assert_eq!(
        duration_millis_with_unit::parse(&text),
        Ok(Duration::new(u64::MAX, 999_000_000))
    );
}

/// Verifies parsing rejects the first millisecond outside Duration.
#[test]
fn test_duration_millis_with_unit_parse_rejects_first_out_of_range_value() {
    assert_eq!(
        duration_millis_with_unit::parse("18446744073709551616000ms"),
        Err(DurationParseError::OutOfRange)
    );
}

/// Verifies malformed and overflowing millisecond text returns exact errors.
#[test]
fn test_duration_millis_with_unit_parse_rejects_invalid_text() {
    for text in ["", "ms", "-1ms", "1.5ms", "1s", " 1ms "] {
        assert_eq!(
            duration_millis_with_unit::parse(text),
            Err(DurationParseError::InvalidSyntax),
            "unexpected result for {text:?}"
        );
    }
    assert_eq!(
        duration_millis_with_unit::parse(
            "340282366920938463463374607431768211456ms"
        ),
        Err(DurationParseError::OutOfRange)
    );
}

/// Verifies postcard round-trips the saturated maximum millisecond value.
#[test]
fn test_duration_millis_with_unit_postcard_saturates_duration_max() {
    let holder = DurationMillisWithUnitHolder {
        duration: Duration::MAX,
    };
    let bytes = postcard::to_stdvec(&holder)
        .expect("maximum duration should serialize");
    let decoded: DurationMillisWithUnitHolder = postcard::from_bytes(&bytes)
        .expect("saturated duration should deserialize");

    assert_eq!(decoded.duration, Duration::new(u64::MAX, 999_000_000));
}
