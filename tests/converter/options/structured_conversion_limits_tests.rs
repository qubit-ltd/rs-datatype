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
    assert_eq!(
        StructuredConversionLimits::builder()
            .max_text_bytes(16)
            .build()
            .max_text_bytes(),
        16
    );
    assert_eq!(
        defaults.max_depth(),
        StructuredConversionLimits::DEFAULT_MAX_DEPTH
    );
    assert_eq!(
        defaults.max_sequence_items(),
        StructuredConversionLimits::DEFAULT_MAX_SEQUENCE_ITEMS
    );
    assert_eq!(
        defaults.max_map_entries(),
        StructuredConversionLimits::DEFAULT_MAX_MAP_ENTRIES
    );
    assert_eq!(
        defaults
            .text()
            .utf8_bytes_limit()
            .expect("text limit")
            .maximum(),
        defaults.max_text_bytes()
    );
    assert_eq!(
        defaults.max_text_bytes_limit().maximum(),
        defaults.max_text_bytes()
    );
    assert_eq!(
        defaults.value().max_depth().expect("depth limit"),
        defaults.max_depth()
    );
    assert_eq!(defaults.max_depth_limit().maximum(), defaults.max_depth());
    assert_eq!(
        defaults.max_sequence_items_limit().maximum(),
        defaults.max_sequence_items()
    );
    assert_eq!(
        defaults.max_map_entries_limit().maximum(),
        defaults.max_map_entries()
    );
    let customized = StructuredConversionLimits::builder()
        .max_depth(4)
        .max_sequence_items(5)
        .max_map_entries(6)
        .build();
    assert_eq!(customized.max_depth(), 4);
    assert_eq!(customized.max_sequence_items(), 5);
    assert_eq!(customized.max_map_entries(), 6);
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
