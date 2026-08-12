// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests for millisecond duration serde adapter.

use std::time::Duration;

use super::internal::DurationMillisHolder;

/// Verifies Duration serializes as an integer millisecond value.
#[test]
fn test_duration_millis_serialize_as_integer() {
    let holder = DurationMillisHolder {
        duration: Duration::from_millis(1500),
    };

    let json =
        serde_json::to_string(&holder).expect("duration should serialize");

    assert_eq!(json, r#"{"duration":1500}"#);
}

/// Verifies numeric millisecond serialization uses half-up rounding.
#[test]
fn test_duration_millis_serialize_uses_half_up_rounding() {
    let holder = DurationMillisHolder {
        duration: Duration::from_micros(1500),
    };

    let json =
        serde_json::to_string(&holder).expect("duration should serialize");

    assert_eq!(json, r#"{"duration":2}"#);
}

/// Verifies numeric serialization preserves whole millisecond counts.
#[test]
fn test_duration_millis_serialize_keeps_millisecond_unit() {
    let holder = DurationMillisHolder {
        duration: Duration::from_millis(2500),
    };

    let json =
        serde_json::to_string(&holder).expect("duration should serialize");

    assert_eq!(json, r#"{"duration":2500}"#);
}

/// Verifies numeric serialization rejects Duration values beyond u64
/// milliseconds.
#[test]
fn test_duration_millis_serialize_rejects_out_of_range_millis() {
    let holder = DurationMillisHolder {
        duration: Duration::new(u64::MAX, 999_999_999),
    };

    let result = serde_json::to_string(&holder);

    assert!(result.is_err());
}

/// Verifies integer millisecond input deserializes into Duration.
#[test]
fn test_duration_millis_deserialize_from_integer() {
    let holder: DurationMillisHolder =
        serde_json::from_str(r#"{"duration":250}"#)
            .expect("duration should deserialize");

    assert_eq!(holder.duration, Duration::from_millis(250));
}

/// Verifies numeric input is interpreted in milliseconds.
#[test]
fn test_duration_millis_deserialize_treats_integer_as_milliseconds() {
    let holder: DurationMillisHolder =
        serde_json::from_str(r#"{"duration":2}"#)
            .expect("duration should deserialize");

    assert_eq!(holder.duration, Duration::from_millis(2));
}

/// Verifies the numeric adapter rejects string input.
#[test]
fn test_duration_millis_rejects_string() {
    let result =
        serde_json::from_str::<DurationMillisHolder>(r#"{"duration":"250ms"}"#);

    assert!(result.is_err());
}

/// Verifies numeric serialization accepts the last rounded u64 millisecond
/// value.
#[test]
fn test_duration_millis_serialize_accepts_u64_max_rounding_boundary() {
    let duration = Duration::from_millis(u64::MAX)
        .checked_add(Duration::from_micros(499))
        .expect("test duration should fit");
    let holder = DurationMillisHolder { duration };
    let json =
        serde_json::to_string(&holder).expect("boundary should serialize");

    assert_eq!(json, format!(r#"{{"duration":{}}}"#, u64::MAX));
}

/// Verifies numeric serialization rejects the first rounded u64 overflow.
#[test]
fn test_duration_millis_serialize_rejects_first_rounded_overflow() {
    let duration = Duration::from_millis(u64::MAX)
        .checked_add(Duration::from_micros(500))
        .expect("test duration should fit");
    let error = serde_json::to_string(&DurationMillisHolder { duration })
        .expect_err("rounded u64 overflow should fail");

    assert!(
        error
            .to_string()
            .contains("duration exceeds u64 milliseconds")
    );
}
