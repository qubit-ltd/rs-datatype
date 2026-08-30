// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Builder for structured conversion limits.

use super::StructuredConversionLimits;

/// Builder for [`StructuredConversionLimits`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StructuredConversionLimitsBuilder {
    /// Structured limits being configured.
    pub(super) limits: StructuredConversionLimits,
}

impl StructuredConversionLimitsBuilder {
    /// Creates a builder initialized with the documented defaults.
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            limits: StructuredConversionLimits::default(),
        }
    }

    /// Configures the normalized text byte maximum.
    #[inline(always)]
    #[must_use]
    pub fn max_text_bytes(self, maximum: u64) -> Self {
        Self {
            limits: self.limits.with_max_text_bytes(maximum),
        }
    }

    /// Configures the depth maximum.
    #[inline(always)]
    #[must_use]
    pub fn max_depth(self, maximum: u64) -> Self {
        Self {
            limits: self.limits.with_max_depth(maximum),
        }
    }

    /// Configures the sequence item maximum.
    #[inline(always)]
    #[must_use]
    pub fn max_sequence_items(self, maximum: u64) -> Self {
        Self {
            limits: self.limits.with_max_sequence_items(maximum),
        }
    }

    /// Configures the map entry maximum.
    #[inline(always)]
    #[must_use]
    pub fn max_map_entries(self, maximum: u64) -> Self {
        Self {
            limits: self.limits.with_max_map_entries(maximum),
        }
    }

    /// Builds the configured structured limits.
    #[inline]
    #[must_use]
    pub fn build(self) -> StructuredConversionLimits {
        self.limits
    }
}
