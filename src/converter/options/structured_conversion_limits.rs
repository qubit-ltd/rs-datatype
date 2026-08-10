// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Resource limits for structured text conversions.

use qubit_budget::ResourceLimit;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;

use super::internal::StructuredConversionLimitsWire;
use crate::converter::ConversionResource;

/// Bounds point checks and structural work introduced by structured conversion.
///
/// The text limit applies to UTF-8 bytes after configured string normalization
/// and covers JSON values, JSON objects converted to string maps, and URLs.
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StructuredConversionLimits {
    /// Maximum normalized structured source text length in bytes.
    max_text_bytes: ResourceLimit<ConversionResource, usize>,
    /// Maximum root-inclusive structured value depth.
    max_depth: ResourceLimit<ConversionResource, usize>,
    /// Maximum number of items in one sequence.
    max_sequence_items: ResourceLimit<ConversionResource, usize>,
    /// Maximum number of entries in one map.
    max_map_entries: ResourceLimit<ConversionResource, usize>,
}

impl StructuredConversionLimits {
    /// Default maximum normalized structured text length in bytes.
    pub const DEFAULT_MAX_TEXT_BYTES: usize = 1_048_576;
    /// Default maximum root-inclusive depth.
    pub const DEFAULT_MAX_DEPTH: usize = 128;
    /// Default maximum sequence item count.
    pub const DEFAULT_MAX_SEQUENCE_ITEMS: usize = 65_536;
    /// Default maximum map entry count.
    pub const DEFAULT_MAX_MAP_ENTRIES: usize = 65_536;

    /// Returns the maximum normalized structured text length in bytes.
    ///
    /// # Returns
    ///
    /// The configured normalized structured text byte limit.
    #[inline(always)]
    #[must_use]
    pub const fn max_text_bytes(&self) -> usize {
        self.max_text_bytes.maximum()
    }

    /// Returns the structured text resource limit.
    #[inline(always)]
    pub const fn max_text_bytes_limit(
        &self,
    ) -> &ResourceLimit<ConversionResource, usize> {
        &self.max_text_bytes
    }

    /// Returns a copy with a different structured text byte limit.
    ///
    /// # Parameters
    ///
    /// * `maximum` - New normalized structured text byte limit.
    ///
    /// # Returns
    ///
    /// Updated limits.
    #[inline(always)]
    pub const fn with_max_text_bytes(mut self, maximum: usize) -> Self {
        self.max_text_bytes = ResourceLimit::new(
            ConversionResource::StructuredTextBytes,
            maximum,
        );
        self
    }

    /// Returns the configured depth maximum.
    #[inline(always)]
    pub const fn max_depth(&self) -> usize {
        self.max_depth.maximum()
    }

    /// Returns the depth resource limit.
    #[inline(always)]
    pub const fn max_depth_limit(
        &self,
    ) -> &ResourceLimit<ConversionResource, usize> {
        &self.max_depth
    }

    /// Returns a copy with a different depth maximum.
    #[inline(always)]
    pub const fn with_max_depth(mut self, maximum: usize) -> Self {
        self.max_depth =
            ResourceLimit::new(ConversionResource::StructuredDepth, maximum);
        self
    }

    /// Returns the configured sequence item maximum.
    #[inline(always)]
    pub const fn max_sequence_items(&self) -> usize {
        self.max_sequence_items.maximum()
    }

    /// Returns the sequence item resource limit.
    #[inline(always)]
    pub const fn max_sequence_items_limit(
        &self,
    ) -> &ResourceLimit<ConversionResource, usize> {
        &self.max_sequence_items
    }

    /// Returns a copy with a different sequence item maximum.
    #[inline(always)]
    pub const fn with_max_sequence_items(mut self, maximum: usize) -> Self {
        self.max_sequence_items =
            ResourceLimit::new(ConversionResource::SequenceItems, maximum);
        self
    }

    /// Returns the configured map entry maximum.
    #[inline(always)]
    pub const fn max_map_entries(&self) -> usize {
        self.max_map_entries.maximum()
    }

    /// Returns the map entry resource limit.
    #[inline(always)]
    pub const fn max_map_entries_limit(
        &self,
    ) -> &ResourceLimit<ConversionResource, usize> {
        &self.max_map_entries
    }

    /// Returns a copy with a different map entry maximum.
    #[inline(always)]
    pub const fn with_max_map_entries(mut self, maximum: usize) -> Self {
        self.max_map_entries =
            ResourceLimit::new(ConversionResource::MapEntries, maximum);
        self
    }
}

impl Default for StructuredConversionLimits {
    /// Creates the default structured conversion limits.
    ///
    /// # Returns
    ///
    /// Limits using the documented one-mebibyte text budget.
    #[inline(always)]
    fn default() -> Self {
        Self {
            max_text_bytes: ResourceLimit::new(
                ConversionResource::StructuredTextBytes,
                Self::DEFAULT_MAX_TEXT_BYTES,
            ),
            max_depth: ResourceLimit::new(
                ConversionResource::StructuredDepth,
                Self::DEFAULT_MAX_DEPTH,
            ),
            max_sequence_items: ResourceLimit::new(
                ConversionResource::SequenceItems,
                Self::DEFAULT_MAX_SEQUENCE_ITEMS,
            ),
            max_map_entries: ResourceLimit::new(
                ConversionResource::MapEntries,
                Self::DEFAULT_MAX_MAP_ENTRIES,
            ),
        }
    }
}

impl Serialize for StructuredConversionLimits {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        StructuredConversionLimitsWire {
            max_text_bytes: self.max_text_bytes(),
            max_depth: self.max_depth(),
            max_sequence_items: self.max_sequence_items(),
            max_map_entries: self.max_map_entries(),
        }
        .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for StructuredConversionLimits {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = StructuredConversionLimitsWire::deserialize(deserializer)?;
        Ok(Self::default()
            .with_max_text_bytes(wire.max_text_bytes)
            .with_max_depth(wire.max_depth)
            .with_max_sequence_items(wire.max_sequence_items)
            .with_max_map_entries(wire.max_map_entries))
    }
}
