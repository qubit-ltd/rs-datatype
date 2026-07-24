// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Exact decimal-text parsing for primitive floating-point targets.

use super::syntax::invalid_numeric_syntax;
use crate::converter::{
    DataConversionError,
    DataConversionOptions,
    FloatRoundingPolicy,
    InvalidValueReason,
};
use crate::datatype::DataType;

/// Decomposes a non-zero finite float into an integer significand and a power
/// of two.
///
/// # Parameters
///
/// * `value` - Non-zero finite floating-point value to decompose.
///
/// # Returns
///
/// The unsigned significand and its signed binary exponent.
#[must_use]
#[inline]
fn finite_float_parts(value: f64) -> (u128, i32) {
    let bits = value.abs().to_bits();
    let exponent = i32::from(((bits >> 52) & 0x7ff) as u16);
    let fraction = u128::from(bits & 0x000f_ffff_ffff_ffff);
    if exponent == 0 {
        (fraction, -1074)
    } else {
        ((1_u128 << 52) | fraction, exponent - 1075)
    }
}

/// Returns the little-endian decimal digits of a positive integer.
///
/// # Parameters
///
/// * `value` - Positive integer to expand.
///
/// # Returns
///
/// Decimal digits with the least significant digit first.
#[must_use]
fn decimal_digits(mut value: u128) -> Vec<u8> {
    let mut digits = Vec::new();
    while value != 0 {
        digits.push((value % 10) as u8);
        value /= 10;
    }
    digits
}

/// Multiplies little-endian decimal digits by a small factor.
///
/// # Parameters
///
/// * `digits` - Mutable little-endian decimal digits.
/// * `factor` - Single decimal multiplication factor.
fn multiply_decimal_digits(digits: &mut Vec<u8>, factor: u8) {
    let mut carry = 0_u16;
    for digit in digits.iter_mut() {
        let product = u16::from(*digit) * u16::from(factor) + carry;
        *digit = (product % 10) as u8;
        carry = product / 10;
    }
    while carry != 0 {
        digits.push((carry % 10) as u8);
        carry /= 10;
    }
}

/// Expands a non-zero finite float into a canonical decimal coefficient.
///
/// # Parameters
///
/// * `value` - Non-zero finite floating-point value.
///
/// # Returns
///
/// Little-endian coefficient digits, the number of low decimal zero digits to
/// ignore, and the signed decimal scale after canonicalization.
///
/// # Panics
///
/// Panics only if the bounded decimal expansion of a finite `f64` contains
/// more trailing zero digits than `i128` can represent.
#[must_use]
fn exact_float_decimal(value: f64) -> (Vec<u8>, usize, i128) {
    let (mut significand, exponent) = finite_float_parts(value);
    let (factor, multiplication_count, mut scale) = if exponent >= 0 {
        (2_u8, exponent.unsigned_abs(), 0_i128)
    } else {
        let denominator_power = exponent.unsigned_abs();
        let cancelled = significand.trailing_zeros().min(denominator_power);
        significand >>= cancelled;
        let remaining = denominator_power - cancelled;
        (5_u8, remaining, i128::from(remaining))
    };

    let mut digits = decimal_digits(significand);
    for _ in 0..multiplication_count {
        multiply_decimal_digits(&mut digits, factor);
    }
    let trailing_zeros = digits.iter().take_while(|digit| **digit == 0).count();
    scale -= i128::try_from(trailing_zeros)
        .expect("a finite float expansion fits in i128");
    (digits, trailing_zeros, scale)
}

/// Tests whether decimal text denotes an exactly representable finite float.
///
/// # Parameters
///
/// * `value` - Valid decimal source text.
/// * `converted` - Finite float parsed from `value`.
///
/// # Returns
///
/// `true` if `converted` has exactly the same mathematical value as `value`.
#[must_use]
fn text_is_exact_float(value: &str, converted: f64) -> bool {
    let unsigned = value.strip_prefix(['+', '-']).unwrap_or(value);
    let exponent_index = unsigned
        .bytes()
        .position(|byte| matches!(byte, b'e' | b'E'));
    let (mantissa, exponent_text) = if let Some(index) = exponent_index {
        (&unsigned[..index], Some(&unsigned[index + 1..]))
    } else {
        (unsigned, None)
    };

    let mut decimal_seen = false;
    let mut fractional_digits = 0i128;
    let mut digit_count = 0usize;
    let mut leading_zeros = 0usize;
    let mut trailing_zeros = 0usize;
    let mut non_zero_seen = false;
    for byte in mantissa.bytes() {
        match byte {
            b'0'..=b'9' => {
                digit_count += 1;
                if !non_zero_seen {
                    if byte == b'0' {
                        leading_zeros += 1;
                    } else {
                        non_zero_seen = true;
                    }
                }
                trailing_zeros =
                    if byte == b'0' { trailing_zeros + 1 } else { 0 };
                if decimal_seen {
                    fractional_digits += 1;
                }
            }
            b'.' if !decimal_seen => decimal_seen = true,
            _ => return false,
        }
    }
    if digit_count == 0 {
        return false;
    }

    if !non_zero_seen {
        return converted == 0.0;
    }
    let significant_digit_count = digit_count - leading_zeros - trailing_zeros;
    if converted == 0.0 {
        return false;
    }
    let exponent = if let Some(exponent_text) = exponent_text {
        let Ok(exponent) = exponent_text.parse::<i64>() else {
            return false;
        };
        exponent
    } else {
        0
    };
    let scale = fractional_digits
        - i128::from(exponent)
        - i128::try_from(trailing_zeros).unwrap_or(i128::MAX);
    let (float_digits, float_trailing_zeros, float_scale) =
        exact_float_decimal(converted);
    if scale != float_scale
        || significant_digit_count != float_digits.len() - float_trailing_zeros
    {
        return false;
    }
    mantissa
        .bytes()
        .filter(u8::is_ascii_digit)
        .skip(leading_zeros)
        .take(significant_digit_count)
        .map(|digit| digit - b'0')
        .eq(float_digits[float_trailing_zeros..].iter().rev().copied())
}

