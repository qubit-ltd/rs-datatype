// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Independently configurable numeric conversion policy.

use serde::Deserialize;
use serde::Serialize;

use super::FloatRoundingPolicy;
use super::FractionalToIntegerPolicy;

/// Groups semantic policies for numeric conversion.
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
    pub const fn with_fractional_to_integer(mut self, policy: FractionalToIntegerPolicy) -> Self {
        self.fractional_to_integer = policy;
        self
    }

    /// Returns the existing-numeric-to-float rounding policy.
    ///
    /// # Returns
    ///
    /// The rounding policy for typed numeric sources.
    #[inline(always)]
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
    pub const fn with_numeric_to_float(mut self, policy: FloatRoundingPolicy) -> Self {
        self.numeric_to_float = policy;
        self
    }

    /// Returns the text-to-float rounding policy.
    ///
    /// # Returns
    ///
    /// The rounding policy for textual float sources.
    #[inline(always)]
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
    pub const fn with_text_to_float(mut self, policy: FloatRoundingPolicy) -> Self {
        self.text_to_float = policy;
        self
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
