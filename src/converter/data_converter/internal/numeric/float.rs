// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Floating-point numeric conversion implementations.

#[cfg(feature = "big-number")]
use bigdecimal::BigDecimal;
#[cfg(feature = "big-number")]
use num_bigint::BigInt;
#[cfg(feature = "big-number")]
use num_traits::{
    FromPrimitive,
    ToPrimitive,
};

use super::super::super::DataConverter;
use super::super::super::string_source::normalize;
use super::integer::signed_magnitude;
use super::syntax::{
    invalid_numeric_syntax,
    parse_text_integer,
};
use crate::converter::{
    DataConversionError,
    DataConversionOptions,
    DataConversionTarget,
    InvalidValueReason,
    NumericConversionPolicy,
};
use crate::datatype::DataType;

/// Converts an integer exactly or lossily to a float.
///
/// Lossy mode accepts finite IEEE rounding. Exact mode additionally requires
/// converting the result back to reproduce `value`. Non-finite results are
/// reported as out of range using `from` and `to`.
#[cfg(feature = "big-number")]
fn bigint_to_f64(
    value: &BigInt,
    policy: NumericConversionPolicy,
    from: DataType,
    to: DataType,
) -> Result<f64, DataConversionError> {
    let converted = value.to_f64().unwrap_or(f64::INFINITY);
    if !converted.is_finite() {
        return Err(DataConversionError::InvalidValue {
            from,
            to,
            reason: InvalidValueReason::OutOfRange,
        });
    }
    if policy == NumericConversionPolicy::Exact
        && BigInt::from_f64(converted).as_ref() != Some(value)
    {
        Err(DataConversionError::InvalidValue {
            from,
            to,
            reason: InvalidValueReason::PrecisionLoss,
        })
    } else {
        Ok(converted)
    }
}

/// Converts an integer exactly or lossily to an `f32`.
#[cfg(feature = "big-number")]
fn bigint_to_f32(
    value: &BigInt,
    policy: NumericConversionPolicy,
    from: DataType,
    to: DataType,
) -> Result<f32, DataConversionError> {
    let converted = value.to_f32().unwrap_or(f32::INFINITY);
    if !converted.is_finite() {
        return Err(DataConversionError::InvalidValue {
            from,
            to,
            reason: InvalidValueReason::OutOfRange,
        });
    }
    if policy == NumericConversionPolicy::Exact
        && BigInt::from_f32(converted).as_ref() != Some(value)
    {
        Err(DataConversionError::InvalidValue {
            from,
            to,
            reason: InvalidValueReason::PrecisionLoss,
        })
    } else {
        Ok(converted)
    }
}

/// Converts a decimal exactly or lossily to a float.
///
/// Lossy mode accepts finite IEEE rounding. Exact mode additionally requires
/// converting the result back to reproduce `value`. Non-finite results are
/// reported as out of range using `from` and `to`.
#[cfg(feature = "big-number")]
fn decimal_to_f64(
    value: &BigDecimal,
    policy: NumericConversionPolicy,
    from: DataType,
    to: DataType,
) -> Result<f64, DataConversionError> {
    let converted = value.to_f64().unwrap_or(f64::INFINITY);
    if !converted.is_finite() {
        return Err(DataConversionError::InvalidValue {
            from,
            to,
            reason: InvalidValueReason::OutOfRange,
        });
    }
    if policy == NumericConversionPolicy::Exact
        && BigDecimal::from_f64(converted).as_ref() != Some(value)
    {
        Err(DataConversionError::InvalidValue {
            from,
            to,
            reason: InvalidValueReason::PrecisionLoss,
        })
    } else {
        Ok(converted)
    }
}

/// Converts a decimal exactly or lossily to an `f32`.
#[cfg(feature = "big-number")]
fn decimal_to_f32(
    value: &BigDecimal,
    policy: NumericConversionPolicy,
    from: DataType,
    to: DataType,
) -> Result<f32, DataConversionError> {
    let converted = value.to_f32().unwrap_or(f32::INFINITY);
    if !converted.is_finite() {
        return Err(DataConversionError::InvalidValue {
            from,
            to,
            reason: InvalidValueReason::OutOfRange,
        });
    }
    if policy == NumericConversionPolicy::Exact
        && BigDecimal::from_f32(converted).as_ref() != Some(value)
    {
        Err(DataConversionError::InvalidValue {
            from,
            to,
            reason: InvalidValueReason::PrecisionLoss,
        })
    } else {
        Ok(converted)
    }
}

