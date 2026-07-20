// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Primitive and text to arbitrary-precision number tests.

#[cfg(feature = "big-decimal")]
use std::str::FromStr;

#[cfg(feature = "big-decimal")]
use bigdecimal::BigDecimal;
#[cfg(feature = "big-integer")]
use num_bigint::BigInt;
#[cfg(any(feature = "big-integer", feature = "big-decimal"))]
use qubit_datatype::DataConverter;
#[cfg(feature = "big-integer")]
use qubit_datatype::{
    ConversionLimit, DataConversionErrorKind, DataConversionOptions, NumericConversionLimits,
    NumericConversionOptions,
};

/// Creates strict options with the supplied BigInteger digit limit.
///
/// # Parameters
///
/// * `maximum` - Maximum significant decimal digits for a materialized
///   BigInteger result.
///
/// # Returns
///
/// Strict conversion options carrying the requested digit limit.
#[cfg(feature = "big-integer")]
fn options_with_big_integer_digit_limit(maximum: usize) -> DataConversionOptions {
    DataConversionOptions::strict().with_numeric_options(
        NumericConversionOptions::strict()
            .with_limits(NumericConversionLimits::default().with_max_big_integer_digits(maximum)),
    )
}

/// Verifies primitive integer conversion preserves the complete value.
#[cfg(feature = "big-integer")]
#[test]
fn test_integer_to_bigint_preserves_value() {
    assert_eq!(
        DataConverter::from(i128::MIN).to::<BigInt>(),
        Ok(BigInt::from(i128::MIN)),
    );
}

/// Verifies decimal text conversion preserves the represented value.
#[cfg(feature = "big-decimal")]
#[test]
fn test_text_to_big_decimal_preserves_value() {
    let expected = BigDecimal::from_str("123.50").expect("valid test decimal");
    assert_eq!(
        DataConverter::from("123.50").to::<BigDecimal>(),
        Ok(expected),
    );
}

/// Verifies primitive sources honor the BigInteger result digit limit.
#[cfg(feature = "big-integer")]
#[test]
fn test_primitive_to_bigint_enforces_result_digit_limit() {
    let options = options_with_big_integer_digit_limit(3);
    let error = DataConverter::from(1_234_u16)
        .to_with::<BigInt>(&options)
        .expect_err("a four-digit result must exceed a three-digit limit");

    assert_eq!(error.kind(), DataConversionErrorKind::LimitExceeded);
    assert_eq!(
        error.limit(),
        Some(&ConversionLimit::BigIntegerDigits { maximum: 3 }),
    );
}

/// Verifies borrowed BigInteger sources are checked before cloning.
#[cfg(feature = "big-integer")]
#[test]
fn test_bigint_to_bigint_enforces_result_digit_limit() {
    let source = BigInt::from(1_234_u16);
    let options = options_with_big_integer_digit_limit(3);
    let error = DataConverter::from(&source)
        .to_with::<BigInt>(&options)
        .expect_err("a four-digit result must exceed a three-digit limit");

    assert_eq!(error.kind(), DataConversionErrorKind::LimitExceeded);
    assert_eq!(
        error.limit(),
        Some(&ConversionLimit::BigIntegerDigits { maximum: 3 }),
    );
}

/// Verifies positive-scale decimals enforce the truncated result digit limit.
#[cfg(all(feature = "big-integer", feature = "big-decimal"))]
#[test]
fn test_positive_scale_decimal_to_bigint_enforces_result_digit_limit() {
    let source = BigDecimal::new(BigInt::from(12_345_u32), 1);
    let options = DataConversionOptions::lossy().with_numeric_options(
        NumericConversionOptions::lossy()
            .with_limits(NumericConversionLimits::default().with_max_big_integer_digits(3)),
    );
    let error = DataConverter::from(&source)
        .to_with::<BigInt>(&options)
        .expect_err("a four-digit quotient must exceed a three-digit limit");

    assert_eq!(error.kind(), DataConversionErrorKind::LimitExceeded);
    assert_eq!(
        error.limit(),
        Some(&ConversionLimit::BigIntegerDigits { maximum: 3 }),
    );
}

/// Verifies an exponent outside `BigInt::pow`'s range is rejected safely.
#[cfg(feature = "big-integer")]
#[test]
fn test_text_to_bigint_rejects_unrepresentable_exponent() {
    let options = options_with_big_integer_digit_limit(usize::MAX);
    let error = DataConverter::from("1e4294967296")
        .to_with::<BigInt>(&options)
        .expect_err("an exponent above u32::MAX must be rejected");

    assert_eq!(error.kind(), DataConversionErrorKind::InvalidValue);
}
