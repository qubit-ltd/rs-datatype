// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Conversion operation limit defaults are covered through the public API.

use qubit_datatype::ConversionOperationLimits;

#[test]
fn test_conversion_operation_limits_default_output_bytes_match_public_constant() {
    let limits = ConversionOperationLimits::default();
    assert_eq!(
        limits.max_output_bytes(),
        ConversionOperationLimits::DEFAULT_MAX_OUTPUT_BYTES
    );
}