/// Tests whether an unsigned integer fits a floating-point mantissa exactly.
fn unsigned_integer_is_exact(value: u128, mantissa_digits: u32) -> bool {
    if value == 0 {
        return true;
    }
    let significant_bits = u128::BITS - value.leading_zeros();
    significant_bits <= mantissa_digits
        || value.trailing_zeros() >= significant_bits - mantissa_digits
}

/// Converts an integer intermediate to `f64` under the numeric policy.
fn integer_to_f64(
    value: (bool, u128),
    policy: NumericConversionPolicy,
    from: DataType,
    to: DataType,
) -> Result<f64, DataConversionError> {
    let (negative, magnitude) = value;
    let converted = if negative {
        -(magnitude as f64)
    } else {
        magnitude as f64
    };
    let exact = unsigned_integer_is_exact(magnitude, f64::MANTISSA_DIGITS);
    if policy == NumericConversionPolicy::Exact && !exact {
        Err(DataConversionError::InvalidValue {
            from,
            to,
            reason: InvalidValueReason::PrecisionLoss,
        })
    } else {
        Ok(converted)
    }
}

/// Converts an integer intermediate to `f32` under the numeric policy.
fn integer_to_f32(
    value: (bool, u128),
    policy: NumericConversionPolicy,
    from: DataType,
    to: DataType,
) -> Result<f32, DataConversionError> {
    let (negative, magnitude) = value;
    let converted = if negative {
        -(magnitude as f32)
    } else {
        magnitude as f32
    };
    if !converted.is_finite() {
        return Err(DataConversionError::InvalidValue {
            from,
            to,
            reason: InvalidValueReason::OutOfRange,
        });
    }
    let exact = unsigned_integer_is_exact(magnitude, f32::MANTISSA_DIGITS);
    if policy == NumericConversionPolicy::Exact && !exact {
        Err(DataConversionError::InvalidValue {
            from,
            to,
            reason: InvalidValueReason::PrecisionLoss,
        })
    } else {
        Ok(converted)
    }
}

