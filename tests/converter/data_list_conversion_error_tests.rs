/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0
 *
 ******************************************************************************/
use std::error::Error;

use qubit_datatype::converter::{
    DataConversionError,
    DataListConversionError,
};

/// Test display text and source propagation for list conversion errors.
#[test]
fn test_data_list_conversion_error_exposes_index_and_source() {
    let error = DataListConversionError {
        index: 3,
        source: DataConversionError::NoValue,
    };

    assert_eq!(
        error.to_string(),
        "Data conversion failed at index 3: No value",
    );
    assert_eq!(Error::source(&error).unwrap().to_string(), "No value");
}
