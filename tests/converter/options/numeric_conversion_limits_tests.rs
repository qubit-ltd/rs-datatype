// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests for numeric conversion resource limits.

use qubit_datatype::converter::NumericConversionLimits;

/// Tests defaults, immutable builders, getters, and Serde round trips.
#[test]
fn test_numeric_conversion_limits_contract() {
    let defaults = NumericConversionLimits::default();
    assert_eq!(NumericConversionLimits::DEFAULT_MAX_TEXT_BYTES, 16_384);
    assert_eq!(
        NumericConversionLimits::DEFAULT_MAX_BIG_INTEGER_DIGITS,
        16_384,
    );
    assert_eq!(defaults.max_text_bytes(), 16_384);
    assert_eq!(defaults.max_big_integer_digits(), 16_384);

    let limits = defaults
        .with_max_text_bytes(32)
        .with_max_big_integer_digits(8);
    assert_eq!(limits.max_text_bytes(), 32);
    assert_eq!(limits.max_big_integer_digits(), 8);

    let wire = serde_json::to_string(&limits)
        .expect("numeric limits should serialize");
    assert_eq!(wire, r#"{"max_text_bytes":32,"max_big_integer_digits":8}"#,);
    assert_eq!(
        serde_json::from_str::<NumericConversionLimits>(&wire)
            .expect("numeric limits should deserialize"),
        limits,
    );
}

/// Tests that unknown resource-limit fields are rejected.
#[test]
fn test_numeric_conversion_limits_reject_unknown_fields() {
    let error = serde_json::from_str::<NumericConversionLimits>(
        r#"{"max_text_bytes":32,"unexpected":1}"#,
    )
    .expect_err("unknown numeric limit fields must be rejected");

    assert!(error.to_string().contains("unknown field `unexpected`"));
}
