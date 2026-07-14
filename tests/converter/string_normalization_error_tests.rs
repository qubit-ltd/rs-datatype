// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
// =============================================================================
//! Tests for string normalization policy outcomes.

use qubit_datatype::converter::StringNormalizationError;

/// Test normalization error identity and value-free display.
#[test]
fn test_string_normalization_error_display() {
    assert_eq!(
        StringNormalizationError::Missing.to_string(),
        "missing string value"
    );
    assert_eq!(
        StringNormalizationError::BlankRejected.to_string(),
        "blank string rejected",
    );
}
