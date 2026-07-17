// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Primitive integer and duration conversion implementations.

use std::time::Duration;

#[cfg(feature = "big-number")]
use num_traits::ToPrimitive;

use super::super::super::DataConverter;
use super::super::super::string_source::normalize;
#[cfg(feature = "big-number")]
use super::big_number::decimal_to_bigint;
use super::syntax::parse_text_integer;
use crate::converter::{
    DataConversionError,
    DataConversionOptions,
    DataConversionTarget,
    InvalidValueReason,
    NumericConversionPolicy,
};
use crate::datatype::DataType;

/// Returns a platform-independent `(negative, magnitude)` representation.
pub(super) fn signed_magnitude(value: i128) -> (bool, u128) {
    (value.is_negative(), value.unsigned_abs())
}

/// Converts a finite primitive float to an integer intermediate.
fn float_to_integer(
    value: f64,
    policy: NumericConversionPolicy,
    from: DataType,
    to: DataType,
) -> Result<(bool, u128), DataConversionError> {
    if !value.is_finite() {
        return Err(DataConversionError::invalid(
            from,
            to,
            InvalidValueReason::NonFinite,
        ));
    }
    if policy == NumericConversionPolicy::Exact && value.fract() != 0.0 {
        return Err(DataConversionError::invalid(
            from,
            to,
            InvalidValueReason::PrecisionLoss,
        ));
    }
    match parse_text_integer(
        &value.trunc().to_string(),
        NumericConversionPolicy::Lossy,
        to,
    ) {
        Ok(value) => Ok(value),
        Err(error) => {
            let to = error.to_type();
            match error.reason().cloned() {
                Some(reason) => {
                    Err(DataConversionError::invalid(from, to, reason))
                }
                None => Err(error),
            }
        }
    }
}

/// Extracts an integer intermediate from a supported source.
pub(in crate::converter::data_converter) fn source_to_integer(
    source: &DataConverter<'_>,
    options: &DataConversionOptions,
    to: DataType,
) -> Result<(bool, u128), DataConversionError> {
    match source {
        DataConverter::Bool(value) => Ok((false, u128::from(*value))),
        DataConverter::Char(value) => Ok((false, u128::from(*value as u32))),
        DataConverter::Int8(value) => Ok(signed_magnitude(i128::from(*value))),
        DataConverter::Int16(value) => Ok(signed_magnitude(i128::from(*value))),
        DataConverter::Int32(value) => Ok(signed_magnitude(i128::from(*value))),
        DataConverter::Int64(value) => Ok(signed_magnitude(i128::from(*value))),
        DataConverter::Int128(value) => Ok(signed_magnitude(*value)),
        DataConverter::UInt8(value) => Ok((false, u128::from(*value))),
        DataConverter::UInt16(value) => Ok((false, u128::from(*value))),
        DataConverter::UInt32(value) => Ok((false, u128::from(*value))),
        DataConverter::UInt64(value) => Ok((false, u128::from(*value))),
        DataConverter::UInt128(value) => Ok((false, *value)),
        DataConverter::Float32(value) => float_to_integer(
            f64::from(*value),
            options.numeric_policy,
            DataType::Float32,
            to,
        ),
        DataConverter::Float64(value) => float_to_integer(
            *value,
            options.numeric_policy,
            DataType::Float64,
            to,
        ),
        #[cfg(feature = "big-number")]
        DataConverter::BigInteger(value) => {
            if let Some(value) = value.to_i128() {
                Ok(signed_magnitude(value))
            } else if let Some(value) = value.to_u128() {
                Ok((false, value))
            } else {
                Err(DataConversionError::invalid(
                    DataType::BigInteger,
                    to,
                    InvalidValueReason::OutOfRange,
                ))
            }
        }
        #[cfg(feature = "big-number")]
        DataConverter::BigDecimal(value) => {
            let integer = decimal_to_bigint(
                value,
                options.numeric_policy,
                DataType::BigDecimal,
                to,
            )?;
            if let Some(value) = integer.to_i128() {
                Ok(signed_magnitude(value))
            } else if let Some(value) = integer.to_u128() {
                Ok((false, value))
            } else {
                Err(DataConversionError::invalid(
                    DataType::BigDecimal,
                    to,
                    InvalidValueReason::OutOfRange,
                ))
            }
        }
        DataConverter::String(value) => {
            let value = normalize(value, options, to)?;
            parse_text_integer(value, options.numeric_policy, to)
        }
        DataConverter::Duration(value) => {
            Ok((false, duration_to_u128(*value, options, to)?))
        }
        DataConverter::Empty(_) => Err(source.missing(to)),
        _ => Err(source.unsupported(to)),
    }
}

