// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Conversion operation limit wire coverage is provided by public tests.

use qubit_datatype::ConversionOperationLimits;

#[test]
fn test_conversion_operation_limits_wire_is_covered_by_public_contract() {
    let limits = ConversionOperationLimits::default();
    assert_eq!(
        limits.max_output_bytes(),
        ConversionOperationLimits::DEFAULT_MAX_OUTPUT_BYTES
    );
}
