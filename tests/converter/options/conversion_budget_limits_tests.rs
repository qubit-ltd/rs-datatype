// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Conversion budget limit tests.

use qubit_datatype::converter::ConversionBudgetLimits;
use qubit_datatype::converter::ConversionResource;

#[test]
fn test_conversion_budget_limits_bind_resources() {
    let limits = ConversionBudgetLimits::default()
        .with_max_items(3)
        .with_max_input_bytes(5)
        .with_max_output_bytes(7)
        .with_max_structured_nodes(11);

    assert_eq!(
        *limits.max_items_limit().resource(),
        ConversionResource::Items
    );
    assert_eq!(limits.max_items_limit().maximum(), 3);
    assert_eq!(
        *limits.max_input_bytes_limit().resource(),
        ConversionResource::InputBytes
    );
    assert_eq!(
        *limits.max_output_bytes_limit().resource(),
        ConversionResource::OutputBytes
    );
    assert_eq!(
        *limits.max_structured_nodes_limit().resource(),
        ConversionResource::StructuredNodes
    );
}
