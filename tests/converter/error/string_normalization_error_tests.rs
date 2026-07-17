// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests for string normalization policy outcomes.

use qubit_datatype::{
    DataConversionErrorKind,
    DataType,
    InvalidValueReason,
    StringNormalizationError,
};

/// Test normalization error identity and value-free display.
#[test]
fn test_string_normalization_error_display() {
    assert_eq!(
        StringNormalizationError::Missing.to_string(),
        "missing string value"
    );
    assert_eq!(
        StringNormalizationError::BlankRejected.to_string(),
        "blank string rejected",
    );
}

/// Test missing-value classification without matching enum variants.
#[test]
fn test_string_normalization_error_is_missing() {
    assert!(StringNormalizationError::Missing.is_missing());
    assert!(!StringNormalizationError::BlankRejected.is_missing());
}

/// Test conversion into a target-aware data conversion error.
#[test]
fn test_string_normalization_error_into_data_conversion_error() {
    let missing = StringNormalizationError::Missing
        .into_data_conversion_error(DataType::UInt16);
    assert_eq!(missing.kind(), DataConversionErrorKind::Missing);
    assert_eq!(missing.from_type(), Some(DataType::String));
    assert_eq!(missing.to_type(), DataType::UInt16);

    let blank = StringNormalizationError::BlankRejected
        .into_data_conversion_error(DataType::Bool);
    assert_eq!(blank.kind(), DataConversionErrorKind::InvalidValue);
    assert_eq!(blank.from_type(), Some(DataType::String));
    assert_eq!(blank.to_type(), DataType::Bool);
    assert_eq!(blank.reason(), Some(&InvalidValueReason::BlankRejected));
}
