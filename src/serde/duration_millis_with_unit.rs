// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Lossy Serde adapter for [`std::time::Duration`] as millisecond text.
//!
//! Serialization rounds to the nearest whole millisecond using half-up
//! rounding, saturates at the largest whole-millisecond value representable by
//! [`std::time::Duration`], and appends `ms`. Deserialization accepts only the
//! matching, untrimmed `<integer>ms` grammar.

use std::time::Duration;

use crate::{
    DurationParseError,
    DurationUnit,
};
use serde::{
    Deserialize,
    Deserializer,
    Serializer,
};

use super::duration_millis::rounded_millis;

/// Deserializes fixed millisecond text matching the required grammar.
///
/// # Parameters
///
/// - `deserializer`: Serde deserializer providing a string value.
///
/// # Returns
///
/// The parsed [`Duration`] with millisecond precision.
///
/// # Errors
///
/// Returns the deserializer error when the input is not a string, does not
/// match the untrimmed `<integer>ms` grammar, exceeds `u128`, or represents a
/// value outside the range of [`Duration`].
#[inline(always)]
pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    let text = String::deserialize(deserializer)?;
    parse(&text).map_err(serde::de::Error::custom)
}

/// Parses a non-negative whole millisecond count with the canonical `ms`
/// symbol.
///
/// The input is not trimmed and must match `<integer>ms` exactly.
///
/// # Parameters
///
/// - `text`: Millisecond text to parse.
///
/// # Returns
///
/// The parsed [`Duration`] with millisecond precision.
///
/// # Errors
///
/// Returns [`DurationParseError::InvalidSyntax`] when `text` does not match the
/// required grammar. Returns [`DurationParseError::OutOfRange`] when the
/// integer exceeds `u128` or the value cannot fit in a [`Duration`].
///
/// # Examples
///
/// ```
/// use std::time::Duration;
///
/// use qubit_datatype::serde::duration_millis_with_unit;
///
/// assert_eq!(
///     duration_millis_with_unit::parse("42ms"),
///     Ok(Duration::from_millis(42))
/// );
/// ```
#[inline(always)]
pub fn parse(text: &str) -> Result<Duration, DurationParseError> {
    let Some(digits) = text.strip_suffix("ms") else {
        return Err(DurationParseError::InvalidSyntax);
    };
    if digits.is_empty() || !digits.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(DurationParseError::InvalidSyntax);
    }
    let millis = digits
        .parse::<u128>()
        .map_err(|_| DurationParseError::OutOfRange)?;
    DurationUnit::Milliseconds
        .duration_from_u128(millis)
        .map_err(|_| DurationParseError::OutOfRange)
}

/// Serializes a [`Duration`] as rounded whole milliseconds with an `ms`
/// suffix.
///
/// # Parameters
///
/// - `duration`: Duration to round and serialize.
/// - `serializer`: Serde serializer receiving the formatted string.
///
/// # Returns
///
/// The serializer result.
///
/// # Errors
///
/// Returns the serializer error if writing the string value fails.
#[inline(always)]
pub fn serialize<S>(
    duration: &Duration,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&format(duration))
}

/// Formats a [`Duration`] as rounded whole milliseconds with an `ms` suffix.
///
/// # Parameters
///
/// - `duration`: Duration to round and format.
///
/// # Returns
///
/// A string in the form `<rounded-millis>ms`. Values whose half-up result would
/// exceed [`Duration::MAX`] saturate at `18446744073709551615999ms`, the
/// largest millisecond value accepted by [`parse`].
#[must_use]
#[inline]
pub fn format(duration: &Duration) -> String {
    let millis = rounded_millis(*duration).min(Duration::MAX.as_millis());
    format!("{millis}ms")
}
