// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests for aggregate conversion limits.

use qubit_datatype::converter::CollectionConversionLimits;
use qubit_datatype::converter::ConversionLimits;
use qubit_datatype::converter::ConversionOperationLimits;
use qubit_datatype::converter::DurationConversionLimits;
use qubit_datatype::converter::NumericConversionLimits;
use qubit_datatype::converter::StructuredConversionLimits;

#[test]
fn test_conversion_limits_aggregate_exact_limit_groups() {
    let limits = ConversionLimits::builder()
        .numeric_limits(NumericConversionLimits::builder().max_text_bytes(11).build())
        .collection_limits(CollectionConversionLimits::builder().max_items(13).build())
        .duration_limits(DurationConversionLimits::builder().max_text_bytes(17).build())
        .structured_limits(StructuredConversionLimits::builder().max_depth(19).build())
        .operation_limits(ConversionOperationLimits::builder().max_items(23).build())
        .build();

    assert_eq!(limits.numeric().max_text_bytes(), 11);
    assert_eq!(limits.collection().max_items(), 13);
    assert_eq!(limits.duration().max_text_bytes(), 17);
    assert_eq!(limits.structured().max_depth(), 19);
    assert_eq!(limits.operation().max_items(), 23);
    let value = serde_json::to_value(&limits).expect("aggregate limits should serialize");
    assert_eq!(
        value
            .as_object()
            .expect("limits wire should be an object")
            .keys()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        ["numeric", "collection", "duration", "structured", "operation"]
    );
}

#[test]
fn test_conversion_limits_default_ref_is_stable() {
    assert!(std::ptr::eq(
        ConversionLimits::default_ref(),
        ConversionLimits::default_ref()
    ));
    assert_eq!(ConversionLimits::default_ref(), &ConversionLimits::default());
}

/// Verifies that existing limits can be customized through their builder.
#[test]
fn test_conversion_limits_into_builder() {
    let limits = ConversionLimits::default()
        .into_builder()
        .operation_limits(ConversionOperationLimits::default().into_builder().max_items(3).build())
        .build();

    assert_eq!(limits.operation().max_items(), 3);
}
