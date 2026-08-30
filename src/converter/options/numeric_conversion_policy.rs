// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Independently configurable numeric conversion policy.
// qubit-style: allow multiple-public-types

use serde::Deserialize;
use serde::Serialize;

use super::FloatRoundingPolicy;
use super::FractionalToIntegerPolicy;

/// Groups semantic policies for numeric conversion.
///
/// # Examples
///
/// ```
/// use qubit_datatype::NumericConversionPolicy;
/// use qubit_datatype::NumericConversionPolicyBuilder;
///
/// let policy = NumericConversionPolicy::builder().build();
/// assert_eq!(policy, NumericConversionPolicyBuilder::new().build());
/// ```
#[must_use]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct NumericConversionPolicy {
    /// Policy governing fractional numeric conversions to integer targets.
    fractional_to_integer: FractionalToIntegerPolicy,
    /// Rounding policy for existing numeric values converted to floating
    /// point.
    numeric_to_float: FloatRoundingPolicy,
    /// Rounding policy for text parsed into floating-point targets.
    text_to_float: FloatRoundingPolicy,
}

impl NumericConversionPolicy {
    /// Creates a builder initialized with the default numeric policy.
    #[inline]
    #[must_use]
    pub fn builder() -> NumericConversionPolicyBuilder {
        NumericConversionPolicyBuilder::new()
    }

    /// Converts this policy into a builder for explicit customization.
    #[inline]
    #[must_use]
    pub fn into_builder(self) -> NumericConversionPolicyBuilder {
        NumericConversionPolicyBuilder { policy: self }
    }
    /// Creates the strict numeric conversion profile used by [`Default`].
    ///
    /// # Returns
    ///
    /// Options rejecting fractional loss and floating-point rounding.
    #[inline(always)]
    pub fn strict() -> Self {
        Self {
            fractional_to_integer: FractionalToIntegerPolicy::Reject,
            numeric_to_float: FloatRoundingPolicy::Exact,
            text_to_float: FloatRoundingPolicy::Exact,
        }
    }

    /// Creates a profile permitting documented truncation and float rounding.
    ///
    /// # Returns
    ///
    /// Options permitting truncation and nearest-even float rounding.
    #[inline(always)]
    pub fn lossy() -> Self {
        Self {
            fractional_to_integer: FractionalToIntegerPolicy::Truncate,
            numeric_to_float: FloatRoundingPolicy::NearestEven,
            text_to_float: FloatRoundingPolicy::NearestEven,
        }
    }

    /// Creates a profile that relaxes only textual floating-point parsing.
    ///
    /// # Returns
    ///
    /// Options allowing nearest-even rounding only for text-to-float parsing.
    #[inline(always)]
    pub fn env_friendly() -> Self {
        Self {
            fractional_to_integer: FractionalToIntegerPolicy::Reject,
            numeric_to_float: FloatRoundingPolicy::Exact,
            text_to_float: FloatRoundingPolicy::NearestEven,
        }
    }

    /// Returns the fractional-to-integer conversion policy.
    ///
    /// # Returns
    ///
    /// The configured fractional conversion policy.
    #[inline(always)]
    #[must_use]
    pub const fn fractional_to_integer(&self) -> FractionalToIntegerPolicy {
        self.fractional_to_integer
    }

    /// Returns a copy with a different fractional-to-integer policy.
    ///
    /// # Parameters
    ///
    /// * `policy` - New fractional conversion policy.
    ///
    /// # Returns
    ///
    /// Updated options.
    #[inline(always)]
    pub(crate) const fn with_fractional_to_integer(mut self, policy: FractionalToIntegerPolicy) -> Self {
        self.fractional_to_integer = policy;
        self
    }

    /// Returns the existing-numeric-to-float rounding policy.
    ///
    /// # Returns
    ///
    /// The rounding policy for typed numeric sources.
    #[inline(always)]
    #[must_use]
    pub const fn numeric_to_float(&self) -> FloatRoundingPolicy {
        self.numeric_to_float
    }

    /// Returns a copy with a different numeric-to-float policy.
    ///
    /// # Parameters
    ///
    /// * `policy` - New typed-numeric-to-float rounding policy.
    ///
    /// # Returns
    ///
    /// Updated options.
    #[inline(always)]
    pub(crate) const fn with_numeric_to_float(mut self, policy: FloatRoundingPolicy) -> Self {
        self.numeric_to_float = policy;
        self
    }

    /// Returns the text-to-float rounding policy.
    ///
    /// # Returns
    ///
    /// The rounding policy for textual float sources.
    #[inline(always)]
    #[must_use]
    pub const fn text_to_float(&self) -> FloatRoundingPolicy {
        self.text_to_float
    }

    /// Returns a copy with a different text-to-float policy.
    ///
    /// # Parameters
    ///
    /// * `policy` - New text-to-float rounding policy.
    ///
    /// # Returns
    ///
    /// Updated options.
    #[inline(always)]
    pub(crate) const fn with_text_to_float(mut self, policy: FloatRoundingPolicy) -> Self {
        self.text_to_float = policy;
        self
    }
}

/// Builder for [`NumericConversionPolicy`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NumericConversionPolicyBuilder {
    /// Numeric policy being configured.
    policy: NumericConversionPolicy,
}

impl NumericConversionPolicyBuilder {
    /// Creates a builder initialized with the documented defaults.
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            policy: NumericConversionPolicy::default(),
        }
    }
    /// Configures fractional-to-integer conversion.
    #[inline(always)]
    #[must_use]
    pub const fn fractional_to_integer(self, policy: FractionalToIntegerPolicy) -> Self {
        Self {
            policy: self.policy.with_fractional_to_integer(policy),
        }
    }
    /// Configures numeric-to-float conversion.
    #[inline(always)]
    #[must_use]
    pub const fn numeric_to_float(self, policy: FloatRoundingPolicy) -> Self {
        Self {
            policy: self.policy.with_numeric_to_float(policy),
        }
    }
    /// Configures text-to-float conversion.
    #[inline(always)]
    #[must_use]
    pub const fn text_to_float(self, policy: FloatRoundingPolicy) -> Self {
        Self {
            policy: self.policy.with_text_to_float(policy),
        }
    }
    /// Builds the configured numeric policy.
    #[inline]
    #[must_use]
    pub const fn build(self) -> NumericConversionPolicy {
        self.policy
    }
}

impl Default for NumericConversionPolicy {
    /// Creates strict numeric conversion policy.
    ///
    /// # Returns
    ///
    /// The strict numeric profile.
    #[inline(always)]
    fn default() -> Self {
        Self::strict()
    }
}
