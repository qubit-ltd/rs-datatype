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

/// Serializable field-based representation of structured limits.
#[derive(Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub(crate) struct StructuredConversionLimitsWire {
    /// Maximum normalized structured text size in bytes.
    pub(crate) max_text_bytes: u64,
    /// Maximum root-inclusive structured depth.
    pub(crate) max_depth: u64,
    /// Maximum structured sequence item count.
    pub(crate) max_sequence_items: u64,
    /// Maximum structured map entry count.
    pub(crate) max_map_entries: u64,
}

impl Default for StructuredConversionLimitsWire {
    /// Creates wire defaults matching [`StructuredConversionLimits`].
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
