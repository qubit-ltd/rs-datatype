// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Value limits for duration conversion.

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

    /// Returns the duration source text byte maximum.
    ///
    /// # Returns
    ///
    /// Maximum accepted UTF-8 byte length for one duration source.
    #[inline(always)]
    pub const fn max_text_bytes(&self) -> usize {
        self.max_text_bytes
    }

    /// Returns a copy with a different duration source text byte maximum.
    ///
    /// # Parameters
    ///
    /// * `maximum` - Maximum accepted UTF-8 source bytes.
    ///
    /// # Returns
    ///
    /// Updated limits.
    #[inline(always)]
    pub const fn with_max_text_bytes(mut self, maximum: usize) -> Self {
        self.max_text_bytes = maximum;
        self
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
