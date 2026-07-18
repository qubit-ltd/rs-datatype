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

use crate::duration::{
    DurationUnit,
    DurationUnitSuffixSet,
    SuffixlessDurationPolicy,
};
use serde::{
    Deserialize,
    Serialize,
};

use super::DurationRoundingPolicy;

/// Controls scalar conversions to and from [`std::time::Duration`].
///
/// Numeric input, suffixless string input, and output formatting use separate
/// policies so a protocol can assign different units to each direction.
/// Explicit unit suffixes in input strings always override the suffixless
/// string policy. [`Self::append_unit_suffix`] affects only string output.
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
/// let duration = DurationConversionOptions::default()
///     .with_numeric_input_unit(DurationUnit::Seconds)
///     .with_suffixless_string_policy(SuffixlessDurationPolicy::Reject)
///     .with_output_unit(DurationUnit::Milliseconds);
/// let options = DataConversionOptions::strict().with_duration_options(duration);
///
/// assert_eq!(
///     DataConverter::from(2_u64).to_with::<Duration>(&options),
///     Ok(Duration::from_secs(2)),
/// );
/// assert_eq!(
///     DataConverter::from(Duration::from_secs(2)).to_with::<String>(&options),
///     Ok("2000ms".to_owned()),
/// );
/// assert!(DataConverter::from("2").to_with::<Duration>(&options).is_err());
/// ```
#[must_use]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct DurationConversionOptions {
    /// Unit assigned to integer sources converted to [`std::time::Duration`].
    numeric_input_unit: DurationUnit,
    /// Policy for Duration strings that omit an explicit unit suffix.
    suffixless_string_policy: SuffixlessDurationPolicy,
    /// Set of explicit unit suffixes accepted in Duration text.
    unit_suffix_set: DurationUnitSuffixSet,
    /// Unit used when converting a Duration to an integer or string.
    output_unit: DurationUnit,
    /// Whether formatted duration strings include the unit suffix.
    append_unit_suffix: bool,
    /// Policy for Duration output that has a remainder in the selected unit.
    rounding_policy: DurationRoundingPolicy,
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
    #[inline(always)]
    pub fn env_friendly() -> Self {
        Self::default()
    }

    /// Returns the unit assigned to integer sources.
    #[inline(always)]
    pub const fn numeric_input_unit(&self) -> DurationUnit {
        self.numeric_input_unit
    }

    /// Returns a copy with a different numeric input unit.
    ///
    /// # Parameters
    ///
    /// * `unit` - Unit assigned to integer sources converted to a Duration.
    ///
    /// # Returns
    ///
    /// Updated options.
    #[inline(always)]
    pub fn with_numeric_input_unit(mut self, unit: DurationUnit) -> Self {
        self.numeric_input_unit = unit;
        self
    }

    /// Returns the policy for Duration text without an explicit suffix.
    #[inline(always)]
    pub const fn suffixless_string_policy(&self) -> SuffixlessDurationPolicy {
        self.suffixless_string_policy
    }

    /// Returns a copy with a different suffixless string policy.
    ///
    /// # Parameters
    ///
    /// * `policy` - Rule for Duration strings without a unit suffix.
    ///
    /// # Returns
    ///
    /// Updated options.
    #[inline(always)]
    pub fn with_suffixless_string_policy(
        mut self,
        policy: SuffixlessDurationPolicy,
    ) -> Self {
        self.suffixless_string_policy = policy;
        self
    }

    /// Returns the set of explicit suffixes accepted in Duration text.
    #[inline(always)]
    pub const fn unit_suffix_set(&self) -> DurationUnitSuffixSet {
        self.unit_suffix_set
    }

    /// Returns a copy with a different accepted Duration suffix set.
    #[inline(always)]
    pub const fn with_unit_suffix_set(
        mut self,
        unit_suffix_set: DurationUnitSuffixSet,
    ) -> Self {
        self.unit_suffix_set = unit_suffix_set;
        self
    }

    /// Returns the unit used for Duration output.
    #[inline(always)]
    pub const fn output_unit(&self) -> DurationUnit {
        self.output_unit
    }

    /// Returns a copy with a different Duration output unit.
    ///
    /// # Parameters
    ///
    /// * `unit` - Unit used for Duration-to-integer and Duration-to-string
    ///   conversions.
    ///
    /// # Returns
    ///
    /// Updated options.
    #[inline(always)]
    pub fn with_output_unit(mut self, unit: DurationUnit) -> Self {
        self.output_unit = unit;
        self
    }

    /// Returns whether formatted Duration text includes a unit suffix.
    #[inline(always)]
    pub const fn append_unit_suffix(&self) -> bool {
        self.append_unit_suffix
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
    #[inline(always)]
    pub fn with_append_unit_suffix(mut self, append_unit_suffix: bool) -> Self {
        self.append_unit_suffix = append_unit_suffix;
        self
    }

    /// Returns the Duration output rounding policy.
    #[inline(always)]
    pub const fn rounding_policy(&self) -> DurationRoundingPolicy {
        self.rounding_policy
    }

    /// Returns a copy with a different Duration output rounding policy.
    #[inline(always)]
    pub const fn with_rounding_policy(
        mut self,
        rounding_policy: DurationRoundingPolicy,
    ) -> Self {
        self.rounding_policy = rounding_policy;
        self
    }
}

impl Default for DurationConversionOptions {
    /// Creates default duration conversion options.
    fn default() -> Self {
        Self {
            numeric_input_unit: DurationUnit::default(),
            suffixless_string_policy: SuffixlessDurationPolicy::default(),
            unit_suffix_set: DurationUnitSuffixSet::default(),
            output_unit: DurationUnit::default(),
            append_unit_suffix: true,
            rounding_policy: DurationRoundingPolicy::default(),
        }
    }
}
