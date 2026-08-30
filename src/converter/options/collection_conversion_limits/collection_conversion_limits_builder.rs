// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Builder for scalar-string collection conversion limits.

use super::CollectionConversionLimits;

/// Builder for [`CollectionConversionLimits`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CollectionConversionLimitsBuilder {
    /// Collection limits being configured.
    pub(super) limits: CollectionConversionLimits,
}

impl CollectionConversionLimitsBuilder {
    /// Creates a builder initialized with the documented defaults.
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self {
            limits: CollectionConversionLimits {
                max_source_bytes: CollectionConversionLimits::DEFAULT_MAX_SOURCE_BYTES,
                max_items: CollectionConversionLimits::DEFAULT_MAX_ITEMS,
            },
        }
    }

    /// Configures the maximum UTF-8 source size.
    #[inline(always)]
    #[must_use]
    pub const fn max_source_bytes(mut self, maximum: u64) -> Self {
        self.limits.max_source_bytes = maximum;
        self
    }

    /// Configures the maximum retained item count.
    #[inline(always)]
    #[must_use]
    pub const fn max_items(mut self, maximum: u64) -> Self {
        self.limits.max_items = maximum;
        self
    }

    /// Builds the configured collection limits.
    #[inline]
    #[must_use]
    pub const fn build(self) -> CollectionConversionLimits {
        self.limits
    }
}
