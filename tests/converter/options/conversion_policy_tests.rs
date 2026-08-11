// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests for the aggregate conversion policy.

use qubit_datatype::converter::ConversionPolicy;
use qubit_datatype::converter::DurationRoundingPolicy;
use qubit_datatype::converter::NumericConversionPolicy;

/// Verifies the policy wire contains semantic fields but no resource limits.
#[test]
fn test_conversion_policy_wire_contains_no_resource_limits() {
    let value = serde_json::to_value(ConversionPolicy::default())
        .expect("the default conversion policy should serialize");
    let object = value
        .as_object()
        .expect("the conversion policy wire should be an object");

    assert_eq!(
        object.keys().map(String::as_str).collect::<Vec<_>>(),
        ["string", "boolean", "numeric", "collection", "duration"],
    );
    let text = value.to_string();
    for forbidden in [
        "max_items",
        "max_input_bytes",
        "max_output_bytes",
        "max_text_bytes",
        "budget",
        "operation",
    ] {
        assert!(
            !text.contains(forbidden),
            "unexpected policy field: {forbidden}",
        );
    }
}

#[test]
fn test_conversion_policy_profiles_preserve_semantic_defaults() {
    assert_eq!(ConversionPolicy::strict(), ConversionPolicy::default());
    assert_eq!(
        ConversionPolicy::lossy().numeric(),
        &NumericConversionPolicy::lossy()
    );
    assert!(ConversionPolicy::lossy().string().trim());
    assert_eq!(
        ConversionPolicy::lossy().duration().rounding_policy(),
        DurationRoundingPolicy::HalfUp
    );
    assert!(
        ConversionPolicy::env_friendly()
            .collection()
            .split_scalar_strings()
    );
    assert!(std::ptr::eq(
        ConversionPolicy::default_ref(),
        ConversionPolicy::default_ref()
    ));
}
