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
    InvalidValueReason,
};

/// Test the conversion error display strings.
#[test]
fn test_data_conversion_error_display() {
    let missing = DataConversionError::Missing {
        from: DataType::String,
        to: DataType::Int32,
    };
    assert_eq!(
        missing.to_string(),
        "Missing value for conversion from string to int32",
    );
    assert_eq!(missing.clone(), missing);
    assert_eq!(
        DataConversionError::Unsupported {
            from: DataType::String,
            to: DataType::Int32,
        }
        .to_string(),
        "Unsupported conversion from string to int32",
    );
    assert_eq!(
        DataConversionError::InvalidValue {
            from: DataType::String,
            to: DataType::Int32,
            reason: InvalidValueReason::PrecisionLoss,
        }
        .to_string(),
        "Invalid conversion from string to int32: precision loss",
    );
    assert_eq!(
        DataConversionError::EmptyCollection {
            to: DataType::Int32,
        }
        .to_string(),
        "Cannot convert the first value of an empty collection to int32",
    );
}

/// Test that structured errors cannot reveal source text.
#[test]
fn test_data_conversion_error_does_not_contain_source_value() {
    let secret = "secret-marker-9271";
    let error = DataConversionError::InvalidValue {
        from: DataType::String,
        to: DataType::Json,
        reason: InvalidValueReason::InvalidSyntax { expected: "JSON" },
    };
    assert!(!error.to_string().contains(secret));
    assert!(!format!("{error:?}").contains(secret));
}
