// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Cumulative budgets applied to one conversion session.

use qubit_budget::ResourceLimit;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;

use super::internal::ConversionBudgetLimitsWire;
use crate::converter::ConversionResource;

/// Cumulative limits shared by single and batch conversions.
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConversionBudgetLimits {
    max_items: ResourceLimit<ConversionResource, usize>,
    max_input_bytes: ResourceLimit<ConversionResource, usize>,
    max_output_bytes: ResourceLimit<ConversionResource, usize>,
    max_structured_nodes: ResourceLimit<ConversionResource, usize>,
}

impl ConversionBudgetLimits {
    /// Default maximum number of top-level items in one session.
    pub const DEFAULT_MAX_ITEMS: usize = 65_536;
    /// Default cumulative normalized input byte budget.
    pub const DEFAULT_MAX_INPUT_BYTES: usize = 64 * 1024 * 1024;
    /// Default cumulative output byte budget.
    pub const DEFAULT_MAX_OUTPUT_BYTES: usize = 64 * 1024 * 1024;
    /// Default cumulative structured node budget.
    pub const DEFAULT_MAX_STRUCTURED_NODES: usize = 262_144;

    /// Returns the configured item maximum.
    #[inline(always)]
    pub const fn max_items(&self) -> usize {
        self.max_items.maximum()
    }

    /// Returns the item resource limit.
    #[inline(always)]
    pub const fn max_items_limit(
        &self,
    ) -> &ResourceLimit<ConversionResource, usize> {
        &self.max_items
    }

    /// Returns a copy with a different item maximum.
    #[inline(always)]
    pub const fn with_max_items(mut self, maximum: usize) -> Self {
        self.max_items = ResourceLimit::new(ConversionResource::Items, maximum);
        self
    }

    /// Returns the configured cumulative input byte maximum.
    #[inline(always)]
    pub const fn max_input_bytes(&self) -> usize {
        self.max_input_bytes.maximum()
    }

    /// Returns the input byte resource limit.
    #[inline(always)]
    pub const fn max_input_bytes_limit(
        &self,
    ) -> &ResourceLimit<ConversionResource, usize> {
        &self.max_input_bytes
    }

    /// Returns a copy with a different cumulative input byte maximum.
    #[inline(always)]
    pub const fn with_max_input_bytes(mut self, maximum: usize) -> Self {
        self.max_input_bytes =
            ResourceLimit::new(ConversionResource::InputBytes, maximum);
        self
    }

    /// Returns the configured cumulative output byte maximum.
    #[inline(always)]
    pub const fn max_output_bytes(&self) -> usize {
        self.max_output_bytes.maximum()
    }

    /// Returns the output byte resource limit.
    #[inline(always)]
    pub const fn max_output_bytes_limit(
        &self,
    ) -> &ResourceLimit<ConversionResource, usize> {
        &self.max_output_bytes
    }

    /// Returns a copy with a different cumulative output byte maximum.
    #[inline(always)]
    pub const fn with_max_output_bytes(mut self, maximum: usize) -> Self {
        self.max_output_bytes =
            ResourceLimit::new(ConversionResource::OutputBytes, maximum);
        self
    }

    /// Returns the configured cumulative structured node maximum.
    #[inline(always)]
    pub const fn max_structured_nodes(&self) -> usize {
        self.max_structured_nodes.maximum()
    }

    /// Returns the structured node resource limit.
    #[inline(always)]
    pub const fn max_structured_nodes_limit(
        &self,
    ) -> &ResourceLimit<ConversionResource, usize> {
        &self.max_structured_nodes
    }

    /// Returns a copy with a different cumulative structured node maximum.
    #[inline(always)]
    pub const fn with_max_structured_nodes(mut self, maximum: usize) -> Self {
        self.max_structured_nodes =
            ResourceLimit::new(ConversionResource::StructuredNodes, maximum);
        self
    }
}

impl Default for ConversionBudgetLimits {
    /// Creates the documented finite default budgets.
    #[inline]
    fn default() -> Self {
        Self {
            max_items: ResourceLimit::new(
                ConversionResource::Items,
                Self::DEFAULT_MAX_ITEMS,
            ),
            max_input_bytes: ResourceLimit::new(
                ConversionResource::InputBytes,
                Self::DEFAULT_MAX_INPUT_BYTES,
            ),
            max_output_bytes: ResourceLimit::new(
                ConversionResource::OutputBytes,
                Self::DEFAULT_MAX_OUTPUT_BYTES,
            ),
            max_structured_nodes: ResourceLimit::new(
                ConversionResource::StructuredNodes,
                Self::DEFAULT_MAX_STRUCTURED_NODES,
            ),
        }
    }
}

impl Serialize for ConversionBudgetLimits {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        ConversionBudgetLimitsWire {
            max_items: self.max_items(),
            max_input_bytes: self.max_input_bytes(),
            max_output_bytes: self.max_output_bytes(),
            max_structured_nodes: self.max_structured_nodes(),
        }
        .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for ConversionBudgetLimits {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = ConversionBudgetLimitsWire::deserialize(deserializer)?;
        Ok(Self::default()
            .with_max_items(wire.max_items)
            .with_max_input_bytes(wire.max_input_bytes)
            .with_max_output_bytes(wire.max_output_bytes)
            .with_max_structured_nodes(wire.max_structured_nodes))
    }
}
