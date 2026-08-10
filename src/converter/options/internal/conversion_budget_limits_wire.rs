// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Private Serde wire representation for cumulative conversion budgets.

use serde::Deserialize;
use serde::Serialize;

use super::super::ConversionBudgetLimits;

#[derive(Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub(crate) struct ConversionBudgetLimitsWire {
    pub(crate) max_items: usize,
    pub(crate) max_input_bytes: usize,
    pub(crate) max_output_bytes: usize,
    pub(crate) max_structured_nodes: usize,
}

impl Default for ConversionBudgetLimitsWire {
    fn default() -> Self {
        let limits = ConversionBudgetLimits::default();
        Self {
            max_items: limits.max_items(),
            max_input_bytes: limits.max_input_bytes(),
            max_output_bytes: limits.max_output_bytes(),
            max_structured_nodes: limits.max_structured_nodes(),
        }
    }
}
