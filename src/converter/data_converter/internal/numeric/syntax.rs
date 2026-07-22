// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Textual numeric syntax parsing.

#[cfg(feature = "big-decimal")]
use std::str::FromStr;

#[cfg(feature = "big-decimal")]
use bigdecimal::BigDecimal;
#[cfg(any(feature = "big-integer", feature = "big-decimal"))]
use num_bigint::BigInt;

use super::super::super::string_source::normalize;
#[cfg(feature = "big-decimal")]
use super::parsed_number::ParsedNumber;
use crate::converter::{
    ConversionLimit,
    DataConversionError,
    DataConversionOptions,
    FractionalToIntegerPolicy,
    InvalidValueReason,
};
use crate::datatype::DataType;

/// Normalizes numeric text and enforces its configured byte limit.
///
/// # Parameters
///
/// * `value` - Raw textual source.
/// * `options` - String and numeric conversion options.
/// * `to` - Numeric target type retained in conversion errors.
///
/// # Returns
///
/// The normalized source text within the configured byte limit.
///
/// # Errors
///
/// Returns a normalization error or a numeric-text resource limit error.
pub(super) fn normalize_numeric_text<'a>(
    value: &'a str,
    options: &DataConversionOptions,
    to: DataType,
) -> Result<&'a str, DataConversionError> {
    let value = normalize(value, options, to)?;
    check_numeric_text_limit(value, options, to)?;
    Ok(value)
}

/// Enforces the configured normalized numeric text byte limit.
pub(in crate::converter::data_converter) fn check_numeric_text_limit(
    value: &str,
    options: &DataConversionOptions,
    to: DataType,
) -> Result<(), DataConversionError> {
    let maximum = options.numeric().limits().max_text_bytes();
    if value.len() > maximum {
        Err(DataConversionError::limit_exceeded(
            DataType::String,
            to,
            ConversionLimit::NumericTextBytes { maximum },
        ))
    } else {
        Ok(())
    }
}

/// Parses a normalized number without selecting a target primitive first.
///
/// # Parameters
///
/// * `value` - Normalized numeric text to parse.
/// * `to` - Target context used to select syntax diagnostics.
///
/// # Returns
///
/// An exact integer or decimal representation, or a non-finite marker.
///
/// # Errors
///
/// Returns an invalid-value [`DataConversionError`] for invalid numeric text.
#[cfg(feature = "big-decimal")]
pub(super) fn parse_number(
    value: &str,
    to: DataType,
) -> Result<ParsedNumber, DataConversionError> {
    if let Some(value) = parse_non_finite_number(value) {
        return Ok(value);
    }
    if is_integer_syntax(value) {
        let negative = value.starts_with('-');
        let digits = value.strip_prefix(['+', '-']).unwrap_or(value);
        let integer = digits.bytes().fold(BigInt::from(0u8), |value, digit| {
            value * 10u8 + (digit - b'0')
        });
        return Ok(ParsedNumber::Integer(if negative {
            -integer
        } else {
            integer
        }));
    }
    match BigDecimal::from_str(value) {
        Ok(value) => Ok(ParsedNumber::Decimal(value)),
        Err(_) => Err(DataConversionError::invalid(
            DataType::String,
            to,
            InvalidValueReason::InvalidSyntax {
                expected: numeric_syntax(to),
            },
        )),
    }
}

/// Parses a case-insensitive non-finite numeric spelling.
///
/// # Parameters
///
/// * `value` - Normalized numeric text to inspect.
///
/// # Returns
///
/// The corresponding non-finite marker, or `None` when `value` is not a
/// supported non-finite spelling.
#[cfg(feature = "big-decimal")]
fn parse_non_finite_number(value: &str) -> Option<ParsedNumber> {
    if value.eq_ignore_ascii_case("nan") {
        Some(ParsedNumber::NaN)
    } else if value.eq_ignore_ascii_case("inf")
        || value.eq_ignore_ascii_case("+inf")
        || value.eq_ignore_ascii_case("infinity")
        || value.eq_ignore_ascii_case("+infinity")
    {
        Some(ParsedNumber::PositiveInfinity)
    } else if value.eq_ignore_ascii_case("-inf")
        || value.eq_ignore_ascii_case("-infinity")
    {
        Some(ParsedNumber::NegativeInfinity)
    } else {
        None
    }
}

