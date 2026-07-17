// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Arbitrary-precision numeric conversion implementations.

use std::time::Duration;

use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_traits::FromPrimitive;

use super::super::super::DataConverter;
use super::super::super::string_source::normalize;
use super::integer::duration_to_u128;
use super::parsed_number::ParsedNumber;
use super::syntax::parse_number;
use crate::converter::{
    DataConversionError,
    DataConversionOptions,
    DataConversionTarget,
    InvalidValueReason,
    NumericConversionPolicy,
};
use crate::datatype::DataType;

/// Converts a decimal to an integer with exactness checks.
///
/// `from` and `to` are used only as error context. Returns the integral value;
/// exact mode rejects any fractional remainder, while lossy mode truncates
/// toward zero. Values that cannot reasonably fit a primitive target are
/// rejected before constructing an impractically large power of ten.
#[cfg(feature = "big-number")]
pub(super) fn decimal_to_bigint(
    value: &BigDecimal,
    policy: NumericConversionPolicy,
    from: DataType,
    to: DataType,
) -> Result<BigInt, DataConversionError> {
    let (coefficient, scale) = value.as_bigint_and_exponent();
    if coefficient == BigInt::from(0u8) {
        return Ok(coefficient);
    }
    if scale <= 0 {
        let exponent = scale.unsigned_abs();
        let coefficient_digits =
            coefficient.to_str_radix(10).trim_start_matches('-').len() as u64;
        if (to.is_integer()
            && to != DataType::BigInteger
            && coefficient_digits.saturating_add(exponent) > 39)
            || exponent > u64::from(u32::MAX)
        {
            return Err(DataConversionError::InvalidValue {
                from,
                to,
                reason: InvalidValueReason::OutOfRange,
            });
        }
        return Ok(coefficient * BigInt::from(10u8).pow(exponent as u32));
    }

    let coefficient_digits =
        coefficient.to_str_radix(10).trim_start_matches('-').len() as u64;
    if scale as u64 >= coefficient_digits {
        return if policy == NumericConversionPolicy::Exact {
            Err(DataConversionError::InvalidValue {
                from,
                to,
                reason: InvalidValueReason::PrecisionLoss,
            })
        } else {
            Ok(BigInt::from(0u8))
        };
    }
    let divisor = BigInt::from(10u8).pow(scale as u32);
    let quotient = &coefficient / &divisor;
    let remainder = coefficient % divisor;
    if policy == NumericConversionPolicy::Exact
        && remainder != BigInt::from(0u8)
    {
        Err(DataConversionError::InvalidValue {
            from,
            to,
            reason: InvalidValueReason::PrecisionLoss,
        })
    } else {
        Ok(quotient)
    }
}

/// Converts a finite float to an integer with exactness checks.
///
/// Returns a `BigInt` after truncation toward zero. Exact mode rejects a
/// fractional source, and every policy rejects non-finite values. `from` and
/// `to` are retained in those errors.
#[cfg(feature = "big-number")]
fn float_to_bigint(
    value: f64,
    policy: NumericConversionPolicy,
    from: DataType,
    to: DataType,
) -> Result<BigInt, DataConversionError> {
    if !value.is_finite() {
        return Err(DataConversionError::InvalidValue {
            from,
            to,
            reason: InvalidValueReason::NonFinite,
        });
    }
    let converted = BigInt::from_f64(value.trunc())
        .expect("finite primitive floats always have a BigInt representation");
    if policy == NumericConversionPolicy::Exact && value.fract() != 0.0 {
        Err(DataConversionError::InvalidValue {
            from,
            to,
            reason: InvalidValueReason::PrecisionLoss,
        })
    } else {
        Ok(converted)
    }
}

