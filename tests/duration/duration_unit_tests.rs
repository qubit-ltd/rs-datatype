// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests for lightweight Duration unit operations.

use std::time::Duration;

use qubit_datatype::DurationUnit;

/// Tests exact and half-up conversion to unit counts.
#[test]
fn test_duration_unit_exact_and_rounded_units() {
    let exact = Duration::from_millis(2);
    let inexact = Duration::from_micros(1_500);

    assert_eq!(DurationUnit::Milliseconds.exact_units(exact), Some(2));
    assert_eq!(DurationUnit::Milliseconds.exact_units(inexact), None);
    assert_eq!(DurationUnit::Milliseconds.rounded_units(inexact), 2);
}

/// Tests that every unit converts boundary counts without losing precision.
#[test]
fn test_duration_unit_converts_supported_counts() {
    let cases = [
        (DurationUnit::Nanoseconds, 42, Duration::from_nanos(42)),
        (DurationUnit::Microseconds, 42, Duration::from_micros(42)),
        (DurationUnit::Milliseconds, 42, Duration::from_millis(42)),
        (DurationUnit::Seconds, 42, Duration::from_secs(42)),
        (DurationUnit::Minutes, 2, Duration::from_secs(120)),
        (DurationUnit::Hours, 2, Duration::from_secs(7_200)),
        (DurationUnit::Days, 2, Duration::from_secs(172_800)),
    ];

    for (unit, value, expected) in cases {
        assert_eq!(unit.duration_from_u128(value), Ok(expected));
    }
}
