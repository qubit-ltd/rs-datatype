// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Cumulative limits applied to one conversion operation.
use qubit_budget::ResourceLimit;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;

use super::internal::ConversionOperationLimitsWire;
use crate::converter::ConversionResource;

mod conversion_operation_limits_builder;

pub use conversion_operation_limits_builder::ConversionOperationLimitsBuilder;

/// Cumulative limits shared by single and batch conversions.
///
/// # Examples
///
/// ```
/// use qubit_datatype::ConversionOperationLimits;
///
/// let limits = ConversionOperationLimits::builder().max_items(8).build();
/// assert_eq!(limits.max_items(), 8);
/// ```
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConversionOperationLimits {
    /// Cumulative item limit for one conversion operation.
    items: ResourceLimit<ConversionResource, u64>,

    /// Cumulative input byte limit.
    input_bytes: ResourceLimit<ConversionResource, u64>,

    /// Cumulative output payload byte limit.
    output_bytes: ResourceLimit<ConversionResource, u64>,

    /// Cumulative structured value node limit.
    structured_nodes: ResourceLimit<ConversionResource, u64>,

    /// Cumulative structured key, string, and number payload byte limit.
    structured_payload_bytes: ResourceLimit<ConversionResource, u64>,
}

impl ConversionOperationLimits {
    /// Default maximum number of top-level items in one session.
    pub const DEFAULT_MAX_ITEMS: u64 = 65_536;
    /// Default cumulative input byte budget.
    pub const DEFAULT_MAX_INPUT_BYTES: u64 = 64 * 1024 * 1024;
    /// Default cumulative output byte budget.
    pub const DEFAULT_MAX_OUTPUT_BYTES: u64 = 64 * 1024 * 1024;
    /// Default cumulative structured node budget.
    pub const DEFAULT_MAX_STRUCTURED_NODES: u64 = 262_144;
    /// Default cumulative structured payload byte budget.
    pub const DEFAULT_MAX_STRUCTURED_PAYLOAD_BYTES: u64 = 64 * 1024 * 1024;

    /// Creates a builder initialized with the default operation limits.
    #[inline]
    #[must_use]
    pub const fn builder() -> ConversionOperationLimitsBuilder {
        ConversionOperationLimitsBuilder::new()
    }

    /// Converts these limits into a builder for explicit customization.
    #[inline]
    #[must_use]
    pub const fn into_builder(self) -> ConversionOperationLimitsBuilder {
        ConversionOperationLimitsBuilder { limits: self }
    }

    /// Returns the configured item maximum.
    #[inline(always)]
    #[must_use]
    pub const fn max_items(&self) -> u64 {
        self.items.maximum()
    }

    /// Returns the item resource limit.
    #[must_use]
    #[inline(always)]
    pub const fn items_limit(&self) -> &ResourceLimit<ConversionResource, u64> {
        &self.items
    }

    /// Returns the configured cumulative input byte maximum.
    #[inline(always)]
    #[must_use]
    pub const fn max_input_bytes(&self) -> u64 {
        self.input_bytes.maximum()
    }

    /// Returns the input byte resource limit.
    #[must_use]
    #[inline(always)]
    pub const fn input_bytes_limit(&self) -> &ResourceLimit<ConversionResource, u64> {
        &self.input_bytes
    }

    /// Returns the configured cumulative output byte maximum.
    #[inline(always)]
    #[must_use]
    pub const fn max_output_bytes(&self) -> u64 {
        self.output_bytes.maximum()
    }

    /// Returns the output byte resource limit.
    #[must_use]
    #[inline(always)]
    pub const fn output_bytes_limit(&self) -> &ResourceLimit<ConversionResource, u64> {
        &self.output_bytes
    }

    /// Returns the configured cumulative structured node maximum.
    #[inline(always)]
    #[must_use]
    pub const fn max_structured_nodes(&self) -> u64 {
        self.structured_nodes.maximum()
    }

    /// Returns the structured node resource limit.
    #[must_use]
    #[inline(always)]
    pub const fn structured_nodes_limit(&self) -> &ResourceLimit<ConversionResource, u64> {
        &self.structured_nodes
    }

    /// Returns the configured cumulative structured payload byte maximum.
    #[inline(always)]
    #[must_use]
    pub const fn max_structured_payload_bytes(&self) -> u64 {
        self.structured_payload_bytes.maximum()
    }

    /// Returns the structured payload byte resource limit.
    #[must_use]
    #[inline(always)]
    pub const fn structured_payload_bytes_limit(&self) -> &ResourceLimit<ConversionResource, u64> {
        &self.structured_payload_bytes
    }
}

impl Default for ConversionOperationLimits {
    /// Creates the documented finite default limits.
    #[inline]
    fn default() -> Self {
        Self {
            items: ResourceLimit::new(ConversionResource::Items, Self::DEFAULT_MAX_ITEMS),
            input_bytes: ResourceLimit::new(ConversionResource::InputBytes, Self::DEFAULT_MAX_INPUT_BYTES),
            output_bytes: ResourceLimit::new(ConversionResource::OutputBytes, Self::DEFAULT_MAX_OUTPUT_BYTES),
            structured_nodes: ResourceLimit::new(
                ConversionResource::StructuredNodes,
                Self::DEFAULT_MAX_STRUCTURED_NODES,
            ),
            structured_payload_bytes: ResourceLimit::new(
                ConversionResource::StructuredPayloadBytes,
                Self::DEFAULT_MAX_STRUCTURED_PAYLOAD_BYTES,
            ),
        }
    }
}

impl Serialize for ConversionOperationLimits {
    /// Serializes the configured limits through the private wire shape.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        ConversionOperationLimitsWire {
            items: self.max_items(),
            input_bytes: self.max_input_bytes(),
            output_bytes: self.max_output_bytes(),
            structured_nodes: self.max_structured_nodes(),
            structured_payload_bytes: self.max_structured_payload_bytes(),
        }
        .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for ConversionOperationLimits {
    /// Deserializes limits from the private wire shape.
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = ConversionOperationLimitsWire::deserialize(deserializer)?;
        Ok(Self::builder()
            .max_items(wire.items)
            .max_input_bytes(wire.input_bytes)
            .max_output_bytes(wire.output_bytes)
            .max_structured_nodes(wire.structured_nodes)
            .max_structured_payload_bytes(wire.structured_payload_bytes)
            .build())
    }
}
