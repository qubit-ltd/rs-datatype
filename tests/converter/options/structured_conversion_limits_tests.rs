// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests for structured conversion resource limits.

use qubit_datatype::StructuredConversionLimits;

/// Tests the default and customized structured text byte limits.
#[test]
fn test_structured_conversion_limits_defaults_and_customization() {
    let defaults = StructuredConversionLimits::default();
    assert_eq!(
        StructuredConversionLimits::DEFAULT_MAX_TEXT_BYTES,
        1_048_576
    );
    assert_eq!(
        defaults.max_text_bytes(),
        StructuredConversionLimits::DEFAULT_MAX_TEXT_BYTES,
    );
    assert_eq!(defaults.with_max_text_bytes(16).max_text_bytes(), 16);
}

/// Tests Serde defaults and unknown-field rejection for structured limits.
#[test]
fn test_structured_conversion_limits_serde_contract() {
    assert_eq!(
        serde_json::from_str::<StructuredConversionLimits>("{}")
            .expect("empty structured limits should use defaults"),
        StructuredConversionLimits::default(),
    );
    assert!(
        serde_json::from_str::<StructuredConversionLimits>(
            r#"{"max_text_bytes":32,"unexpected":true}"#,
        )
        .expect_err("unknown structured limit fields must be rejected")
        .to_string()
        .contains("unknown field `unexpected`")
    );
}
