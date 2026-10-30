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
use qubit_datatype::DataConversionErrorKind;
use qubit_datatype::DataConverter;
use qubit_datatype::DataType;
use qubit_datatype::InvalidValueReason;
use qubit_datatype::StringConversionPolicy;

#[derive(Clone, Debug)]
enum SessionOperation {
    BorrowedText(String),
    OwnedText(String),
    ValidIntegerText(&'static str, u16),
    InvalidIntegerText,
    IntegerOutput(i32),
}

#[derive(Clone, Copy, Debug)]
struct SessionBudgetModel {
    items_limit: u64,
    input_limit: u64,
    output_limit: u64,
    items_used: u64,
    input_used: u64,
    output_used: u64,
}

#[derive(Debug)]
enum ExpectedOutcome {
    String(String),
    Integer(u16),
    Error {
        kind: DataConversionErrorKind,
        resource: Option<ConversionResource>,
        reason: Option<InvalidValueReason>,
    },
}

impl SessionBudgetModel {
    fn new(items_limit: u64, input_limit: u64, output_limit: u64) -> Self {
        Self {
            items_limit,
            input_limit,
            output_limit,
            items_used: 0,
            input_used: 0,
            output_used: 0,
        }
    }

    fn apply(&mut self, operation: &SessionOperation) -> ExpectedOutcome {
        let (input_bytes, output, invalid_syntax) = match operation {
            SessionOperation::BorrowedText(text) | SessionOperation::OwnedText(text) => {
                (text.len() as u64, Some(text.clone()), false)
            }
            SessionOperation::ValidIntegerText(text, _) => (text.len() as u64, None, false),
            SessionOperation::InvalidIntegerText => (3, None, true),
            SessionOperation::IntegerOutput(value) => (0, Some(value.to_string()), false),
        };
        let output_bytes = output.as_ref().map_or(0, |text| text.len() as u64);

        if self.items_used == self.items_limit {
            return Self::limit_error(ConversionResource::Items);
        }
        self.items_used += 1;
        if input_bytes > self.input_limit - self.input_used {
            return Self::limit_error(ConversionResource::InputBytes);
        }
        self.input_used += input_bytes;
        if invalid_syntax {
            return ExpectedOutcome::Error {
                kind: DataConversionErrorKind::InvalidValue,
                resource: None,
                reason: Some(InvalidValueReason::InvalidSyntax { expected: "integer" }),
            };
        }
        if output_bytes > self.output_limit - self.output_used {
            return Self::limit_error(ConversionResource::OutputBytes);
        }
        self.output_used += output_bytes;
        match operation {
            SessionOperation::ValidIntegerText(_, value) => ExpectedOutcome::Integer(*value),
            SessionOperation::InvalidIntegerText => unreachable!(),
            _ => ExpectedOutcome::String(output.expect("string-producing operation")),
        }
    }

