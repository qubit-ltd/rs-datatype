// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Conversion budget wire coverage is provided by public options tests.

use qubit_datatype::ConversionBudgetLimits;

#[test]
fn test_conversion_budget_wire_is_covered_by_public_options_contract() {
    let limits = ConversionBudgetLimits::default();
    assert_eq!(
        limits.max_output_bytes(),
        ConversionBudgetLimits::DEFAULT_MAX_OUTPUT_BYTES
    );
}
