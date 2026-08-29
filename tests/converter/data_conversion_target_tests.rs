// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests for downstream conversion targets and shared sessions.

use qubit_datatype::ConversionLimits;
use qubit_datatype::ConversionOperationLimits;
use qubit_datatype::ConversionPolicy;
use qubit_datatype::ConversionResource;
use qubit_datatype::ConversionSession;
use qubit_datatype::DataConverter;
use qubit_datatype::DataType;

use super::internal::Port;
use super::internal::Text;

#[test]
fn test_data_conversion_target_supports_downstream_newtype() {
    let port = DataConverter::from("8080")
        .to::<Port>()
        .expect("string should convert through local target implementation");
    assert_eq!(port, Port(8080));
}

#[test]
fn test_data_conversion_target_consuming_api_supports_downstream_newtype() {
    let port = DataConverter::from(String::from("8080"))
        .into_target::<Port>()
        .expect("owned string should use the downstream conversion fallback");
    assert_eq!(port, Port(8080));
}

#[test]
fn test_delegate_does_not_double_charge_item_or_input_budgets() {
    let policy = ConversionPolicy::strict();
    let limits = ConversionLimits::builder()
        .operation_limits(
            ConversionOperationLimits::builder()
                .max_items(1)
                .max_input_bytes(2)
                .build(),
        )
        .build();
    let mut session = ConversionSession::new(&policy, &limits);

    assert_eq!(DataConverter::from("42").to_in::<Port>(&mut session), Ok(Port(42)),);
    let error = DataConverter::from("1")
        .to_in::<Port>(&mut session)
        .expect_err("a second top-level item should exceed the shared budget");
    assert_eq!(
        error.budget_error().map(|error| *error.resource()),
        Some(ConversionResource::Items),
    );
}

#[test]
fn test_delegate_preserves_output_budget() {
    let policy = ConversionPolicy::strict();
    let limits = ConversionLimits::builder()
        .operation_limits(ConversionOperationLimits::builder().max_output_bytes(1).build())
        .build();
    let mut session = ConversionSession::new(&policy, &limits);

    let error = DataConverter::from(42_u16)
        .to_in::<Text>(&mut session)
        .expect_err("delegated String output should share the one-byte budget");
    assert_eq!(error.to_type(), DataType::String);
}

#[test]
fn test_delegate_owned_preserves_output_budget() {
    let policy = ConversionPolicy::strict();
    let limits = ConversionLimits::builder()
        .operation_limits(ConversionOperationLimits::builder().max_output_bytes(1).build())
        .build();
    let mut session = ConversionSession::new(&policy, &limits);

    let _ = DataConverter::from(42_u16)
        .into_target_in::<Text>(&mut session)
        .expect_err("owned delegation must share the one-byte output budget");
}
