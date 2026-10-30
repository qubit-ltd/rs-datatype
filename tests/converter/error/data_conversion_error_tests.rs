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
use qubit_datatype::converter::DataConversionErrorKind;
use qubit_datatype::converter::DataListConversionError;
use qubit_datatype::converter::InvalidValueReason;

/// Test the conversion error constructors and accessors.
#[test]
fn test_data_conversion_error_constructors_and_accessors() {
    let missing = DataConversionError::missing(DataType::String, DataType::Bool);
    assert_eq!(missing.kind(), DataConversionErrorKind::Missing);
    assert_eq!(missing.json_diagnostic(), None);

    let reason = InvalidValueReason::OutOfRange;
    let error = DataConversionError::invalid(DataType::Int64, DataType::UInt8, reason.clone());
    assert_eq!(error.kind(), DataConversionErrorKind::InvalidValue);
    assert!(!error.is_missing());
    assert_eq!(error.from_type(), Some(DataType::Int64));
    assert_eq!(error.to_type(), DataType::UInt8);
    assert_eq!(error.reason(), Some(&reason));
    assert_eq!(error.resource(), None);
    assert_eq!(error.json_diagnostic(), None);

    let empty = DataConversionError::empty_collection(DataType::String);
    assert_eq!(empty.kind(), DataConversionErrorKind::EmptyCollection);
    assert_eq!(empty.from_type(), None);
    assert_eq!(empty.to_type(), DataType::String);
    assert_eq!(empty.reason(), None);
    assert_eq!(empty.resource(), None);
    assert_eq!(empty.json_diagnostic(), None);
    assert_eq!(
        DataConversionError::unsupported(DataType::String, DataType::Bool).json_diagnostic(),
        None
    );
}

/// Test the resource-limit constructor, accessors, and list wrapping.
#[test]
fn test_data_conversion_error_limit_exceeded_contract() {
    let policy = ConversionPolicy::default();
    let limits = ConversionLimits::builder()
        .operation_limits(ConversionOperationLimits::builder().max_output_bytes(4).build())
        .build();
    let mut session = ConversionSession::new(&policy, &limits);
    DataConverter::from("one")
        .to_in::<String>(&mut session)
        .expect("initial output should fit");
    let error = DataConverter::from("two")
        .to_in::<String>(&mut session)
        .expect_err("second output should exceed remaining capacity");

    assert_eq!(error.kind(), DataConversionErrorKind::LimitExceeded);
    assert!(!error.is_missing());
    assert_eq!(error.from_type(), Some(DataType::String));
    assert_eq!(error.to_type(), DataType::String);
    assert_eq!(error.reason(), None);
    assert_eq!(error.resource(), Some(ConversionResource::OutputBytes));
    assert_eq!(error.configured_limit(), Some(4));
    assert_eq!(error.observed_lower_bound(), None);
    assert_eq!(error.observation_is_exact(), None);
    assert_eq!(error.used(), Some(3));
    assert_eq!(error.remaining(), Some(1));
    assert_eq!(error.requested(), Some(3));

    let cause = Error::source(&error).expect("budget error must expose its underlying cause");
    let budget = cause
        .downcast_ref::<BudgetError<ConversionResource, u64>>()
        .expect("cause must be the native budget error, not the private wrapper");
    assert_eq!(budget.resource(), &ConversionResource::OutputBytes);
    assert_eq!(budget.configured_limit(), 4);
    assert_eq!(budget.remaining(), Some(1));
    assert_eq!(budget.requested(), Some(3));

    let list_error = DataListConversionError::new(7, error.clone());
    assert_eq!(list_error.source_index(), 7);
    assert_eq!(list_error.conversion_error(), &error);
    assert_eq!(
        list_error.conversion_error().resource(),
        Some(ConversionResource::OutputBytes),
    );
}

/// Test that ordinary conversion errors do not expose a fabricated cause.
#[test]
fn test_data_conversion_error_non_budget_errors_have_no_source() {
    let errors = [
        DataConversionError::missing(DataType::String, DataType::Bool),
        DataConversionError::unsupported(DataType::String, DataType::Bool),
        DataConversionError::empty_collection(DataType::Bool),
        DataConversionError::invalid(DataType::String, DataType::Bool, InvalidValueReason::OutOfRange),
    ];

    for error in errors {
        assert!(Error::source(&error).is_none(), "unexpected source for {error:?}");
    }
}

/// Test non-accounting errors do not fabricate resource details.
#[test]
fn test_data_conversion_error_quantity_contract() {
    let error = DataConversionError::invalid(DataType::String, DataType::Json, InvalidValueReason::OutOfRange);

    assert_eq!(error.kind(), DataConversionErrorKind::InvalidValue);
    assert_eq!(error.from_type(), Some(DataType::String));
    assert_eq!(error.to_type(), DataType::Json);
    assert_eq!(error.quantity_resource(), None);
    assert_eq!(error.resource(), None);
}

/// Test that structured errors cannot reveal source text.
#[test]
fn test_data_conversion_error_does_not_contain_source_value() {
    let secret = "secret-marker-9271";
    let error = DataConverter::from(secret)
        .to::<u16>()
        .expect_err("the secret marker is not a valid integer");

    assert_eq!(error.kind(), DataConversionErrorKind::InvalidValue);
    assert!(!error.to_string().contains(secret));
    assert!(!format!("{error:?}").contains(secret));
    let mut cause = Error::source(&error);
    while let Some(current) = cause {
        assert!(!current.to_string().contains(secret));
        assert!(!format!("{current:?}").contains(secret));
        cause = current.source();
    }

    let policy = ConversionPolicy::default();
    let limits = ConversionLimits::builder()
        .operation_limits(ConversionOperationLimits::builder().max_output_bytes(1).build())
        .build();
    let mut session = ConversionSession::new(&policy, &limits);
    let error = DataConverter::from(secret)
        .to_in::<String>(&mut session)
        .expect_err("the secret text should exceed the output budget");
    assert_eq!(error.kind(), DataConversionErrorKind::LimitExceeded);
    let mut cause = Some(&error as &dyn Error);
    while let Some(current) = cause {
        assert!(!current.to_string().contains(secret));
        assert!(!format!("{current:?}").contains(secret));
        cause = current.source();
    }

    #[cfg(feature = "json")]
    {
        let error = DataConverter::from(secret)
            .to::<serde_json::Value>()
            .expect_err("the secret marker is not valid JSON");

        assert_eq!(error.kind(), DataConversionErrorKind::InvalidValue);
        assert!(!error.to_string().contains(secret));
        assert!(!format!("{error:?}").contains(secret));
    }
}
