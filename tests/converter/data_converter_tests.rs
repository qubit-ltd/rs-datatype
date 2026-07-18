// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Public DataConverter contract and result-category matrix tests.

use chrono::NaiveDate;
use qubit_datatype::{DataConversionError, DataConverter, DataType, InvalidValueReason};

enum MatrixOutcome {
    Supported(i32),
    Unsupported,
    InvalidSyntax,
    Missing,
}

/// Assert the exact result category for an i32 conversion matrix row.
fn assert_i32_matrix_outcome(converter: DataConverter<'_>, expected: MatrixOutcome) {
    let actual = converter.to::<i32>();
    match expected {
        MatrixOutcome::Supported(expected_value) => {
            assert_eq!(actual, Ok(expected_value));
        }
        MatrixOutcome::Unsupported => assert!(matches!(
            actual,
            Err(ref error) if error
                == &DataConversionError::unsupported(DataType::Date, DataType::Int32),
        )),
        MatrixOutcome::InvalidSyntax => assert!(matches!(
            actual,
            Err(ref error) if error == &DataConversionError::invalid(
                DataType::String,
                DataType::Int32,
                InvalidValueReason::InvalidSyntax {
                    expected: "integer",
                },
            ),
        )),
        MatrixOutcome::Missing => assert!(matches!(
            actual,
            Err(ref error) if error
                == &DataConversionError::missing(DataType::Int32, DataType::Int32),
        )),
    }
}

/// Test supported, unsupported, invalid, and missing result categories.
#[test]
fn test_data_converter_source_target_matrix_classifies_results() {
    let date = NaiveDate::from_ymd_opt(2026, 7, 12).expect("test date should be valid");
    let cases = [
        (DataConverter::from("42"), MatrixOutcome::Supported(42)),
        (DataConverter::from(date), MatrixOutcome::Unsupported),
        (DataConverter::from("bad"), MatrixOutcome::InvalidSyntax),
        (
            DataConverter::Empty(DataType::Int32),
            MatrixOutcome::Missing,
        ),
    ];

    for (converter, expected) in cases {
        assert_i32_matrix_outcome(converter, expected);
    }
}
