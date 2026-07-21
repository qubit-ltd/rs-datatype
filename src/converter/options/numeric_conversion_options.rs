// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Independently configurable numeric conversion options.

use serde::{
    Deserialize,
    Serialize,
};

use super::{
    FloatRoundingPolicy,
    FractionalToIntegerPolicy,
    NumericConversionLimits,
};

/// Groups policies and resource limits for numeric conversion.
#[must_use]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct NumericConversionOptions {
    /// Policy governing fractional numeric conversions to integer targets.
    fractional_to_integer: FractionalToIntegerPolicy,
    /// Rounding policy for existing numeric values converted to floating
    /// point.
    numeric_to_float: FloatRoundingPolicy,
    /// Rounding policy for text parsed into floating-point targets.
    text_to_float: FloatRoundingPolicy,
    /// Resource limits applied during numeric conversion.
    limits: NumericConversionLimits,
}

impl NumericConversionOptions {
    /// Creates the strict numeric conversion profile used by [`Default`].
    #[inline(always)]
    pub fn strict() -> Self {
        Self {
            fractional_to_integer: FractionalToIntegerPolicy::Reject,
            numeric_to_float: FloatRoundingPolicy::Exact,
            text_to_float: FloatRoundingPolicy::Exact,
            limits: NumericConversionLimits::default(),
        }
    }

    /// Creates a profile permitting documented truncation and float rounding.
    #[inline(always)]
    pub fn lossy() -> Self {
        Self {
            fractional_to_integer: FractionalToIntegerPolicy::Truncate,
            numeric_to_float: FloatRoundingPolicy::NearestEven,
            text_to_float: FloatRoundingPolicy::NearestEven,
            limits: NumericConversionLimits::default(),
        }
    }

    /// Creates a profile that relaxes only textual floating-point parsing.
    #[inline(always)]
    pub fn env_friendly() -> Self {
        Self {
            fractional_to_integer: FractionalToIntegerPolicy::Reject,
            numeric_to_float: FloatRoundingPolicy::Exact,
            text_to_float: FloatRoundingPolicy::NearestEven,
            limits: NumericConversionLimits::default(),
        }
    }

    /// Returns the fractional-to-integer conversion policy.
    #[inline(always)]
    pub const fn fractional_to_integer(&self) -> FractionalToIntegerPolicy {
        self.fractional_to_integer
    }

    /// Returns a copy with a different fractional-to-integer policy.
    #[inline(always)]
    pub const fn with_fractional_to_integer(
        mut self,
        policy: FractionalToIntegerPolicy,
    ) -> Self {
        self.fractional_to_integer = policy;
        self
    }

    /// Returns the existing-numeric-to-float rounding policy.
    #[inline(always)]
    pub const fn numeric_to_float(&self) -> FloatRoundingPolicy {
        self.numeric_to_float
    }

    /// Returns a copy with a different numeric-to-float policy.
    #[inline(always)]
    pub const fn with_numeric_to_float(
        mut self,
        policy: FloatRoundingPolicy,
    ) -> Self {
        self.numeric_to_float = policy;
        self
    }

    /// Returns the text-to-float rounding policy.
    #[inline(always)]
    pub const fn text_to_float(&self) -> FloatRoundingPolicy {
        self.text_to_float
    }

    /// Returns a copy with a different text-to-float policy.
    #[inline(always)]
    pub const fn with_text_to_float(
        mut self,
        policy: FloatRoundingPolicy,
    ) -> Self {
        self.text_to_float = policy;
        self
    }

    /// Returns the numeric conversion resource limits.
    #[inline(always)]
    pub const fn limits(&self) -> &NumericConversionLimits {
        &self.limits
    }

    /// Returns a copy with different numeric conversion resource limits.
    #[inline(always)]
    pub const fn with_limits(
        mut self,
        limits: NumericConversionLimits,
    ) -> Self {
        self.limits = limits;
        self
    }
}

impl Default for NumericConversionOptions {
    /// Creates strict numeric conversion options.
    #[inline(always)]
    fn default() -> Self {
        Self::strict()
    }
}
