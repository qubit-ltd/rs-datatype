// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Regression tests for cumulative String output budgets.

#[cfg(feature = "big-decimal")]
use bigdecimal::BigDecimal;
#[cfg(any(feature = "big-integer", feature = "big-decimal"))]
use num_bigint::BigInt;
use qubit_datatype::ConversionLimits;
use qubit_datatype::ConversionOperationLimits;
use qubit_datatype::ConversionPolicy;
use qubit_datatype::ConversionResource;
use qubit_datatype::ConversionSession;
use qubit_datatype::DataConverter;
#[cfg(any(feature = "big-integer", feature = "big-decimal"))]
use qubit_datatype::DataType;

fn limits_with_output_limit(maximum: usize) -> ConversionLimits {
    ConversionLimits::builder()
        .operation_limits(
            ConversionOperationLimits::builder()
                .max_output_bytes(u64::try_from(maximum).unwrap())
                .build(),
        )
        .build()
}

#[test]
fn test_string_output_budget_counts_borrowed_and_owned_payloads_equally() {
    let policy = ConversionPolicy::default();
    let limits = limits_with_output_limit(3);
    let mut borrowed_session = ConversionSession::new(&policy, &limits);
    let mut owned_session = ConversionSession::new(&policy, &limits);

    assert_eq!(
        DataConverter::from(42_i32).to_in::<String>(&mut borrowed_session),
        Ok("42".to_owned()),
    );
    assert_eq!(
        DataConverter::from(42_i32).into_target_in::<String>(&mut owned_session),
        Ok("42".to_owned()),
    );
    assert_eq!(borrowed_session.output_bytes_used(), 2);
    assert_eq!(borrowed_session.output_bytes_remaining(), 1);
    assert_eq!(owned_session.output_bytes_used(), 2);
    assert_eq!(owned_session.output_bytes_remaining(), 1);
}

#[test]
fn test_string_output_budget_failure_is_transactional() {
    let policy = ConversionPolicy::default();
    let limits = limits_with_output_limit(3);
    let mut session = ConversionSession::new(&policy, &limits);

    assert_eq!(
        DataConverter::from(12_i32).to_in::<String>(&mut session),
        Ok("12".to_owned()),
    );
    let first = DataConverter::from(345_i32)
        .to_in::<String>(&mut session)
        .expect_err("three more bytes should exceed the remaining budget");
    let second = DataConverter::from(7_i32)
        .to_in::<String>(&mut session)
        .expect("the rejected render must not consume output bytes");

    assert_eq!(second, "7");
    assert_eq!(first.resource(), Some(ConversionResource::OutputBytes));
    assert_eq!(first.configured_limit(), Some(3));
    assert_eq!(first.remaining(), Some(1));
    assert_eq!(first.requested(), Some(3));
}

#[test]
fn test_string_output_budget_counts_utf8_bytes() {
    let policy = ConversionPolicy::default();
    for source_owned in [false, true] {
        for consume_converter in [false, true] {
            for maximum in [1_usize, 2] {
                let limits = limits_with_output_limit(maximum);
                let mut session = ConversionSession::new(&policy, &limits);
                let result = if source_owned {
                    let converter = DataConverter::from(String::from("é"));
                    if consume_converter {
                        converter.into_target_in::<String>(&mut session)
                    } else {
                        converter.to_in::<String>(&mut session)
                    }
                } else {
                    let converter = DataConverter::from("é");
                    if consume_converter {
                        converter.into_target_in::<String>(&mut session)
                    } else {
                        converter.to_in::<String>(&mut session)
                    }
                };
                if maximum == 1 {
                    let error = result.expect_err("two UTF-8 bytes exceed one byte");
                    assert_eq!(error.resource(), Some(ConversionResource::OutputBytes));
                    assert_eq!(session.output_bytes_used(), 0);
                    assert_eq!(session.output_bytes_remaining(), 1);
                    assert_eq!(DataConverter::from("a").to_in::<String>(&mut session).unwrap(), "a",);
                    assert_eq!(session.output_bytes_used(), 1);
                    assert_eq!(session.output_bytes_remaining(), 0);
                } else {
                    assert_eq!(result.unwrap(), "é");
                    assert_eq!(session.output_bytes_used(), 2);
                    assert_eq!(session.output_bytes_remaining(), 0);
                    let error = DataConverter::from("a").to_in::<String>(&mut session).unwrap_err();
                    assert_eq!(error.resource(), Some(ConversionResource::OutputBytes));
                    assert_eq!(session.output_bytes_used(), 2);
                    assert_eq!(session.output_bytes_remaining(), 0);
                }
            }
        }
    }
}

