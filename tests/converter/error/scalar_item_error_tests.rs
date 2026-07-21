// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests for scalar item iteration errors.

use std::error::Error;

use qubit_datatype::{
    ConversionLimit,
    DataConversionErrorKind,
    DataType,
    InvalidValueReason,
    ScalarItemError,
};

/// Test construction and access to the encapsulated source index.
#[test]
fn test_scalar_item_error_constructor_and_accessor() {
    let error = ScalarItemError::new(4);

    assert_eq!(error.source_index(), 4);
    assert_eq!(error.maximum_items(), None);
    assert!(matches!(error, ScalarItemError::BlankRejected { .. }));
}

/// Test collection item-limit accessors and target-aware conversion.
#[test]
fn test_scalar_item_error_item_limit_exceeded_contract() {
    let error = ScalarItemError::ItemLimitExceeded {
        source_index: 5,
        maximum: 2,
    };
    assert_eq!(error.source_index(), 5);
    assert_eq!(error.maximum_items(), Some(2));
    assert_eq!(
        error.to_string(),
        "scalar collection exceeds the 2-item limit at source index 5",
    );

    let converted = error.into_data_conversion_error(DataType::UInt16);
    assert_eq!(converted.kind(), DataConversionErrorKind::LimitExceeded);
    assert_eq!(converted.from_type(), Some(DataType::String));
    assert_eq!(converted.to_type(), DataType::UInt16);
    assert_eq!(
        converted.limit(),
        Some(&ConversionLimit::CollectionItems { maximum: 2 }),
    );
}

/// Test that a rejected item reports its original unfiltered source index.
#[test]
fn test_scalar_item_error_reports_source_index() {
    let error = ScalarItemError::new(3);
    let copied = error;

    assert_eq!(copied.source_index(), 3);
    assert_eq!(error, copied);
    assert_eq!(
        error.to_string(),
        "blank scalar item rejected at source index 3",
    );
    assert!(error.source().is_none());
}

/// Test conversion into a target-aware scalar conversion error.
#[test]
fn test_scalar_item_error_into_data_conversion_error() {
    let error =
        ScalarItemError::new(3).into_data_conversion_error(DataType::UInt16);

    assert_eq!(error.kind(), DataConversionErrorKind::InvalidValue);
    assert_eq!(error.from_type(), Some(DataType::String));
    assert_eq!(error.to_type(), DataType::UInt16);
    assert_eq!(error.reason(), Some(&InvalidValueReason::BlankRejected));
}

/// Test conversion into a list error preserving the original source index.
#[test]
fn test_scalar_item_error_into_list_conversion_error() {
    let error =
        ScalarItemError::new(3).into_list_conversion_error(DataType::Bool);

    assert_eq!(error.source_index(), 3);
    assert_eq!(
        error.conversion_error().kind(),
        DataConversionErrorKind::InvalidValue,
    );
    assert_eq!(error.conversion_error().from_type(), Some(DataType::String));
    assert_eq!(error.conversion_error().to_type(), DataType::Bool);
    assert_eq!(
        error.conversion_error().reason(),
        Some(&InvalidValueReason::BlankRejected),
    );
}
