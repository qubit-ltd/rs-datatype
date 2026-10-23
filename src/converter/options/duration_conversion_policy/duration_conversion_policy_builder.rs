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
/// use qubit_datatype::{DurationConversionPolicyBuilder, DurationUnit};
///
/// let policy = DurationConversionPolicyBuilder::new()
///     .numeric_input_unit(DurationUnit::Seconds)
///     .output_unit(DurationUnit::Milliseconds)
///     .build();
/// assert_eq!(policy.numeric_input_unit(), DurationUnit::Seconds);
/// assert_eq!(policy.output_unit(), DurationUnit::Milliseconds);
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

    /// Configures the numeric input unit.
    #[inline]
    #[must_use]
    pub fn numeric_input_unit(self, unit: DurationUnit) -> Self {
        Self {
            policy: self.policy.with_numeric_input_unit(unit),
        }
    }

    /// Configures suffixless string handling.
    #[inline]
    #[must_use]
    pub fn suffixless_string_policy(self, policy: SuffixlessDurationPolicy) -> Self {
        Self {
            policy: self.policy.with_suffixless_string_policy(policy),
        }
    }

    /// Configures explicit unit parsing.
    #[inline]
    #[must_use]
    pub const fn unit_parse_mode(self, mode: DurationUnitParseMode) -> Self {
        Self {
            policy: self.policy.with_unit_parse_mode(mode),
        }
    }

    /// Configures the output unit.
    #[inline]
    #[must_use]
    pub fn output_unit(self, unit: DurationUnit) -> Self {
        Self {
            policy: self.policy.with_output_unit(unit),
        }
    }

    /// Configures whether formatted output includes a unit suffix.
    #[inline]
    #[must_use]
    pub fn append_unit_suffix(self, enabled: bool) -> Self {
        Self {
            policy: self.policy.with_append_unit_suffix(enabled),
        }
    }

    /// Configures duration rounding.
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
