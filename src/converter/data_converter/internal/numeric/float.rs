// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Primitive floating-point target dispatch.

use super::super::super::DataConverter;
use super::super::super::string_source::normalize;
#[cfg(feature = "big-integer")]
use super::float_big_number::{bigint_to_f32, bigint_to_f64};
#[cfg(feature = "big-decimal")]
use super::float_big_number::{decimal_to_f32, decimal_to_f64};
use super::float_text::{parse_text_f32, parse_text_f64};
use super::integer::signed_magnitude;
use crate::converter::{
    DataConversionError, DataConversionOptions, DataConversionTarget, InvalidValueReason,
    NumericConversionPolicy,
};
use crate::datatype::DataType;

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
        Err(DataConversionError::invalid(
            from,
            to,
            InvalidValueReason::PrecisionLoss,
        ))
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
        return Err(DataConversionError::invalid(
            from,
            to,
            InvalidValueReason::OutOfRange,
        ));
    }
    let exact = unsigned_integer_is_exact(magnitude, f32::MANTISSA_DIGITS);
    if policy == NumericConversionPolicy::Exact && !exact {
        Err(DataConversionError::invalid(
            from,
            to,
            InvalidValueReason::PrecisionLoss,
        ))
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
        #[cfg(feature = "big-integer")]
        DataConverter::BigInteger(value) => {
            bigint_to_f64(value, options.numeric_policy, DataType::BigInteger, to)
        }
        #[cfg(feature = "big-decimal")]
        DataConverter::BigDecimal(value) => {
            decimal_to_f64(value, options.numeric_policy, DataType::BigDecimal, to)
        }
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
                    return Err(source.invalid(to, InvalidValueReason::OutOfRange));
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
            #[cfg(feature = "big-integer")]
            DataConverter::BigInteger(value) => {
                bigint_to_f32(value, options.numeric_policy, DataType::BigInteger, to)
            }
            #[cfg(feature = "big-decimal")]
            DataConverter::BigDecimal(value) => {
                decimal_to_f32(value, options.numeric_policy, DataType::BigDecimal, to)
            }
            DataConverter::String(value) => {
                let value = normalize(value, options, to)?;
                parse_text_f32(value, options, to)
            }
            DataConverter::Empty(_) => Err(source.missing(to)),
            _ => Err(source.unsupported(to)),
        }
    }
}
