// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Resource limits for structured text conversions.

use qubit_budget::ResourceLimit;
use qubit_budget::StringLimits;
use qubit_budget::StructureLimits;
use qubit_budget::json::JsonValueLimits;
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
    text: StringLimits<ConversionResource>,
    /// Shared JSON value limits for structured traversal.
    value: JsonValueLimits<ConversionResource, u64>,
}

impl StructuredConversionLimits {
    /// Default maximum normalized structured text length in bytes.
    pub const DEFAULT_MAX_TEXT_BYTES: u64 = 1_048_576;
    /// Default maximum root-inclusive depth.
    pub const DEFAULT_MAX_DEPTH: u64 = 128;
    /// Default maximum sequence item count.
    pub const DEFAULT_MAX_SEQUENCE_ITEMS: u64 = 65_536;
    /// Default maximum map entry count.
    pub const DEFAULT_MAX_MAP_ENTRIES: u64 = 65_536;

    /// Returns the maximum normalized structured text length in bytes.
    ///
    /// # Returns
    ///
    /// The configured normalized structured text byte limit.
    #[inline(always)]
    #[must_use]
    pub fn max_text_bytes(&self) -> u64 {
        self.text
            .utf8_bytes_limit()
            .map_or(0, |limit| limit.maximum())
    }

    /// Returns the composed text-byte limits.
    #[inline]
    pub const fn text(&self) -> &StringLimits<ConversionResource> {
        &self.text
    }

    /// Returns the structured text resource limit.
    #[inline(always)]
    pub fn max_text_bytes_limit(
        &self,
    ) -> &ResourceLimit<ConversionResource, u64> {
        self.text
            .utf8_bytes_limit()
            .expect("structured text limit is configured")
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
    pub fn with_max_text_bytes(mut self, maximum: u64) -> Self {
        self.text = StringLimits::builder()
            .utf8_bytes_limit(ResourceLimit::new(
                ConversionResource::StructuredTextBytes,
                maximum,
            ))
            .build();
        self.value = rebuild_value(
            &self.value,
            rebuild_structure(
                self.value.structure_limits(),
                None,
                None,
                None,
                None,
            ),
            Some(ResourceLimit::new(
                ConversionResource::StructuredTextBytes,
                maximum,
            )),
        );
        self
    }

    /// Returns the composed JSON value limits used by structured traversal.
    #[inline]
    pub const fn value(&self) -> &JsonValueLimits<ConversionResource, u64> {
        &self.value
    }

    /// Returns the configured depth maximum.
    #[inline(always)]
    #[must_use]
    pub fn max_depth(&self) -> u64 {
        self.value
            .max_depth()
            .expect("structured depth limit is configured")
    }

    /// Returns the depth resource limit.
    #[inline(always)]
    pub fn max_depth_limit(&self) -> ResourceLimit<ConversionResource, u64> {
        self.value
            .structure_limits()
            .depth_limit()
            .cloned()
            .expect("structured depth limit is configured")
    }

    /// Returns a copy with a different depth maximum.
    #[inline(always)]
    pub fn with_max_depth(mut self, maximum: u64) -> Self {
        self.value = rebuild_value(
            &self.value,
            rebuild_structure(
                self.value.structure_limits(),
                Some(ResourceLimit::new(
                    ConversionResource::StructuredDepth,
                    maximum,
                )),
                None,
                None,
                None,
            ),
            None,
        );
        self
    }

    /// Returns the configured sequence item maximum.
    #[inline(always)]
    #[must_use]
    pub fn max_sequence_items(&self) -> u64 {
        self.value
            .max_sequence_items()
            .expect("structured sequence-item limit is configured")
    }

    /// Returns the sequence item resource limit.
    #[inline(always)]
    pub fn max_sequence_items_limit(
        &self,
    ) -> ResourceLimit<ConversionResource, u64> {
        self.value
            .structure_limits()
            .sequence_items_limit()
            .cloned()
            .expect("structured sequence-item limit is configured")
    }

    /// Returns a copy with a different sequence item maximum.
    #[inline(always)]
    pub fn with_max_sequence_items(mut self, maximum: u64) -> Self {
        self.value = rebuild_value(
            &self.value,
            rebuild_structure(
                self.value.structure_limits(),
                None,
                Some(ResourceLimit::new(
                    ConversionResource::SequenceItems,
                    maximum,
                )),
                None,
                None,
            ),
            None,
        );
        self
    }

    /// Returns the configured map entry maximum.
    #[inline(always)]
    #[must_use]
    pub fn max_map_entries(&self) -> u64 {
        self.value
            .max_map_entries()
            .expect("structured map-entry limit is configured")
    }

    /// Returns the map entry resource limit.
    #[inline(always)]
    pub fn max_map_entries_limit(
        &self,
    ) -> ResourceLimit<ConversionResource, u64> {
        self.value
            .structure_limits()
            .map_entries_limit()
            .cloned()
            .expect("structured map-entry limit is configured")
    }

    /// Returns a copy with a different map entry maximum.
    #[inline(always)]
    pub fn with_max_map_entries(mut self, maximum: u64) -> Self {
        self.value = rebuild_value(
            &self.value,
            rebuild_structure(
                self.value.structure_limits(),
                None,
                None,
                Some(ResourceLimit::new(
                    ConversionResource::MapEntries,
                    maximum,
                )),
                None,
            ),
            None,
        );
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
            text: StringLimits::builder()
                .utf8_bytes_limit(ResourceLimit::new(
                    ConversionResource::StructuredTextBytes,
                    Self::DEFAULT_MAX_TEXT_BYTES,
                ))
                .build(),
            value: JsonValueLimits::<ConversionResource, u64>::builder()
                .structure_limits(
                    StructureLimits::<ConversionResource, u64>::builder()
                        .depth_limit(ResourceLimit::new(
                            ConversionResource::StructuredDepth,
                            Self::DEFAULT_MAX_DEPTH,
                        ))
                        .sequence_items_limit(ResourceLimit::new(
                            ConversionResource::SequenceItems,
                            Self::DEFAULT_MAX_SEQUENCE_ITEMS,
                        ))
                        .map_entries_limit(ResourceLimit::new(
                            ConversionResource::MapEntries,
                            Self::DEFAULT_MAX_MAP_ENTRIES,
                        ))
                        .build(),
                )
                .string_bytes_limit(ResourceLimit::new(
                    ConversionResource::StructuredTextBytes,
                    Self::DEFAULT_MAX_TEXT_BYTES,
                ))
                .build(),
        }
    }
}

