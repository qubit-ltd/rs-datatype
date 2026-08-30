// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Builder for numeric conversion limits.

use super::NumericConversionLimits;

/// Builder for [`NumericConversionLimits`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NumericConversionLimitsBuilder {
    /// Numeric limits being configured.
    pub(super) limits: NumericConversionLimits,
}

impl NumericConversionLimitsBuilder {
    /// Creates a builder initialized with the documented defaults.
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            limits: NumericConversionLimits::default(),
        }
    }

    /// Configures the numeric text byte maximum.
    #[inline(always)]
    #[must_use]
    pub fn max_text_bytes(self, maximum: u64) -> Self {
        Self {
            limits: self.limits.with_max_text_bytes(maximum),
        }
    }

    /// Configures the BigInt digit maximum.
    #[inline(always)]
    #[must_use]
    pub fn max_big_integer_digits(self, maximum: u64) -> Self {
        Self {
            limits: self.limits.with_max_big_integer_digits(maximum),
        }
    }

    /// Configures the BigDecimal coefficient digit maximum.
    #[inline]
    #[must_use]
    pub fn max_big_decimal_coefficient_digits(self, maximum: u64) -> Self {
        Self {
            limits: self.limits.with_max_big_decimal_coefficient_digits(maximum),
        }
    }

    /// Configures the BigDecimal scale magnitude maximum.
    #[inline]
    #[must_use]
    pub fn max_big_decimal_scale_magnitude(self, maximum: u64) -> Self {
        Self {
            limits: self.limits.with_max_big_decimal_scale_magnitude(maximum),
        }
    }

    /// Builds the configured numeric limits.
    #[inline]
    #[must_use]
    pub fn build(self) -> NumericConversionLimits {
        self.limits
    }
}
