// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Regression tests for cumulative String output budgets.

use qubit_datatype::ConversionLimits;
use qubit_datatype::ConversionOperationLimits;
use qubit_datatype::ConversionPolicy;
use qubit_datatype::ConversionResource;
use qubit_datatype::ConversionSession;
use qubit_datatype::DataConverter;

fn limits_with_output_limit(maximum: usize) -> ConversionLimits {
    ConversionLimits::builder()
        .operation_limits(
            ConversionOperationLimits::builder()
                .max_output_bytes(u64::try_from(maximum).unwrap())
                .build(),
        )
        .build()
}

#[test]
fn test_string_output_budget_counts_borrowed_and_owned_payloads_equally() {
    let policy = ConversionPolicy::default();
    let limits = limits_with_output_limit(3);
    let mut borrowed_session = ConversionSession::new(&policy, &limits);
    let mut owned_session = ConversionSession::new(&policy, &limits);

    assert_eq!(
        DataConverter::from(42_i32).to_in::<String>(&mut borrowed_session),
        Ok("42".to_owned()),
    );
    assert_eq!(
        DataConverter::from(42_i32).into_target_in::<String>(&mut owned_session),
        Ok("42".to_owned()),
    );
}

#[test]
fn test_string_output_budget_failure_is_transactional() {
    let policy = ConversionPolicy::default();
    let limits = limits_with_output_limit(3);
    let mut session = ConversionSession::new(&policy, &limits);

    assert_eq!(
        DataConverter::from(12_i32).to_in::<String>(&mut session),
        Ok("12".to_owned()),
    );
    let first = DataConverter::from(345_i32)
        .to_in::<String>(&mut session)
        .expect_err("three more bytes should exceed the remaining budget");
    let second = DataConverter::from(7_i32)
        .to_in::<String>(&mut session)
        .expect("the rejected render must not consume output bytes");

    assert_eq!(second, "7");
    assert_eq!(first.resource(), Some(ConversionResource::OutputBytes));
    assert_eq!(first.configured_limit(), Some(3));
    assert_eq!(first.remaining(), Some(1));
    assert_eq!(first.requested(), Some(3));
}

#[test]
fn test_string_output_budget_counts_utf8_bytes() {
    let policy = ConversionPolicy::default();
    let limits = limits_with_output_limit("é".len());

    assert_eq!(
        DataConverter::from("é").to_with::<String>(&policy, &limits),
        Ok("é".to_owned()),
    );
}

#[test]
fn test_owned_string_output_reuses_unchanged_storage() {
    let source = String::from("unchanged");
    let pointer = source.as_ptr();
    let output = DataConverter::from(source)
        .into_target::<String>()
        .expect("owned String conversion should succeed");
    assert_eq!(output.as_ptr(), pointer);
}
