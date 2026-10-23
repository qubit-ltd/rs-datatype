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
///
/// # Examples
///
/// ```
/// use qubit_datatype::ConversionOperationLimits;
///
/// let limits = ConversionOperationLimits::builder()
///     .max_items(128)
///     .max_input_bytes(4_096)
///     .build();
/// assert_eq!(limits.max_items(), 128);
/// assert_eq!(limits.max_input_bytes(), 4_096);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConversionOperationLimitsBuilder {
    /// Operation limits being configured.
    pub(super) limits: ConversionOperationLimits,
}

impl ConversionOperationLimitsBuilder {
    /// Creates a builder initialized with the documented defaults.
    ///
    /// # Returns
    ///
    /// A builder containing the default cumulative limits for conversion
    /// operations.
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
    ///
    /// # Parameters
    ///
    /// - `maximum`: Maximum number of top-level items allowed in one operation.
    ///
    /// # Returns
    ///
    /// This builder with the item maximum updated.
    #[inline]
    #[must_use]
    pub const fn max_items(mut self, maximum: u64) -> Self {
        self.limits.items = ResourceLimit::new(ConversionResource::Items, maximum);
        self
    }

    /// Configures the cumulative input byte maximum.
    ///
    /// # Parameters
    ///
    /// - `maximum`: Maximum cumulative number of input bytes allowed in one
    ///   operation.
    ///
    /// # Returns
    ///
    /// This builder with the input byte maximum updated.
    #[inline]
    #[must_use]
    pub const fn max_input_bytes(mut self, maximum: u64) -> Self {
        self.limits.input_bytes = ResourceLimit::new(ConversionResource::InputBytes, maximum);
        self
    }

    /// Configures the cumulative output byte maximum.
    ///
    /// # Parameters
    ///
    /// - `maximum`: Maximum cumulative number of output bytes allowed in one
    ///   operation.
    ///
    /// # Returns
    ///
    /// This builder with the output byte maximum updated.
    #[inline]
    #[must_use]
    pub const fn max_output_bytes(mut self, maximum: u64) -> Self {
        self.limits.output_bytes = ResourceLimit::new(ConversionResource::OutputBytes, maximum);
        self
    }

    /// Configures the cumulative structured node maximum.
    ///
    /// # Parameters
    ///
    /// - `maximum`: Maximum number of structured value nodes allowed in one
    ///   operation.
    ///
    /// # Returns
    ///
    /// This builder with the structured node maximum updated.
    #[inline]
    #[must_use]
    pub const fn max_structured_nodes(mut self, maximum: u64) -> Self {
        self.limits.structured_nodes = ResourceLimit::new(ConversionResource::StructuredNodes, maximum);
        self
    }

    /// Configures the cumulative structured payload maximum.
    ///
    /// # Parameters
    ///
    /// - `maximum`: Maximum cumulative payload bytes in structured keys,
    ///   strings, and numbers.
    ///
    /// # Returns
    ///
    /// This builder with the structured payload byte maximum updated.
    #[inline]
    #[must_use]
    pub const fn max_structured_payload_bytes(mut self, maximum: u64) -> Self {
        self.limits.structured_payload_bytes = ResourceLimit::new(ConversionResource::StructuredPayloadBytes, maximum);
        self
    }

    /// Builds the configured operation limits.
    ///
    /// # Returns
    ///
    /// The operation limits assembled from the builder's configured maxima.
    #[inline]
    pub const fn build(self) -> ConversionOperationLimits {
        self.limits
    }
}
