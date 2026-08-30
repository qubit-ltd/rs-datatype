// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Builder for cumulative conversion operation limits.

use qubit_budget::ResourceLimit;

use super::ConversionOperationLimits;
use crate::converter::ConversionResource;

/// Builder for [`ConversionOperationLimits`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConversionOperationLimitsBuilder {
    /// Operation limits being configured.
    pub(super) limits: ConversionOperationLimits,
}

impl ConversionOperationLimitsBuilder {
    /// Creates a builder initialized with the documented defaults.
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self {
            limits: ConversionOperationLimits {
                items: ResourceLimit::new(ConversionResource::Items, ConversionOperationLimits::DEFAULT_MAX_ITEMS),
                input_bytes: ResourceLimit::new(
                    ConversionResource::InputBytes,
                    ConversionOperationLimits::DEFAULT_MAX_INPUT_BYTES,
                ),
                output_bytes: ResourceLimit::new(
                    ConversionResource::OutputBytes,
                    ConversionOperationLimits::DEFAULT_MAX_OUTPUT_BYTES,
                ),
                structured_nodes: ResourceLimit::new(
                    ConversionResource::StructuredNodes,
                    ConversionOperationLimits::DEFAULT_MAX_STRUCTURED_NODES,
                ),
                structured_payload_bytes: ResourceLimit::new(
                    ConversionResource::StructuredPayloadBytes,
                    ConversionOperationLimits::DEFAULT_MAX_STRUCTURED_PAYLOAD_BYTES,
                ),
            },
        }
    }

    /// Configures the item maximum.
    #[inline(always)]
    #[must_use]
    pub const fn max_items(mut self, maximum: u64) -> Self {
        self.limits.items = ResourceLimit::new(ConversionResource::Items, maximum);
        self
    }

    /// Configures the cumulative input byte maximum.
    #[inline(always)]
    #[must_use]
    pub const fn max_input_bytes(mut self, maximum: u64) -> Self {
        self.limits.input_bytes = ResourceLimit::new(ConversionResource::InputBytes, maximum);
        self
    }

    /// Configures the cumulative output byte maximum.
    #[inline(always)]
    #[must_use]
    pub const fn max_output_bytes(mut self, maximum: u64) -> Self {
        self.limits.output_bytes = ResourceLimit::new(ConversionResource::OutputBytes, maximum);
        self
    }

    /// Configures the cumulative structured node maximum.
    #[inline(always)]
    #[must_use]
    pub const fn max_structured_nodes(mut self, maximum: u64) -> Self {
        self.limits.structured_nodes = ResourceLimit::new(ConversionResource::StructuredNodes, maximum);
        self
    }

    /// Configures the cumulative structured payload maximum.
    #[inline(always)]
    #[must_use]
    pub const fn max_structured_payload_bytes(mut self, maximum: u64) -> Self {
        self.limits.structured_payload_bytes = ResourceLimit::new(ConversionResource::StructuredPayloadBytes, maximum);
        self
    }

    /// Builds the configured operation limits.
    #[inline]
    #[must_use]
    pub const fn build(self) -> ConversionOperationLimits {
        self.limits
    }
}
