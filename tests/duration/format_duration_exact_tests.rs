// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests for exact Duration formatting.

use std::time::Duration;

use qubit_datatype::{
    DurationTextOptions,
    format_duration_exact,
    parse_duration_text,
};

/// Tests canonical formatting and round-tripping at semantic boundaries.
#[test]
fn test_format_duration_exact_round_trips_boundaries() {
    let options = DurationTextOptions::default();
    let cases = [
        (Duration::ZERO, "0ms"),
        (Duration::from_secs(172_800), "2d"),
        (Duration::from_secs(7_200), "2h"),
        (Duration::from_secs(120), "2min"),
        (Duration::from_secs(42), "42s"),
        (Duration::from_millis(2_500), "2500ms"),
        (Duration::from_micros(500), "500µs"),
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
