// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Serde adapter for [`std::time::Duration`] as whole milliseconds.
//!
//! Serialization emits a rounded `u64` millisecond value and returns an error
//! when the rounded result exceeds `u64::MAX` milliseconds. This upper bound is
//! narrower than the full range of [`Duration`]. Deserialization accepts a
//! `u64` millisecond value and converts it back to [`Duration`].
//!
//! # Example
//!
//! ```rust
//! use std::time::Duration;
//! use serde::Serialize;
//!
//! #[derive(Serialize)]
//! struct NumericDelay {
//!     #[serde(with = "qubit_datatype::serde::duration_millis")]
//!     delay: Duration,
//! }
//!
//! #[derive(Serialize)]
//! struct TextDelay {
//!     #[serde(with = "qubit_datatype::serde::duration_millis_with_unit")]
//!     delay: Duration,
//! }
//!
//! #[derive(Serialize)]
//! struct ExactDelay {
//!     #[serde(with = "qubit_datatype::serde::duration_millis_with_unit_exact")]
//!     delay: Duration,
//! }
//!
//! fn main() {
//!     let rounded = NumericDelay { delay: Duration::from_micros(1_500) };
//!     assert_eq!(serde_json::to_value(&rounded).unwrap(), serde_json::json!({"delay": 2}));
//!     let inexact = ExactDelay { delay: Duration::from_micros(1_500) };
//!     assert!(serde_json::to_string(&inexact).is_err());
//!     let exact = ExactDelay { delay: Duration::from_millis(2) };
//!     assert_eq!(serde_json::to_value(&exact).unwrap(), serde_json::json!({"delay": "2ms"}));
//!     assert!(serde_json::to_string(&NumericDelay { delay: Duration::MAX }).is_err());
//!     assert_eq!(
//!         serde_json::to_value(&TextDelay { delay: Duration::MAX }).unwrap(),
//!         serde_json::json!({"delay": "18446744073709551615999ms"}),
//!     );
//! }
//! ```

use std::time::Duration;

use serde::Deserialize;
use serde::Deserializer;
use serde::Serializer;
use serde::ser::Error as SerializeError;

use crate::DurationUnit;

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
#[inline]
pub(super) fn rounded_millis(duration: Duration) -> u128 {
    DurationUnit::Milliseconds.rounded_units(duration)
}

/// Serializes a [`Duration`] as a rounded `u64` millisecond count.
///
/// # Type Parameters
///
/// - `S`: Serde serializer type.
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
/// Returns the serializer error if the half-up rounded millisecond count
/// exceeds `u64::MAX`, or if writing the integer value fails.
#[inline]
pub fn serialize<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let millis =
        u64::try_from(rounded_millis(*duration)).map_err(|_| S::Error::custom("duration exceeds u64 milliseconds"))?;
    serializer.serialize_u64(millis)
}

/// Deserializes a [`Duration`] from a `u64` millisecond count.
///
/// # Type Parameters
///
/// - `'de`: Lifetime of data borrowed from the deserializer input.
/// - `D`: Serde deserializer type.
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
#[inline]
pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    let millis = u64::deserialize(deserializer)?;
    Ok(Duration::from_millis(millis))
}
