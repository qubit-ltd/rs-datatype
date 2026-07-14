// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0
// =============================================================================
use std::error::Error;

use qubit_datatype::DataType;
use qubit_datatype::converter::{
    DataConversionError,
    DataListConversionError,
};

/// Test display text and source propagation for list conversion errors.
#[test]
fn test_data_list_conversion_error_exposes_index_and_source() {
    let error = DataListConversionError {
        source_index: 3,
        source: DataConversionError::Missing {
            from: DataType::String,
            to: DataType::UInt16,
        },
    };

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
