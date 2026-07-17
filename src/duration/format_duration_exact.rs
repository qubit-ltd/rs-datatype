// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Exact canonical formatting for Duration values.

use std::time::Duration;

use super::DurationUnit;

/// Formats a Duration with the largest exact supported unit.
///
/// Zero is formatted as `0ms`; microseconds use the canonical ASCII `us`
/// suffix. The resulting text round-trips through an ASCII Duration parser.
///
/// # Parameters
///
/// * `duration` - Duration to format without precision loss.
///
/// # Returns
///
/// Canonical unit-suffixed text.
#[must_use]
pub fn format_duration_exact(duration: Duration) -> String {
    if duration.is_zero() {
        return "0ms".to_owned();
    }
    for unit in [
        DurationUnit::Days,
        DurationUnit::Hours,
        DurationUnit::Minutes,
        DurationUnit::Seconds,
        DurationUnit::Milliseconds,
        DurationUnit::Microseconds,
    ] {
        if let Some(value) = unit.exact_units(duration) {
            return format!("{value}{}", unit.suffix());
        }
    }
    format!("{}ns", duration.as_nanos())
}
