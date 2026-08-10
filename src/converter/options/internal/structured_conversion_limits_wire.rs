// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Private Serde wire representation for structured conversion limits.

use serde::Deserialize;
use serde::Serialize;

use super::super::StructuredConversionLimits;

#[derive(Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub(crate) struct StructuredConversionLimitsWire {
    pub(crate) max_text_bytes: usize,
    pub(crate) max_depth: usize,
    pub(crate) max_sequence_items: usize,
    pub(crate) max_map_entries: usize,
}

impl Default for StructuredConversionLimitsWire {
    fn default() -> Self {
        let limits = StructuredConversionLimits::default();
        Self {
            max_text_bytes: limits.max_text_bytes(),
            max_depth: limits.max_depth(),
            max_sequence_items: limits.max_sequence_items(),
            max_map_entries: limits.max_map_entries(),
        }
    }
}
