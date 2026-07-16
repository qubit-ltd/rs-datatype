// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Textual numeric syntax parsing.

#[cfg(feature = "big-number")]
use std::str::FromStr;

#[cfg(feature = "big-number")]
use bigdecimal::BigDecimal;
#[cfg(feature = "big-number")]
use num_bigint::BigInt;

#[cfg(feature = "big-number")]
use super::parsed_number::ParsedNumber;
use crate::converter::{
    DataConversionError,
    InvalidValueReason,
    NumericConversionPolicy,
};
use crate::datatype::DataType;

/// Parses a normalized number without selecting a target primitive first.
///
/// `value` must already have passed string normalization; `to` supplies target
/// context and selects the expected syntax label on failure. Returns an exact
/// integer/decimal representation or a non-finite marker. Invalid decimal text
/// returns [`DataConversionError::InvalidValue`].
#[cfg(feature = "big-number")]
pub(super) fn parse_number(
    value: &str,
    to: DataType,
) -> Result<ParsedNumber, DataConversionError> {
    let lower = value.to_ascii_lowercase();
    match lower.as_str() {
        "nan" => return Ok(ParsedNumber::NaN),
        "inf" | "+inf" | "infinity" | "+infinity" => {
            return Ok(ParsedNumber::PositiveInfinity);
        }
        "-inf" | "-infinity" => return Ok(ParsedNumber::NegativeInfinity),
        _ => {}
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
        Err(_) => Err(DataConversionError::InvalidValue {
            from: DataType::String,
            to,
            reason: InvalidValueReason::InvalidSyntax {
                expected: numeric_syntax(to),
            },
        }),
    }
}

/// Tests whether text uses the canonical integer grammar.
///
/// Returns `true` for one or more ASCII digits with an optional leading sign,
/// and `false` for empty, whitespace-containing, or otherwise invalid text.
pub(in crate::converter::data_converter) fn is_integer_syntax(
    value: &str,
) -> bool {
    let digits = value.strip_prefix(['+', '-']).unwrap_or(value);
    !digits.is_empty() && digits.bytes().all(|byte| byte.is_ascii_digit())
}

/// Returns the stable syntax label for a numeric target.
///
/// `to` selects the label embedded in invalid-syntax errors. The return value
/// is static and contains no source data.
fn numeric_syntax(to: DataType) -> &'static str {
    match to {
        DataType::BigDecimal => "decimal number with optional exponent",
        DataType::Float32 | DataType::Float64 => "number",
        _ => "integer",
    }
}

/// Creates a contextual invalid numeric syntax error.
pub(super) fn invalid_numeric_syntax(to: DataType) -> DataConversionError {
    DataConversionError::InvalidValue {
        from: DataType::String,
        to,
        reason: InvalidValueReason::InvalidSyntax {
            expected: numeric_syntax(to),
        },
    }
}

