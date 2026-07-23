// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests for unit-suffixed duration serde adapter.

use std::time::Duration;

use qubit_datatype::DurationParseError;
use qubit_datatype::serde::duration_with_unit;
use serde::de::value::{
    Error as ValueError,
    StringDeserializer,
};

use super::internal::{
    BorrowedStrOnlyDeserializer,
    DurationWithUnitHolder,
};

/// Verifies exact Duration serialization emits a unit-suffixed string.
#[test]
fn test_duration_with_unit_serialize_as_exact_string() {
    let holder = DurationWithUnitHolder {
        duration: Duration::from_millis(1500),
    };

    let json =
        serde_json::to_string(&holder).expect("duration should serialize");

    assert_eq!(json, r#"{"duration":"1500ms"}"#);
}

/// Verifies strict supported unit spellings deserialize correctly.
#[test]
fn test_duration_with_unit_deserialize_from_supported_units() {
    let cases = [
        ("42ns", Duration::from_nanos(42)),
        ("42us", Duration::from_micros(42)),
        ("42µs", Duration::from_micros(42)),
        ("42μs", Duration::from_micros(42)),
        ("42ms", Duration::from_millis(42)),
        ("42s", Duration::from_secs(42)),
        ("2min", Duration::from_secs(120)),
        ("2h", Duration::from_secs(7200)),
        ("2d", Duration::from_secs(172800)),
    ];

    for (text, expected) in cases {
        let json = format!(r#"{{"duration":"{text}"}}"#);
        let holder: DurationWithUnitHolder =
            serde_json::from_str(&json).expect("duration should deserialize");
        assert_eq!(holder.duration, expected);
    }

    {
        let unsupported = "42m";
        let json = format!(r#"{{"duration":"{unsupported}"}}"#);
        assert!(serde_json::from_str::<DurationWithUnitHolder>(&json).is_err());
    }
}

/// Verifies the exact adapter rejects integer input.
#[test]
fn test_duration_with_unit_deserialize_from_integer_millis() {
    assert!(
        serde_json::from_str::<DurationWithUnitHolder>(r#"{"duration":250}"#)
            .is_err()
    );
}

/// Verifies the exact adapter accepts an owned string deserializer.
#[test]
fn test_duration_with_unit_deserialize_from_owned_string() {
    let deserializer =
        StringDeserializer::<ValueError>::new("42ns".to_string());
    let duration = duration_with_unit::deserialize(deserializer)
        .expect("owned duration text should deserialize");

    assert_eq!(duration, Duration::from_nanos(42));
}

/// Verifies the exact adapter can consume borrowed text without requesting an
/// owned string.
#[test]
fn test_duration_with_unit_deserialize_from_borrowed_only_str() {
    let deserializer = BorrowedStrOnlyDeserializer::new("42ns");
    let duration = duration_with_unit::deserialize(deserializer)
        .expect("borrowed duration text should deserialize");

    assert_eq!(duration, Duration::from_nanos(42));
}

/// Verifies the exact adapter rejects non-string scalar input.
#[test]
fn test_duration_with_unit_deserialize_rejects_non_string_scalars() {
    for json in ["250", "-1", "1.5", "true"] {
        let document = format!(r#"{{"duration":{json}}}"#);
        assert!(
            serde_json::from_str::<DurationWithUnitHolder>(&document).is_err(),
            "expected non-string input to fail: {document}"
        );
    }
}

/// Verifies the exact adapter rejects unsupported units through serde.
#[test]
fn test_duration_with_unit_rejects_invalid_unit() {
    let result = serde_json::from_str::<DurationWithUnitHolder>(
        r#"{"duration":"250fortnights"}"#,
    );

    assert!(result.is_err());
}

/// Verifies exact formatting emits the expected millisecond text.
#[test]
fn test_duration_with_unit_format() {
    let text = duration_with_unit::format(&Duration::from_millis(500));

    assert_eq!(text, "500ms");
}

/// Verifies exact formatting selects the largest lossless unit.
#[test]
fn test_duration_with_unit_format_selects_largest_exact_unit() {
    let cases = [
        (Duration::ZERO, "0ms"),
        (Duration::from_secs(2 * 24 * 60 * 60), "2d"),
        (Duration::from_secs(2 * 60 * 60), "2h"),
        (Duration::from_secs(2 * 60), "2min"),
        (Duration::from_secs(42), "42s"),
        (Duration::from_millis(2500), "2500ms"),
        (Duration::from_micros(500), "500µs"),
        (Duration::from_nanos(42), "42ns"),
    ];

    for (duration, expected) in cases {
        assert_eq!(duration_with_unit::format(&duration), expected);
    }
}

/// Verifies exact serialization preserves sub-millisecond precision.
#[test]
fn test_duration_with_unit_serialize_preserves_sub_millisecond_precision() {
    let holder = DurationWithUnitHolder {
        duration: Duration::from_micros(1500),
    };

    let json =
        serde_json::to_string(&holder).expect("duration should serialize");

    assert_eq!(json, r#"{"duration":"1500µs"}"#);
}

/// Verifies exact formatting round-trips Duration::MAX.
#[test]
fn test_duration_with_unit_format_round_trips_duration_max() {
    let text = duration_with_unit::format(&Duration::MAX);
    let parsed = duration_with_unit::parse(&text)
        .expect("formatted maximum should parse");

    assert_eq!(parsed, Duration::MAX);
}

/// Verifies exact formatting round-trips semantic unit boundaries.
#[test]
fn test_duration_with_unit_format_round_trips_semantic_boundaries() {
    let seconds = [
        0,
        1,
        59,
        60,
        3599,
        3600,
        86_399,
        86_400,
        u64::from(u32::MAX),
        u64::MAX,
    ];
    let nanoseconds = [
        0,
        1,
        999,
        1_000,
        999_999,
        1_000_000,
        499_999_999,
        500_000_000,
        999_999_999,
    ];

    for seconds in seconds {
        for nanoseconds in nanoseconds {
            let duration = Duration::new(seconds, nanoseconds);
            let text = duration_with_unit::format(&duration);
            let parsed = duration_with_unit::parse(&text)
                .expect("formatted boundary duration should parse");
            assert_eq!(parsed, duration, "failed to round-trip {text}");
        }
    }
}

/// Verifies exact parsing rejects empty or whitespace-only text.
#[test]
fn test_duration_with_unit_parse_rejects_empty_text() {
    let result = duration_with_unit::parse(" ");

    assert!(result.is_err());
}

/// Verifies exact Duration text is not implicitly trimmed.
#[test]
fn test_duration_with_unit_parse_rejects_surrounding_whitespace() {
    assert!(duration_with_unit::parse(" 2ms ").is_err());
    assert!(
        serde_json::from_str::<DurationWithUnitHolder>(
            r#"{"duration":" 2ms "}"#
        )
        .is_err()
    );
}

/// Verifies exact parsing rejects suffixless numbers.
#[test]
fn test_duration_with_unit_parse_rejects_bare_numbers() {
    assert!(duration_with_unit::parse("2").is_err());
}

/// Verifies serde rejects invalid numbers and non-scalar values.
#[test]
fn test_duration_with_unit_deserialize_rejects_invalid_number_and_non_scalar() {
    assert!(
        serde_json::from_str::<DurationWithUnitHolder>(r#"{"duration":-1}"#)
            .is_err()
    );
    assert!(
        serde_json::from_str::<DurationWithUnitHolder>(r#"{"duration":1.5}"#)
            .is_err()
    );
    for json in [
        r#"{"duration":null}"#,
        r#"{"duration":true}"#,
        r#"{"duration":[]}"#,
        r#"{"duration":{}}"#,
    ] {
        assert!(
            serde_json::from_str::<DurationWithUnitHolder>(json).is_err(),
            "expected error for {json}"
        );
    }
}

/// Verifies exact parsing reports syntax, unit, and range failures.
#[test]
fn test_duration_with_unit_parse_errors_and_overflows() {
    assert!(duration_with_unit::parse("18446744073709551616000ns").is_ok());
    assert!(duration_with_unit::parse("18446744073709551616").is_err());

    assert_eq!(
        duration_with_unit::parse("340282366920938463463374607431768211456ns"),
        Err(DurationParseError::OutOfRange)
    );
    assert_eq!(
        duration_with_unit::parse(&format!("{}s", u128::MAX)),
        Err(DurationParseError::OutOfRange)
    );
    assert_eq!(
        duration_with_unit::parse("x12ms"),
        Err(DurationParseError::InvalidSyntax)
    );
    assert_eq!(
        duration_with_unit::parse("12.5s"),
        Err(DurationParseError::InvalidSyntax)
    );
    assert_eq!(
        duration_with_unit::parse("12fortnights"),
        Err(DurationParseError::UnsupportedUnit)
    );

    let vm = u64::MAX / 60 + 1;
    assert_eq!(
        duration_with_unit::parse(&format!("{vm}min")),
        Err(DurationParseError::OutOfRange)
    );
    let vh = u64::MAX / (60 * 60) + 1;
    assert_eq!(
        duration_with_unit::parse(&format!("{vh}h")),
        Err(DurationParseError::OutOfRange)
    );
    let vd = u64::MAX / (24 * 60 * 60) + 1;
    assert_eq!(
        duration_with_unit::parse(&format!("{vd}d")),
        Err(DurationParseError::OutOfRange)
    );
}

/// Verifies the direct serializer emits exact unit-suffixed text.
#[test]
fn test_duration_with_unit_serialize_function() {
    let mut buf = Vec::new();
    let mut ser = serde_json::Serializer::new(&mut buf);
    duration_with_unit::serialize(&Duration::from_millis(7), &mut ser)
        .expect("serialize");
    assert_eq!(
        String::from_utf8(buf).expect("serialized text should be UTF-8"),
        r#""7ms""#
    );
}

/// Verifies exact Duration text round-trips through postcard.
#[test]
fn test_duration_with_unit_postcard_round_trip() {
    let holder = DurationWithUnitHolder {
        duration: Duration::from_nanos(42),
    };
    let bytes =
        postcard::to_stdvec(&holder).expect("duration should serialize");
    let decoded: DurationWithUnitHolder =
        postcard::from_bytes(&bytes).expect("duration should deserialize");

    assert_eq!(decoded, holder);
}
