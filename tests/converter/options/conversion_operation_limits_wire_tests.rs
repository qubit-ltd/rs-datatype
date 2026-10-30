// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Conversion operation limit defaults are covered through the public API.

use qubit_datatype::ConversionOperationLimits;
use serde_json::from_str;

#[test]
fn test_conversion_operation_limits_default_output_bytes_match_public_constant() {
    let limits = ConversionOperationLimits::default();
    assert_eq!(
        limits.max_output_bytes(),
        ConversionOperationLimits::DEFAULT_MAX_OUTPUT_BYTES
    );
}

#[test]
fn test_conversion_operation_limits_deserialize_defaults() {
    let limits = from_str::<ConversionOperationLimits>("{}").expect("omitted operation limits should use defaults");
    assert_eq!(limits, ConversionOperationLimits::default());
}

#[test]
fn test_conversion_operation_limits_deserialize_partial_output_override() {
    let limits = from_str::<ConversionOperationLimits>(r#"{"output_bytes":7}"#)
        .expect("partial operation limits should deserialize");
    let expected = ConversionOperationLimits::default()
        .into_builder()
        .max_output_bytes(7)
        .build();
    assert_eq!(limits, expected);
}

#[test]
fn test_conversion_operation_limits_deserialize_rejects_unknown_fields() {
    for wire in [r#"{"output_bytes":7,"unexpected":1}"#, r#"{"max_output_bytes":7}"#] {
        assert!(from_str::<ConversionOperationLimits>(wire).is_err());
    }
}
