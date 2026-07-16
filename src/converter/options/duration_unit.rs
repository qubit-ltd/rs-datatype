// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Duration Unit
//!
//! Defines supported units for duration conversion.

use std::time::Duration;

use serde::{
    Deserialize,
    Serialize,
};

use super::super::error::DurationOverflowError;

/// Unit used when converting [`Duration`] values to and from scalar values.
#[must_use]
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum DurationUnit {
    /// Nanoseconds.
    Nanoseconds,
    /// Microseconds.
    Microseconds,
    /// Milliseconds.
    #[default]
    Milliseconds,
    /// Seconds.
    Seconds,
    /// Minutes.
    Minutes,
    /// Hours.
    Hours,
    /// Days.
    Days,
}

impl DurationUnit {
    /// Returns the canonical suffix for this duration unit.
    ///
    /// # Returns
    ///
    /// The ASCII suffix used when formatting this unit.
    #[inline]
    pub const fn suffix(self) -> &'static str {
        match self {
            DurationUnit::Nanoseconds => "ns",
            DurationUnit::Microseconds => "us",
            DurationUnit::Milliseconds => "ms",
            DurationUnit::Seconds => "s",
            DurationUnit::Minutes => "m",
            DurationUnit::Hours => "h",
            DurationUnit::Days => "d",
        }
    }

    /// Parses a duration unit suffix.
    ///
    /// # Parameters
    ///
    /// * `suffix` - Candidate unit suffix.
    ///
    /// # Returns
    ///
    /// Returns the matched unit, or `None` when the suffix is unsupported.
    #[inline]
    pub fn from_suffix(suffix: &str) -> Option<Self> {
        match suffix {
            "ns" => Some(DurationUnit::Nanoseconds),
            "us" | "µs" | "μs" => Some(DurationUnit::Microseconds),
            "ms" => Some(DurationUnit::Milliseconds),
            "s" => Some(DurationUnit::Seconds),
            "m" => Some(DurationUnit::Minutes),
            "h" => Some(DurationUnit::Hours),
            "d" => Some(DurationUnit::Days),
            _ => None,
        }
    }

    /// Converts an integer value in this unit to a [`Duration`].
    ///
    /// # Parameters
    ///
    /// * `value` - Non-negative integer value expressed in this unit.
    ///
    /// # Returns
    ///
    /// The corresponding [`Duration`].
    ///
    /// # Errors
    ///
    /// Returns [`DurationOverflowError`] when converting the value to seconds
    /// would overflow the range supported by [`Duration`].
    #[inline(always)]
    pub fn duration_from_u64(
        self,
        value: u64,
    ) -> Result<Duration, DurationOverflowError> {
        self.duration_from_u128(u128::from(value))
    }

    /// Converts an integer value in this unit to a [`Duration`].
    ///
    /// # Parameters
    ///
    /// * `value` - Non-negative integer value expressed in this unit.
    ///
    /// # Returns
    ///
    /// Returns the exactly represented [`Duration`].
    ///
    /// # Errors
    ///
    /// Returns [`DurationOverflowError`] when the final seconds component
    /// exceeds `u64::MAX`.
    pub fn duration_from_u128(
        self,
        value: u128,
    ) -> Result<Duration, DurationOverflowError> {
        match self {
            DurationUnit::Nanoseconds => {
                duration_from_subseconds(value, 1_000_000_000, 1)
            }
            DurationUnit::Microseconds => {
                duration_from_subseconds(value, 1_000_000, 1_000)
            }
            DurationUnit::Milliseconds => {
                duration_from_subseconds(value, 1_000, 1_000_000)
            }
            DurationUnit::Seconds => checked_secs(value, 1),
            DurationUnit::Minutes => checked_secs(value, 60),
            DurationUnit::Hours => checked_secs(value, 60 * 60),
            DurationUnit::Days => checked_secs(value, 24 * 60 * 60),
        }
    }

    /// Converts a [`Duration`] to this unit using half-up rounding.
    ///
    /// # Parameters
    ///
    /// * `duration` - Duration to format as an integer unit count.
    ///
    /// # Returns
    ///
    /// The rounded number of units represented by the duration.
    pub fn rounded_units(self, duration: Duration) -> u128 {
        let total_nanos = duration.as_nanos();
        let unit_nanos = self.nanos_per_unit();
        let quotient = total_nanos / unit_nanos;
        let remainder = total_nanos % unit_nanos;
        let rounding_threshold = unit_nanos.div_ceil(2);
        if remainder >= rounding_threshold {
            quotient + 1
        } else {
            quotient
        }
    }

    /// Returns the number of nanoseconds in one unit.
    ///
    /// # Returns
    ///
    /// Nanoseconds per unit.
    pub(crate) const fn nanos_per_unit(self) -> u128 {
        match self {
            DurationUnit::Nanoseconds => 1,
            DurationUnit::Microseconds => 1_000,
            DurationUnit::Milliseconds => 1_000_000,
            DurationUnit::Seconds => 1_000_000_000,
            DurationUnit::Minutes => 60 * 1_000_000_000,
            DurationUnit::Hours => 60 * 60 * 1_000_000_000,
            DurationUnit::Days => 24 * 60 * 60 * 1_000_000_000,
        }
    }
}

/// Converts a value multiplied by a second factor into a [`Duration`].
///
/// # Parameters
///
/// * `value` - Non-negative integer value.
/// * `seconds_per_unit` - Number of seconds in each unit.
/// # Returns
///
/// The corresponding [`Duration`].
///
/// # Errors
///
/// Returns [`DurationOverflowError`] when the multiplication overflows `u64`
/// seconds.
#[inline]
fn checked_secs(
    value: u128,
    seconds_per_unit: u128,
) -> Result<Duration, DurationOverflowError> {
    let seconds = value
        .checked_mul(seconds_per_unit)
        .filter(|seconds| *seconds <= u128::from(u64::MAX))
        .ok_or(DurationOverflowError)?;
    Ok(Duration::from_secs(seconds as u64))
}

/// Decomposes subsecond unit counts without multiplying the full value.
///
/// `value` is divided by `units_per_second`; the remainder is multiplied by
/// `nanos_per_unit` to form the subsecond component. Returns the exact
/// duration, or an error when the whole-second component exceeds `u64::MAX`.
/// Callers must provide mutually consistent factors whose remainder product
/// fits `u32` and remains below one billion nanoseconds.
#[inline]
fn duration_from_subseconds(
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
