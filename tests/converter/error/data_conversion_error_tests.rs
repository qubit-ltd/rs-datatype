// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # DataConversionError Unit Tests
//!
//! Tests for reusable data conversion errors.

use qubit_datatype::DataType;
use qubit_datatype::converter::{
    DataConversionError,
    DataConversionErrorKind,
    InvalidValueReason,
};

/// Test the conversion error constructors and accessors.
#[test]
fn test_data_conversion_error_constructors_and_accessors() {
    let reason = InvalidValueReason::OutOfRange;
    let error = DataConversionError::invalid(
        DataType::Int64,
        DataType::UInt8,
        reason.clone(),
    );
    assert_eq!(error.kind(), DataConversionErrorKind::InvalidValue);
    assert!(!error.is_missing());
    assert_eq!(error.from_type(), Some(DataType::Int64));
    assert_eq!(error.to_type(), DataType::UInt8);
    assert_eq!(error.reason(), Some(&reason));

    let empty = DataConversionError::empty_collection(DataType::String);
    assert_eq!(empty.kind(), DataConversionErrorKind::EmptyCollection);
    assert_eq!(empty.from_type(), None);
    assert_eq!(empty.to_type(), DataType::String);
    assert_eq!(empty.reason(), None);
}

/// Test the conversion error display strings.
#[test]
fn test_data_conversion_error_display() {
    let missing =
        DataConversionError::missing(DataType::String, DataType::Int32);
    assert_eq!(
        missing.to_string(),
        "Missing value for conversion from string to int32",
    );
    assert_eq!(missing.clone(), missing);
    assert_eq!(
        DataConversionError::unsupported(DataType::String, DataType::Int32)
            .to_string(),
        "Unsupported conversion from string to int32",
    );
    assert_eq!(
        DataConversionError::invalid(
            DataType::String,
            DataType::Int32,
            InvalidValueReason::PrecisionLoss,
        )
        .to_string(),
        "Invalid conversion from string to int32: precision loss",
    );
    assert_eq!(
        DataConversionError::empty_collection(DataType::Int32).to_string(),
        "Cannot convert the first value of an empty collection to int32",
    );
}

/// Test that structured errors cannot reveal source text.
#[test]
fn test_data_conversion_error_does_not_contain_source_value() {
    let secret = "secret-marker-9271";
    let error = DataConversionError::invalid(
        DataType::String,
        DataType::Json,
        InvalidValueReason::InvalidSyntax { expected: "JSON" },
    );
    assert!(!error.to_string().contains(secret));
    assert!(!format!("{error:?}").contains(secret));
}
