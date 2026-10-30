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

mod structured_conversion_limits_builder;

pub use structured_conversion_limits_builder::StructuredConversionLimitsBuilder;

/// Bounds point checks and structural work introduced by structured conversion.
///
/// The text limit bounds normalized source text on supported text conversion
/// paths. It is not a per-string limit for already materialized JSON values.
/// The reusable value-limit template also carries a string-byte limit with the
/// same maximum. Consumers that apply this template directly enforce that
/// limit. Built-in conversion sessions select the template's structure limits
/// and add cumulative node and payload budgets; they do not apply its
/// string-byte limit. This template is not a snapshot of a session's effective
/// budgets.
///
/// Depth includes the root value as depth one. Sequence and map limits apply
/// independently to the direct items or entries of each container. A zero
/// container limit permits only empty containers; a zero depth limit rejects
/// every root value.
///
/// # Examples
///
/// ```
/// use qubit_datatype::StructuredConversionLimits;
///
/// let limits = StructuredConversionLimits::builder().max_depth(4).build();
/// assert_eq!(limits.max_depth(), 4);
/// ```
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
    /// Default maximum depth including the root value, which has depth one.
    pub const DEFAULT_MAX_DEPTH: u64 = 128;
    /// Default maximum direct item count for each sequence.
    pub const DEFAULT_MAX_SEQUENCE_ITEMS: u64 = 65_536;
    /// Default maximum direct entry count for each map.
    pub const DEFAULT_MAX_MAP_ENTRIES: u64 = 65_536;

    /// Creates a builder initialized with the default structured limits.
    ///
    /// # Returns
    ///
    /// A builder initialized from the default limits.
    #[inline]
    #[must_use]
    pub fn builder() -> StructuredConversionLimitsBuilder {
        StructuredConversionLimitsBuilder::new()
    }

    /// Converts these limits into a builder for explicit customization.
    ///
    /// # Returns
    ///
    /// A builder containing these limits.
    #[inline]
    #[must_use]
    pub fn into_builder(self) -> StructuredConversionLimitsBuilder {
        StructuredConversionLimitsBuilder { limits: self }
    }

    /// Returns the maximum normalized source-text length in bytes.
    ///
    /// This bounds supported text conversion paths; it does not impose a
    /// per-string limit on already materialized JSON values. The reusable
    /// [`value`](Self::value) template carries a string-byte limit with the
    /// same maximum, which consumers applying that template directly
    /// enforce. Public `AdmittedConversion::decode_json` also checks the exact
    /// input byte length, including whitespace, before decoding; it does not
    /// normalize or charge the source again.
    ///
    /// # Returns
    ///
    /// The configured normalized structured text byte limit.
    #[inline]
    #[must_use]
    pub fn max_text_bytes(&self) -> u64 {
        self.text.utf8_bytes_limit().map_or(0, |limit| limit.maximum())
    }

    /// Returns the composed limits for normalized structured source text.
    ///
    /// Built-in conversion sessions use this text limit on supported text
    /// paths; it is not a per-string limit for already materialized JSON
    /// values.
    ///
    /// # Returns
    ///
    /// A borrow of the configured structured text limits.
    #[inline]
    #[must_use]
    pub const fn text(&self) -> &StringLimits<ConversionResource> {
        &self.text
    }

    /// Returns the structured text resource limit.
    ///
    /// # Panics
    ///
    /// Panics if the structured text limit is not configured.
    ///
    /// # Returns
    ///
    /// The configured structured text resource limit.
    #[inline]
    #[must_use]
    pub fn max_text_bytes_limit(&self) -> &ResourceLimit<ConversionResource, u64> {
        self.text
            .utf8_bytes_limit()
            .expect("structured text limit is configured")
    }

    /// Returns the reusable JSON value-limit template.
    ///
    /// The template carries a string-byte limit matching
    /// [`max_text_bytes`](Self::max_text_bytes), so consumers that apply it
    /// directly enforce that limit. Built-in conversion sessions select its
    /// structure limits and add cumulative node and payload budgets without
    /// applying its string-byte limit. It is not a snapshot of a session's
    /// effective budgets.
    ///
    /// # Returns
    ///
    /// A borrow of the reusable JSON value-limit template.
    #[inline]
    #[must_use]
    pub const fn value(&self) -> &JsonValueLimits<ConversionResource, u64> {
        &self.value
    }

    /// Returns the configured maximum depth, counting the root value as one.
    ///
    /// # Panics
    ///
    /// Panics if the structured depth limit is not configured.
    ///
    /// # Returns
    ///
    /// The inclusive maximum depth, counting the root as one.
    #[inline]
    #[must_use]
    pub fn max_depth(&self) -> u64 {
        self.value.max_depth().expect("structured depth limit is configured")
    }

    /// Returns the depth resource limit.
    ///
    /// # Panics
    ///
    /// Panics if the structured depth limit is not configured.
    ///
    /// # Returns
    ///
    /// The configured depth resource limit.
    #[inline]
    #[must_use]
    pub fn max_depth_limit(&self) -> ResourceLimit<ConversionResource, u64> {
        self.value
            .structure_limits()
            .depth_limit()
            .cloned()
            .expect("structured depth limit is configured")
    }

    /// Returns the maximum number of direct items allowed in each sequence.
    ///
    /// # Panics
    ///
    /// Panics if the structured sequence-item limit is not configured.
    ///
    /// # Returns
    ///
    /// The maximum number of direct items in each sequence.
    #[inline]
    #[must_use]
    pub fn max_sequence_items(&self) -> u64 {
        self.value
            .max_sequence_items()
            .expect("structured sequence-item limit is configured")
    }

    /// Returns the sequence item resource limit.
    ///
    /// # Panics
    ///
    /// Panics if the structured sequence-item limit is not configured.
    ///
    /// # Returns
    ///
    /// The configured sequence item resource limit.
    #[inline]
    #[must_use]
    pub fn max_sequence_items_limit(&self) -> ResourceLimit<ConversionResource, u64> {
        self.value
            .structure_limits()
            .sequence_items_limit()
            .cloned()
            .expect("structured sequence-item limit is configured")
    }

    /// Returns the maximum number of direct entries allowed in each map.
    ///
    /// # Panics
    ///
    /// Panics if the structured map-entry limit is not configured.
    ///
    /// # Returns
    ///
    /// The maximum number of direct entries in each map.
    #[inline]
    #[must_use]
    pub fn max_map_entries(&self) -> u64 {
        self.value
            .max_map_entries()
            .expect("structured map-entry limit is configured")
    }

    /// Returns the map entry resource limit.
    ///
    /// # Panics
    ///
    /// Panics if the structured map-entry limit is not configured.
    ///
    /// # Returns
    ///
    /// The configured map entry resource limit.
    #[inline]
    #[must_use]
    pub fn max_map_entries_limit(&self) -> ResourceLimit<ConversionResource, u64> {
        self.value
            .structure_limits()
            .map_entries_limit()
            .cloned()
            .expect("structured map-entry limit is configured")
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
    #[inline]
    pub(crate) fn with_max_text_bytes(mut self, maximum: u64) -> Self {
        self.text = self
            .text
            .into_builder()
            .utf8_bytes_limit(ResourceLimit::new(ConversionResource::StructuredTextBytes, maximum))
            .build();
        self.value = self
            .value
            .into_builder()
            .string_bytes_limit(ResourceLimit::new(ConversionResource::StructuredTextBytes, maximum))
            .build();
        self
    }

    /// Returns a copy with a different depth maximum.
    #[inline]
    pub(crate) fn with_max_depth(mut self, maximum: u64) -> Self {
        let structure = (*self.value.structure_limits())
            .into_builder()
            .depth_limit(ResourceLimit::new(ConversionResource::StructuredDepth, maximum))
            .build();
        self.value = self.value.into_builder().structure_limits(structure).build();
        self
    }

    /// Returns a copy with a different sequence item maximum.
    #[inline]
    pub(crate) fn with_max_sequence_items(mut self, maximum: u64) -> Self {
        let structure = (*self.value.structure_limits())
            .into_builder()
            .sequence_items_limit(ResourceLimit::new(ConversionResource::SequenceItems, maximum))
            .build();
        self.value = self.value.into_builder().structure_limits(structure).build();
        self
    }

    /// Returns a copy with a different map entry maximum.
    #[inline]
    pub(crate) fn with_max_map_entries(mut self, maximum: u64) -> Self {
        let structure = (*self.value.structure_limits())
            .into_builder()
            .map_entries_limit(ResourceLimit::new(ConversionResource::MapEntries, maximum))
            .build();
        self.value = self.value.into_builder().structure_limits(structure).build();
        self
    }
}

impl Default for StructuredConversionLimits {
    /// Creates the default structured conversion limits.
    ///
    /// # Returns
    ///
    /// Limits using the documented one-mebibyte text budget.
    #[inline]
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

impl Serialize for StructuredConversionLimits {
    /// Serializes the configured limits through the private wire shape.
    ///
    /// # Type Parameters
    ///
    /// * `S` - Serializer used to encode the wire representation.
    ///
    /// # Parameters
    ///
    /// * `serializer` - Serializer that receives the wire representation.
    ///
    /// # Returns
    ///
    /// The serializer's successful output.
    ///
    /// # Errors
    ///
    /// Returns the serializer's error if it cannot encode the wire
    /// representation.
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
    /// Deserializes limits from the private wire shape.
    ///
    /// # Type Parameters
    ///
    /// * `D` - Deserializer used to decode the wire representation.
    ///
    /// # Parameters
    ///
    /// * `deserializer` - Deserializer providing the wire representation.
    ///
    /// # Returns
    ///
    /// The structured limits decoded from the input.
    ///
    /// # Errors
    ///
    /// Returns the deserializer's error if the input does not match the wire
    /// representation or a field cannot be decoded.
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
