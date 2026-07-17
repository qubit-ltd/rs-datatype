// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests for numeric conversion policy wire behavior.

use qubit_datatype::converter::NumericConversionPolicy;

/// Test defaults and stable snake-case Serde names.
#[test]
fn test_numeric_conversion_policy_serde_and_default() {
    assert_eq!(
        NumericConversionPolicy::default(),
        NumericConversionPolicy::Exact
    );
    assert_eq!(
        serde_json::to_string(&NumericConversionPolicy::Exact)
            .expect("exact policy should serialize"),
        r#""exact""#,
    );
    assert_eq!(
        serde_json::from_str::<NumericConversionPolicy>(r#""lossy""#)
            .expect("lossy policy should deserialize"),
        NumericConversionPolicy::Lossy,
    );
}

/// Test that the environment-variable numeric profile is exact.
#[test]
fn test_numeric_conversion_policy_env_friendly_is_exact() {
    assert_eq!(
        NumericConversionPolicy::env_friendly(),
        NumericConversionPolicy::Exact,
    );
}