/// Returns the sign of an explicitly named IEEE infinity.
///
/// # Parameters
///
/// * `value` - Text to match case-insensitively against infinity names.
///
/// # Returns
///
/// `Some(true)` for negative infinity, `Some(false)` for positive infinity,
/// or `None` when `value` is not an explicit infinity.
fn explicit_infinity_is_negative(value: &str) -> Option<bool> {
    if ["inf", "+inf", "infinity", "+infinity"]
        .iter()
        .any(|candidate| value.eq_ignore_ascii_case(candidate))
    {
        Some(false)
    } else if ["-inf", "-infinity"]
        .iter()
        .any(|candidate| value.eq_ignore_ascii_case(candidate))
    {
        Some(true)
    } else {
        None
    }
}

/// Parses a text source as `f64` and enforces exactness when requested.
///
/// # Parameters
///
/// * `value` - Normalized text to parse.
/// * `options` - Numeric exactness policy.
/// * `to` - Target type retained in conversion errors.
///
/// # Returns
///
/// The parsed `f64`, including explicitly named non-finite values.
///
/// # Errors
///
/// Returns a syntax, range, or precision error when parsing or the configured
/// exactness check fails.
pub(super) fn parse_text_f64(
    value: &str,
    options: &DataConversionOptions,
    to: DataType,
) -> Result<f64, DataConversionError> {
    let explicit_nan = value.eq_ignore_ascii_case("nan");
    let explicit_infinity = explicit_infinity_is_negative(value);
    let converted = if explicit_nan {
        f64::NAN
    } else if let Some(negative) = explicit_infinity {
        if negative {
            f64::NEG_INFINITY
        } else {
            f64::INFINITY
        }
    } else {
        value
            .parse::<f64>()
            .map_err(|_| invalid_numeric_syntax(to))?
    };
    if !explicit_nan && explicit_infinity.is_none() && !converted.is_finite() {
        return Err(DataConversionError::invalid(
            DataType::String,
            to,
            InvalidValueReason::OutOfRange,
        ));
    }
    if options.numeric().text_to_float() == FloatRoundingPolicy::Exact
        && converted.is_finite()
        && !text_is_exact_float(value, converted)
    {
        Err(DataConversionError::invalid(
            DataType::String,
            to,
            InvalidValueReason::PrecisionLoss,
        ))
    } else {
        Ok(converted)
    }
}

/// Parses a text source directly as `f32` and enforces exactness.
///
/// # Parameters
///
/// * `value` - Normalized text to parse.
/// * `options` - Numeric exactness policy.
/// * `to` - Target type retained in conversion errors.
///
/// # Returns
///
/// The parsed `f32`, including explicitly named non-finite values.
///
/// # Errors
///
/// Returns a syntax, range, or precision error when parsing or the configured
/// exactness check fails.
pub(super) fn parse_text_f32(
    value: &str,
    options: &DataConversionOptions,
    to: DataType,
) -> Result<f32, DataConversionError> {
    let explicit_nan = value.eq_ignore_ascii_case("nan");
    let explicit_infinity = explicit_infinity_is_negative(value);
    let converted = if explicit_nan {
        f32::NAN
    } else if let Some(negative) = explicit_infinity {
        if negative {
            f32::NEG_INFINITY
        } else {
            f32::INFINITY
        }
    } else {
        value
            .parse::<f32>()
            .map_err(|_| invalid_numeric_syntax(to))?
    };
    if !explicit_nan && explicit_infinity.is_none() && !converted.is_finite() {
        return Err(DataConversionError::invalid(
            DataType::String,
            to,
            InvalidValueReason::OutOfRange,
        ));
    }
    if options.numeric().text_to_float() == FloatRoundingPolicy::Exact
        && converted.is_finite()
        && !text_is_exact_float(value, f64::from(converted))
    {
        Err(DataConversionError::invalid(
            DataType::String,
            to,
            InvalidValueReason::PrecisionLoss,
        ))
    } else {
        Ok(converted)
    }
}
