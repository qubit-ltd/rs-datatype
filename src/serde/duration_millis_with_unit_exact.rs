// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Exact Serde adapter for [`std::time::Duration`] as millisecond text.
//!
//! Serialization accepts only durations exactly representable as whole
//! milliseconds and emits the canonical `<integer>ms` form. Deserialization
//! uses the same strict grammar as [`super::duration_millis_with_unit`].

use std::time::Duration;

use serde::Deserializer;
use serde::Serializer;
use serde::ser::Error as SerializeError;

use crate::DurationParseError;
use crate::DurationPrecisionLossError;

/// Deserializes strict canonical millisecond text.
///
/// # Type Parameters
///
/// * `'de` - Lifetime of data borrowed by the deserializer.
/// * `D` - Serde deserializer type.
///
/// # Parameters
///
/// * `deserializer` - Deserializer producing the millisecond text.
///
/// # Errors
///
/// Returns the deserializer error when the input is not a string or does not
/// match the strict `<integer>ms` grammar and Duration range.
#[inline(always)]
pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    super::duration_millis_with_unit::deserialize(deserializer)
}

/// Parses strict canonical millisecond text.
///
/// # Parameters
///
/// * `text` - Canonical `<integer>ms` text to parse.
///
/// # Errors
///
/// Returns [`DurationParseError`] when the input exceeds the default text
/// limit, has invalid syntax, or cannot fit in [`Duration`].
#[inline(always)]
pub fn parse(text: &str) -> Result<Duration, DurationParseError> {
    super::duration_millis_with_unit::parse(text)
}

/// Serializes a Duration only when it has exact millisecond precision.
///
/// # Type Parameters
///
/// * `S` - Serde serializer type.
///
/// # Parameters
///
/// * `duration` - Duration to serialize.
/// * `serializer` - Serializer receiving the canonical text.
///
/// # Errors
///
/// Returns the serializer error when the Duration contains submillisecond
/// precision or the serializer rejects the resulting string.
#[inline]
pub fn serialize<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let text = format(duration).map_err(S::Error::custom)?;
    serializer.serialize_str(&text)
}

/// Formats a Duration in exact canonical millisecond text.
///
/// # Parameters
///
/// * `duration` - Duration to format.
///
/// # Errors
///
/// Returns [`DurationPrecisionLossError`] if formatting as milliseconds would
/// discard any nanoseconds.
#[inline]
pub fn format(duration: &Duration) -> Result<String, DurationPrecisionLossError> {
    if !duration.subsec_nanos().is_multiple_of(1_000_000) {
        return Err(DurationPrecisionLossError);
    }
    Ok(format!("{}ms", duration.as_millis()))
}
