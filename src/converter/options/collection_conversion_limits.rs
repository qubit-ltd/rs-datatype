// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Value limits for scalar-string collection conversion.

use serde::Deserialize;
use serde::Serialize;

/// Limits the source and retained items of one collection conversion.
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct CollectionConversionLimits {
    /// Maximum UTF-8 bytes accepted from one scalar collection source.
    max_source_bytes: usize,

    /// Maximum items retained by one scalar collection conversion.
    max_items: usize,
}

impl CollectionConversionLimits {
    /// Default maximum scalar collection source length in bytes.
    pub const DEFAULT_MAX_SOURCE_BYTES: usize = 1_048_576;

    /// Default maximum number of retained scalar collection items.
    pub const DEFAULT_MAX_ITEMS: usize = 65_536;

    /// Returns the source text byte maximum.
    ///
    /// # Returns
    ///
    /// Maximum accepted UTF-8 byte length for one scalar collection source.
    #[inline(always)]
    pub const fn max_source_bytes(&self) -> usize {
        self.max_source_bytes
    }

    /// Returns a copy with a different source text byte maximum.
    ///
    /// # Parameters
    ///
    /// * `maximum` - Maximum accepted UTF-8 source bytes.
    ///
    /// # Returns
    ///
    /// Updated limits.
    #[inline(always)]
    pub const fn with_max_source_bytes(mut self, maximum: usize) -> Self {
        self.max_source_bytes = maximum;
        self
    }

    /// Returns the retained-item maximum.
    ///
    /// # Returns
    ///
    /// Maximum items retained after empty-item policy handling.
    #[inline(always)]
    pub const fn max_items(&self) -> usize {
        self.max_items
    }

    /// Returns a copy with a different retained-item maximum.
    ///
    /// # Parameters
    ///
    /// * `maximum` - Maximum retained items; zero permits only an empty result.
    ///
    /// # Returns
    ///
    /// Updated limits.
    #[inline(always)]
    pub const fn with_max_items(mut self, maximum: usize) -> Self {
        self.max_items = maximum;
        self
    }
}

impl Default for CollectionConversionLimits {
    /// Creates the documented finite collection limits.
    #[inline(always)]
    fn default() -> Self {
        Self {
            max_source_bytes: Self::DEFAULT_MAX_SOURCE_BYTES,
            max_items: Self::DEFAULT_MAX_ITEMS,
        }
    }
}
