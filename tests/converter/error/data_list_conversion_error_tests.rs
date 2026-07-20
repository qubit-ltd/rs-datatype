// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
use std::error::Error;

use qubit_datatype::DataType;
use qubit_datatype::converter::{DataConversionError, DataListConversionError, InvalidValueReason};

/// Test construction and access to the encapsulated error context.
#[test]
fn test_data_list_conversion_error_constructors_and_accessors() {
    let source = DataConversionError::invalid(
        DataType::String,
        DataType::UInt16,
        InvalidValueReason::OutOfRange,
    );
    let error = DataListConversionError::new(3, source.clone());

    assert_eq!(error.source_index(), 3);
    assert_eq!(error.conversion_error(), &source);
    assert_eq!(error.clone().into_conversion_error(), source);

    let error = DataListConversionError::new(3, source.clone());
    assert_eq!(error.into_parts(), (3, source));
}

/// Test display text and source propagation for list conversion errors.
#[test]
fn test_data_list_conversion_error_exposes_index_and_source() {
    let error = DataListConversionError::new(
        3,
        DataConversionError::missing(DataType::String, DataType::UInt16),
    );

    assert_eq!(
        error.to_string(),
        "Data conversion failed at source index 3: Missing value for conversion from string to uint16",
    );
    assert_eq!(
        Error::source(&error)
            .expect("list error should expose its source")
            .to_string(),
        "Missing value for conversion from string to uint16",
    );
}
