// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Focused tests for conversion-session and scalar-source accounting.

#![cfg(feature = "converter")]

use qubit_datatype::CollectionConversionLimits;
use qubit_datatype::ConversionLimits;
use qubit_datatype::ConversionOperationLimits;
use qubit_datatype::ConversionPolicy;
use qubit_datatype::ConversionResource;
use qubit_datatype::ConversionSession;
use qubit_datatype::DataConverter;
use qubit_datatype::ScalarStringDataConverters;

#[test]
fn one_session_accumulates_multiple_conversions() {
    let policy = ConversionPolicy::default();
    let limits = ConversionLimits::default().with_operation_limits(
        ConversionOperationLimits::default().with_max_input_bytes(3),
    );
    let mut session = ConversionSession::new(&policy, &limits);

    DataConverter::from("12")
        .to_in::<u16>(&mut session)
        .expect("first conversion should fit");
    let error = DataConverter::from("34")
        .to_in::<u16>(&mut session)
        .expect_err("the shared input-byte budget should be exhausted");

    assert_eq!(
        error.budget_error().map(|error| *error.resource()),
        Some(ConversionResource::InputBytes),
    );
}

#[test]
fn delimiter_only_source_is_checked_before_scan() {
    let policy = ConversionPolicy::env_friendly();
    let limits = ConversionLimits::default().with_collection_limits(
        CollectionConversionLimits::default().with_max_source_bytes(3),
    );

    let error = ScalarStringDataConverters::from(",,,,")
        .to_vec_with::<String>(&policy, &limits)
        .expect_err(
            "the complete scalar source should be checked before splitting",
        );

    assert_eq!(
        error
            .conversion_error()
            .budget_error()
            .map(|error| *error.resource()),
        Some(ConversionResource::CollectionSourceBytes),
    );
}

#[test]
fn to_first_checks_complete_source_before_short_circuiting() {
    let policy = ConversionPolicy::env_friendly();
    let limits = ConversionLimits::default().with_collection_limits(
        CollectionConversionLimits::default().with_max_source_bytes(3),
    );

    let error = ScalarStringDataConverters::from("1,long-tail")
        .to_first_with::<u16>(&policy, &limits)
        .expect_err("the complete source should be checked before the first item is returned");

    assert_eq!(
        error.budget_error().map(|error| *error.resource()),
        Some(ConversionResource::CollectionSourceBytes),
    );
}
