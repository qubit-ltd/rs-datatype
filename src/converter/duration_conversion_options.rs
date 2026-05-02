/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! # Duration Conversion Options
//!
//! Defines options that control duration conversion.
//!

use super::duration_unit::DurationUnit;

/// Options that control [`std::time::Duration`] conversion.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DurationConversionOptions {
    /// Unit used for suffixless strings and integer conversions.
    pub unit: DurationUnit,
    /// Whether formatted duration strings include the unit suffix.
    pub append_unit_suffix: bool,
}

impl DurationConversionOptions {
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
