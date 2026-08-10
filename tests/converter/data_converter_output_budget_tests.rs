// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Regression tests for cumulative String output budgets.

use std::time::Duration;

use qubit_budget::BudgetError;
use qubit_datatype::ConversionBudgetLimits;
use qubit_datatype::ConversionResource;
use qubit_datatype::DataConversionOptions;
use qubit_datatype::DataConverter;
use qubit_datatype::DurationConversionOptions;
use qubit_datatype::DurationUnit;

fn options_with_output_limit(maximum: usize) -> DataConversionOptions {
    DataConversionOptions::default().with_budget_limits(
        ConversionBudgetLimits::default().with_max_output_bytes(maximum),
    )
}

#[test]
fn test_string_output_budget_counts_borrowed_and_owned_payloads_equally() {
    let options = options_with_output_limit(3);
    let mut borrowed_session = options.session();
    let mut owned_session = options.session();

    assert_eq!(
        DataConverter::from(42_i32).to_in::<String>(&mut borrowed_session),
        Ok("42".to_owned())
    );
    assert_eq!(
        DataConverter::from(42_i32)
            .into_target_in::<String>(&mut owned_session),
        Ok("42".to_owned())
    );
    assert_eq!(borrowed_session.used_output_bytes(), 2);
    assert_eq!(owned_session.used_output_bytes(), 2);
}

#[test]
fn test_string_output_budget_rejects_without_consuming_failed_output() {
    let options = options_with_output_limit(2);
    let mut session = options.session();

    assert_eq!(
        DataConverter::from(123_i32).to_in::<String>(&mut session),
        Err(DataConverter::from(123_i32)
            .to_with::<String>(&options)
            .unwrap_err())
    );
    assert_eq!(session.used_output_bytes(), 0);
    assert!(matches!(
        DataConverter::from(123_i32).to_in::<String>(&mut session),
        Err(error) if error.budget_error()
            == Some(&BudgetError::Insufficient {
                resource: ConversionResource::OutputBytes,
                limit: 2,
                remaining: 2,
                requested: 3,
            })
    ));
}

#[test]
fn test_string_map_json_output_does_not_consume_string_budget() {
    #[cfg(feature = "json")]
    {
        let options = options_with_output_limit(0);
        let mut session = options.session();
        let source = std::collections::HashMap::from([(
            "key".to_owned(),
            "value".to_owned(),
        )]);

        let result = DataConverter::from(source)
            .to_in::<serde_json::Value>(&mut session);
        assert!(result.is_ok());
        assert_eq!(session.used_output_bytes(), 0);
    }
}

#[test]
fn test_builtin_string_outputs_use_exact_utf8_payload_lengths() {
    let options = options_with_output_limit(4);
    let mut session = options.session();
    assert_eq!(
        DataConverter::from(true).to_in::<String>(&mut session),
        Ok("true".to_owned())
    );
    assert_eq!(session.used_output_bytes(), 4);

    let duration_options = options_with_output_limit(2).with_duration_options(
        DurationConversionOptions::default()
            .with_output_unit(DurationUnit::Seconds)
            .with_append_unit_suffix(true),
    );
    let mut duration_session = duration_options.session();
    assert_eq!(
        DataConverter::from(Duration::from_secs(1))
            .to_in::<String>(&mut duration_session),
        Ok("1s".to_owned())
    );
    assert_eq!(duration_session.used_output_bytes(), 2);

    let insufficient_options = options_with_output_limit(3);
    let mut insufficient_session = insufficient_options.session();
    let error = DataConverter::from(true)
        .to_in::<String>(&mut insufficient_session)
        .expect_err("one byte below true's payload must fail");
    assert_eq!(
        error.budget_error(),
        Some(&BudgetError::Insufficient {
            resource: ConversionResource::OutputBytes,
            limit: 3,
            remaining: 3,
            requested: 4,
        })
    );
    assert_eq!(insufficient_session.used_output_bytes(), 0);
}

#[test]
fn test_string_map_output_budget_has_exact_and_one_below_boundaries() {
    #[cfg(feature = "json")]
    {
        let source = std::collections::HashMap::from([
            ("b".to_owned(), "2".to_owned()),
            ("a".to_owned(), "1".to_owned()),
        ]);
        let expected = r#"{"a":"1","b":"2"}"#;
        let exact = options_with_output_limit(expected.len());
        let mut exact_session = exact.session();
        assert_eq!(
            DataConverter::from(source.clone())
                .to_in::<String>(&mut exact_session),
            Ok(expected.to_owned())
        );
        assert_eq!(exact_session.used_output_bytes(), expected.len());

        let below = options_with_output_limit(expected.len() - 1);
        let mut below_session = below.session();
        let error = DataConverter::from(source)
            .to_in::<String>(&mut below_session)
            .expect_err("one byte below canonical JSON output must fail");
        assert_eq!(below_session.used_output_bytes(), 0);
        assert_eq!(
            error.budget_error(),
            Some(&BudgetError::Insufficient {
                resource: ConversionResource::OutputBytes,
                limit: expected.len() - 1,
                remaining: expected.len() - 1,
                requested: expected.len(),
            })
        );
    }
}

#[test]
fn test_string_output_budget_is_cumulative_and_transactional() {
    let options = options_with_output_limit(3);
    let mut session = options.session();
    assert_eq!(
        DataConverter::from(12_i32).to_in::<String>(&mut session),
        Ok("12".to_owned())
    );
    let error = DataConverter::from(345_i32)
        .to_in::<String>(&mut session)
        .expect_err("the second output exceeds the remaining budget");
    assert_eq!(session.used_output_bytes(), 2);
    assert_eq!(
        error.budget_error(),
        Some(&BudgetError::Insufficient {
            resource: ConversionResource::OutputBytes,
            limit: 3,
            remaining: 1,
            requested: 3,
        })
    );
}

#[test]
fn test_string_output_budget_counts_utf8_bytes() {
    let options = options_with_output_limit("é".len());
    let mut session = options.session();
    assert_eq!(
        DataConverter::from("é").to_in::<String>(&mut session),
        Ok("é".to_owned())
    );
    assert_eq!(session.used_output_bytes(), 2);
}

#[test]
fn test_owned_string_output_reuses_unchanged_storage() {
    let source = String::from("unchanged");
    let pointer = source.as_ptr();
    let options = DataConversionOptions::default();
    let mut session = options.session();
    let output = DataConverter::from(source)
        .into_target_in::<String>(&mut session)
        .expect("owned String conversion should succeed");
    assert_eq!(output.as_ptr(), pointer);
}