#[test]
fn test_owned_string_output_reuses_unchanged_storage() {
    let source = String::from("unchanged");
    let pointer = source.as_ptr();
    let output = DataConverter::from(source)
        .into_target::<String>()
        .expect("owned String conversion should succeed");
    assert_eq!(output.as_ptr(), pointer);
}

#[cfg(any(feature = "big-integer", feature = "big-decimal"))]
#[test]
fn test_big_number_output_preflight_preserves_session() {
    let policy = ConversionPolicy::default();
    let limits = limits_with_output_limit(8);
    let mut session = ConversionSession::new(&policy, &limits);
    assert_eq!(
        DataConverter::from(123_i32).to_in::<String>(&mut session),
        Ok("123".to_owned())
    );

    #[cfg(feature = "big-integer")]
    let error = DataConverter::from(BigInt::from(1_u8) << 50_000_usize)
        .to_in::<String>(&mut session)
        .expect_err("large integer cannot fit the remaining five bytes");
    #[cfg(all(not(feature = "big-integer"), feature = "big-decimal"))]
    let error = DataConverter::from(BigDecimal::new(BigInt::from(1_u8) << 50_000_usize, 0))
        .to_in::<String>(&mut session)
        .expect_err("large decimal cannot fit the remaining five bytes");

    assert_eq!(error.resource(), Some(ConversionResource::OutputBytes));
    assert_eq!(error.used(), Some(3));
    assert_eq!(error.remaining(), Some(5));
    assert!(error.requested().expect("preflight reports its lower bound") > 5);
    #[cfg(feature = "big-integer")]
    assert_eq!(error.from_type(), Some(DataType::BigInteger));
    #[cfg(all(not(feature = "big-integer"), feature = "big-decimal"))]
    assert_eq!(error.from_type(), Some(DataType::BigDecimal));
    assert_eq!(error.to_type(), DataType::String);
    assert_eq!(session.output_bytes_used(), 3);
    assert_eq!(session.items_used(), 2);
}

#[cfg(feature = "big-integer")]
#[test]
fn test_big_integer_output_preflight_preserves_display_text_and_boundary() {
    let policy = ConversionPolicy::default();
    for value in [BigInt::from(0_i8), 1.into(), (-1).into(), 99.into(), (-99).into()] {
        let expected = value.to_string();
        let exact_limits = limits_with_output_limit(expected.len());
        assert_eq!(
            DataConverter::from(&value).to_with::<String>(&policy, &exact_limits),
            Ok(expected.clone()),
        );
        if !expected.is_empty() {
            let short_limits = limits_with_output_limit(expected.len() - 1);
            let mut session = ConversionSession::new(&policy, &short_limits);
            let error = DataConverter::from(&value)
                .to_in::<String>(&mut session)
                .expect_err("one byte below the display length must fail");
            assert_eq!(error.resource(), Some(ConversionResource::OutputBytes));
            assert_eq!(session.output_bytes_used(), 0);
        }
    }
}

#[cfg(feature = "big-decimal")]
#[test]
fn test_big_decimal_output_preflight_preserves_display_text_and_boundary() {
    let policy = ConversionPolicy::default();
    let coefficients = [BigInt::from(0_i8), 1.into(), (-1).into(), BigInt::from(10_u8).pow(40)];
    for coefficient in coefficients {
        for scale in [0_i64, 40, 10_000, -10_000] {
            let value = BigDecimal::new(coefficient.clone(), scale);
            let expected = value.to_string();
            let exact_limits = limits_with_output_limit(expected.len());
            assert_eq!(
                DataConverter::from(&value).to_with::<String>(&policy, &exact_limits),
                Ok(expected.clone()),
            );
            let short_limits = limits_with_output_limit(expected.len() - 1);
            let mut session = ConversionSession::new(&policy, &short_limits);
            let error = DataConverter::from(&value)
                .to_in::<String>(&mut session)
                .expect_err("one byte below the display length must fail");
            assert_eq!(error.resource(), Some(ConversionResource::OutputBytes));
            assert_eq!(session.output_bytes_used(), 0);
        }
    }
}
