// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Builder for numeric conversion policy.

use super::NumericConversionPolicy;
use crate::converter::options::FloatRoundingPolicy;
use crate::converter::options::FractionalToIntegerPolicy;

/// Builder for [`NumericConversionPolicy`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NumericConversionPolicyBuilder {
    /// Numeric policy being configured.
    pub(super) policy: NumericConversionPolicy,
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
