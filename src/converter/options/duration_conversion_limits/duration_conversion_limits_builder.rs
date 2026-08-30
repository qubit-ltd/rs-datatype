// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Builder for duration conversion limits.

use super::DurationConversionLimits;

/// Builder for [`DurationConversionLimits`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DurationConversionLimitsBuilder {
    /// Duration limits being configured.
    pub(super) limits: DurationConversionLimits,
}

impl DurationConversionLimitsBuilder {
    /// Creates a builder initialized with the documented default.
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self {
            limits: DurationConversionLimits {
                max_text_bytes: DurationConversionLimits::DEFAULT_MAX_TEXT_BYTES,
            },
        }
    }

    /// Configures the maximum duration source text size.
    #[inline(always)]
    #[must_use]
    pub const fn max_text_bytes(mut self, maximum: usize) -> Self {
        self.limits.max_text_bytes = maximum;
        self
    }

    /// Builds the configured duration limits.
    #[inline]
    #[must_use]
    pub const fn build(self) -> DurationConversionLimits {
        self.limits
    }
}
