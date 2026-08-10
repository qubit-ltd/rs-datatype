// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_datatype::ConversionBudgetLimits;
use qubit_datatype::DataConversionErrorKind;
use qubit_datatype::DataConversionOptions;
use qubit_datatype::DataConverter;
use qubit_datatype::DataType;
use qubit_datatype::StructuredConversionLimits;

#[cfg(feature = "json")]
use super::internal::Document;
use super::internal::Port;
use super::internal::Text;

/// Verifies that a downstream newtype can own its target conversion.
#[test]
fn test_data_conversion_target_supports_downstream_newtype() {
    let port = DataConverter::from("8080")
        .to::<Port>()
        .expect("string should convert through local target implementation");

    assert_eq!(port, Port(8080));
}

/// Verifies the consuming API falls back to a downstream target's borrowed
/// conversion implementation.
#[test]
fn test_data_conversion_target_consuming_api_supports_downstream_newtype() {
    let port = DataConverter::from(String::from("8080"))
        .into_target::<Port>()
        .expect("owned string should use the downstream conversion fallback");

    assert_eq!(port, Port(8080));
}

#[test]
fn test_delegate_does_not_double_charge_item_or_input_budgets() {
    let options = DataConversionOptions::strict().with_budget_limits(
        ConversionBudgetLimits::default()
            .with_max_items(1)
            .with_max_input_bytes(2),
    );
    let mut session = options.session();
    let port = DataConverter::from("42")
        .to_in::<Port>(&mut session)
        .expect(
            "transparent delegation should fit one item and two input bytes",
        );
    assert_eq!(port, Port(42));
    assert_eq!(session.used_items(), 1);
    assert_eq!(session.used_input_bytes(), 2);
}

#[test]
fn test_delegate_preserves_output_budget() {
    let options = DataConversionOptions::strict().with_budget_limits(
        ConversionBudgetLimits::default().with_max_output_bytes(1),
    );
    let mut session = options.session();
    let error = DataConverter::from(42_u16)
        .to_in::<Text>(&mut session)
        .expect_err("delegated String output should share the one-byte budget");
    assert_eq!(error.to_type(), DataType::String);
    assert_eq!(session.used_output_bytes(), 0);
}

#[test]
fn test_delegate_owned_preserves_output_budget() {
    let options = DataConversionOptions::strict().with_budget_limits(
        ConversionBudgetLimits::default().with_max_output_bytes(1),
    );
    let mut session = options.session();
    let _ = DataConverter::from(42_u16)
        .into_target_in::<Text>(&mut session)
        .expect_err("owned delegation must share the one-byte output budget");
    assert_eq!(session.used_output_bytes(), 0);
}

#[cfg(feature = "json")]
#[test]
fn test_delegate_preserves_structural_budget() {
    let options = DataConversionOptions::strict().with_structured_limits(
        StructuredConversionLimits::default().with_max_depth(1),
    );
    let mut session = options.session();
    let error = DataConverter::from(r#"{"outer":{"inner":1}}"#)
        .to_in::<Document>(&mut session)
        .expect_err("delegated JSON parsing must retain the depth limit");
    assert_eq!(error.kind(), DataConversionErrorKind::LimitExceeded);
    assert_eq!(error.to_type(), DataType::Json);
}