/// Tests whether text uses the canonical integer grammar.
///
/// # Parameters
///
/// * `value` - Text to validate without normalization.
///
/// # Returns
///
/// `true` for one or more ASCII digits with an optional leading sign.
pub(in crate::converter::data_converter) fn is_integer_syntax(
    value: &str,
) -> bool {
    let digits = value.strip_prefix(['+', '-']).unwrap_or(value);
    !digits.is_empty() && digits.bytes().all(|byte| byte.is_ascii_digit())
}

/// Returns the stable syntax label for a numeric target.
///
/// # Parameters
///
/// * `to` - Numeric target whose grammar should be described.
///
/// # Returns
///
/// A static, source-value-free syntax label for invalid-syntax errors.
fn numeric_syntax(to: DataType) -> &'static str {
    match to {
        DataType::BigDecimal => "decimal number with optional exponent",
        DataType::Float32 | DataType::Float64 => "number",
        _ => "integer",
    }
}

/// Creates a contextual invalid numeric syntax error.
///
/// # Parameters
///
/// * `to` - Target type and grammar used by the diagnostic.
///
/// # Returns
///
/// A source-value-free invalid-syntax conversion error.
#[inline(always)]
pub(super) fn invalid_numeric_syntax(to: DataType) -> DataConversionError {
    DataConversionError::invalid(
        DataType::String,
        to,
        InvalidValueReason::InvalidSyntax {
            expected: numeric_syntax(to),
        },
    )
}

/// Reports whether text explicitly names a non-finite value.
///
/// # Parameters
///
/// * `value` - Text to compare with supported non-finite names.
///
/// # Returns
///
/// `true` for a case-insensitive NaN or infinity spelling.
fn is_explicit_non_finite(value: &str) -> bool {
    [
        "nan",
        "inf",
        "+inf",
        "-inf",
        "infinity",
        "+infinity",
        "-infinity",
    ]
    .iter()
    .any(|candidate| value.eq_ignore_ascii_case(candidate))
}

/// Splits an optional leading sign from numeric text.
///
/// # Parameters
///
/// * `value` - Numeric text whose optional sign should be removed.
///
/// # Returns
///
/// Whether the sign is negative and the unsigned remainder.
fn split_sign(value: &str) -> (bool, &str) {
    match value.as_bytes().first() {
        Some(b'-') => (true, &value[1..]),
        Some(b'+') => (false, &value[1..]),
        _ => (false, value),
    }
}

/// Splits and validates an optional decimal exponent.
///
/// Exponents outside `i64` saturate toward the matching bound so later range
/// checks can reject them without allocating an exponent-sized buffer.
///
/// # Parameters
///
/// * `value` - Unsigned numeric text.
/// * `to` - Target type used in syntax errors.
///
/// # Returns
///
/// The mantissa text and signed base-ten exponent.
///
/// # Errors
///
/// Returns an invalid-syntax error when the exponent is missing or malformed.
fn split_exponent(
    value: &str,
    to: DataType,
) -> Result<(&str, i64), DataConversionError> {
    let Some(index) =
        value.bytes().position(|byte| matches!(byte, b'e' | b'E'))
    else {
        return Ok((value, 0));
    };
    let mantissa = &value[..index];
    let exponent_text = &value[index + 1..];
    let digits = exponent_text
        .strip_prefix(['+', '-'])
        .unwrap_or(exponent_text);
    if digits.is_empty() || digits.bytes().any(|byte| !byte.is_ascii_digit()) {
        return Err(invalid_numeric_syntax(to));
    }
    let exponent = match exponent_text.parse::<i64>() {
        Ok(exponent) => exponent,
        Err(_) if exponent_text.starts_with('-') => i64::MIN,
        Err(_) => i64::MAX,
    };
    Ok((mantissa, exponent))
}

/// Validates a decimal mantissa and counts its digits.
///
/// # Parameters
///
/// * `mantissa` - Mantissa without a sign or exponent.
/// * `to` - Target type used in syntax errors.
///
/// # Returns
///
/// The total number of digits and the number following the decimal point.
///
/// # Errors
///
/// Returns an invalid-syntax error for an empty mantissa, repeated decimal
/// points, non-digit characters, or a mantissa without digits.
fn analyze_mantissa(
    mantissa: &str,
    to: DataType,
) -> Result<(usize, usize), DataConversionError> {
    if mantissa.is_empty() {
        return Err(invalid_numeric_syntax(to));
    }
    let mut digit_count = 0usize;
    let mut decimal_seen = false;
    let mut fractional_digits = 0usize;
    for byte in mantissa.bytes() {
        match byte {
            b'0'..=b'9' => {
                digit_count += 1;
                if decimal_seen {
                    fractional_digits += 1;
                }
            }
            b'.' if !decimal_seen => decimal_seen = true,
            _ => return Err(invalid_numeric_syntax(to)),
        }
    }
    if digit_count == 0 {
        return Err(invalid_numeric_syntax(to));
    }
    Ok((digit_count, fractional_digits))
}

