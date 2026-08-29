// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests for cumulative conversion operation limits.

use qubit_datatype::converter::ConversionOperationLimits;
use qubit_datatype::converter::ConversionResource;

/// Verifies operation resources use non-`max_` wire field names.
#[test]
fn test_conversion_operation_limits_resource_fields_have_non_max_names() {
    let value =
        serde_json::to_value(ConversionOperationLimits::default()).expect("default operation limits should serialize");
    let object = value
        .as_object()
        .expect("operation limits should serialize as an object");

    assert_eq!(
        object.keys().map(String::as_str).collect::<Vec<_>>(),
        [
            "items",
            "input_bytes",
            "output_bytes",
            "structured_nodes",
            "structured_payload_bytes",
        ],
    );
    assert!(value.get("max_items").is_none());
    assert!(value.get("max_structured_payload_bytes").is_none());
}

#[test]
fn test_conversion_operation_limits_bind_resources_and_defaults() {
    let limits = ConversionOperationLimits::builder()
        .max_items(3)
        .max_input_bytes(5)
        .max_output_bytes(7)
        .max_structured_nodes(11)
        .max_structured_payload_bytes(13)
        .build();

    assert_eq!(limits.items_limit().resource(), &ConversionResource::Items);
    assert_eq!(limits.input_bytes_limit().resource(), &ConversionResource::InputBytes);
    assert_eq!(limits.output_bytes_limit().resource(), &ConversionResource::OutputBytes);
    assert_eq!(
        limits.structured_nodes_limit().resource(),
        &ConversionResource::StructuredNodes
    );
    assert_eq!(
        limits.structured_payload_bytes_limit().resource(),
        &ConversionResource::StructuredPayloadBytes
    );
    assert_eq!(limits.max_items(), 3);
    assert_eq!(limits.max_input_bytes(), 5);
    assert_eq!(limits.max_output_bytes(), 7);
    assert_eq!(limits.max_structured_nodes(), 11);
    assert_eq!(limits.max_structured_payload_bytes(), 13);
    assert_eq!(
        ConversionOperationLimits::DEFAULT_MAX_STRUCTURED_PAYLOAD_BYTES,
        64 * 1024 * 1024
    );
}

#[test]
fn test_conversion_operation_limits_deserialize_all_fields() {
    let limits: ConversionOperationLimits = serde_json::from_str(
        r#"{"items":3,"input_bytes":5,"output_bytes":7,"structured_nodes":11,"structured_payload_bytes":13}"#,
    )
    .expect("all operation limit fields should deserialize");

    assert_eq!(limits.max_items(), 3);
    assert_eq!(limits.max_input_bytes(), 5);
    assert_eq!(limits.max_output_bytes(), 7);
    assert_eq!(limits.max_structured_nodes(), 11);
    assert_eq!(limits.max_structured_payload_bytes(), 13);
}
