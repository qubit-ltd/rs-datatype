// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Numeric syntax validation tests.

#[cfg(feature = "big-decimal")]
use std::str::FromStr;

#[cfg(feature = "big-decimal")]
use bigdecimal::BigDecimal;
#[cfg(feature = "big-integer")]
use num_bigint::BigInt;
use qubit_datatype::ConversionLimits;
use qubit_datatype::ConversionPolicy;
use qubit_datatype::ConversionResource;
use qubit_datatype::DataConverter;
use qubit_datatype::DataType;
use qubit_datatype::InvalidValueReason;
use qubit_datatype::NumericConversionLimits;

/// Verifies malformed signs are classified as invalid numeric syntax.
#[test]
fn test_numeric_text_rejects_duplicate_signs() {
    assert!(matches!(
        DataConverter::from("--1").to::<f64>(),
        Err(conversion_error) if matches!(conversion_error.reason(), Some(InvalidValueReason::InvalidSyntax { .. })
    )));
}

/// Verifies numeric text at the configured byte boundary preserves its
/// conversion-limit fact.
#[test]
fn test_numeric_text_limit_preserves_conversion_limit_fact() {
    let options = ConversionPolicy::default();
    let limits = ConversionLimits::builder()
        .numeric_limits(NumericConversionLimits::builder().max_text_bytes(3).build())
        .build();

    assert_eq!(DataConverter::from("123").to_with::<u32>(&options, &limits), Ok(123),);
    let error = DataConverter::from("1234")
        .to_with::<u32>(&options, &limits)
        .expect_err("four bytes must exceed the configured limit");
    assert_eq!(error.from_type(), Some(DataType::String));
    assert_eq!(error.to_type(), DataType::UInt32);
    assert_eq!(error.resource(), Some(ConversionResource::NumericTextBytes));
    assert_eq!(error.configured_limit(), Some(3));
    assert_eq!(error.observed_lower_bound(), Some(4));
}

/// Verifies decimal parsing at digit boundaries across both entry paths.
#[cfg(feature = "big-integer")]
#[test]
fn test_big_integer_text_decimal_boundaries_preserve_exact_values() {
    for digits_count in [8_usize, 9, 10, 17, 18, 19, 4096] {
        let digits = "9".repeat(digits_count);
        let expected = BigInt::from(10_u8).pow(u32::try_from(digits_count).unwrap()) - BigInt::from(1_u8);
        assert_eq!(
            DataConverter::from(digits.as_str()).to::<BigInt>(),
            Ok(expected.clone())
        );
        assert_eq!(
            DataConverter::from(format!("{digits}.000")).to::<BigInt>(),
            Ok(expected)
        );
    }
}

/// Verifies integer-number text preserves exact values at digit boundaries.
#[cfg(feature = "big-decimal")]
#[test]
fn test_big_decimal_integer_text_decimal_boundaries_preserve_exact_values() {
    for digits_count in [8_usize, 9, 10, 17, 18, 19, 4096] {
        let digits = "9".repeat(digits_count);
        let expected = BigDecimal::from_str(&digits).expect("the expected integer should parse");
        assert_eq!(DataConverter::from(digits.as_str()).to::<BigDecimal>(), Ok(expected),);
    }
}
