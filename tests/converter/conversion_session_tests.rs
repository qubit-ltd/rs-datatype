// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Conversion session accounting tests.

use qubit_datatype::ConversionLimits;
use qubit_datatype::ConversionOperationLimits;
use qubit_datatype::ConversionPolicy;
use qubit_datatype::ConversionResource;
use qubit_datatype::ConversionSession;
use qubit_datatype::DataConverter;
use qubit_datatype::StringConversionPolicy;

#[test]
fn test_one_session_accumulates_multiple_conversions() {
    let policy = ConversionPolicy::default();
    let limits = ConversionLimits::default().with_operation_limits(
        ConversionOperationLimits::default().with_max_input_bytes(3),
    );
    let mut session = ConversionSession::new(&policy, &limits);

    DataConverter::from("12")
        .to_in::<u16>(&mut session)
        .expect("first text should fit");
    let error = DataConverter::from("34")
        .to_in::<u16>(&mut session)
        .expect_err("second text should exceed cumulative bytes");

    assert_eq!(
        error.budget_error().map(|facts| *facts.resource()),
        Some(ConversionResource::InputBytes),
    );
}

#[test]
fn test_independent_sessions_reset_operation_usage() {
    let policy = ConversionPolicy::default();
    let limits = ConversionLimits::default().with_operation_limits(
        ConversionOperationLimits::default().with_max_input_bytes(2),
    );

    for _ in 0..2 {
        DataConverter::from("12")
            .to_with::<u16>(&policy, &limits)
            .expect("each public conversion should create a fresh session");
    }
}

#[test]
fn test_trimmed_text_is_charged_once() {
    let policy = ConversionPolicy::default()
        .with_string_policy(StringConversionPolicy::default().with_trim(true));
    let limits = ConversionLimits::default().with_operation_limits(
        ConversionOperationLimits::default().with_max_input_bytes(2),
    );

    DataConverter::from(" 12 ")
        .to_with::<u16>(&policy, &limits)
        .expect("trimmed text should consume its normalized length once");
}
