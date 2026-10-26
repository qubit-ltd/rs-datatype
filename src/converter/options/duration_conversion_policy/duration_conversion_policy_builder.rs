// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Builder for duration conversion policy.

use super::DurationConversionPolicy;
use crate::converter::options::DurationRoundingPolicy;
use crate::duration::DurationUnit;
use crate::duration::DurationUnitParseMode;
use crate::duration::SuffixlessDurationPolicy;

/// Builder for [`DurationConversionPolicy`].
///
/// # Examples
///
/// ```
/// use std::time::Duration;
///
/// use qubit_datatype::{
///     ConversionLimits, ConversionPolicy, DataConverter, DurationConversionPolicyBuilder,
///     DurationUnit, SuffixlessDurationPolicy,
/// };
///
/// let policy = DurationConversionPolicyBuilder::new()
///     .numeric_input_unit(DurationUnit::Seconds)
///     .suffixless_string_policy(SuffixlessDurationPolicy::Reject)
///     .output_unit(DurationUnit::Milliseconds)
///     .build();
/// let conversion = ConversionPolicy::builder().duration_policy(policy).build();
/// let limits = ConversionLimits::default();
///
/// assert_eq!(
///     DataConverter::from(2_u64).to_with::<Duration>(&conversion, &limits),
///     Ok(Duration::from_secs(2)),
/// );
/// assert_eq!(
///     DataConverter::from("2ms").to_with::<Duration>(&conversion, &limits),
///     Ok(Duration::from_millis(2)),
/// );
/// assert!(DataConverter::from("2").to_with::<Duration>(&conversion, &limits).is_err());
/// assert_eq!(
///     DataConverter::from(Duration::from_secs(2)).to_with::<String>(&conversion, &limits),
///     Ok("2000ms".to_owned()),
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DurationConversionPolicyBuilder {
    /// Duration policy being configured.
    pub(super) policy: DurationConversionPolicy,
}

impl DurationConversionPolicyBuilder {
    /// Creates a builder initialized with the documented defaults.
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            policy: DurationConversionPolicy::default(),
        }
    }

    /// Sets the unit assigned to integer inputs converted to
    /// [`std::time::Duration`].
    ///
    /// This does not affect explicit unit suffixes in text or the handling of
    /// suffixless text; see [`Self::suffixless_string_policy`].
    ///
    /// # Parameters
    ///
    /// * `unit` - Unit used to interpret integer input.
    ///
    /// # Returns
    ///
    /// The builder with its numeric input unit set. See
    /// [`DurationConversionPolicy::numeric_input_unit`].
    #[inline]
    #[must_use]
    pub fn numeric_input_unit(self, unit: DurationUnit) -> Self {
        Self {
            policy: self.policy.with_numeric_input_unit(unit),
        }
    }

    /// Sets how duration text without an explicit unit suffix is interpreted.
    ///
    /// Explicit suffixes take precedence and are parsed according to
    /// [`Self::unit_parse_mode`]. This setting does not affect integer input;
    /// see [`Self::numeric_input_unit`].
    ///
    /// # Parameters
    ///
    /// * `policy` - Whether suffixless text is rejected or assigned a unit.
    ///
    /// # Returns
    ///
    /// The builder with its suffixless text policy set. See
    /// [`DurationConversionPolicy::suffixless_string_policy`].
    #[inline]
    #[must_use]
    pub fn suffixless_string_policy(self, policy: SuffixlessDurationPolicy) -> Self {
        Self {
            policy: self.policy.with_suffixless_string_policy(policy),
        }
    }

    /// Sets how strictly explicit duration unit symbols are parsed.
    ///
    /// This affects only text that contains a unit suffix; it does not choose
    /// a unit for suffixless text or integer input.
    ///
    /// # Parameters
    ///
    /// * `mode` - Whether explicit unit parsing accepts lenient aliases.
    ///
    /// # Returns
    ///
    /// The builder with its explicit unit parse mode set. See
    /// [`DurationConversionPolicy::unit_parse_mode`].
    #[inline]
    #[must_use]
    pub const fn unit_parse_mode(self, mode: DurationUnitParseMode) -> Self {
        Self {
            policy: self.policy.with_unit_parse_mode(mode),
        }
    }

    /// Sets the unit used when converting a [`std::time::Duration`] to an
    /// integer or text.
    ///
    /// This does not change the unit assigned to integer input or the parsing
    /// of duration text.
    ///
    /// # Parameters
    ///
    /// * `unit` - Unit used to represent duration output.
    ///
    /// # Returns
    ///
    /// The builder with its output unit set. See
    /// [`DurationConversionPolicy::output_unit`].
    #[inline]
    #[must_use]
    pub fn output_unit(self, unit: DurationUnit) -> Self {
        Self {
            policy: self.policy.with_output_unit(unit),
        }
    }

    /// Sets whether formatted duration text includes its unit suffix.
    ///
    /// This affects string output only; it does not change integer output or
    /// the units used to parse input.
    ///
    /// # Parameters
    ///
    /// * `enabled` - Whether formatted duration text includes the unit suffix.
    ///
    /// # Returns
    ///
    /// The builder with its suffix formatting setting. See
    /// [`DurationConversionPolicy::append_unit_suffix`].
    #[inline]
    #[must_use]
    pub fn append_unit_suffix(self, enabled: bool) -> Self {
        Self {
            policy: self.policy.with_append_unit_suffix(enabled),
        }
    }

    /// Sets how output is rounded when the selected unit cannot represent a
    /// duration exactly.
    ///
    /// This affects conversion output, not input parsing.
    ///
    /// # Parameters
    ///
    /// * `policy` - Rounding behavior for duration output.
    ///
    /// # Returns
    ///
    /// The builder with its rounding policy set. See
    /// [`DurationConversionPolicy::rounding_policy`].
    #[inline]
    #[must_use]
    pub const fn rounding_policy(self, policy: DurationRoundingPolicy) -> Self {
        Self {
            policy: self.policy.with_rounding_policy(policy),
        }
    }

    /// Builds the configured duration policy.
    ///
    /// # Returns
    ///
    /// The duration policy assembled from the configured input and output
    /// behavior.
    #[inline]
    pub fn build(self) -> DurationConversionPolicy {
        self.policy
    }
}