/// Reports whether the fractional portion contains a non-zero digit.
///
/// # Parameters
///
/// * `mantissa` - Validated mantissa text.
/// * `integer_digit_count` - Number of flattened digits before the effective
///   decimal point.
/// * `decimal_position` - Effective decimal point position after applying the
///   exponent.
///
/// # Returns
///
/// `true` when truncating the effective fractional portion loses information.
fn fractional_part_is_non_zero(
    mantissa: &str,
    integer_digit_count: usize,
    decimal_position: i128,
) -> bool {
    for (digit_index, byte) in
        mantissa.bytes().filter(u8::is_ascii_digit).enumerate()
    {
        if (decimal_position <= 0 || digit_index >= integer_digit_count)
            && byte != b'0'
        {
            return true;
        }
    }
    false
}

/// Accumulates the effective integer digits into a `u128` magnitude.
///
/// # Parameters
///
/// * `mantissa` - Validated mantissa text.
/// * `integer_digit_count` - Number of flattened digits to consume.
/// * `to` - Target type used in range errors.
///
/// # Returns
///
/// The parsed unsigned magnitude.
///
/// # Errors
///
/// Returns an out-of-range error when the magnitude exceeds `u128`.
fn parse_integer_magnitude(
    mantissa: &str,
    integer_digit_count: usize,
    to: DataType,
) -> Result<u128, DataConversionError> {
    let mut magnitude = 0u128;
    for byte in mantissa
        .bytes()
        .filter(u8::is_ascii_digit)
        .take(integer_digit_count)
    {
        magnitude = magnitude
            .checked_mul(10)
            .and_then(|value| value.checked_add(u128::from(byte - b'0')))
            .ok_or(DataConversionError::invalid(
                DataType::String,
                to,
                InvalidValueReason::OutOfRange,
            ))?;
    }
    Ok(magnitude)
}

/// Parses decimal text into a platform-independent integer intermediate.
///
/// Reject mode rejects a non-zero fractional part. Truncate mode discards it
/// toward zero. Exponents are processed structurally, so extreme values are
/// rejected without allocating an exponent-sized buffer.
///
/// # Parameters
///
/// * `value` - Normalized decimal text to parse.
/// * `policy` - Fractional-value conversion policy.
/// * `to` - Target type retained in conversion errors.
///
/// # Returns
///
/// The sign and `u128` magnitude of the parsed integer.
///
/// # Errors
///
/// Returns a syntax, non-finite, precision-loss, or range error associated
/// with `to` when the input cannot be converted under `policy`.
pub(super) fn parse_text_integer(
    value: &str,
    policy: FractionalToIntegerPolicy,
    to: DataType,
) -> Result<(bool, u128), DataConversionError> {
    if is_explicit_non_finite(value) {
        return Err(DataConversionError::invalid(
            DataType::String,
            to,
            InvalidValueReason::NonFinite,
        ));
    }

    let (negative, unsigned) = split_sign(value);
    if unsigned.is_empty() {
        return Err(invalid_numeric_syntax(to));
    }
    let (mantissa, exponent) = split_exponent(unsigned, to)?;
    let (digit_count, fractional_digits) = analyze_mantissa(mantissa, to)?;
    let decimal_position =
        (digit_count - fractional_digits) as i128 + i128::from(exponent);
    let integer_digit_count = if decimal_position <= 0 {
        0
    } else {
        usize::try_from(decimal_position)
            .unwrap_or(usize::MAX)
            .min(digit_count)
    };
    if policy == FractionalToIntegerPolicy::Reject
        && fractional_part_is_non_zero(
            mantissa,
            integer_digit_count,
            decimal_position,
        )
    {
        return Err(DataConversionError::invalid(
            DataType::String,
            to,
            InvalidValueReason::PrecisionLoss,
        ));
    }

    let mut magnitude =
        parse_integer_magnitude(mantissa, integer_digit_count, to)?;
    if decimal_position > digit_count as i128 && magnitude != 0 {
        let zero_count = u32::try_from(decimal_position - digit_count as i128)
            .map_err(|_| {
                DataConversionError::invalid(
                    DataType::String,
                    to,
                    InvalidValueReason::OutOfRange,
                )
            })?;
        let multiplier = 10u128.checked_pow(zero_count).ok_or(
            DataConversionError::invalid(
                DataType::String,
                to,
                InvalidValueReason::OutOfRange,
            ),
        )?;
        magnitude = magnitude.checked_mul(multiplier).ok_or(
            DataConversionError::invalid(
                DataType::String,
                to,
                InvalidValueReason::OutOfRange,
            ),
        )?;
    }

    Ok((negative && magnitude != 0, magnitude))
}

