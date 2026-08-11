// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Focused tests for structured conversion accounting.

#![cfg(all(feature = "converter", feature = "json"))]

use qubit_datatype::ConversionLimits;
use qubit_datatype::ConversionOperationLimits;
use qubit_datatype::ConversionPolicy;
use qubit_datatype::ConversionResource;
use qubit_datatype::DataConverter;
use qubit_datatype::StructuredConversionLimits;

#[test]
fn materialized_json_charges_payload_not_only_nodes() {
    let value = serde_json::json!({"k": "abcd"});
    let limits = ConversionLimits::default().with_operation_limits(
        ConversionOperationLimits::default()
            .with_max_structured_payload_bytes(4),
    );

    let error = DataConverter::from(&value)
        .to_with::<serde_json::Value>(&ConversionPolicy::default(), &limits)
        .expect_err(
            "the key and string payload total should exceed four bytes",
        );

    assert_eq!(
        error.budget_error().map(|error| *error.resource()),
        Some(ConversionResource::StructuredPayloadBytes),
    );
}

#[test]
fn deeply_nested_materialized_json_fails_without_recursive_traversal() {
    let mut value = serde_json::Value::Null;
    for _ in 0..20_000 {
        value = serde_json::Value::Array(vec![value]);
    }
    let limits = ConversionLimits::default().with_structured_limits(
        StructuredConversionLimits::default().with_max_depth(128),
    );

    let error = DataConverter::from(&value)
        .to_with::<serde_json::Value>(&ConversionPolicy::default(), &limits)
        .expect_err(
            "the configured depth limit should reject the materialized value",
        );

    assert_eq!(
        error.budget_error().map(|error| *error.resource()),
        Some(ConversionResource::StructuredDepth),
    );
    std::mem::forget(value);
}

#[test]
fn string_map_and_json_charge_equivalent_key_and_string_payload() {
    let map =
        std::collections::HashMap::from([("k".to_owned(), "abcd".to_owned())]);
    let json = serde_json::json!({"k": "abcd"});
    let limits = ConversionLimits::default().with_operation_limits(
        ConversionOperationLimits::default()
            .with_max_structured_payload_bytes(4),
    );
    let policy = ConversionPolicy::default();

    let map_error = DataConverter::from(&map)
        .to_with::<std::collections::HashMap<String, String>>(&policy, &limits)
        .expect_err("the string map payload should exceed four bytes");
    let json_error = DataConverter::from(&json)
        .to_with::<serde_json::Value>(&policy, &limits)
        .expect_err("the JSON payload should exceed four bytes");

    assert_eq!(map_error.budget_error(), json_error.budget_error());
}
