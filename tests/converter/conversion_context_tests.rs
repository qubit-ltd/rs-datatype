// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests for the conversion-target execution context.

use qubit_datatype::AdmittedConversion;
use qubit_datatype::ConversionLimits;
use qubit_datatype::ConversionOperationLimits;
use qubit_datatype::ConversionPolicy;
use qubit_datatype::ConversionResource;
use qubit_datatype::ConversionSession;
use qubit_datatype::ConversionStringWriter;
use qubit_datatype::DataConversionError;
use qubit_datatype::DataConversionTarget;
use qubit_datatype::DataConverter;
use qubit_datatype::DataType;
use qubit_datatype::DataTypeOf;
use qubit_datatype::InvalidValueReason;
#[cfg(feature = "json")]
use qubit_datatype::StructuredConversionLimits;
#[cfg(feature = "json")]
use serde_json::Value;

/// A target that delegates conversion without re-admitting its source.
#[derive(Debug, PartialEq, Eq)]
struct Port(u16);

impl DataTypeOf for Port {
    const DATA_TYPE: DataType = DataType::UInt16;
}

impl DataConversionTarget for Port {
    /// Delegates to the built-in integer conversion through the bound context.
    fn convert(input: AdmittedConversion<'_, '_, '_>) -> Result<Self, DataConversionError> {
        input.convert::<u16>().map(Self)
    }
}

/// A target that abandons a partially rendered output value.
#[derive(Debug)]
struct RejectedString;

impl DataTypeOf for RejectedString {
    const DATA_TYPE: DataType = DataType::String;
}

/// A target that uses every JSON context operation during one conversion.
#[cfg(feature = "json")]
#[derive(Debug, PartialEq)]
struct JsonRoundTrip(Vec<u8>);

#[cfg(feature = "json")]
impl DataTypeOf for JsonRoundTrip {
    const DATA_TYPE: DataType = DataType::Json;
}

#[cfg(feature = "json")]
impl DataConversionTarget for JsonRoundTrip {
    /// Decodes, accounts, and encodes JSON through the bound context.
    fn convert(mut input: AdmittedConversion<'_, '_, '_>) -> Result<Self, DataConversionError> {
        let text = match input.source() {
            DataConverter::String(value) => value.to_string(),
            _ => return Err(input.unsupported()),
        };
        let decoded: Value = input.decode_json(text.as_bytes())?;
        input.account_json_value(&decoded)?;
        input.encode_json(&decoded).map(Self)
    }
}

impl DataConversionTarget for RejectedString {
    /// Renders output and then returns a conversion error to exercise rollback.
    fn convert(mut input: AdmittedConversion<'_, '_, '_>) -> Result<Self, DataConversionError> {
        let from = input.from_type();
        let to = input.to_type();
        input.write_string(|writer: &mut ConversionStringWriter<'_>| {
            writer.write_str("discard")?;
            Err(DataConversionError::invalid(from, to, InvalidValueReason::OutOfRange))
        })?;
        Ok(Self)
    }
}

/// Verifies nested delegation preserves the outer admission exactly once.
#[test]
fn test_admitted_conversion_does_not_double_charge_top_level_item() {
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

    assert_eq!(DataConverter::from("42").to_in::<Port>(&mut session), Ok(Port(42)));
    assert_eq!(session.items_used(), 1);
    assert_eq!(session.input_bytes_used(), 2);
    assert_eq!(
        DataConverter::from("1")
            .to_in::<Port>(&mut session)
            .expect_err("the second top-level item must exhaust the budget")
            .resource(),
        Some(ConversionResource::Items),
    );
}

/// Verifies context-managed output is transactional when the target rejects it.
#[test]
fn test_context_write_string_rolls_back_rendered_output() {
    let policy = ConversionPolicy::strict();
    let limits = ConversionLimits::builder()
        .operation_limits(ConversionOperationLimits::builder().max_output_bytes(16).build())
        .build();
    let mut session = ConversionSession::new(&policy, &limits);

    let error = DataConverter::from("source")
        .to_in::<RejectedString>(&mut session)
        .expect_err("the target must reject its own rendered output");
    assert_eq!(error.from_type(), Some(DataType::String));
    assert_eq!(error.to_type(), DataType::String);
    assert_eq!(session.output_bytes_used(), 0);
}

