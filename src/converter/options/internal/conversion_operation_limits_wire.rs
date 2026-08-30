// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Private Serde wire representation for conversion operation limits.

use serde::Deserialize;
use serde::Serialize;

use super::super::ConversionOperationLimits;

/// Serializable field-based representation of operation limits.
#[derive(Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub(crate) struct ConversionOperationLimitsWire {
    /// Cumulative converted-item limit.
    pub(crate) items: u64,
    /// Cumulative input-byte limit.
    pub(crate) input_bytes: u64,
    /// Cumulative output-byte limit.
    pub(crate) output_bytes: u64,
    /// Cumulative structured-node limit.
    pub(crate) structured_nodes: u64,
    /// Cumulative structured-payload byte limit.
    pub(crate) structured_payload_bytes: u64,
}

impl Default for ConversionOperationLimitsWire {
    /// Creates wire defaults matching [`ConversionOperationLimits`].
    fn default() -> Self {
        let limits = ConversionOperationLimits::default();
        Self {
            items: limits.max_items(),
            input_bytes: limits.max_input_bytes(),
            output_bytes: limits.max_output_bytes(),
            structured_nodes: limits.max_structured_nodes(),
            structured_payload_bytes: limits.max_structured_payload_bytes(),
        }
    }
}