    fn limit_error(resource: ConversionResource) -> ExpectedOutcome {
        ExpectedOutcome::Error {
            kind: DataConversionErrorKind::LimitExceeded,
            resource: Some(resource),
            reason: None,
        }
    }
}

fn run_session_sequence(items_limit: u64, input_limit: u64, output_limit: u64, operations: &[SessionOperation]) {
    let policy = ConversionPolicy::default();
    let limits = ConversionLimits::builder()
        .operation_limits(
            ConversionOperationLimits::builder()
                .max_items(items_limit)
                .max_input_bytes(input_limit)
                .max_output_bytes(output_limit)
                .build(),
        )
        .build();
    let mut session = ConversionSession::new(&policy, &limits);
    let mut model = SessionBudgetModel::new(items_limit, input_limit, output_limit);

    for (index, operation) in operations.iter().enumerate() {
        let context = format!(
            "operation #{index} {operation:?}, limits items={items_limit}, input={input_limit}, output={output_limit}"
        );
        let expected = model.apply(operation);
        let actual = match operation {
            SessionOperation::BorrowedText(text) => DataConverter::from(text.as_str())
                .to_in::<String>(&mut session)
                .map(ActualValue::String),
            SessionOperation::OwnedText(text) => DataConverter::from(text.clone())
                .into_target_in::<String>(&mut session)
                .map(ActualValue::String),
            SessionOperation::ValidIntegerText(text, _) => DataConverter::from(*text)
                .to_in::<u16>(&mut session)
                .map(ActualValue::Integer),
            SessionOperation::InvalidIntegerText => DataConverter::from("bad")
                .to_in::<u16>(&mut session)
                .map(ActualValue::Integer),
            SessionOperation::IntegerOutput(value) => DataConverter::from(*value)
                .to_in::<String>(&mut session)
                .map(ActualValue::String),
        };
        match (expected, actual) {
            (ExpectedOutcome::String(expected), Ok(ActualValue::String(actual))) => {
                assert_eq!(actual, expected, "{context}")
            }
            (ExpectedOutcome::Integer(expected), Ok(ActualValue::Integer(actual))) => {
                assert_eq!(actual, expected, "{context}")
            }
            (ExpectedOutcome::Error { kind, resource, reason }, Err(error)) => {
                assert_eq!(error.kind(), kind, "{context}");
                assert_eq!(error.resource(), resource, "{context}");
                assert_eq!(error.reason(), reason.as_ref(), "{context}");
                let (from, to) = match operation {
                    SessionOperation::BorrowedText(_) | SessionOperation::OwnedText(_) => {
                        (DataType::String, DataType::String)
                    }
                    SessionOperation::ValidIntegerText(_, _) | SessionOperation::InvalidIntegerText => {
                        (DataType::String, DataType::UInt16)
                    }
                    SessionOperation::IntegerOutput(_) => (DataType::Int32, DataType::String),
                };
                assert_eq!(error.from_type(), Some(from), "{context}");
                assert_eq!(error.to_type(), to, "{context}");
            }
            (expected, actual) => panic!("{context}: expected {expected:?}, got {actual:?}"),
        }

        assert_eq!(session.items_used(), model.items_used, "{context}");
        assert_eq!(session.input_bytes_used(), model.input_used, "{context}");
        assert_eq!(session.output_bytes_used(), model.output_used, "{context}");
        assert_eq!(
            session.items_remaining(),
            model.items_limit - model.items_used,
            "{context}"
        );
        assert_eq!(
            session.input_bytes_remaining(),
            model.input_limit - model.input_used,
            "{context}"
        );
        assert_eq!(
            session.output_bytes_remaining(),
            model.output_limit - model.output_used,
            "{context}"
        );
    }
}

#[derive(Debug)]
enum ActualValue {
    String(String),
    Integer(u16),
}

#[test]
fn test_session_model_fixed_sequences() {
    run_session_sequence(
        8,
        0,
        3,
        &[
            SessionOperation::IntegerOutput(12),
            SessionOperation::IntegerOutput(345),
            SessionOperation::IntegerOutput(7),
        ],
    );
    run_session_sequence(
        4,
        1,
        8,
        &[
            SessionOperation::BorrowedText("12".into()),
            SessionOperation::IntegerOutput(7),
        ],
    );
    run_session_sequence(
        1,
        8,
        8,
        &[
            SessionOperation::BorrowedText("1".into()),
            SessionOperation::BorrowedText("2".into()),
        ],
    );
    run_session_sequence(
        4,
        8,
        8,
        &[
            SessionOperation::InvalidIntegerText,
            SessionOperation::ValidIntegerText("12", 12),
        ],
    );
    run_session_sequence(
        4,
        5,
        5,
        &[
            SessionOperation::BorrowedText("汉".into()),
            SessionOperation::BorrowedText("é".into()),
        ],
    );
    run_session_sequence(
        4,
        4,
        5,
        &[
            SessionOperation::BorrowedText("汉".into()),
            SessionOperation::BorrowedText("é".into()),
        ],
    );
    run_session_sequence(0, 0, 0, &[SessionOperation::IntegerOutput(1)]);
}

proptest::proptest! {
    #![proptest_config(proptest::test_runner::Config::with_cases(256))]

    #[test]
    fn test_shared_session_matches_budget_model(
        items_limit in 0u64..=64,
        input_limit in 0u64..=256,
        output_limit in 0u64..=256,
        operations in proptest::collection::vec(session_operation_strategy(), 1..=64),
    ) {
        run_session_sequence(items_limit, input_limit, output_limit, &operations);
    }
}

fn session_operation_strategy() -> impl proptest::strategy::Strategy<Value = SessionOperation> {
    use proptest::strategy::Just;
    use proptest::strategy::Strategy;
    let text = proptest::collection::vec(
        proptest::prop_oneof![Just("a"), Just("12"), Just("é"), Just("汉"), Just("🙂")],
        0..=8,
    )
    .prop_map(|parts| parts.concat());
    proptest::prop_oneof![
        3 => text.clone().prop_map(SessionOperation::BorrowedText),
        3 => text.prop_map(SessionOperation::OwnedText),
        2 => proptest::prop_oneof![
            Just(("0", 0)), Just(("1", 1)), Just(("12", 12)), Just(("65535", u16::MAX)),
        ].prop_map(|(text, value)| SessionOperation::ValidIntegerText(text, value)),
        2 => Just(SessionOperation::InvalidIntegerText),
        2 => (-1000i32..=1000).prop_map(SessionOperation::IntegerOutput),
    ]
}

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

