// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Numeric Conversion Policy
//!
//! Defines whether numeric conversions may lose information.

use serde::{
    Deserialize,
    Serialize,
};

/// Controls whether numeric conversions must preserve the source value.
///
/// [`Self::Exact`] rejects fractional truncation, integer-to-float rounding,
/// and duration unit remainders. [`Self::Lossy`] permits the documented
/// conversion for each target family but still rejects non-finite or
/// out-of-range results where no target value exists.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum NumericConversionPolicy {
    /// Reject conversions that require truncation, rounding, or precision
    /// loss.
    #[default]
    Exact,
    /// Permit the explicitly documented lossy conversion behavior.
    Lossy,
}

impl NumericConversionPolicy {
    /// Returns the numeric policy used for environment-variable input.
    ///
    /// Environment variables are textual, but that does not imply permission
    /// to truncate or round their numeric values. The profile therefore uses
    /// exact conversion today while keeping that decision local to this type.
    ///
    /// # Returns
    ///
    /// The numeric conversion policy for environment-variable values.
    #[inline(always)]
    pub const fn env_friendly() -> Self {
        Self::Exact
    }
}
