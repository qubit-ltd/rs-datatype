// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Numeric wire coverage is provided by public options tests.

use qubit_datatype::NumericConversionLimits;

#[test]
fn test_numeric_wire_is_covered_by_public_options_contract() {
    let limits = NumericConversionLimits::default();
    assert_eq!(
        limits.max_text_bytes(),
        NumericConversionLimits::DEFAULT_MAX_TEXT_BYTES
    );
}
