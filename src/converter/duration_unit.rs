/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! # Duration Unit
//!
//! Defines supported units for duration conversion.
//!

use std::time::Duration;

/// Unit used when converting [`Duration`] values to and from scalar values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DurationUnit {
    /// Nanoseconds.
    Nanoseconds,
    /// Microseconds.
    Microseconds,
    /// Milliseconds.
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
    /// Returns an error message when converting the value to seconds would
    /// overflow the range supported by [`Duration`].
    pub fn duration_from_u64(self, value: u64) -> Result<Duration, String> {
        match self {
            DurationUnit::Nanoseconds => Ok(Duration::from_nanos(value)),
            DurationUnit::Microseconds => Ok(Duration::from_micros(value)),
            DurationUnit::Milliseconds => Ok(Duration::from_millis(value)),
            DurationUnit::Seconds => Ok(Duration::from_secs(value)),
            DurationUnit::Minutes => checked_secs(value, 60, "minutes"),
            DurationUnit::Hours => checked_secs(value, 60 * 60, "hours"),
            DurationUnit::Days => checked_secs(value, 24 * 60 * 60, "days"),
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
    const fn nanos_per_unit(self) -> u128 {
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

impl Default for DurationUnit {
    /// Creates the default duration unit.
    fn default() -> Self {
        DurationUnit::Milliseconds
    }
}

/// Converts a value multiplied by a second factor into a [`Duration`].
///
/// # Parameters
///
/// * `value` - Non-negative integer value.
/// * `seconds_per_unit` - Number of seconds in each unit.
/// * `unit_name` - Unit name used in overflow diagnostics.
///
/// # Returns
///
/// The corresponding [`Duration`].
///
/// # Errors
///
/// Returns an error message when the multiplication overflows `u64` seconds.
fn checked_secs(value: u64, seconds_per_unit: u64, unit_name: &str) -> Result<Duration, String> {
    value
        .checked_mul(seconds_per_unit)
        .map(Duration::from_secs)
        .ok_or_else(|| format!("duration {unit_name} overflow u64 seconds"))
}
