// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Supported units for Duration conversion and text codecs.

use std::time::Duration;

use serde::{Deserialize, Serialize};

use super::DurationOverflowError;

/// Unit used to interpret or format a Duration value.
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
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
    /// Parses a supported Duration unit suffix.
    ///
    /// # Parameters
    ///
    /// * `suffix` - Candidate unit suffix.
    ///
    /// # Returns
    ///
    /// The matched unit, including Unicode microsecond aliases, or `None`.
    #[inline]
    pub fn from_suffix(suffix: &str) -> Option<Self> {
        match suffix {
            "ns" => Some(Self::Nanoseconds),
            "us" | "µs" | "μs" => Some(Self::Microseconds),
            "ms" => Some(Self::Milliseconds),
            "s" => Some(Self::Seconds),
            "m" => Some(Self::Minutes),
            "h" => Some(Self::Hours),
            "d" => Some(Self::Days),
            _ => None,
        }
    }

    /// Returns the canonical ASCII suffix for this unit.
    ///
    /// # Returns
    ///
    /// The canonical suffix.
    #[must_use]
    #[inline(always)]
    pub const fn suffix(self) -> &'static str {
        match self {
            Self::Nanoseconds => "ns",
            Self::Microseconds => "us",
            Self::Milliseconds => "ms",
            Self::Seconds => "s",
            Self::Minutes => "m",
            Self::Hours => "h",
            Self::Days => "d",
        }
    }

    /// Converts a unit count to a Duration.
    ///
    /// # Parameters
    ///
    /// * `value` - Non-negative integer expressed in this unit.
    ///
    /// # Returns
    ///
    /// The exactly represented Duration.
    ///
    /// # Errors
    ///
    /// Returns [`DurationOverflowError`] when the result exceeds Duration.
    #[inline(always)]
    pub fn duration_from_u64(self, value: u64) -> Result<Duration, DurationOverflowError> {
        self.duration_from_u128(u128::from(value))
    }

    /// Converts a wide unit count to a Duration.
    ///
    /// # Parameters
    ///
    /// * `value` - Non-negative integer expressed in this unit.
    ///
    /// # Returns
    ///
    /// The exactly represented Duration.
    ///
    /// # Errors
    ///
    /// Returns [`DurationOverflowError`] when the result exceeds Duration.
    pub fn duration_from_u128(self, value: u128) -> Result<Duration, DurationOverflowError> {
        match self {
            Self::Nanoseconds => duration_from_subseconds(value, 1_000_000_000, 1),
            Self::Microseconds => duration_from_subseconds(value, 1_000_000, 1_000),
            Self::Milliseconds => duration_from_subseconds(value, 1_000, 1_000_000),
            Self::Seconds => checked_seconds(value, 1),
            Self::Minutes => checked_seconds(value, 60),
            Self::Hours => checked_seconds(value, 60 * 60),
            Self::Days => checked_seconds(value, 24 * 60 * 60),
        }
    }

    /// Converts a Duration to an exact count in this unit.
    ///
    /// # Parameters
    ///
    /// * `duration` - Duration to express in this unit.
    ///
    /// # Returns
    ///
    /// The exact count, or `None` when the unit would lose precision.
    #[must_use]
    #[inline]
    pub fn exact_units(self, duration: Duration) -> Option<u128> {
        let total_nanos = duration.as_nanos();
        let nanos_per_unit = self.nanos_per_unit();
        if total_nanos.is_multiple_of(nanos_per_unit) {
            Some(total_nanos / nanos_per_unit)
        } else {
            None
        }
    }

    /// Converts a Duration to this unit using half-up rounding.
    ///
    /// # Parameters
    ///
    /// * `duration` - Duration to express in this unit.
    ///
    /// # Returns
    ///
    /// The rounded unit count.
    #[must_use]
    pub fn rounded_units(self, duration: Duration) -> u128 {
        let total_nanos = duration.as_nanos();
        let unit_nanos = self.nanos_per_unit();
        let quotient = total_nanos / unit_nanos;
        let remainder = total_nanos % unit_nanos;
        if remainder >= unit_nanos.div_ceil(2) {
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
    #[must_use]
    #[inline(always)]
    pub(crate) const fn nanos_per_unit(self) -> u128 {
        match self {
            Self::Nanoseconds => 1,
            Self::Microseconds => 1_000,
            Self::Milliseconds => 1_000_000,
            Self::Seconds => 1_000_000_000,
            Self::Minutes => 60 * 1_000_000_000,
            Self::Hours => 60 * 60 * 1_000_000_000,
            Self::Days => 24 * 60 * 60 * 1_000_000_000,
        }
    }
}

/// Converts a whole-unit count into seconds without overflow.
fn checked_seconds(value: u128, seconds_per_unit: u128) -> Result<Duration, DurationOverflowError> {
    let seconds = value
        .checked_mul(seconds_per_unit)
        .filter(|seconds| *seconds <= u128::from(u64::MAX))
        .ok_or(DurationOverflowError)?;
    Ok(Duration::from_secs(seconds as u64))
}

/// Decomposes a subsecond count without multiplying the complete value.
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
