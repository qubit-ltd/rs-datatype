// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests for lightweight Duration text parsing and formatting.

use std::time::Duration;

use qubit_datatype::{
    DurationParseError,
    DurationTextOptions,
    DurationUnit,
    DurationUnitSuffixSet,
    SuffixlessDurationPolicy,
    format_duration_exact,
    parse_duration_text,
};

/// Tests ASCII and extended microsecond suffix profiles.
#[test]
fn test_parse_duration_text_respects_suffix_set() {
    let ascii = DurationTextOptions::default()
        .with_unit_suffix_set(DurationUnitSuffixSet::Ascii);
    let extended = DurationTextOptions::default();

    assert_eq!(
        parse_duration_text("2us", &ascii),
        Ok(Duration::from_micros(2)),
    );
    assert_eq!(
        parse_duration_text("2µs", &ascii),
        Err(DurationParseError::InvalidSyntax),
    );
    assert_eq!(
        parse_duration_text("2µs", &extended),
        Ok(Duration::from_micros(2)),
    );
    assert_eq!(
        parse_duration_text("2μs", &extended),
        Ok(Duration::from_micros(2)),
    );
}

/// Tests rejection and assignment policies for suffixless text.
#[test]
fn test_parse_duration_text_respects_suffixless_policy() {
    let reject = DurationTextOptions::default()
        .with_suffixless_policy(SuffixlessDurationPolicy::Reject);
    let seconds = DurationTextOptions::default().with_suffixless_policy(
        SuffixlessDurationPolicy::Assume(DurationUnit::Seconds),
    );

    assert_eq!(
        parse_duration_text("2", &reject),
        Err(DurationParseError::InvalidSyntax),
    );
    assert_eq!(
        parse_duration_text("2", &seconds),
        Ok(Duration::from_secs(2))
    );
}

/// Tests structured syntax, unsupported-unit, and range errors.
#[test]
fn test_parse_duration_text_classifies_errors() {
    let options = DurationTextOptions::default();

    assert_eq!(
        parse_duration_text("12.5s", &options),
        Err(DurationParseError::InvalidSyntax),
    );
    assert_eq!(
        parse_duration_text("12fortnights", &options),
        Err(DurationParseError::UnsupportedUnit {
            unit: "fortnights".to_owned(),
        }),
    );
    assert_eq!(
        parse_duration_text(
            "340282366920938463463374607431768211456ns",
            &options,
        ),
        Err(DurationParseError::OutOfRange),
    );
    assert_eq!(
        parse_duration_text(&format!("{}d", u128::MAX), &options),
        Err(DurationParseError::OutOfRange),
    );
}

/// Tests canonical formatting and round-tripping at semantic boundaries.
#[test]
fn test_format_duration_exact_round_trips_boundaries() {
    let options = DurationTextOptions::default()
        .with_unit_suffix_set(DurationUnitSuffixSet::Ascii);
    let cases = [
        (Duration::ZERO, "0ms"),
        (Duration::from_secs(172_800), "2d"),
        (Duration::from_secs(7_200), "2h"),
        (Duration::from_secs(120), "2m"),
        (Duration::from_secs(42), "42s"),
        (Duration::from_millis(2_500), "2500ms"),
        (Duration::from_micros(500), "500us"),
        (Duration::from_nanos(42), "42ns"),
    ];

    for (duration, expected) in cases {
        let text = format_duration_exact(duration);
        assert_eq!(text, expected);
        assert_eq!(parse_duration_text(&text, &options), Ok(duration));
    }

    let maximum = format_duration_exact(Duration::MAX);
    assert_eq!(parse_duration_text(&maximum, &options), Ok(Duration::MAX));
}
