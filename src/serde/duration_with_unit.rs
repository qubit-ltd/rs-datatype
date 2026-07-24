// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Exact Serde adapter for [`std::time::Duration`] as a unit-suffixed string.
//!
//! Serialization emits the preferred exact form by selecting the largest
//! supported unit that represents the value without losing precision. Zero is
//! represented as `0ms`. Deserialization accepts the broader strict grammar
//! with `ns`, `us`, `µs`, `μs`, `ms`, `s`, `min`, `h`, or `d` suffixes.

use std::{
    fmt,
    time::Duration,
};

use crate::{
    DurationParseError,
    DurationTextOptions,
    DurationUnitParseMode,
    SuffixlessDurationPolicy,
    format_duration_exact,
    parse_duration_text,
};
use serde::de::{
    Error as DeserializeError,
    Visitor,
};
use serde::{
    Deserializer,
    Serializer,
};

/// Strict Duration text profile.
const DURATION_TEXT_OPTIONS: DurationTextOptions = DurationTextOptions::new(
    SuffixlessDurationPolicy::Reject,
    DurationUnitParseMode::Strict,
);

/// Parses borrowed or owned exact unit-suffixed duration text.
struct DurationWithUnitVisitor;

impl Visitor<'_> for DurationWithUnitVisitor {
    type Value = Duration;

    /// Describes the strict unit-suffixed string accepted by this visitor.
    ///
    /// # Parameters
    ///
    /// * `formatter` - Formatter receiving the expected input description.
    ///
    /// # Returns
    ///
    /// The formatter result after writing the duration grammar.
    #[inline(always)]
    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a strict unit-suffixed duration string")
    }

    /// Parses a borrowed or transient string without taking ownership.
    ///
    /// # Parameters
    ///
    /// * `value` - Duration text supplied by the deserializer.
    ///
    /// # Returns
    ///
    /// The parsed [`Duration`].
    ///
    /// # Errors
    ///
    /// Returns the visitor error when [`parse`] rejects the text.
    #[inline(always)]
    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: DeserializeError,
    {
        parse(value).map_err(E::custom)
    }

    /// Parses an owned string supplied by a non-borrowing deserializer.
    ///
    /// # Parameters
    ///
    /// * `value` - Owned duration text supplied by the deserializer.
    ///
    /// # Returns
    ///
    /// The parsed [`Duration`].
    ///
    /// # Errors
    ///
    /// Returns the visitor error when [`parse`] rejects the text.
    #[inline(always)]
    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
    where
        E: DeserializeError,
    {
        self.visit_str(&value)
    }
}

/// Serializes a [`Duration`] as an exact string such as `"500µs"`.
///
/// # Parameters
///
/// - `duration`: Duration to serialize.
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
    let text = format(duration);
    serializer.serialize_str(&text)
}

/// Deserializes a [`Duration`] from an exact strict unit-suffixed string.
///
/// # Parameters
///
/// - `deserializer`: Serde deserializer providing a string value.
///
/// # Returns
///
/// The parsed [`Duration`].
///
/// # Errors
///
/// Returns the deserializer error when the input is not a string or parsing
/// reports [`DurationParseError::LimitExceeded`],
/// [`DurationParseError::InvalidSyntax`],
/// [`DurationParseError::NonCanonicalUnit`],
/// [`DurationParseError::UnsupportedUnit`], or
/// [`DurationParseError::OutOfRange`].
#[inline(always)]
pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_str(DurationWithUnitVisitor)
}

/// Formats a [`Duration`] with the largest exact supported unit.
///
/// # Parameters
///
/// - `duration`: Duration to format.
///
/// # Returns
///
/// A preferred unit-suffixed string. Zero is formatted as `0ms`.
#[must_use]
#[inline(always)]
pub fn format(duration: &Duration) -> String {
    format_duration_exact(*duration)
}

/// Parses a [`Duration`] from a string with a supported unit.
///
/// Supported strict suffixes are `ns`, `us`, `µs`, `μs`, `ms`, `s`, `min`,
/// `h`, and `d`. Bare integers and the Lenient-only `m` alias are rejected.
///
/// # Parameters
///
/// - `text`: Duration text to parse.
///
/// # Returns
///
/// The parsed [`Duration`].
///
/// # Errors
///
/// Returns [`DurationParseError::LimitExceeded`] when the source text exceeds
/// the parser's length limit,
/// [`DurationParseError::InvalidSyntax`] for malformed text,
/// [`DurationParseError::NonCanonicalUnit`] for a supported alias,
/// [`DurationParseError::UnsupportedUnit`] for an unknown unit, and
/// [`DurationParseError::OutOfRange`] when the value cannot fit in a
/// [`Duration`].
///
/// # Examples
///
/// ```
/// use std::time::Duration;
///
/// use qubit_datatype::serde::duration_with_unit;
///
/// assert_eq!(
///     duration_with_unit::parse("42µs"),
///     Ok(Duration::from_micros(42))
/// );
/// ```
#[inline(always)]
pub fn parse(text: &str) -> Result<Duration, DurationParseError> {
    parse_duration_text(text, &DURATION_TEXT_OPTIONS)
}
