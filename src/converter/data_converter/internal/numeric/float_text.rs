// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Exact decimal-text parsing for primitive floating-point targets.

use num_bigint::BigUint;

use super::syntax::invalid_numeric_syntax;
use crate::converter::{
    DataConversionError, DataConversionOptions, InvalidValueReason, NumericConversionPolicy,
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
fn finite_float_parts(value: f64) -> (u128, i128) {
    let bits = value.abs().to_bits();
    let exponent = i128::from(((bits >> 52) & 0x7ff) as u16);
    let fraction = u128::from(bits & 0x000f_ffff_ffff_ffff);
    if exponent == 0 {
        (fraction, -1074)
    } else {
        ((1_u128 << 52) | fraction, exponent - 1075)
    }
}

/// Removes factors of two and five from an unsigned integer.
///
/// # Parameters
///
/// * `value` - Non-zero integer to factor.
///
/// # Returns
///
/// The remaining coprime value, the number of factors of two, and the number
/// of factors of five.
fn factor_u128(mut value: u128) -> (u128, i128, i128) {
    let twos = value.trailing_zeros();
    value >>= twos;
    let mut fives = 0i128;
    while value.is_multiple_of(5) {
        value /= 5;
        fives += 1;
    }
    (value, i128::from(twos), fives)
}

/// Compares a bounded decimal coefficient with a parsed finite float.
///
/// # Parameters
///
/// * `coefficient` - Non-zero coefficient before the decimal scale.
/// * `scale` - Signed number of base-ten fractional digits.
/// * `converted` - Non-zero finite float parsed from the source text.
///
/// # Returns
///
/// `true` if both values have identical prime-factor decompositions.
fn bounded_coefficient_is_exact_float(coefficient: u128, scale: i128, converted: f64) -> bool {
    let (decimal_residual, decimal_twos, decimal_fives) = factor_u128(coefficient);
    let (float_significand, float_exponent) = finite_float_parts(converted);
    let (float_residual, float_twos, float_fives) = factor_u128(float_significand);
    decimal_residual == float_residual
        && decimal_twos - scale == float_twos + float_exponent
        && decimal_fives - scale == float_fives
}

/// Compares an unbounded decimal coefficient with a parsed finite float.
///
/// # Parameters
///
/// * `mantissa` - Decimal mantissa containing digits and at most one point.
/// * `significant_digit_count` - Number of coefficient digits to retain.
/// * `scale` - Signed number of base-ten fractional digits.
/// * `converted` - Non-zero finite float parsed from the source text.
///
/// # Returns
///
/// `true` if both values have identical prime-factor decompositions.
fn unbounded_coefficient_is_exact_float(
    mantissa: &str,
    significant_digit_count: usize,
    scale: i128,
    converted: f64,
) -> bool {
    let digits = mantissa
        .bytes()
        .filter(u8::is_ascii_digit)
        .take(significant_digit_count)
        .collect::<Vec<_>>();
    let Some(mut coefficient) = BigUint::parse_bytes(&digits, 10) else {
        return false;
    };
    let decimal_twos = coefficient.trailing_zeros().unwrap_or(0);
    let (float_significand, float_exponent) = finite_float_parts(converted);
    let (float_residual, float_twos, float_fives) = factor_u128(float_significand);
    if i128::from(decimal_twos) - scale != float_twos + float_exponent {
        return false;
    }
    coefficient >>= decimal_twos;
    let mut decimal_fives = 0i128;
    while (&coefficient % 5_u8).bits() == 0 {
        coefficient /= 5_u8;
        decimal_fives += 1;
    }

    coefficient == BigUint::from(float_residual) && decimal_fives - scale == float_fives
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
    let mut trailing_zeros = 0usize;
    for byte in mantissa.bytes() {
        match byte {
            b'0'..=b'9' => {
                digit_count += 1;
                trailing_zeros = if byte == b'0' { trailing_zeros + 1 } else { 0 };
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

    let significant_digit_count = digit_count - trailing_zeros;
    if significant_digit_count == 0 {
        return converted == 0.0;
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
    let mut coefficient = 0u128;
    for (digit_index, byte) in mantissa.bytes().filter(u8::is_ascii_digit).enumerate() {
        if digit_index == significant_digit_count {
            break;
        }
        let Some(next) = coefficient
            .checked_mul(10)
            .and_then(|value| value.checked_add(u128::from(byte - b'0')))
        else {
            return converted != 0.0
                && unbounded_coefficient_is_exact_float(
                    mantissa,
                    significant_digit_count,
                    scale,
                    converted,
                );
        };
        coefficient = next;
    }
    if coefficient == 0 {
        return converted == 0.0;
    }
    converted != 0.0 && bounded_coefficient_is_exact_float(coefficient, scale, converted)
}

/// Returns the sign of an explicitly named IEEE infinity.
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
    if options.numeric_policy == NumericConversionPolicy::Exact
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
    if options.numeric_policy == NumericConversionPolicy::Exact
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
