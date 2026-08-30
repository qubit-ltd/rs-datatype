// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests for domain-level bounded String construction.

use qubit_datatype::ConversionLimits;
use qubit_datatype::ConversionOperationLimits;
use qubit_datatype::ConversionPolicy;
use qubit_datatype::ConversionResource;
use qubit_datatype::ConversionSession;
use qubit_datatype::DataConversionError;
use qubit_datatype::DataConversionErrorKind;
use qubit_datatype::DataType;
use qubit_datatype::InvalidValueReason;

/// Verifies successful output is committed once through the public facade.
#[test]
fn test_write_string_commits_successful_output() {
    let policy = ConversionPolicy::default();
    let limits = ConversionLimits::default();
    let mut session = ConversionSession::new(&policy, &limits);

    let output = session
        .write_string(DataType::Int32, DataType::String, |writer| {
            writer.write_str("value=")?;
            writer.write_display(&42)
        })
        .expect("bounded output should render");

    assert_eq!(output, "value=42");
    assert_eq!(session.output_bytes_used(), 8);
}

/// Verifies a rendering failure rolls back buffered output bytes.
#[test]
fn test_write_string_rolls_back_render_failure() {
    let policy = ConversionPolicy::default();
    let limits = ConversionLimits::default();
    let mut session = ConversionSession::new(&policy, &limits);

    let error = session
        .write_string(DataType::String, DataType::String, |writer| {
            writer.write_str("discarded")?;
            Err(DataConversionError::invalid(
                DataType::String,
                DataType::String,
                InvalidValueReason::OutOfRange,
            ))
        })
        .expect_err("render failure should discard output");

    assert_eq!(error.kind(), DataConversionErrorKind::InvalidValue);
    assert_eq!(session.output_bytes_used(), 0);
}

/// Verifies the public writer maps output exhaustion to a domain error and
/// leaves the cumulative counter unchanged.
#[test]
fn test_write_string_maps_output_budget_failure() {
    let policy = ConversionPolicy::default();
    let limits = ConversionLimits::builder()
        .operation_limits(ConversionOperationLimits::builder().max_output_bytes(1).build())
        .build();
    let mut session = ConversionSession::new(&policy, &limits);

    let error = session
        .write_string(DataType::String, DataType::String, |writer| {
            writer.write_str("too large")
        })
        .expect_err("output above the cumulative limit should fail");

    assert_eq!(error.kind(), DataConversionErrorKind::LimitExceeded);
    assert_eq!(error.resource(), Some(ConversionResource::OutputBytes));
    assert_eq!(session.output_bytes_used(), 0);
}
