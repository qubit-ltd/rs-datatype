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
    let limits = ConversionLimits::builder()
        .operation_limits(ConversionOperationLimits::builder().max_input_bytes(3).build())
        .build();
    let mut session = ConversionSession::new(&policy, &limits);

    DataConverter::from("12")
        .to_in::<u16>(&mut session)
        .expect("first text should fit");
    let error = DataConverter::from("34")
        .to_in::<u16>(&mut session)
        .expect_err("second text should exceed cumulative bytes");

    assert_eq!(error.resource(), Some(ConversionResource::InputBytes));
    assert_eq!(error.configured_limit(), Some(3));
    assert_eq!(error.used(), Some(2));
    assert_eq!(error.remaining(), Some(1));
    assert_eq!(error.requested(), Some(2));
}

#[test]
fn test_independent_sessions_reset_operation_usage() {
    let policy = ConversionPolicy::default();
    let limits = ConversionLimits::builder()
        .operation_limits(ConversionOperationLimits::builder().max_input_bytes(2).build())
        .build();

    for _ in 0..2 {
        DataConverter::from("12")
            .to_with::<u16>(&policy, &limits)
            .expect("each public conversion should create a fresh session");
    }
}

#[test]
fn test_trimmed_text_is_charged_by_raw_input_length() {
    let policy = ConversionPolicy::builder()
        .string_policy(StringConversionPolicy::builder().trim(true).build())
        .build();
    let limits = ConversionLimits::builder()
        .operation_limits(ConversionOperationLimits::builder().max_input_bytes(2).build())
        .build();

    let error = DataConverter::from(" 12 ")
        .to_with::<u16>(&policy, &limits)
        .expect_err("raw whitespace must count toward the input budget");

    assert_eq!(error.resource(), Some(ConversionResource::InputBytes));
}

#[test]
fn test_failed_conversion_keeps_attempted_item_and_input_accounting() {
    let policy = ConversionPolicy::default();
    let limits = ConversionLimits::builder()
        .operation_limits(
            ConversionOperationLimits::builder()
                .max_items(2)
                .max_input_bytes(5)
                .build(),
        )
        .build();
    let mut session = ConversionSession::new(&policy, &limits);

    let _ = DataConverter::from("bad")
        .to_in::<u16>(&mut session)
        .expect_err("invalid syntax should fail after admission");
    DataConverter::from("1")
        .to_in::<u16>(&mut session)
        .expect("the second attempted item should still fit");
    let error = DataConverter::from("2")
        .to_in::<u16>(&mut session)
        .expect_err("the failed attempt must retain its item budget usage");

    assert_eq!(error.resource(), Some(ConversionResource::Items));
}

/// Verifies public conversions share the configured output budget.
#[test]
fn test_public_conversion_output_accounting_uses_shared_budget() {
    let policy = ConversionPolicy::default();
    let limits = ConversionLimits::builder()
        .operation_limits(ConversionOperationLimits::builder().max_output_bytes(2).build())
        .build();
    let mut session = ConversionSession::new(&policy, &limits);

    DataConverter::from("ok")
        .to_in::<String>(&mut session)
        .expect("exact native length should consume budget");
    let error = DataConverter::from("x")
        .to_in::<String>(&mut session)
        .expect_err("remaining budget should be exhausted");

    assert_eq!(error.resource(), Some(ConversionResource::OutputBytes));
}

/// Verifies scalar item admission produces one-use conversion tokens.
#[test]
fn test_scalar_admission_charges_items_once() {
    let policy = ConversionPolicy::env_friendly();
    let limits = ConversionLimits::builder()
        .operation_limits(ConversionOperationLimits::builder().max_items(1).build())
        .build();
    let mut session = ConversionSession::new(&policy, &limits);

    let mut source = session.admit_scalar_string_source(",,abc,d").expect("source fits");
    let admitted = source
        .next_item()
        .expect("first retained item")
        .expect("the first scalar item should fit");
    assert_eq!(admitted.source_index(), 2);
    assert_eq!(admitted.convert::<String>(), Ok(String::from("abc")));

    let error = source
        .next_item()
        .expect("second retained item")
        .expect_err("the second scalar item should exceed the item budget");
    assert_eq!(error.conversion_error().resource(), Some(ConversionResource::Items));
}

/// Verifies external adapters can charge one scalar-string source before
/// splitting it into individually admitted items.
#[test]
fn test_scalar_string_source_admission_charges_input_once() {
    let policy = ConversionPolicy::default();
    let limits = ConversionLimits::builder()
        .operation_limits(ConversionOperationLimits::builder().max_input_bytes(3).build())
        .build();
    let mut session = ConversionSession::new(&policy, &limits);

    let _ = session
        .admit_scalar_string_source("abc")
        .expect("the exact source length should fit");
    assert_eq!(session.input_bytes_used(), 3);

    let error = session
        .admit_scalar_string_source("d")
        .expect_err("a second source should exceed the cumulative input budget");
    assert_eq!(error.resource(), Some(ConversionResource::InputBytes));
}

/// Exercises the session's public configuration and usage observability
/// methods.
#[test]
fn test_session_observability_methods() {
    let policy = ConversionPolicy::default();
    let limits = ConversionLimits::default();
    let mut session = ConversionSession::new(&policy, &limits);

    assert_eq!(session.policy(), &policy);
    assert_eq!(session.limits(), &limits);
    DataConverter::from("ok")
        .to_in::<String>(&mut session)
        .expect("public conversion should fit");

    assert_eq!(session.items_limit(), limits.operation().max_items());
    assert_eq!(session.input_bytes_limit(), limits.operation().max_input_bytes());
    assert_eq!(session.output_bytes_limit(), limits.operation().max_output_bytes());
    assert_eq!(
        session.structured_nodes_limit(),
        limits.operation().max_structured_nodes()
    );
    assert_eq!(
        session.structured_payload_bytes_limit(),
        limits.operation().max_structured_payload_bytes()
    );
    assert!(session.items_remaining() <= session.items_limit());
    assert!(session.items_used() <= session.items_limit());
    assert!(session.input_bytes_used() <= session.input_bytes_limit());
    assert!(session.input_bytes_remaining() <= session.input_bytes_limit());
    assert!(session.output_bytes_used() <= session.output_bytes_limit());
    assert!(session.output_bytes_remaining() <= session.output_bytes_limit());
    assert_eq!(session.structured_nodes_used(), 0);
    assert_eq!(session.structured_payload_bytes_used(), 0);
}