    assert_eq!(session.items_used(), 0);
    assert_eq!(session.input_bytes_used(), 0);
    assert_eq!(session.output_bytes_used(), 0);
    assert_eq!(session.items_remaining(), session.items_limit());
    assert_eq!(session.input_bytes_remaining(), session.input_bytes_limit());
    assert_eq!(session.output_bytes_remaining(), session.output_bytes_limit());

    assert_eq!(session.policy(), &policy);
    assert_eq!(session.limits(), &limits);
    assert_eq!(
        DataConverter::from("ok")
            .to_in::<String>(&mut session)
            .expect("public conversion should fit"),
        "ok",
    );

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
    assert_eq!(session.items_used(), 1);
    assert_eq!(session.input_bytes_used(), 2);
    assert_eq!(session.output_bytes_used(), 2);
    assert_eq!(session.items_remaining(), session.items_limit() - 1);
    assert_eq!(session.input_bytes_remaining(), session.input_bytes_limit() - 2);
    assert_eq!(session.output_bytes_remaining(), session.output_bytes_limit() - 2);
    assert_eq!(session.structured_nodes_used(), 0);
    assert_eq!(session.structured_payload_bytes_used(), 0);
}

/// Verifies public remaining-capacity accessors at exact exhaustion.
#[test]
fn test_session_observability_at_exact_exhaustion() {
    let policy = ConversionPolicy::default();
    let limits = ConversionLimits::builder()
        .operation_limits(
            ConversionOperationLimits::builder()
                .max_items(1)
                .max_input_bytes(2)
                .max_output_bytes(2)
                .build(),
        )
        .build();
    let mut session = ConversionSession::new(&policy, &limits);

    assert_eq!(
        DataConverter::from("ok")
            .to_in::<String>(&mut session)
            .expect("the exact item and byte budgets should fit"),
        "ok",
    );
    assert_eq!(session.items_used(), 1);
    assert_eq!(session.input_bytes_used(), 2);
    assert_eq!(session.output_bytes_used(), 2);
    assert_eq!(session.items_remaining(), 0);
    assert_eq!(session.input_bytes_remaining(), 0);
    assert_eq!(session.output_bytes_remaining(), 0);
}
