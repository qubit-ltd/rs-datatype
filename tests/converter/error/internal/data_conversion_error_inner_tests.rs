// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Private error representation coverage is provided by public error tests.

use qubit_datatype::DataConversionError;
use qubit_datatype::DataConversionErrorKind;
use qubit_datatype::DataType;

#[test]
fn test_private_error_representation_is_covered_by_public_contract() {
    assert_eq!(
        DataConversionError::missing(DataType::String, DataType::Bool).kind(),
        DataConversionErrorKind::Missing
    );
}
