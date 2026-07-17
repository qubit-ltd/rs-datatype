// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Boolean option deserialization validation tests.

use qubit_datatype::BooleanConversionOptions;

/// Verifies unchecked serialized literal sets are validated on construction.
#[test]
fn test_deserialization_rejects_overlapping_boolean_literals() {
    let json = r#"{
        "true_literals":["yes"],
        "false_literals":["YES"],
        "case_sensitive":false,
        "numeric_policy":"zero_or_one"
    }"#;
    assert!(serde_json::from_str::<BooleanConversionOptions>(json).is_err());
}
