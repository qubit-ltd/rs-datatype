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
use qubit_datatype::ScalarItem;
use qubit_datatype::StringConversionPolicy;
#[cfg(feature = "json")]
use serde::Deserialize;
#[cfg(feature = "json")]
use serde::Deserializer;
#[cfg(feature = "json")]
use serde::Serialize;
#[cfg(feature = "json")]
use serde::de::DeserializeSeed;

#[test]
fn test_one_session_accumulates_multiple_conversions() {
    let policy = ConversionPolicy::default();
    let limits = ConversionLimits::default()
        .with_operation_limits(ConversionOperationLimits::default().with_max_input_bytes(3));
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
    let limits = ConversionLimits::default()
        .with_operation_limits(ConversionOperationLimits::default().with_max_input_bytes(2));

    for _ in 0..2 {
        DataConverter::from("12")
            .to_with::<u16>(&policy, &limits)
            .expect("each public conversion should create a fresh session");
    }
}

#[test]
fn test_trimmed_text_is_charged_by_raw_input_length() {
    let policy = ConversionPolicy::default()
        .with_string_policy(StringConversionPolicy::default().with_trim(true));
    let limits = ConversionLimits::default()
        .with_operation_limits(ConversionOperationLimits::default().with_max_input_bytes(2));

    let error = DataConverter::from(" 12 ")
        .to_with::<u16>(&policy, &limits)
        .expect_err("raw whitespace must count toward the input budget");

    assert_eq!(
        error.budget_error().map(|facts| *facts.resource()),
        Some(ConversionResource::InputBytes),
    );
}

#[test]
fn test_failed_conversion_keeps_attempted_item_and_input_accounting() {
    let policy = ConversionPolicy::default();
    let limits = ConversionLimits::default().with_operation_limits(
        ConversionOperationLimits::default()
            .with_max_items(2)
            .with_max_input_bytes(5),
    );
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

    assert_eq!(
        error.budget_error().map(|facts| *facts.resource()),
        Some(ConversionResource::Items),
    );
}

/// Test native-length session methods share the configured output budget.
#[test]
fn test_native_length_output_accounting_uses_shared_budget() {
    let policy = ConversionPolicy::default();
    let limits = ConversionLimits::default()
        .with_operation_limits(ConversionOperationLimits::default().with_max_output_bytes(2));
    let mut session = ConversionSession::new(&policy, &limits);

    session
        .check_output_bytes_usize("ok".len())
        .expect("exact native length should fit");
    session
        .consume_output_bytes_usize("ok".len())
        .expect("exact native length should consume budget");
    let error = session
        .check_output_bytes_usize("x".len())
        .expect_err("remaining budget should be exhausted");

    assert_eq!(
        error.budget_error().map(|facts| *facts.resource()),
        Some(ConversionResource::OutputBytes)
    );
}

/// Verifies scalar source and item admission produce one-use conversion tokens.
#[test]
fn test_scalar_admission_charges_source_and_items_once() {
    let policy = ConversionPolicy::default();
    let limits = ConversionLimits::default().with_operation_limits(
        ConversionOperationLimits::default()
            .with_max_items(1)
            .with_max_input_bytes(3),
    );
    let mut session = ConversionSession::new(&policy, &limits);

    session
        .admit_scalar_source_bytes_usize(3)
        .expect("the complete scalar source should fit");
    let admitted = session
        .admit_scalar_item(ScalarItem {
            source_index: 2,
            value: "abc",
        })
        .expect("the first scalar item should fit");
    assert_eq!(admitted.source_index(), 2);

    let error = session
        .admit_scalar_item(ScalarItem {
            source_index: 3,
            value: "d",
        })
        .expect_err("the second scalar item should exceed the item budget");
    assert_eq!(error.resource(), &ConversionResource::Items);
}

/// Exercises the direct session accounting adapters and observability methods.
#[test]
fn test_direct_session_accounting_adapters() {
    use std::fmt::Write;

    let policy = ConversionPolicy::default();
    let limits = ConversionLimits::default();
    let mut session = ConversionSession::new(&policy, &limits);

    assert_eq!(session.policy(), &policy);
    assert_eq!(session.limits(), &limits);
    session
        .consume_input_bytes(1)
        .expect("input bytes should fit");
    session
        .check_collection_source_bytes(1)
        .expect("source bytes should fit");
    session
        .check_output_bytes(1)
        .expect("output bytes should fit");
    session
        .consume_output_bytes(1)
        .expect("output bytes should fit");
    let rendered = session
        .try_write_string(|writer| write!(writer.as_fmt(), "ok"))
        .expect("rendered output should fit");
    assert_eq!(rendered, "ok");

    assert_eq!(session.items_limit(), limits.operation().max_items());
    assert_eq!(
        session.input_bytes_limit(),
        limits.operation().max_input_bytes()
    );
    assert_eq!(
        session.output_bytes_limit(),
        limits.operation().max_output_bytes()
    );
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

#[cfg(feature = "json")]
#[derive(Debug, Deserialize, Serialize, PartialEq)]
struct SessionJsonValue {
    name: String,
}

#[cfg(feature = "json")]
struct SessionJsonSeed;

#[cfg(feature = "json")]
impl<'de> DeserializeSeed<'de> for SessionJsonSeed {
    type Value = SessionJsonValue;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        SessionJsonValue::deserialize(deserializer)
    }
}

/// Exercises JSON-specific session entry points and structured adapters.
#[cfg(feature = "json")]
#[test]
fn test_json_session_accounting_adapters() {
    let policy = ConversionPolicy::default();
    let limits = ConversionLimits::default();
    let mut session = ConversionSession::new(&policy, &limits);
    let input = br#"{"name":"value"}"#;

    let decoded: SessionJsonValue = session
        .decode_json(input)
        .expect("JSON should decode within the shared budget");
    assert_eq!(decoded.name, "value");
    let seeded: SessionJsonValue = session
        .decode_json_seed(SessionJsonSeed, input)
        .expect("seeded JSON should decode within the shared budget");
    assert_eq!(seeded.name, "value");
    let encoded = session
        .encode_json(&decoded)
        .expect("JSON should encode within the shared budget");
    assert_eq!(encoded, input);
    assert!(session.structured_nodes_used() > 0);
    assert!(session.structured_payload_bytes_used() > 0);
}
