// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Supported units for Duration conversion and text codecs.

use std::time::Duration;

use serde::{
    Deserialize,
    Serialize,
};

use super::{
    DurationOverflowError,
    DurationParseError,
};

/// Unit used to interpret or format a Duration value.
///
/// This enum intentionally defines a closed supported-unit vocabulary.
/// Exhaustive matching is part of its API contract; adding a variant is a
/// deliberate breaking change.
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
    /// Parses a Duration unit symbol accepted by strict mode.
    ///
    /// # Parameters
    ///
    /// * `symbol` - Unit suffix without its numeric prefix.
    ///
    /// # Returns
    ///
    /// The unit represented by a stable strict symbol.
    ///
    /// # Errors
    ///
    /// Returns [`DurationParseError::NonCanonicalUnit`] when `symbol` is a
    /// Lenient-only alias, or [`DurationParseError::UnsupportedUnit`] when it
    /// is not a supported unit symbol.
    #[inline]
    pub fn parse_strict(symbol: &str) -> Result<Self, DurationParseError> {
        if let Some(unit) = Self::strict_symbol(symbol) {
            return Ok(unit);
        }
        if let Some((_, canonical)) = Self::lenient_alias(symbol) {
            return Err(DurationParseError::NonCanonicalUnit { canonical });
        }
        Err(DurationParseError::UnsupportedUnit)
    }

    /// Parses a Duration unit symbol accepted by lenient mode.
    ///
    /// # Parameters
    ///
    /// * `symbol` - Unit suffix without its numeric prefix.
    ///
    /// # Returns
    ///
    /// The unit represented by a strict symbol or documented Lenient-only
    /// alias.
    ///
    /// # Errors
    ///
    /// Returns [`DurationParseError::UnsupportedUnit`] when `symbol` is not a
    /// supported unit symbol.
    #[inline]
    pub fn parse_lenient(symbol: &str) -> Result<Self, DurationParseError> {
        Self::strict_symbol(symbol)
            .or_else(|| Self::lenient_alias(symbol).map(|(unit, _)| unit))
            .ok_or(DurationParseError::UnsupportedUnit)
    }

    /// Resolves a stable symbol accepted by strict mode.
    ///
    /// # Parameters
    ///
    /// * `symbol` - Unit suffix without its numeric prefix.
    ///
    /// # Returns
    ///
    /// `Some` containing the represented unit when `symbol` is accepted by
    /// strict mode; otherwise, `None`.
    fn strict_symbol(symbol: &str) -> Option<Self> {
        match symbol {
            "ns" => Some(Self::Nanoseconds),
            "us" | "µs" | "μs" => Some(Self::Microseconds),
            "ms" => Some(Self::Milliseconds),
            "s" => Some(Self::Seconds),
            "min" => Some(Self::Minutes),
            "h" => Some(Self::Hours),
            "d" => Some(Self::Days),
            _ => None,
        }
    }

    /// Resolves an alias accepted only by lenient mode.
    ///
    /// # Parameters
    ///
    /// * `symbol` - Unit suffix without its numeric prefix.
    ///
    /// # Returns
    ///
    /// `Some` containing the represented unit and its preferred output symbol
    /// when `symbol` is a Lenient-only alias; otherwise, `None`.
    #[inline(always)]
    fn lenient_alias(symbol: &str) -> Option<(Self, &'static str)> {
        match symbol {
            "m" => Some((Self::Minutes, "min")),
            _ => None,
        }
    }

    /// Returns the preferred Duration unit symbol for output.
    ///
    /// # Returns
    ///
    /// The preferred suffix, which is `µs` for microseconds and `min` for
    /// minutes.
    #[must_use]
    #[inline(always)]
    pub const fn symbol(self) -> &'static str {
        match self {
            Self::Nanoseconds => "ns",
            Self::Microseconds => "µs",
            Self::Milliseconds => "ms",
            Self::Seconds => "s",
            Self::Minutes => "min",
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
    pub fn duration_from_u64(
        self,
        value: u64,
    ) -> Result<Duration, DurationOverflowError> {
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
    pub fn duration_from_u128(
        self,
        value: u128,
    ) -> Result<Duration, DurationOverflowError> {
        match self {
            Self::Nanoseconds => {
                duration_from_subseconds(value, 1_000_000_000, 1)
            }
            Self::Microseconds => {
                duration_from_subseconds(value, 1_000_000, 1_000)
            }
            Self::Milliseconds => {
                duration_from_subseconds(value, 1_000, 1_000_000)
            }
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
    #[inline]
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
fn checked_seconds(
    value: u128,
    seconds_per_unit: u128,
) -> Result<Duration, DurationOverflowError> {
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
