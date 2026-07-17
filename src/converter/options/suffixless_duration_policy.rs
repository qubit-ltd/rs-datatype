// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Suffixless Duration Policy
//!
//! Defines how Duration strings without an explicit unit are interpreted.

use serde::{
    Deserialize,
    Serialize,
};

use super::duration_unit::DurationUnit;

/// Controls conversion of Duration strings that omit a unit suffix.
///
/// Explicitly suffixed strings such as `"10ms"` do not use this policy.
/// [`Self::Reject`] requires every Duration string to name its unit, while
/// [`Self::Assume`] assigns a configured unit to the suffixless integer.
///
/// # Examples
///
/// ```
/// use std::time::Duration;
///
/// use qubit_datatype::{
///     DataConversionOptions,
///     DataConverter,
///     DurationConversionOptions,
///     DurationUnit,
///     SuffixlessDurationPolicy,
/// };
///
/// let reject = DataConversionOptions::strict().with_duration_options(
///     DurationConversionOptions::default()
///         .with_suffixless_string_policy(SuffixlessDurationPolicy::Reject),
/// );
/// let assume_seconds = DataConversionOptions::strict().with_duration_options(
///     DurationConversionOptions::default().with_suffixless_string_policy(
///         SuffixlessDurationPolicy::Assume(DurationUnit::Seconds),
///     ),
/// );
///
/// assert!(DataConverter::from("5").to_with::<Duration>(&reject).is_err());
/// assert_eq!(
///     DataConverter::from("5").to_with::<Duration>(&assume_seconds),
///     Ok(Duration::from_secs(5)),
/// );
/// ```
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SuffixlessDurationPolicy {
    /// Reject Duration strings without an explicit unit suffix.
    Reject,
    /// Interpret suffixless integer strings using the contained unit.
    Assume(DurationUnit),
}

impl Default for SuffixlessDurationPolicy {
    /// Preserves the default convention that suffixless strings are
    /// milliseconds.
    fn default() -> Self {
        Self::Assume(DurationUnit::Milliseconds)
    }
}
