// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Serde adapter for [`std::time::Duration`] as whole milliseconds.
//!
//! Serialization emits a rounded `u64` millisecond value. Deserialization
//! accepts a `u64` millisecond value and converts it back to [`Duration`].

use std::time::Duration;

use crate::DurationUnit;
use serde::ser::Error as SerializeError;
use serde::{
    Deserialize,
    Deserializer,
    Serializer,
};

/// Converts a duration to whole milliseconds using half-up rounding.
///
/// # Parameters
///
/// - `duration`: Duration to convert.
///
/// # Returns
///
/// The rounded millisecond count.
#[must_use]
#[inline(always)]
pub(super) fn rounded_millis(duration: Duration) -> u128 {
    DurationUnit::Milliseconds.rounded_units(duration)
}

/// Serializes a [`Duration`] as a rounded `u64` millisecond count.
///
/// # Parameters
///
/// - `duration`: Duration to serialize.
/// - `serializer`: Serde serializer receiving the millisecond count.
///
/// # Returns
///
/// The serializer result.
///
/// # Errors
///
/// Returns the serializer error if converting or writing the integer value
/// fails.
#[inline]
pub fn serialize<S>(
    duration: &Duration,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let millis = u64::try_from(rounded_millis(*duration))
        .map_err(|_| S::Error::custom("duration exceeds u64 milliseconds"))?;
    serializer.serialize_u64(millis)
}

/// Deserializes a [`Duration`] from a `u64` millisecond count.
///
/// # Parameters
///
/// - `deserializer`: Serde deserializer providing a millisecond count.
///
/// # Returns
///
/// A [`Duration`] with millisecond precision.
///
/// # Errors
///
/// Returns the deserializer error when the input is not a valid `u64`.
#[inline(always)]
pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    let millis = u64::deserialize(deserializer)?;
    Ok(Duration::from_millis(millis))
}