/// Parses decimal text into a platform-independent integer intermediate.
///
/// Exact mode rejects a non-zero fractional part. Lossy mode truncates toward
/// zero. Exponents are processed structurally, so extreme values are rejected
/// without allocating an exponent-sized buffer.
pub(super) fn parse_text_integer(
    value: &str,
    policy: NumericConversionPolicy,
    to: DataType,
) -> Result<(bool, u128), DataConversionError> {
    let lower = value.to_ascii_lowercase();
    if matches!(
        lower.as_str(),
        "nan"
            | "inf"
            | "+inf"
            | "-inf"
            | "infinity"
            | "+infinity"
            | "-infinity"
    ) {
        return Err(DataConversionError::InvalidValue {
            from: DataType::String,
            to,
            reason: InvalidValueReason::NonFinite,
        });
    }

    let (negative, unsigned) = match value.as_bytes().first() {
        Some(b'-') => (true, &value[1..]),
        Some(b'+') => (false, &value[1..]),
        _ => (false, value),
    };
    if unsigned.is_empty() {
        return Err(invalid_numeric_syntax(to));
    }

    let exponent_index = unsigned
        .bytes()
        .position(|byte| matches!(byte, b'e' | b'E'));
    let (mantissa, exponent) = if let Some(index) = exponent_index {
        let mantissa = &unsigned[..index];
        let exponent_text = &unsigned[index + 1..];
        let exponent_bytes = exponent_text.as_bytes();
        if exponent_bytes.is_empty()
            || exponent_bytes[1..]
                .iter()
                .any(|byte| !byte.is_ascii_digit())
            || !matches!(exponent_bytes[0], b'+' | b'-' | b'0'..=b'9')
        {
            return Err(invalid_numeric_syntax(to));
        }
        let digits = exponent_text
            .strip_prefix(['+', '-'])
            .unwrap_or(exponent_text);
        if digits.is_empty()
            || digits.bytes().any(|byte| !byte.is_ascii_digit())
        {
            return Err(invalid_numeric_syntax(to));
        }
        let exponent = match exponent_text.parse::<i64>() {
            Ok(exponent) => exponent,
            Err(_) if exponent_text.starts_with('-') => i64::MIN,
            Err(_) => i64::MAX,
        };
        (mantissa, exponent)
    } else {
        (unsigned, 0)
    };
    if mantissa.is_empty() {
        return Err(invalid_numeric_syntax(to));
    }

    let mut digits = String::with_capacity(mantissa.len());
    let mut decimal_seen = false;
    let mut fractional_digits = 0usize;
    for byte in mantissa.bytes() {
        match byte {
            b'0'..=b'9' => {
                digits.push(char::from(byte));
                if decimal_seen {
                    fractional_digits += 1;
                }
            }
            b'.' if !decimal_seen => decimal_seen = true,
            _ => return Err(invalid_numeric_syntax(to)),
        }
    }
    if digits.is_empty() {
        return Err(invalid_numeric_syntax(to));
    }

    let decimal_position =
        (digits.len() - fractional_digits) as i128 + i128::from(exponent);
    let integer_digit_count = if decimal_position <= 0 {
        0
    } else {
        usize::try_from(decimal_position)
            .unwrap_or(usize::MAX)
            .min(digits.len())
    };
    let fractional_non_zero = if decimal_position <= 0 {
        digits.bytes().any(|byte| byte != b'0')
    } else if decimal_position < digits.len() as i128 {
        digits.as_bytes()[integer_digit_count..]
            .iter()
            .any(|byte| *byte != b'0')
    } else {
        false
    };
    if policy == NumericConversionPolicy::Exact && fractional_non_zero {
        return Err(DataConversionError::InvalidValue {
            from: DataType::String,
            to,
            reason: InvalidValueReason::PrecisionLoss,
        });
    }

    let mut magnitude = 0u128;
    for byte in digits.bytes().take(integer_digit_count) {
        let Some(next) = magnitude.checked_mul(10) else {
            return Err(DataConversionError::InvalidValue {
                from: DataType::String,
                to,
                reason: InvalidValueReason::OutOfRange,
            });
        };
        let Some(next) = next.checked_add(u128::from(byte - b'0')) else {
            return Err(DataConversionError::InvalidValue {
                from: DataType::String,
                to,
                reason: InvalidValueReason::OutOfRange,
            });
        };
        magnitude = next;
    }
    if decimal_position > digits.len() as i128 && magnitude != 0 {
        let zero_count =
            match u32::try_from(decimal_position - digits.len() as i128) {
                Ok(zero_count) => zero_count,
                Err(_) => {
                    return Err(DataConversionError::InvalidValue {
                        from: DataType::String,
                        to,
                        reason: InvalidValueReason::OutOfRange,
                    });
                }
            };
        let multiplier = 10u128.checked_pow(zero_count).ok_or(
            DataConversionError::InvalidValue {
                from: DataType::String,
                to,
                reason: InvalidValueReason::OutOfRange,
            },
        )?;
        magnitude = magnitude.checked_mul(multiplier).ok_or(
            DataConversionError::InvalidValue {
                from: DataType::String,
                to,
                reason: InvalidValueReason::OutOfRange,
            },
        )?;
    }

    Ok((negative && magnitude != 0, magnitude))
}
