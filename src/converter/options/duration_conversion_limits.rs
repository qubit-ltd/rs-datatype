// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Value limits for duration conversion.
// qubit-style: allow multiple-public-types

use serde::Deserialize;
use serde::Serialize;

use crate::duration::DurationTextOptions;

/// Limits the normalized text accepted by one duration conversion.
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct DurationConversionLimits {
    /// Maximum accepted duration source text length in UTF-8 bytes.
    max_text_bytes: usize,
}

impl DurationConversionLimits {
    /// Default maximum duration source text length in UTF-8 bytes.
    pub const DEFAULT_MAX_TEXT_BYTES: usize = DurationTextOptions::DEFAULT_MAX_TEXT_BYTES;

    /// Creates a builder initialized with the default duration limits.
    #[inline]
    #[must_use]
    pub const fn builder() -> DurationConversionLimitsBuilder {
        DurationConversionLimitsBuilder::new()
    }

    /// Converts these limits into a builder for explicit customization.
    #[inline]
    #[must_use]
    pub const fn into_builder(self) -> DurationConversionLimitsBuilder {
        DurationConversionLimitsBuilder { limits: self }
    }

    /// Returns the duration source text byte maximum.
    ///
    /// # Returns
    ///
    /// Maximum accepted UTF-8 byte length for one duration source.
    #[inline(always)]
    #[must_use]
    pub const fn max_text_bytes(&self) -> usize {
        self.max_text_bytes
    }
}

/// Builder for [`DurationConversionLimits`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DurationConversionLimitsBuilder {
    /// Duration limits being configured.
    limits: DurationConversionLimits,
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

impl Default for DurationConversionLimits {
    /// Creates the documented finite duration limits.
    #[inline(always)]
    fn default() -> Self {
        Self {
            max_text_bytes: Self::DEFAULT_MAX_TEXT_BYTES,
        }
    }
}
