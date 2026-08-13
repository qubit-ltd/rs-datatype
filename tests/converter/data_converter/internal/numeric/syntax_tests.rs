// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Numeric syntax validation tests.

use qubit_budget::BudgetError;
use qubit_budget::Observation;
use qubit_datatype::ConversionLimits;
use qubit_datatype::ConversionPolicy;
use qubit_datatype::ConversionResource;
use qubit_datatype::DataConversionError;
use qubit_datatype::DataConverter;
use qubit_datatype::DataType;
use qubit_datatype::InvalidValueReason;
use qubit_datatype::NumericConversionLimits;

/// Verifies malformed signs are classified as invalid numeric syntax.
#[test]
fn test_numeric_text_rejects_duplicate_signs() {
    assert!(matches!(
        DataConverter::from("--1").to::<f64>(),
        Err(conversion_error) if matches!(conversion_error.reason(), Some(InvalidValueReason::InvalidSyntax { .. })
    )));
}

/// Verifies numeric text at the configured byte boundary preserves its
/// conversion-limit fact.
#[test]
fn test_numeric_text_limit_preserves_conversion_limit_fact() {
    let options = ConversionPolicy::default();
    let limits = ConversionLimits::default()
        .with_numeric_limits(NumericConversionLimits::default().with_max_text_bytes(3));

    assert_eq!(
        DataConverter::from("123").to_with::<u32>(&options, &limits),
        Ok(123),
    );
    assert_eq!(
        DataConverter::from("1234").to_with::<u32>(&options, &limits),
        Err(DataConversionError::limit_exceeded(
            DataType::String,
            DataType::UInt32,
            BudgetError::LimitExceeded {
                resource: ConversionResource::NumericTextBytes,
                observed: Observation::Exact(4),
                maximum: 3,
            },
        )),
    );
}
