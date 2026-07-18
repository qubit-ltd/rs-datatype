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
use qubit_datatype::converter::{DataConversionError, DataConversionErrorKind, InvalidValueReason};

/// Test the conversion error constructors and accessors.
#[test]
fn test_data_conversion_error_constructors_and_accessors() {
    let reason = InvalidValueReason::OutOfRange;
    let error = DataConversionError::invalid(DataType::Int64, DataType::UInt8, reason.clone());
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