/// Extracts an arbitrary-precision integer from a supported source.
///
/// `options` controls decimal/float exactness and duration units; `to` supplies
/// the final target context. Returns missing, unsupported, syntax, range, or
/// precision errors with the original source type when extraction fails.
#[cfg(feature = "big-number")]
pub(super) fn source_to_bigint(
    source: &DataConverter<'_>,
    options: &DataConversionOptions,
    to: DataType,
) -> Result<BigInt, DataConversionError> {
    match source {
        DataConverter::Bool(value) => Ok(BigInt::from(u8::from(*value))),
        DataConverter::Char(value) => Ok(BigInt::from(*value as u32)),
        DataConverter::Int8(value) => Ok(BigInt::from(*value)),
        DataConverter::Int16(value) => Ok(BigInt::from(*value)),
        DataConverter::Int32(value) => Ok(BigInt::from(*value)),
        DataConverter::Int64(value) => Ok(BigInt::from(*value)),
        DataConverter::Int128(value) => Ok(BigInt::from(*value)),
        DataConverter::UInt8(value) => Ok(BigInt::from(*value)),
        DataConverter::UInt16(value) => Ok(BigInt::from(*value)),
        DataConverter::UInt32(value) => Ok(BigInt::from(*value)),
        DataConverter::UInt64(value) => Ok(BigInt::from(*value)),
        DataConverter::UInt128(value) => Ok(BigInt::from(*value)),
        DataConverter::Float32(value) => float_to_bigint(
            f64::from(*value),
            options.numeric_policy,
            DataType::Float32,
            to,
        ),
        DataConverter::Float64(value) => float_to_bigint(
            *value,
            options.numeric_policy,
            DataType::Float64,
            to,
        ),
        DataConverter::BigInteger(value) => Ok(value.as_ref().clone()),
        DataConverter::BigDecimal(value) => decimal_to_bigint(
            value.as_ref(),
            options.numeric_policy,
            DataType::BigDecimal,
            to,
        ),
        DataConverter::String(value) => {
            let value = normalize(value, options, to)?;
            match parse_number(value, to)? {
                ParsedNumber::Integer(value) => Ok(value),
                ParsedNumber::Decimal(value) => decimal_to_bigint(
                    &value,
                    options.numeric_policy,
                    DataType::String,
                    to,
                ),
                ParsedNumber::NaN
                | ParsedNumber::PositiveInfinity
                | ParsedNumber::NegativeInfinity => {
                    Err(source.invalid(to, InvalidValueReason::NonFinite))
                }
            }
        }
        DataConverter::Duration(value) => {
            duration_to_bigint(*value, options, to)
        }
        DataConverter::Empty(_) => Err(source.missing(to)),
        _ => Err(source.unsupported(to)),
    }
}

/// Converts a duration to arbitrary-precision integer units.
#[cfg(feature = "big-number")]
#[inline(always)]
pub(super) fn duration_to_bigint(
    duration: Duration,
    options: &DataConversionOptions,
    to: DataType,
) -> Result<BigInt, DataConversionError> {
    duration_to_u128(duration, options, to).map(BigInt::from)
}

#[cfg(feature = "big-number")]
impl DataConversionTarget for BigInt {
    #[inline(always)]
    fn convert_from(
        source: &DataConverter<'_>,
        options: &DataConversionOptions,
    ) -> Result<Self, DataConversionError> {
        source_to_bigint(source, options, DataType::BigInteger)
    }
}

#[cfg(feature = "big-number")]
impl DataConversionTarget for BigDecimal {
    fn convert_from(
        source: &DataConverter<'_>,
        options: &DataConversionOptions,
    ) -> Result<Self, DataConversionError> {
        match source {
            DataConverter::BigDecimal(value) => Ok(value.as_ref().clone()),
            DataConverter::Float32(value) => match BigDecimal::from_f32(*value)
            {
                Some(value) => Ok(value),
                None => Err(source.invalid(
                    DataType::BigDecimal,
                    InvalidValueReason::NonFinite,
                )),
            },
            DataConverter::Float64(value) => match BigDecimal::from_f64(*value)
            {
                Some(value) => Ok(value),
                None => Err(source.invalid(
                    DataType::BigDecimal,
                    InvalidValueReason::NonFinite,
                )),
            },
            DataConverter::String(value) => {
                let value = normalize(value, options, DataType::BigDecimal)?;
                match parse_number(value, DataType::BigDecimal)? {
                    ParsedNumber::Integer(value) => Ok(BigDecimal::from(value)),
                    ParsedNumber::Decimal(value) => Ok(value),
                    ParsedNumber::NaN
                    | ParsedNumber::PositiveInfinity
                    | ParsedNumber::NegativeInfinity => Err(source.invalid(
                        DataType::BigDecimal,
                        InvalidValueReason::NonFinite,
                    )),
                }
            }
            DataConverter::Empty(_) => {
                Err(source.missing(DataType::BigDecimal))
            }
            DataConverter::Duration(_) | DataConverter::StringMap(_) => {
                Err(source.unsupported(DataType::BigDecimal))
            }
            #[cfg(feature = "chrono")]
            DataConverter::Date(_)
            | DataConverter::Time(_)
            | DataConverter::DateTime(_)
            | DataConverter::Instant(_) => {
                Err(source.unsupported(DataType::BigDecimal))
            }
            #[cfg(feature = "url")]
            DataConverter::Url(_) => {
                Err(source.unsupported(DataType::BigDecimal))
            }
            #[cfg(feature = "json")]
            DataConverter::Json(_) => {
                Err(source.unsupported(DataType::BigDecimal))
            }
            _ => Ok(BigDecimal::from(source_to_bigint(
                source,
                options,
                DataType::BigDecimal,
            )?)),
        }
    }
}
