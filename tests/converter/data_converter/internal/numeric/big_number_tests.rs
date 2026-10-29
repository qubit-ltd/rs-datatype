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
#[cfg(feature = "big-integer")]
use qubit_datatype::ConversionLimits;
#[cfg(feature = "big-integer")]
use qubit_datatype::ConversionPolicy;
#[cfg(feature = "big-integer")]
use qubit_datatype::ConversionResource;
#[cfg(feature = "big-integer")]
use qubit_datatype::DataConversionErrorKind;
#[cfg(any(feature = "big-integer", feature = "big-decimal"))]
use qubit_datatype::DataConverter;
#[cfg(feature = "big-integer")]
use qubit_datatype::InvalidValueReason;
#[cfg(feature = "big-integer")]
use qubit_datatype::NumericConversionLimits;

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
fn limits_with_big_integer_digit_limit(maximum: u64) -> ConversionLimits {
    ConversionLimits::builder()
        .numeric_limits(
            NumericConversionLimits::builder()
                .max_big_integer_digits(maximum)
                .build(),
        )
        .build()
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
    assert_eq!(DataConverter::from("123.50").to::<BigDecimal>(), Ok(expected),);
}

/// Verifies fixed-width overflow is reported before decimal power construction.
#[cfg(feature = "big-decimal")]
#[test]
fn test_decimal_fixed_integer_overflow_preserves_error_priority() {
    use qubit_datatype::InvalidValueReason;

    let coefficient = num_bigint::BigInt::from(10_u8).pow(10_000);
    let policy = qubit_datatype::ConversionPolicy::strict();
    let limits = qubit_datatype::ConversionLimits::default();
    for coefficient in [coefficient.clone(), -coefficient] {
        let value = BigDecimal::new(coefficient.clone(), 5_000);
        let error = DataConverter::from(&value)
            .to_with::<i32>(&policy, &limits)
            .expect_err("a huge integral result must be out of range");
        assert_eq!(error.reason(), Some(&InvalidValueReason::OutOfRange));

        let fractional = BigDecimal::new(coefficient + 1, 5_000);
        let error = DataConverter::from(&fractional)
            .to_with::<i32>(&policy, &limits)
            .expect_err("strict conversion must prioritize fractional precision loss");
        assert_eq!(error.reason(), Some(&InvalidValueReason::PrecisionLoss));
    }
}

/// Verifies positive-scale decimal conversion preserves fixed-width boundaries.
#[cfg(feature = "big-decimal")]
#[test]
fn test_decimal_positive_scale_preserves_fixed_integer_boundaries() {
    use qubit_datatype::InvalidValueReason;

    let minimum = BigDecimal::new(num_bigint::BigInt::from(i128::MIN) * 10_u8, 1);
    assert_eq!(DataConverter::from(&minimum).to::<i128>(), Ok(i128::MIN));

    let maximum = BigDecimal::new(num_bigint::BigInt::from(u128::MAX) * 10_u8, 1);
    assert_eq!(DataConverter::from(&maximum).to::<u128>(), Ok(u128::MAX));

    let overflow = BigDecimal::new((num_bigint::BigInt::from(u128::MAX) + 1_u8) * 10_u8, 1);
    let error = DataConverter::from(&overflow)
        .to::<u128>()
        .expect_err("the next value above u128::MAX must be out of range");
    assert_eq!(error.reason(), Some(&InvalidValueReason::OutOfRange));

    let negative_fraction = BigDecimal::from_str("-12.9").expect("valid test decimal");
    assert_eq!(
        DataConverter::from(&negative_fraction).to_with::<i32>(
            &qubit_datatype::ConversionPolicy::lossy(),
            &qubit_datatype::ConversionLimits::default(),
        ),
        Ok(-12),
    );

    let less_than_one = BigDecimal::from_str("0.1").expect("valid test decimal");
    let error = DataConverter::from(&less_than_one)
        .to::<i32>()
        .expect_err("strict conversion must reject a nonzero fraction below one");
    assert_eq!(error.reason(), Some(&InvalidValueReason::PrecisionLoss));
}

/// Verifies primitive sources honor the BigInteger result digit limit.
#[cfg(feature = "big-integer")]
#[test]
fn test_primitive_to_bigint_enforces_result_digit_limit() {
    let policy = ConversionPolicy::strict();
    let limits = limits_with_big_integer_digit_limit(3);
    let error = DataConverter::from(1_234_u16)
        .to_with::<BigInt>(&policy, &limits)
        .expect_err("a four-digit result must exceed a three-digit limit");

    assert_eq!(error.kind(), DataConversionErrorKind::LimitExceeded);
    assert_eq!(error.resource(), Some(ConversionResource::BigIntegerDigits));
    assert_eq!(error.configured_limit(), Some(3));
    assert_eq!(error.observed_lower_bound(), Some(4));
}

/// Verifies borrowed BigInteger sources are checked before cloning.
#[cfg(feature = "big-integer")]
#[test]
fn test_bigint_to_bigint_enforces_result_digit_limit() {
    let source = BigInt::from(1_234_u16);
    let policy = ConversionPolicy::strict();
    let limits = limits_with_big_integer_digit_limit(3);
    let error = DataConverter::from(&source)
        .to_with::<BigInt>(&policy, &limits)
        .expect_err("a four-digit result must exceed a three-digit limit");

    assert_eq!(error.kind(), DataConversionErrorKind::LimitExceeded);
    assert_eq!(error.resource(), Some(ConversionResource::BigIntegerDigits));
    assert_eq!(error.configured_limit(), Some(3));
    assert_eq!(error.observed_lower_bound(), Some(4));
}

