// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Checked conversion helpers used by [`super::DurationUnit`].

use std::time::Duration;

use super::super::DurationOverflowError;

/// Converts a whole-unit count into seconds without overflow.
///
/// # Parameters
///
/// * `value` - Whole-unit count to convert.
/// * `seconds_per_unit` - Number of seconds represented by one unit.
///
/// # Returns
///
/// The exact represented duration.
///
/// # Errors
///
/// Returns [`DurationOverflowError`] when multiplication or `Duration`'s
/// seconds range would overflow.
#[inline]
pub(super) fn checked_seconds(value: u128, seconds_per_unit: u128) -> Result<Duration, DurationOverflowError> {
    let seconds = value
        .checked_mul(seconds_per_unit)
        .filter(|seconds| *seconds <= u128::from(u64::MAX))
        .ok_or(DurationOverflowError)?;
    Ok(Duration::from_secs(seconds as u64))
}

/// Decomposes a subsecond count without multiplying the complete value.
///
/// # Parameters
///
/// * `value` - Subsecond-unit count to convert.
/// * `units_per_second` - Number of source units in one second.
/// * `nanos_per_unit` - Nanoseconds represented by one source unit.
///
/// # Returns
///
/// The exact represented duration.
///
/// # Errors
///
/// Returns [`DurationOverflowError`] when the whole-seconds component exceeds
/// `Duration`'s range.
#[inline]
pub(super) fn duration_from_subseconds(
    value: u128,
    units_per_second: u128,
    nanos_per_unit: u32,
) -> Result<Duration, DurationOverflowError> {
    let seconds = value / units_per_second;
    if seconds > u128::from(u64::MAX) {
        return Err(DurationOverflowError);
    }
    let nanos = (value % units_per_second) as u32 * nanos_per_unit;
    Ok(Duration::new(seconds as u64, nanos))
}