/// Tests whether decimal text denotes an exactly representable finite float.
fn text_is_exact_float(
    value: &str,
    converted: f64,
    mantissa_digits: u32,
    to: DataType,
) -> bool {
    let unsigned = value.strip_prefix(['+', '-']).unwrap_or(value);
    let exponent_index = unsigned
        .bytes()
        .position(|byte| matches!(byte, b'e' | b'E'));
    let (mantissa, exponent) = if let Some(index) = exponent_index {
        let Ok(exponent) = unsigned[index + 1..].parse::<i64>() else {
            return false;
        };
        (&unsigned[..index], exponent)
    } else {
        (unsigned, 0)
    };

    let mut decimal_seen = false;
    let mut fractional_digits = 0i128;
    let mut digit_count = 0usize;
    let mut trailing_zeros = 0usize;
    for byte in mantissa.bytes() {
        match byte {
            b'0'..=b'9' => {
                digit_count += 1;
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

    let significant_digit_count = digit_count - trailing_zeros;
    let mut coefficient = 0u128;
    for (digit_index, byte) in
        mantissa.bytes().filter(u8::is_ascii_digit).enumerate()
    {
        if digit_index == significant_digit_count {
            break;
        }
        let Some(next) = coefficient.checked_mul(10) else {
            return false;
        };
        let Some(next) = next.checked_add(u128::from(byte - b'0')) else {
            return false;
        };
        coefficient = next;
    }
    if coefficient == 0 {
        return converted == 0.0;
    }

    let scale = fractional_digits
        - i128::from(exponent)
        - i128::try_from(trailing_zeros).unwrap_or(i128::MAX);
    if scale <= 0 {
        let Ok(integer) =
            parse_text_integer(value, NumericConversionPolicy::Exact, to)
        else {
            return false;
        };
        return unsigned_integer_is_exact(integer.1, mantissa_digits);
    }
    let Ok(scale) = u32::try_from(scale) else {
        return false;
    };
    for _ in 0..scale {
        if !coefficient.is_multiple_of(5) {
            return false;
        }
        coefficient /= 5;
    }
    let cancelled_twos = coefficient.trailing_zeros().min(scale);
    coefficient >>= cancelled_twos;
    let significant_bits = u128::BITS - coefficient.leading_zeros();
    converted != 0.0 && significant_bits <= mantissa_digits
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
fn parse_text_f64(
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
        return Err(DataConversionError::InvalidValue {
            from: DataType::String,
            to,
            reason: InvalidValueReason::OutOfRange,
        });
    }
    if options.numeric_policy == NumericConversionPolicy::Exact
        && converted.is_finite()
        && !text_is_exact_float(value, converted, f64::MANTISSA_DIGITS, to)
    {
        Err(DataConversionError::InvalidValue {
            from: DataType::String,
            to,
            reason: InvalidValueReason::PrecisionLoss,
        })
    } else {
        Ok(converted)
    }
}

/// Parses a text source directly as `f32` and enforces exactness.
fn parse_text_f32(
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
        return Err(DataConversionError::InvalidValue {
            from: DataType::String,
            to,
            reason: InvalidValueReason::OutOfRange,
        });
    }
    if options.numeric_policy == NumericConversionPolicy::Exact
        && converted.is_finite()
        && !text_is_exact_float(
            value,
            f64::from(converted),
            f32::MANTISSA_DIGITS,
            to,
        )
    {
        Err(DataConversionError::InvalidValue {
            from: DataType::String,
            to,
            reason: InvalidValueReason::PrecisionLoss,
        })
    } else {
        Ok(converted)
    }
}

/// Converts a source to f64 before target-width validation.
///
/// `options` controls exactness and `to` identifies the eventual float target.
/// Returns contextual missing, unsupported, syntax, range, non-finite, or
/// precision errors when an `f64` intermediate cannot be produced.
fn source_to_f64(
    source: &DataConverter<'_>,
    options: &DataConversionOptions,
    to: DataType,
) -> Result<f64, DataConversionError> {
    match source {
        DataConverter::Float64(value) => Ok(*value),
        DataConverter::Float32(value) => Ok(f64::from(*value)),
        DataConverter::Bool(value) => Ok(if *value { 1.0 } else { 0.0 }),
        DataConverter::Char(value) => Ok(f64::from(*value as u32)),
        DataConverter::Int8(value) => integer_to_f64(
            signed_magnitude(i128::from(*value)),
            options.numeric_policy,
            DataType::Int8,
            to,
        ),
        DataConverter::Int16(value) => integer_to_f64(
            signed_magnitude(i128::from(*value)),
            options.numeric_policy,
            DataType::Int16,
            to,
        ),
        DataConverter::Int32(value) => integer_to_f64(
            signed_magnitude(i128::from(*value)),
            options.numeric_policy,
            DataType::Int32,
            to,
        ),
        DataConverter::Int64(value) => integer_to_f64(
            signed_magnitude(i128::from(*value)),
            options.numeric_policy,
            DataType::Int64,
            to,
        ),
        DataConverter::Int128(value) => integer_to_f64(
            signed_magnitude(*value),
            options.numeric_policy,
            DataType::Int128,
            to,
        ),
        DataConverter::UInt8(value) => integer_to_f64(
            (false, u128::from(*value)),
            options.numeric_policy,
            DataType::UInt8,
            to,
        ),
        DataConverter::UInt16(value) => integer_to_f64(
            (false, u128::from(*value)),
            options.numeric_policy,
            DataType::UInt16,
            to,
        ),
        DataConverter::UInt32(value) => integer_to_f64(
            (false, u128::from(*value)),
            options.numeric_policy,
            DataType::UInt32,
            to,
        ),
        DataConverter::UInt64(value) => integer_to_f64(
            (false, u128::from(*value)),
            options.numeric_policy,
            DataType::UInt64,
            to,
        ),
        DataConverter::UInt128(value) => integer_to_f64(
            (false, *value),
            options.numeric_policy,
            DataType::UInt128,
            to,
        ),
        #[cfg(feature = "big-number")]
        DataConverter::BigInteger(value) => bigint_to_f64(
            value,
            options.numeric_policy,
            DataType::BigInteger,
            to,
        ),
        #[cfg(feature = "big-number")]
        DataConverter::BigDecimal(value) => decimal_to_f64(
            value,
            options.numeric_policy,
            DataType::BigDecimal,
            to,
        ),
        DataConverter::String(value) => {
            let value = normalize(value, options, to)?;
            parse_text_f64(value, options, to)
        }
        DataConverter::Empty(_) => Err(source.missing(to)),
        _ => Err(source.unsupported(to)),
    }
}

impl DataConversionTarget for f64 {
    #[inline(always)]
    fn convert_from(
        source: &DataConverter<'_>,
        options: &DataConversionOptions,
    ) -> Result<Self, DataConversionError> {
        source_to_f64(source, options, DataType::Float64)
    }
}

impl DataConversionTarget for f32 {
    fn convert_from(
        source: &DataConverter<'_>,
        options: &DataConversionOptions,
    ) -> Result<Self, DataConversionError> {
        let to = DataType::Float32;
        match source {
            DataConverter::Float32(value) => Ok(*value),
            DataConverter::Float64(value) => {
                if value.is_nan() {
                    return Ok(f32::NAN);
                }
                if *value == f64::INFINITY {
                    return Ok(f32::INFINITY);
                }
                if *value == f64::NEG_INFINITY {
                    return Ok(f32::NEG_INFINITY);
                }
                let converted = *value as f32;
                if !converted.is_finite() {
                    return Err(
                        source.invalid(to, InvalidValueReason::OutOfRange)
                    );
                }
                if options.numeric_policy == NumericConversionPolicy::Exact
                    && f64::from(converted) != *value
                {
                    Err(source.invalid(to, InvalidValueReason::PrecisionLoss))
                } else {
                    Ok(converted)
                }
            }
            DataConverter::Bool(value) => Ok(if *value { 1.0 } else { 0.0 }),
            DataConverter::Char(value) => Ok(*value as u32 as f32),
            DataConverter::Int8(value) => integer_to_f32(
                signed_magnitude(i128::from(*value)),
                options.numeric_policy,
                DataType::Int8,
                to,
            ),
            DataConverter::Int16(value) => integer_to_f32(
                signed_magnitude(i128::from(*value)),
                options.numeric_policy,
                DataType::Int16,
                to,
            ),
            DataConverter::Int32(value) => integer_to_f32(
                signed_magnitude(i128::from(*value)),
                options.numeric_policy,
                DataType::Int32,
                to,
            ),
            DataConverter::Int64(value) => integer_to_f32(
                signed_magnitude(i128::from(*value)),
                options.numeric_policy,
                DataType::Int64,
                to,
            ),
            DataConverter::Int128(value) => integer_to_f32(
                signed_magnitude(*value),
                options.numeric_policy,
                DataType::Int128,
                to,
            ),
            DataConverter::UInt8(value) => integer_to_f32(
                (false, u128::from(*value)),
                options.numeric_policy,
                DataType::UInt8,
                to,
            ),
            DataConverter::UInt16(value) => integer_to_f32(
                (false, u128::from(*value)),
                options.numeric_policy,
                DataType::UInt16,
                to,
            ),
            DataConverter::UInt32(value) => integer_to_f32(
                (false, u128::from(*value)),
                options.numeric_policy,
                DataType::UInt32,
                to,
            ),
            DataConverter::UInt64(value) => integer_to_f32(
                (false, u128::from(*value)),
                options.numeric_policy,
                DataType::UInt64,
                to,
            ),
            DataConverter::UInt128(value) => integer_to_f32(
                (false, *value),
                options.numeric_policy,
                DataType::UInt128,
                to,
            ),
            #[cfg(feature = "big-number")]
            DataConverter::BigInteger(value) => bigint_to_f32(
                value,
                options.numeric_policy,
                DataType::BigInteger,
                to,
            ),
            #[cfg(feature = "big-number")]
            DataConverter::BigDecimal(value) => decimal_to_f32(
                value,
                options.numeric_policy,
                DataType::BigDecimal,
                to,
            ),
            DataConverter::String(value) => {
                let value = normalize(value, options, to)?;
                parse_text_f32(value, options, to)
            }
            DataConverter::Empty(_) => Err(source.missing(to)),
            _ => Err(source.unsupported(to)),
        }
    }
}