/// Verifies clearly oversized positive borrowed integers retain a conservative
/// observation.
#[cfg(feature = "big-integer")]
#[test]
fn test_large_borrowed_bigint_reports_lower_bound() {
    let source = BigInt::from(1_u8) << 99_usize;
    let limits = limits_with_big_integer_digit_limit(3);
    let error = DataConverter::from(&source)
        .to_with::<BigInt>(&ConversionPolicy::strict(), &limits)
        .expect_err("a 30-digit integer must exceed a three-digit limit");

    assert_eq!(error.kind(), DataConversionErrorKind::LimitExceeded);
    assert_eq!(error.resource(), Some(ConversionResource::BigIntegerDigits));
    assert_eq!(error.configured_limit(), Some(3));
    assert_eq!(error.observed_lower_bound(), Some(4));
    assert_eq!(error.observation_is_exact(), Some(false));
}

/// Verifies clearly oversized owned negative integers retain a conservative
/// observation.
#[cfg(feature = "big-integer")]
#[test]
fn test_large_owned_negative_bigint_reports_lower_bound() {
    let source = -(BigInt::from(1_u8) << 99_usize);
    let limits = limits_with_big_integer_digit_limit(3);
    let error = DataConverter::from(source)
        .to_with::<BigInt>(&ConversionPolicy::strict(), &limits)
        .expect_err("a negative 30-digit integer must exceed a three-digit limit");

    assert_eq!(error.kind(), DataConversionErrorKind::LimitExceeded);
    assert_eq!(error.resource(), Some(ConversionResource::BigIntegerDigits));
    assert_eq!(error.configured_limit(), Some(3));
    assert_eq!(error.observed_lower_bound(), Some(4));
    assert_eq!(error.observation_is_exact(), Some(false));
}

/// Verifies zero and the decimal limit boundary are handled correctly.
#[cfg(feature = "big-integer")]
#[test]
fn test_bigint_zero_and_decimal_limit_boundary() {
    let limits = limits_with_big_integer_digit_limit(3);
    assert_eq!(
        DataConverter::from(BigInt::from(0_u8)).to_with::<BigInt>(&ConversionPolicy::strict(), &limits),
        Ok(BigInt::from(0_u8)),
    );
    assert_eq!(
        DataConverter::from(BigInt::from(999_u16)).to_with::<BigInt>(&ConversionPolicy::strict(), &limits),
        Ok(BigInt::from(999_u16)),
    );

    let error = DataConverter::from(BigInt::from(1_000_u16))
        .to_with::<BigInt>(&ConversionPolicy::strict(), &limits)
        .expect_err("1000 must exceed a three-digit limit");
    assert_eq!(error.kind(), DataConversionErrorKind::LimitExceeded);
    assert_eq!(error.resource(), Some(ConversionResource::BigIntegerDigits));
    assert_eq!(error.configured_limit(), Some(3));
    assert_eq!(error.observed_lower_bound(), Some(4));
    assert_eq!(error.observation_is_exact(), Some(true));
}

/// Verifies positive-scale decimals enforce the truncated result digit limit.
#[cfg(all(feature = "big-integer", feature = "big-decimal"))]
#[test]
fn test_positive_scale_decimal_to_bigint_enforces_result_digit_limit() {
    let source = BigDecimal::new(BigInt::from(12_345_u32), 1);
    let policy = ConversionPolicy::lossy();
    let limits = limits_with_big_integer_digit_limit(3);
    let error = DataConverter::from(&source)
        .to_with::<BigInt>(&policy, &limits)
        .expect_err("a four-digit quotient must exceed a three-digit limit");

    assert_eq!(error.kind(), DataConversionErrorKind::LimitExceeded);
    assert_eq!(error.resource(), Some(ConversionResource::BigIntegerDigits));
    assert_eq!(error.configured_limit(), Some(3));
    assert_eq!(error.observed_lower_bound(), Some(4));
}

/// Verifies an exponent outside `BigInt::pow`'s range is rejected safely.
#[cfg(feature = "big-integer")]
#[test]
fn test_text_to_bigint_rejects_unrepresentable_exponent() {
    let policy = ConversionPolicy::strict();
    let limits = limits_with_big_integer_digit_limit(u64::MAX);
    let error = DataConverter::from("1e4294967296")
        .to_with::<BigInt>(&policy, &limits)
        .expect_err("an exponent above u32::MAX must be rejected");

    assert_eq!(error.kind(), DataConversionErrorKind::InvalidValue);
    assert_eq!(error.reason(), Some(&InvalidValueReason::OutOfRange));
}

/// Verifies digit limits reject oversized results before exponent conversion.
#[cfg(feature = "big-integer")]
#[test]
fn test_text_to_bigint_digit_limit_precedes_exponent_range_check() {
    let policy = ConversionPolicy::strict();
    let maximum = u64::from(u32::MAX);
    let limits = limits_with_big_integer_digit_limit(maximum);
    let error = DataConverter::from("1e4294967296")
        .to_with::<BigInt>(&policy, &limits)
        .expect_err("the digit budget must reject the result first");

    assert_eq!(error.kind(), DataConversionErrorKind::LimitExceeded);
    assert_eq!(error.resource(), Some(ConversionResource::BigIntegerDigits));
    assert_eq!(error.configured_limit(), Some(maximum));
    assert_eq!(error.observed_lower_bound(), Some(4_294_967_297));
}