/// Parses decimal text into an arbitrary-precision integer.
///
/// Reject mode rejects a non-zero fractional part. Truncate mode discards it
/// toward zero. Exponents are processed structurally and the resulting
/// allocation is capped to prevent compact inputs from causing unbounded
/// memory growth.
///
/// # Parameters
///
/// * `value` - Normalized decimal text to parse.
/// * `policy` - Fractional-value conversion policy.
/// * `maximum_digits` - Maximum decimal digits in the materialized result.
/// * `to` - Target type retained in conversion errors.
///
/// # Returns
///
/// The parsed arbitrary-precision integer.
///
/// # Errors
///
/// Returns a syntax, non-finite, precision-loss, resource-limit, or range error
/// associated with `to` when the input cannot be converted under `policy` and
/// `maximum_digits`.
#[cfg(any(feature = "big-integer", feature = "big-decimal"))]
pub(super) fn parse_text_bigint(
    value: &str,
    policy: FractionalToIntegerPolicy,
    maximum_digits: usize,
    to: DataType,
) -> Result<BigInt, DataConversionError> {
    if is_explicit_non_finite(value) {
        return Err(DataConversionError::invalid(
            DataType::String,
            to,
            InvalidValueReason::NonFinite,
        ));
    }
    let (negative, unsigned) = split_sign(value);
    if unsigned.is_empty() {
        return Err(invalid_numeric_syntax(to));
    }
    let (mantissa, exponent) = split_exponent(unsigned, to)?;
    let (digit_count, fractional_digits) = analyze_mantissa(mantissa, to)?;
    let decimal_position =
        (digit_count - fractional_digits) as i128 + i128::from(exponent);
    let integer_digit_count = if decimal_position <= 0 {
        0
    } else {
        usize::try_from(decimal_position)
            .unwrap_or(usize::MAX)
            .min(digit_count)
    };
    if policy == FractionalToIntegerPolicy::Reject
        && fractional_part_is_non_zero(
            mantissa,
            integer_digit_count,
            decimal_position,
        )
    {
        return Err(DataConversionError::invalid(
            DataType::String,
            to,
            InvalidValueReason::PrecisionLoss,
        ));
    }
    let appended_zeros =
        decimal_position.saturating_sub(digit_count as i128).max(0);
    let first_non_zero = mantissa
        .bytes()
        .filter(u8::is_ascii_digit)
        .take(integer_digit_count)
        .position(|digit| digit != b'0');
    let result_digits = first_non_zero.map_or(0_i128, |index| {
        (integer_digit_count - index) as i128 + appended_zeros
    });
    if result_digits > maximum_digits as i128 {
        return Err(DataConversionError::limit_exceeded(
            DataType::String,
            to,
            ConversionLimit::BigIntegerDigits {
                maximum: maximum_digits,
            },
        ));
    }
    let mut integer = mantissa
        .bytes()
        .filter(u8::is_ascii_digit)
        .take(integer_digit_count)
        .fold(BigInt::from(0_u8), |value, digit| {
            value * 10_u8 + (digit - b'0')
        });
    if appended_zeros > 0 && integer != BigInt::from(0_u8) {
        let exponent = u32::try_from(appended_zeros).map_err(|_| {
            DataConversionError::invalid(
                DataType::String,
                to,
                InvalidValueReason::OutOfRange,
            )
        })?;
        integer *= BigInt::from(10_u8).pow(exponent);
    }
    Ok(if negative { -integer } else { integer })
}
