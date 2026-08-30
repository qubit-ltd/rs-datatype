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

mod collection_conversion_limits_builder;

pub use collection_conversion_limits_builder::CollectionConversionLimitsBuilder;

/// Limits the source and retained items of one collection conversion.
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct CollectionConversionLimits {
    /// Maximum UTF-8 bytes accepted from one scalar collection source.
    max_source_bytes: u64,

    /// Maximum items retained by one scalar collection conversion.
    max_items: u64,
}

impl CollectionConversionLimits {
    /// Default maximum scalar collection source length in bytes.
    pub const DEFAULT_MAX_SOURCE_BYTES: u64 = 1_048_576;

    /// Default maximum number of retained scalar collection items.
    pub const DEFAULT_MAX_ITEMS: u64 = 65_536;

    /// Creates a builder initialized with the default collection limits.
    #[inline]
    #[must_use]
    pub const fn builder() -> CollectionConversionLimitsBuilder {
        CollectionConversionLimitsBuilder::new()
    }

    /// Converts these limits into a builder for explicit customization.
    #[inline]
    #[must_use]
    pub const fn into_builder(self) -> CollectionConversionLimitsBuilder {
        CollectionConversionLimitsBuilder { limits: self }
    }

    /// Returns the source text byte maximum.
    ///
    /// # Returns
    ///
    /// Maximum accepted UTF-8 byte length for one scalar collection source.
    #[inline(always)]
    #[must_use]
    pub const fn max_source_bytes(&self) -> u64 {
        self.max_source_bytes
    }

    /// Returns the retained-item maximum.
    ///
    /// # Returns
    ///
    /// Maximum items retained after empty-item policy handling.
    #[inline(always)]
    #[must_use]
    pub const fn max_items(&self) -> u64 {
        self.max_items
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