/// Verifies JSON context operations share type binding and cumulative budgets.
#[cfg(feature = "json")]
#[test]
fn test_context_json_operations_bind_budget_errors_to_the_active_conversion() {
    let policy = ConversionPolicy::strict();
    let limits = ConversionLimits::builder()
        .operation_limits(ConversionOperationLimits::builder().max_output_bytes(8).build())
        .build();
    let mut session = ConversionSession::new(&policy, &limits);

    let error = DataConverter::from(r#"{"ok":true}"#)
        .to_in::<JsonRoundTrip>(&mut session)
        .expect_err("encoded JSON should exceed the bound output budget");
    assert_eq!(error.resource(), Some(ConversionResource::OutputBytes));
    assert_eq!(error.from_type(), Some(DataType::String));
    assert_eq!(error.to_type(), DataType::Json);
    assert_eq!(session.output_bytes_used(), 0);
    assert!(session.structured_nodes_used() > 0);
}

/// Public JSON decoding observes the text point limit before parsing.
#[cfg(feature = "json")]
#[test]
fn test_context_json_decode_checks_text_limit_before_parsing() {
    let policy = ConversionPolicy::strict();
    for text in ["null", "\"µ\"", "xxxx", " null "] {
        let limits = ConversionLimits::builder()
            .structured_limits(StructuredConversionLimits::builder().max_text_bytes(3).build())
            .build();
        let mut session = ConversionSession::new(&policy, &limits);
        let error = DataConverter::from(text)
            .to_in::<JsonRoundTrip>(&mut session)
            .expect_err("JSON bytes exceed the text point limit");
        assert_eq!(error.resource(), Some(ConversionResource::StructuredTextBytes));
        assert_eq!(error.from_type(), Some(DataType::String));
        assert_eq!(error.to_type(), DataType::Json);
        assert_eq!(error.configured_limit(), Some(3));
        assert_eq!(error.observed_lower_bound(), Some(text.len() as u64));
        assert_eq!(session.items_used(), 1);
        assert_eq!(session.input_bytes_used(), text.len() as u64);
        assert_eq!(session.structured_nodes_used(), 0);
        assert_eq!(session.structured_payload_bytes_used(), 0);
        assert_eq!(session.output_bytes_used(), 0);
        let builtin = DataConverter::from(text)
            .to_with::<Value>(&policy, &limits)
            .expect_err("the built-in path also checks these bytes first");
        assert_eq!(builtin.resource(), Some(ConversionResource::StructuredTextBytes));
    }
}

/// Text exactly at the configured byte maximum remains accepted.
#[cfg(feature = "json")]
#[test]
fn test_context_json_decode_accepts_text_at_byte_limit() {
    let policy = ConversionPolicy::strict();
    let limits = ConversionLimits::builder()
        .structured_limits(StructuredConversionLimits::builder().max_text_bytes(4).build())
        .build();
    for text in ["null", "\"µ\""] {
        assert!(DataConverter::from(text).to_with::<Value>(&policy, &limits).is_ok());
        assert!(
            DataConverter::from(text)
                .to_with::<JsonRoundTrip>(&policy, &limits)
                .is_ok()
        );
    }
}

/// A text-limit rejection retains admission but leaves structural work
/// untouched.
#[cfg(feature = "json")]
#[test]
fn test_context_json_decode_text_rejection_allows_later_conversion() {
    let policy = ConversionPolicy::strict();
    let limits = ConversionLimits::builder()
        .structured_limits(StructuredConversionLimits::builder().max_text_bytes(3).build())
        .operation_limits(
            ConversionOperationLimits::builder()
                .max_items(2)
                .max_input_bytes(5)
                .build(),
        )
        .build();
    let mut session = ConversionSession::new(&policy, &limits);
    let error = DataConverter::from("null")
        .to_in::<JsonRoundTrip>(&mut session)
        .expect_err("four bytes exceed the point limit");
    assert_eq!(error.resource(), Some(ConversionResource::StructuredTextBytes));
    assert_eq!(session.structured_nodes_used(), 0);
    assert_eq!(session.structured_payload_bytes_used(), 0);
    assert_eq!(session.output_bytes_used(), 0);
    assert!(DataConverter::from("0").to_in::<JsonRoundTrip>(&mut session).is_ok());
    assert_eq!(session.items_used(), 2);
    assert_eq!(session.input_bytes_used(), 5);
    assert_eq!(session.output_bytes_used(), 1);
}
