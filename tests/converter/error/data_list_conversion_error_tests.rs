// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
use std::error::Error;

use qubit_budget::BudgetError;
use qubit_datatype::ConversionLimits;
use qubit_datatype::ConversionOperationLimits;
use qubit_datatype::ConversionPolicy;
use qubit_datatype::ConversionResource;
use qubit_datatype::ConversionSession;
use qubit_datatype::DataConverter;
use qubit_datatype::DataType;
use qubit_datatype::converter::DataConversionError;
use qubit_datatype::converter::DataListConversionError;
use qubit_datatype::converter::InvalidValueReason;

/// Test construction and access to the encapsulated error context.
#[test]
fn test_data_list_conversion_error_constructors_and_accessors() {
    let source = DataConversionError::invalid(DataType::String, DataType::UInt16, InvalidValueReason::OutOfRange);
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
    let error = DataListConversionError::new(3, DataConversionError::missing(DataType::String, DataType::UInt16));

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

/// Test that list errors preserve the complete resource failure cause chain.
#[test]
fn test_data_list_conversion_error_preserves_budget_cause_chain() {
    let policy = ConversionPolicy::default();
    let limits = ConversionLimits::builder()
        .operation_limits(ConversionOperationLimits::builder().max_output_bytes(4).build())
        .build();
    let mut session = ConversionSession::new(&policy, &limits);
    DataConverter::from("one")
        .to_in::<String>(&mut session)
        .expect("initial output should fit");
    let conversion_error = DataConverter::from("two")
        .to_in::<String>(&mut session)
        .expect_err("second output should exceed remaining capacity");
    let list_error = DataListConversionError::new(7, conversion_error);

    assert_eq!(list_error.source_index(), 7);
    let conversion_cause = Error::source(&list_error)
        .expect("list error must expose its conversion error")
        .downcast_ref::<DataConversionError>()
        .expect("first cause must be DataConversionError");
    let budget = Error::source(conversion_cause)
        .expect("conversion error must expose the budget error")
        .downcast_ref::<BudgetError<ConversionResource, u64>>()
        .expect("second cause must be the native budget error");
    assert_eq!(budget.resource(), &ConversionResource::OutputBytes);
    assert_eq!(budget.remaining(), Some(1));
    assert_eq!(budget.requested(), Some(3));
}
