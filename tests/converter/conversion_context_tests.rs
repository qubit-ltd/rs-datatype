// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests for the conversion-target execution context.

use qubit_datatype::ConversionContext;
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
use serde_json::Value;

/// A target that delegates conversion without re-admitting its source.
#[derive(Debug, PartialEq, Eq)]
struct Port(u16);

impl DataTypeOf for Port {
    const DATA_TYPE: DataType = DataType::UInt16;
}

impl DataConversionTarget for Port {
    /// Delegates to the built-in integer conversion through the bound context.
    fn convert_from(
        source: &DataConverter<'_>,
        context: &mut ConversionContext<'_, '_>,
    ) -> Result<Self, DataConversionError> {
        // SAFETY: `source` is the value admitted by `DataConverter::to_in`.
        unsafe { context.delegate::<u16>(source) }.map(Self)
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
    fn convert_from(
        source: &DataConverter<'_>,
        context: &mut ConversionContext<'_, '_>,
    ) -> Result<Self, DataConversionError> {
        let DataConverter::String(value) = source else {
            return Err(DataConversionError::unsupported(source.data_type(), DataType::Json));
        };
        let decoded: Value = context.decode_json(value.as_bytes())?;
        context.account_json_value(&decoded)?;
        context.encode_json(&decoded).map(Self)
    }
}

impl DataConversionTarget for RejectedString {
    /// Renders output and then returns a conversion error to exercise rollback.
    fn convert_from(
        _source: &DataConverter<'_>,
        context: &mut ConversionContext<'_, '_>,
    ) -> Result<Self, DataConversionError> {
        context.write_string(|writer: &mut ConversionStringWriter<'_>| {
            writer.write_str("discard")?;
            Err(DataConversionError::invalid(
                DataType::String,
                DataType::String,
                InvalidValueReason::OutOfRange,
            ))
        })?;
        Ok(Self)
    }
}

/// Verifies nested delegation preserves the outer admission exactly once.
#[test]
fn test_context_delegate_does_not_double_charge_top_level_item() {
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