/// Converts a duration to unsigned integer units under the numeric policy.
///
/// The duration unit comes from `options`. Exact mode rejects a remainder;
/// lossy mode uses half-up rounding. `to` is retained as target context.
pub(in crate::converter::data_converter) fn duration_to_u128(
    duration: Duration,
    options: &DataConversionOptions,
    to: DataType,
) -> Result<u128, DataConversionError> {
    if options.numeric_policy == NumericConversionPolicy::Exact {
        options
            .duration
            .output_unit
            .exact_units(duration)
            .ok_or_else(|| {
                DataConversionError::invalid(
                    DataType::Duration,
                    to,
                    InvalidValueReason::PrecisionLoss,
                )
            })
    } else {
        Ok(options.duration.output_unit.rounded_units(duration))
    }
}

/// Converts a supported source to a signed primitive range.
///
/// Returns an `i128` intermediate or an out-of-range error associated with
/// `to`. Source parsing and policy errors are propagated unchanged.
fn to_i128(
    source: &DataConverter<'_>,
    options: &DataConversionOptions,
    to: DataType,
) -> Result<i128, DataConversionError> {
    let (negative, magnitude) = source_to_integer(source, options, to)?;
    if negative && magnitude == 1u128 << 127 {
        return Ok(i128::MIN);
    }
    let value = match i128::try_from(magnitude) {
        Ok(value) => value,
        Err(_) => {
            return Err(DataConversionError::invalid(
                source.data_type(),
                to,
                InvalidValueReason::OutOfRange,
            ));
        }
    };
    Ok(if negative { -value } else { value })
}

/// Converts a supported source to an unsigned primitive range.
///
/// Returns a `u128` intermediate or an out-of-range error associated with
/// `to`. Negative and otherwise unrepresentable values are rejected.
fn to_u128(
    source: &DataConverter<'_>,
    options: &DataConversionOptions,
    to: DataType,
) -> Result<u128, DataConversionError> {
    let (negative, magnitude) = source_to_integer(source, options, to)?;
    if negative {
        Err(DataConversionError::invalid(
            source.data_type(),
            to,
            InvalidValueReason::OutOfRange,
        ))
    } else {
        Ok(magnitude)
    }
}

/// Checks a signed target range.
///
/// `T` must support checked conversion from `i128`. Returns the converted value
/// or an out-of-range error containing `source` and `to` type context.
fn checked_signed<T>(
    value: i128,
    source: &DataConverter<'_>,
    to: DataType,
) -> Result<T, DataConversionError>
where
    T: TryFrom<i128>,
{
    match T::try_from(value) {
        Ok(value) => Ok(value),
        Err(_) => Err(DataConversionError::invalid(
            source.data_type(),
            to,
            InvalidValueReason::OutOfRange,
        )),
    }
}

/// Checks an unsigned target range.
///
/// `T` must support checked conversion from `u128`. Returns the converted value
/// or an out-of-range error containing `source` and `to` type context.
fn checked_unsigned<T>(
    value: u128,
    source: &DataConverter<'_>,
    to: DataType,
) -> Result<T, DataConversionError>
where
    T: TryFrom<u128>,
{
    match T::try_from(value) {
        Ok(value) => Ok(value),
        Err(_) => Err(DataConversionError::invalid(
            source.data_type(),
            to,
            InvalidValueReason::OutOfRange,
        )),
    }
}

macro_rules! impl_signed_target {
    ($target:ty, $data_type:expr) => {
        impl DataConversionTarget for $target {
            fn convert_from(
                source: &DataConverter<'_>,
                options: &DataConversionOptions,
            ) -> Result<Self, DataConversionError> {
                checked_signed(
                    to_i128(source, options, $data_type)?,
                    source,
                    $data_type,
                )
            }
        }
    };
}

macro_rules! impl_unsigned_target {
    ($target:ty, $data_type:expr) => {
        impl DataConversionTarget for $target {
            fn convert_from(
                source: &DataConverter<'_>,
                options: &DataConversionOptions,
            ) -> Result<Self, DataConversionError> {
                checked_unsigned(
                    to_u128(source, options, $data_type)?,
                    source,
                    $data_type,
                )
            }
        }
    };
}

impl_signed_target!(i8, DataType::Int8);
impl_signed_target!(i16, DataType::Int16);
impl_signed_target!(i32, DataType::Int32);
impl_signed_target!(i64, DataType::Int64);
impl_signed_target!(i128, DataType::Int128);
impl_unsigned_target!(u8, DataType::UInt8);
impl_unsigned_target!(u16, DataType::UInt16);
impl_unsigned_target!(u32, DataType::UInt32);
impl_unsigned_target!(u64, DataType::UInt64);
impl_unsigned_target!(u128, DataType::UInt128);
