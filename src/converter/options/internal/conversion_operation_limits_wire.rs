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

#[derive(Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub(crate) struct ConversionOperationLimitsWire {
    pub(crate) items: usize,
    pub(crate) input_bytes: usize,
    pub(crate) output_bytes: usize,
    pub(crate) structured_nodes: usize,
    pub(crate) structured_payload_bytes: usize,
}

impl Default for ConversionOperationLimitsWire {
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
