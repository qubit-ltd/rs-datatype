// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests for variant-specific data conversion error details.

use qubit_datatype::{
    DataConversionError,
    DataType,
    InvalidValueReason,
};

/// Tests the public diagnostics produced by every private error variant.
#[test]
fn test_data_conversion_error_inner_display() {
    assert_eq!(
        DataConversionError::missing(DataType::String, DataType::Int32)
            .to_string(),
        "Missing value for conversion from string to int32",
    );
    assert_eq!(
        DataConversionError::empty_collection(DataType::Int32).to_string(),
        "Cannot convert the first value of an empty collection to int32",
    );
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
}
