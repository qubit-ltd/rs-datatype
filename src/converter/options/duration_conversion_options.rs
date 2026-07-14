// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Duration Conversion Options
//!
//! Defines options that control duration conversion.

use super::duration_unit::DurationUnit;
use serde::{
    Deserialize,
    Serialize,
};

/// Controls scalar conversions to and from [`std::time::Duration`].
///
/// [`Self::unit`] is used for integers and for duration strings without a
/// suffix. [`Self::append_unit_suffix`] affects duration-to-string formatting;
/// it does not change accepted input suffixes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct DurationConversionOptions {
    /// Unit used for suffixless strings and integer conversions.
    pub unit: DurationUnit,
    /// Whether formatted duration strings include the unit suffix.
    pub append_unit_suffix: bool,
}

impl DurationConversionOptions {
    /// Creates duration options for environment-variable input.
    ///
    /// The current environment profile intentionally matches the default
    /// millisecond representation. Keeping the constructor here allows the
    /// profile to evolve without changing composite option builders.
    ///
    /// # Returns
    ///
    /// Environment-friendly duration conversion options.
    #[inline]
    pub fn env_friendly() -> Self {
        Self::default()
    }

    /// Returns a copy with a different duration unit.
    ///
    /// # Parameters
    ///
    /// * `unit` - New duration unit.
    ///
    /// # Returns
    ///
    /// Updated options.
    #[must_use]
    pub fn with_unit(mut self, unit: DurationUnit) -> Self {
        self.unit = unit;
        self
    }

    /// Returns a copy with suffix formatting enabled or disabled.
    ///
    /// # Parameters
    ///
    /// * `append_unit_suffix` - Whether formatted strings include the unit
    ///   suffix.
    ///
    /// # Returns
    ///
    /// Updated options.
    #[must_use]
    pub fn with_append_unit_suffix(mut self, append_unit_suffix: bool) -> Self {
        self.append_unit_suffix = append_unit_suffix;
        self
    }
}

impl Default for DurationConversionOptions {
    /// Creates default duration conversion options.
    fn default() -> Self {
        Self {
            unit: DurationUnit::default(),
            append_unit_suffix: true,
        }
    }
}