fn rebuild_structure(
    source: &StructureLimits<ConversionResource, u64>,
    depth: Option<ResourceLimit<ConversionResource, u64>>,
    sequence_items: Option<ResourceLimit<ConversionResource, u64>>,
    map_entries: Option<ResourceLimit<ConversionResource, u64>>,
    key_bytes: Option<ResourceLimit<ConversionResource, u64>>,
) -> StructureLimits<ConversionResource, u64> {
    let mut builder = StructureLimits::builder();
    if let Some(limit) = depth.or_else(|| source.depth_limit().cloned()) {
        builder = builder.depth_limit(limit);
    }
    if let Some(limit) =
        sequence_items.or_else(|| source.sequence_items_limit().cloned())
    {
        builder = builder.sequence_items_limit(limit);
    }
    if let Some(limit) =
        map_entries.or_else(|| source.map_entries_limit().cloned())
    {
        builder = builder.map_entries_limit(limit);
    }
    if let Some(limit) = key_bytes.or_else(|| source.key_bytes_limit().cloned())
    {
        builder = builder.key_bytes_limit(limit);
    }
    builder.build()
}

fn rebuild_value(
    source: &JsonValueLimits<ConversionResource, u64>,
    structure: StructureLimits<ConversionResource, u64>,
    string_bytes: Option<ResourceLimit<ConversionResource, u64>>,
) -> JsonValueLimits<ConversionResource, u64> {
    let mut builder = JsonValueLimits::builder().structure_limits(structure);
    if let Some(limit) =
        string_bytes.or_else(|| source.string_bytes_limit().cloned())
    {
        builder = builder.string_bytes_limit(limit);
    }
    if let Some(limit) = source.number_bytes_limit().cloned() {
        builder = builder.number_bytes_limit(limit);
    }
    if let Some(limit) = source.payload_bytes_limit().cloned() {
        builder = builder.payload_bytes_limit(limit);
    }
    builder.build()
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
