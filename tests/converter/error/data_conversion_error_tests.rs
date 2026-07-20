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
    ConversionLimit, DataConversionError, DataConversionErrorKind, DataListConversionError,
    InvalidValueReason,
};

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
    assert_eq!(error.limit(), None);

    let empty = DataConversionError::empty_collection(DataType::String);
    assert_eq!(empty.kind(), DataConversionErrorKind::EmptyCollection);
    assert_eq!(empty.from_type(), None);
    assert_eq!(empty.to_type(), DataType::String);
    assert_eq!(empty.reason(), None);
    assert_eq!(empty.limit(), None);
}

/// Test the resource-limit constructor, accessors, and list wrapping.
#[test]
fn test_data_conversion_error_limit_exceeded_contract() {
    let limit = ConversionLimit::BigIntegerDigits { maximum: 12 };
    let error = DataConversionError::limit_exceeded(DataType::String, DataType::BigInteger, limit);

    assert_eq!(error.kind(), DataConversionErrorKind::LimitExceeded);
    assert!(!error.is_missing());
    assert_eq!(error.from_type(), Some(DataType::String));
    assert_eq!(error.to_type(), DataType::BigInteger);
    assert_eq!(error.reason(), None);
    assert_eq!(error.limit(), Some(&limit));

    let list_error = DataListConversionError::new(7, error.clone());
    assert_eq!(list_error.source_index(), 7);
    assert_eq!(list_error.conversion_error(), &error);
    assert_eq!(list_error.conversion_error().limit(), Some(&limit));
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
