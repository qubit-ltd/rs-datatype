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
use qubit_datatype::DataConversionError;
use qubit_datatype::DataConversionErrorKind;
use qubit_datatype::DataConversionTarget;
use qubit_datatype::DataConverter;
use qubit_datatype::DataType;
use qubit_datatype::DataTypeOf;
use qubit_datatype::InvalidValueReason;
#[cfg(feature = "json")]
use serde_json::Value;

use super::internal::Port;
use super::internal::Text;

#[derive(Debug)]
struct CrossNumeric;

impl DataTypeOf for CrossNumeric {
    const DATA_TYPE: DataType = DataType::String;
}

impl DataConversionTarget for CrossNumeric {
    fn convert(input: qubit_datatype::AdmittedConversion<'_, '_, '_>) -> Result<Self, DataConversionError> {
        input.convert::<u32>().map(|_| Self)
    }
}

#[cfg(feature = "json")]
#[derive(Debug)]
struct CrossJson;

#[cfg(feature = "json")]
impl DataTypeOf for CrossJson {
    const DATA_TYPE: DataType = DataType::String;
}

#[cfg(feature = "json")]
impl DataConversionTarget for CrossJson {
    fn convert(input: qubit_datatype::AdmittedConversion<'_, '_, '_>) -> Result<Self, DataConversionError> {
        input.convert::<Value>().map(|_| Self)
    }
}

#[cfg(feature = "json")]
#[derive(Debug)]
struct SameJson(Value);

#[cfg(feature = "json")]
impl DataTypeOf for SameJson {
    const DATA_TYPE: DataType = DataType::Json;
}

#[cfg(feature = "json")]
impl DataConversionTarget for SameJson {
    fn convert(input: qubit_datatype::AdmittedConversion<'_, '_, '_>) -> Result<Self, DataConversionError> {
        input.convert::<Value>().map(Self)
    }
}

#[test]
fn test_delegate_rejects_different_data_type() {
    for consume in [false, true] {
        let policy = ConversionPolicy::strict();
        let limits = ConversionLimits::default();
        let mut session = ConversionSession::new(&policy, &limits);
        let source = DataConverter::from("42");
        let result = if consume {
            source.into_target_in::<CrossNumeric>(&mut session)
        } else {
            source.to_in::<CrossNumeric>(&mut session)
        };
        let error = result.expect_err("different descriptors must be rejected");
        assert_eq!(error.kind(), DataConversionErrorKind::Unsupported);
        assert_eq!(error.from_type(), Some(DataType::String));
        assert_eq!(error.to_type(), DataType::String);
        assert_eq!(session.items_used(), 1);
        assert_eq!(session.input_bytes_used(), 2);
        assert_eq!(session.output_bytes_used(), 0);
    }
}

#[cfg(feature = "json")]
#[test]
fn test_delegate_rejects_cross_json_without_structural_work() {
    for consume in [false, true] {
        let value = Value::Bool(true);
        let policy = ConversionPolicy::strict();
        let limits = ConversionLimits::builder()
            .operation_limits(ConversionOperationLimits::builder().max_structured_nodes(0).build())
            .build();
        let mut session = ConversionSession::new(&policy, &limits);
        let source = DataConverter::from(&value);
        let result = if consume {
            source.into_target_in::<CrossJson>(&mut session)
        } else {
            source.to_in::<CrossJson>(&mut session)
        };
        let error = result.expect_err("cross-descriptor JSON delegation must not run");
        assert_eq!(error.kind(), DataConversionErrorKind::Unsupported);
        assert_eq!(error.from_type(), Some(DataType::Json));
        assert_eq!(error.to_type(), DataType::String);
        assert_eq!(session.items_used(), 1);
        assert_eq!(session.structured_nodes_used(), 0);
        assert_eq!(session.output_bytes_used(), 0);
    }
}

#[cfg(feature = "json")]
#[test]
fn test_same_descriptor_json_delegation_obeys_exact_budget() {
    for consume in [false, true] {
        for maximum in [0, 1] {
            let value = Value::Bool(true);
            let policy = ConversionPolicy::strict();
            let limits = ConversionLimits::builder()
                .operation_limits(
                    ConversionOperationLimits::builder()
                        .max_structured_nodes(maximum)
                        .build(),
                )
                .build();
            let mut session = ConversionSession::new(&policy, &limits);
            let source = DataConverter::from(&value);
            let result = if consume {
                source.into_target_in::<SameJson>(&mut session)
            } else {
                source.to_in::<SameJson>(&mut session)
            };
            if maximum == 0 {
                let error = result.expect_err("zero nodes must reject JSON admission");
                assert_eq!(error.kind(), DataConversionErrorKind::LimitExceeded);
                assert_eq!(error.resource(), Some(ConversionResource::StructuredNodes));
                assert_eq!(error.to_type(), DataType::Json);
            } else {
                let SameJson(result) = result.expect("one bool node fits exactly");
                assert_eq!(result, value);
                assert_eq!(session.structured_nodes_used(), 1);
            }
            assert_eq!(session.items_used(), 1);
        }
    }
}

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
fn test_data_conversion_target_newtype_preserves_bound_error_context() {
    let error = DataConverter::from("0")
        .to::<Port>()
        .expect_err("zero is not a usable network port");

    assert_eq!(error.kind(), DataConversionErrorKind::InvalidValue);
    assert_eq!(error.from_type(), Some(DataType::String));
    assert_eq!(error.to_type(), DataType::UInt16);
    assert_eq!(error.reason(), Some(&InvalidValueReason::OutOfRange));
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
    assert_eq!(error.resource(), Some(ConversionResource::Items));
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
