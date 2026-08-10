// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Conversion session accounting tests.

use qubit_budget::BudgetError;
use qubit_datatype::DataConverter;
use qubit_datatype::StringConversionOptions;
use qubit_datatype::converter::ConversionBudgetLimits;
use qubit_datatype::converter::ConversionResource;
use qubit_datatype::converter::DataConversionOptions;

#[test]
fn test_conversion_session_accumulates_budget_facts() {
    let options = DataConversionOptions::default().with_budget_limits(
        ConversionBudgetLimits::default()
            .with_max_items(1)
            .with_max_input_bytes(3)
            .with_max_output_bytes(2)
            .with_max_structured_nodes(1),
    );
    let mut session = options.session();

    session.consume_item().expect("first item should fit");
    let error = session.consume_item().expect_err("second item must fail");
    assert_eq!(
        error,
        BudgetError::Insufficient {
            resource: ConversionResource::Items,
            limit: 1,
            remaining: 0,
            requested: 1,
        }
    );
}

#[test]
fn test_conversion_session_checks_point_limits() {
    let options = DataConversionOptions::default();
    let session = options.session();
    let error = session
        .check_numeric_text_bytes(
            options.numeric().limits().max_text_bytes() + 1,
        )
        .expect_err("oversized numeric text must fail");
    assert_eq!(
        error,
        BudgetError::LimitExceeded {
            resource: ConversionResource::NumericTextBytes,
            actual: options.numeric().limits().max_text_bytes() + 1,
            maximum: options.numeric().limits().max_text_bytes(),
        }
    );
}

#[test]
fn test_data_converter_to_in_consumes_shared_input_budget() {
    let options = DataConversionOptions::default().with_budget_limits(
        ConversionBudgetLimits::default().with_max_input_bytes(3),
    );
    let mut session = options.session();
    DataConverter::from("12")
        .to_in::<u16>(&mut session)
        .expect("first text should fit");
    let error = DataConverter::from("34")
        .to_in::<u16>(&mut session)
        .expect_err("second text should exceed cumulative bytes");
    assert_eq!(
        error.budget_error().map(|facts| *facts.resource()),
        Some(ConversionResource::InputBytes)
    );
}

#[test]
fn test_data_converter_input_budget_counts_trimmed_text_once() {
    let options = DataConversionOptions::default()
        .with_string_options(StringConversionOptions::default().with_trim(true))
        .with_budget_limits(
            ConversionBudgetLimits::default().with_max_input_bytes(2),
        );
    let mut session = options.session();

    DataConverter::from(" 12 ")
        .to_in::<u16>(&mut session)
        .expect("trimmed text should consume its normalized length once");
    assert_eq!(session.used_input_bytes(), 2);
}
