/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! # Reusable Data Conversion
//!
//! Provides `DataConverter`, a lightweight source-value wrapper used to convert
//! common runtime values to supported Rust target types.
//!

use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt;
use std::time::Duration;

use bigdecimal::BigDecimal;
use chrono::{
    DateTime,
    NaiveDate,
    NaiveDateTime,
    NaiveTime,
    Utc,
};
use num_bigint::BigInt;
use num_traits::ToPrimitive;
use url::Url;

use super::data_conversion_error::DataConversionError;
use super::data_conversion_options::DataConversionOptions;
use super::data_conversion_result::DataConversionResult;
use super::data_convert_to::DataConvertTo;
use super::duration_unit::DurationUnit;
use crate::datatype::DataType;

/// A lightweight wrapper for converting common data values.
///
/// `DataConverter` borrows source values when possible and owns them only when
/// created from owned inputs such as `String`, `BigInt`, or `serde_json::Value`.
/// It does not depend on higher-level value containers.
///
/// # Examples
///
/// ```
/// use std::time::Duration;
///
/// use qubit_datatype::converter::{
///     DataConversionResult,
///     DataConverter,
/// };
///
/// fn read_timeout() -> DataConversionResult<Duration> {
///     DataConverter::from("1500000000ns").to::<Duration>()
/// }
///
/// let timeout = read_timeout().expect("duration text should convert");
/// assert_eq!(timeout, Duration::new(1, 500_000_000));
///
/// let port: u16 = DataConverter::from("8080")
///     .to()
///     .expect("port text should convert to u16");
/// assert_eq!(port, 8080);
///
/// let enabled: bool = DataConverter::from("true")
///     .to()
///     .expect("boolean text should convert to bool");
/// assert!(enabled);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum DataConverter<'a> {
    /// Empty source with a known data type.
    Empty(DataType),
    /// Boolean source.
    Bool(bool),
    /// Character source.
    Char(char),
    /// 8-bit signed integer source.
    Int8(i8),
    /// 16-bit signed integer source.
    Int16(i16),
    /// 32-bit signed integer source.
    Int32(i32),
    /// 64-bit signed integer source.
    Int64(i64),
    /// 128-bit signed integer source.
    Int128(i128),
    /// 8-bit unsigned integer source.
    UInt8(u8),
    /// 16-bit unsigned integer source.
    UInt16(u16),
    /// 32-bit unsigned integer source.
    UInt32(u32),
    /// 64-bit unsigned integer source.
    UInt64(u64),
    /// 128-bit unsigned integer source.
    UInt128(u128),
    /// Platform-dependent signed integer source.
    IntSize(isize),
    /// Platform-dependent unsigned integer source.
    UIntSize(usize),
    /// 32-bit floating point source.
    Float32(f32),
    /// 64-bit floating point source.
    Float64(f64),
    /// Big integer source.
    BigInteger(Cow<'a, BigInt>),
    /// Big decimal source.
    BigDecimal(Cow<'a, BigDecimal>),
    /// String source.
    String(Cow<'a, str>),
    /// Date source.
    Date(NaiveDate),
    /// Time source.
    Time(NaiveTime),
    /// Date-time source.
    DateTime(NaiveDateTime),
    /// UTC instant source.
    Instant(DateTime<Utc>),
    /// Duration source.
    Duration(Duration),
    /// URL source.
    Url(Cow<'a, Url>),
    /// String map source.
    StringMap(Cow<'a, HashMap<String, String>>),
    /// JSON value source.
    Json(Cow<'a, serde_json::Value>),
}

impl DataConverter<'_> {
    /// Converts this source value to the requested target type.
    ///
    /// # Type Parameters
    ///
    /// * `T` - Target type to convert to.
    ///
    /// # Returns
    ///
    /// Returns the converted value when the conversion is supported and the
    /// source value is valid for the target type.
    ///
    /// # Errors
    ///
    /// Returns [`DataConversionError::NoValue`] for empty sources,
    /// [`DataConversionError::ConversionFailed`] for unsupported source-target
    /// pairs, or a detailed conversion error for invalid source content.
    #[inline]
    pub fn to<T>(&self) -> DataConversionResult<T>
    where
        Self: DataConvertTo<T>,
    {
        self.to_with(&DataConversionOptions::default())
    }

    /// Converts this source value to the requested target type using options.
    ///
    /// # Type Parameters
    ///
    /// * `T` - Target type to convert to.
    ///
    /// # Parameters
    ///
    /// * `options` - Conversion options used for string, boolean, and
    ///   collection-related parsing.
    ///
    /// # Returns
    ///
    /// Returns the converted value when the conversion is supported and the
    /// source value is valid for the target type.
    ///
    /// # Errors
    ///
    /// Returns [`DataConversionError::NoValue`] for empty or option-defined
    /// missing sources, [`DataConversionError::ConversionFailed`] for
    /// unsupported source-target pairs, or a detailed conversion error for
    /// invalid source content.
    #[inline]
    pub fn to_with<T>(&self, options: &DataConversionOptions) -> DataConversionResult<T>
    where
        Self: DataConvertTo<T>,
    {
        <Self as DataConvertTo<T>>::convert(self, options)
    }

    /// Returns the data type represented by this source value.
    ///
    /// # Returns
    ///
    /// Returns the [`DataType`] corresponding to this converter's source
    /// variant. Empty sources return their preserved data type.
    #[inline]
    pub const fn data_type(&self) -> DataType {
        match self {
            DataConverter::Empty(data_type) => *data_type,
            DataConverter::Bool(_) => DataType::Bool,
            DataConverter::Char(_) => DataType::Char,
            DataConverter::Int8(_) => DataType::Int8,
            DataConverter::Int16(_) => DataType::Int16,
            DataConverter::Int32(_) => DataType::Int32,
            DataConverter::Int64(_) => DataType::Int64,
            DataConverter::Int128(_) => DataType::Int128,
            DataConverter::UInt8(_) => DataType::UInt8,
            DataConverter::UInt16(_) => DataType::UInt16,
            DataConverter::UInt32(_) => DataType::UInt32,
            DataConverter::UInt64(_) => DataType::UInt64,
            DataConverter::UInt128(_) => DataType::UInt128,
            DataConverter::IntSize(_) => DataType::IntSize,
            DataConverter::UIntSize(_) => DataType::UIntSize,
            DataConverter::Float32(_) => DataType::Float32,
            DataConverter::Float64(_) => DataType::Float64,
            DataConverter::BigInteger(_) => DataType::BigInteger,
            DataConverter::BigDecimal(_) => DataType::BigDecimal,
            DataConverter::String(_) => DataType::String,
            DataConverter::Date(_) => DataType::Date,
            DataConverter::Time(_) => DataType::Time,
            DataConverter::DateTime(_) => DataType::DateTime,
            DataConverter::Instant(_) => DataType::Instant,
            DataConverter::Duration(_) => DataType::Duration,
            DataConverter::Url(_) => DataType::Url,
            DataConverter::StringMap(_) => DataType::StringMap,
            DataConverter::Json(_) => DataType::Json,
        }
    }

    /// Builds a conversion-failed error for this source value and target type.
    fn unsupported(&self, target_type: DataType) -> DataConversionError {
        DataConversionError::ConversionFailed {
            from: self.data_type(),
            to: target_type,
        }
    }
}

macro_rules! impl_from_copy {
    ($source_type:ty, $variant:ident) => {
        impl<'a> From<$source_type> for DataConverter<'a> {
            #[inline]
            fn from(value: $source_type) -> Self {
                DataConverter::$variant(value)
            }
        }

        impl<'a> From<&'a $source_type> for DataConverter<'a> {
            #[inline]
            fn from(value: &'a $source_type) -> Self {
                DataConverter::$variant(*value)
            }
        }
    };
}

macro_rules! impl_from_cow {
    ($source_type:ty, $variant:ident) => {
        impl<'a> From<$source_type> for DataConverter<'a> {
            #[inline]
            fn from(value: $source_type) -> Self {
                DataConverter::$variant(Cow::Owned(value))
            }
        }

        impl<'a> From<&'a $source_type> for DataConverter<'a> {
            #[inline]
            fn from(value: &'a $source_type) -> Self {
                DataConverter::$variant(Cow::Borrowed(value))
            }
        }
    };
}

impl_from_copy!(bool, Bool);
impl_from_copy!(char, Char);
impl_from_copy!(i8, Int8);
impl_from_copy!(i16, Int16);
impl_from_copy!(i32, Int32);
impl_from_copy!(i64, Int64);
impl_from_copy!(i128, Int128);
impl_from_copy!(u8, UInt8);
impl_from_copy!(u16, UInt16);
impl_from_copy!(u32, UInt32);
impl_from_copy!(u64, UInt64);
impl_from_copy!(u128, UInt128);
impl_from_copy!(isize, IntSize);
impl_from_copy!(usize, UIntSize);
impl_from_copy!(f32, Float32);
impl_from_copy!(f64, Float64);
impl_from_copy!(NaiveDate, Date);
impl_from_copy!(NaiveTime, Time);
impl_from_copy!(NaiveDateTime, DateTime);
impl_from_copy!(DateTime<Utc>, Instant);
impl_from_copy!(Duration, Duration);

impl_from_cow!(BigInt, BigInteger);
impl_from_cow!(BigDecimal, BigDecimal);
impl_from_cow!(Url, Url);
impl_from_cow!(HashMap<String, String>, StringMap);
impl_from_cow!(serde_json::Value, Json);

impl<'a> From<&'a str> for DataConverter<'a> {
    /// Creates a converter from a borrowed string slice.
    #[inline]
    fn from(value: &'a str) -> Self {
        DataConverter::String(Cow::Borrowed(value))
    }
}

impl<'a> From<&'a String> for DataConverter<'a> {
    /// Creates a converter from a borrowed [`String`].
    #[inline]
    fn from(value: &'a String) -> Self {
        DataConverter::String(Cow::Borrowed(value.as_str()))
    }
}

impl<'a> From<String> for DataConverter<'a> {
    /// Creates a converter from an owned [`String`].
    #[inline]
    fn from(value: String) -> Self {
        DataConverter::String(Cow::Owned(value))
    }
}

/// Parses a string using the configured bool conversion rules.
fn parse_bool_string(value: &str, options: &DataConversionOptions) -> DataConversionResult<bool> {
    let value = options.string.normalize(value)?;
    if let Some(parsed) = options.boolean.parse(&value) {
        Ok(parsed)
    } else {
        Err(DataConversionError::ConversionError(format!(
            "Cannot convert '{value}' to boolean"
        )))
    }
}

/// Parses a string using the configured duration conversion rules.
///
/// # Parameters
///
/// * `value` - Source string to parse.
/// * `options` - Conversion options providing string normalization and the
///   default unit for suffixless duration strings.
///
/// # Returns
///
/// The parsed [`Duration`].
///
/// # Errors
///
/// Returns [`DataConversionError::NoValue`] when string normalization treats
/// the value as missing, or [`DataConversionError::ConversionError`] when the
/// value is empty, has an invalid integer, uses an unsupported suffix, or
/// overflows [`Duration`].
fn parse_duration_string(
    value: &str,
    options: &DataConversionOptions,
) -> DataConversionResult<Duration> {
    let value = options.string.normalize(value)?;
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(DataConversionError::ConversionError(
            "Cannot convert empty string to Duration".to_string(),
        ));
    }
    let (number, unit) = split_duration_number_and_unit(trimmed)?;
    let value = number.parse::<u64>().map_err(|_| {
        DataConversionError::ConversionError(format!(
            "Cannot convert '{trimmed}' to Duration: invalid duration value"
        ))
    })?;
    let unit = match unit {
        Some(unit) => DurationUnit::from_suffix(unit).ok_or_else(|| {
            DataConversionError::ConversionError(format!(
                "Cannot convert '{trimmed}' to Duration: unsupported duration unit '{unit}'"
            ))
        })?,
        None => options.duration.unit,
    };
    unit.duration_from_u64(value).map_err(|message| {
        DataConversionError::ConversionError(format!(
            "Cannot convert '{trimmed}' to Duration: {message}"
        ))
    })
}

/// Splits duration text into number and optional unit suffix.
///
/// # Parameters
///
/// * `text` - Non-empty duration text.
///
/// # Returns
///
/// The number text and optional unit suffix.
///
/// # Errors
///
/// Returns [`DataConversionError::ConversionError`] when the numeric value is
/// missing.
fn split_duration_number_and_unit(text: &str) -> DataConversionResult<(&str, Option<&str>)> {
    let Some(split_at) = text.find(|ch: char| !ch.is_ascii_digit()) else {
        return Ok((text, None));
    };
    let (number, unit) = text.split_at(split_at);
    if number.is_empty() {
        return Err(DataConversionError::ConversionError(
            "Cannot convert duration string: duration value is missing".to_string(),
        ));
    }
    Ok((number, Some(unit)))
}

/// Converts a signed integer to [`Duration`] using configured duration options.
///
/// # Parameters
///
/// * `value` - Signed integer value to convert.
/// * `options` - Conversion options providing the source integer unit.
///
/// # Returns
///
/// The corresponding [`Duration`].
///
/// # Errors
///
/// Returns [`DataConversionError::ConversionError`] when the value is negative,
/// larger than `u64::MAX`, or overflows while applying the configured unit.
fn signed_integer_to_duration(
    value: i128,
    options: &DataConversionOptions,
) -> DataConversionResult<Duration> {
    if value < 0 {
        return Err(DataConversionError::ConversionError(format!(
            "Cannot convert {value} to Duration: duration value must be non-negative"
        )));
    }
    let value = range_check(value, 0i128, u64::MAX as i128, "Duration")?;
    options
        .duration
        .unit
        .duration_from_u64(value as u64)
        .map_err(|message| {
            DataConversionError::ConversionError(format!(
                "Cannot convert {value} to Duration: {message}"
            ))
        })
}

/// Converts an unsigned integer to [`Duration`] using configured duration options.
///
/// # Parameters
///
/// * `value` - Unsigned integer value to convert.
/// * `options` - Conversion options providing the source integer unit.
///
/// # Returns
///
/// The corresponding [`Duration`].
///
/// # Errors
///
/// Returns [`DataConversionError::ConversionError`] when the value is larger
/// than `u64::MAX` or overflows while applying the configured unit.
fn unsigned_integer_to_duration(
    value: u128,
    options: &DataConversionOptions,
) -> DataConversionResult<Duration> {
    let value = range_check(value, 0u128, u64::MAX as u128, "Duration")?;
    options
        .duration
        .unit
        .duration_from_u64(value as u64)
        .map_err(|message| {
            DataConversionError::ConversionError(format!(
                "Cannot convert {value} to Duration: {message}"
            ))
        })
}

/// Formats a [`Duration`] using configured duration options.
///
/// # Parameters
///
/// * `duration` - Duration to format.
/// * `options` - Conversion options providing the target unit and suffix
///   behavior.
///
/// # Returns
///
/// The formatted duration string. The numeric value is rounded half-up in the
/// configured unit.
fn format_duration(duration: Duration, options: &DataConversionOptions) -> String {
    let unit = options.duration.unit;
    let value = unit.rounded_units(duration);
    if options.duration.append_unit_suffix {
        format!("{}{}", value, unit.suffix())
    } else {
        value.to_string()
    }
}

/// Checks that a value is inside a closed range.
fn range_check<T>(value: T, min: T, max: T, target: &str) -> DataConversionResult<T>
where
    T: PartialOrd + fmt::Display + Copy,
{
    if value < min || value > max {
        Err(DataConversionError::ConversionError(format!(
            "Cannot convert value to {target}: value must be in range [{min}, {max}]: actual {value}"
        )))
    } else {
        Ok(value)
    }
}

/// Converts a finite floating point value to `i128`.
fn checked_float_to_i128(value: f64, source: &str, target: &str) -> DataConversionResult<i128> {
    if !value.is_finite() {
        return Err(DataConversionError::ConversionError(format!(
            "Cannot convert non-finite {source} value to {target}"
        )));
    }
    value.to_i128().ok_or_else(|| {
        DataConversionError::ConversionError(format!("{source} value out of {target} range"))
    })
}

/// Converts a `BigInt` to `f32` and rejects infinite conversion results.
fn checked_bigint_to_f32(value: &BigInt) -> DataConversionResult<f32> {
    let converted = value.to_f32().unwrap_or(f32::INFINITY);
    if converted.is_finite() {
        Ok(converted)
    } else {
        Err(DataConversionError::ConversionError(
            "BigInteger value out of f32 range".to_string(),
        ))
    }
}

/// Converts a `BigDecimal` to `f32` and rejects infinite conversion results.
fn checked_bigdecimal_to_f32(value: &BigDecimal) -> DataConversionResult<f32> {
    let converted = value.to_f32().unwrap_or(f32::INFINITY);
    if converted.is_finite() {
        Ok(converted)
    } else {
        Err(DataConversionError::ConversionError(
            "BigDecimal value out of f32 range".to_string(),
        ))
    }
}

/// Converts a `u128` to `f32` and rejects values that overflow to infinity.
fn checked_u128_to_f32(value: u128) -> DataConversionResult<f32> {
    let converted = value as f32;
    if converted.is_finite() {
        Ok(converted)
    } else {
        Err(DataConversionError::ConversionError(
            "u128 value out of f32 range".to_string(),
        ))
    }
}

/// Converts a `BigInt` to `f64` and rejects infinite conversion results.
fn checked_bigint_to_f64(value: &BigInt) -> DataConversionResult<f64> {
    let converted = value.to_f64().unwrap_or(f64::INFINITY);
    if converted.is_finite() {
        Ok(converted)
    } else {
        Err(DataConversionError::ConversionError(
            "BigInteger value out of f64 range".to_string(),
        ))
    }
}

/// Converts a `BigDecimal` to `f64` and rejects infinite conversion results.
fn checked_bigdecimal_to_f64(value: &BigDecimal) -> DataConversionResult<f64> {
    let converted = value.to_f64().unwrap_or(f64::INFINITY);
    if converted.is_finite() {
        Ok(converted)
    } else {
        Err(DataConversionError::ConversionError(
            "BigDecimal value out of f64 range".to_string(),
        ))
    }
}

/// Converts source values accepted by signed integer targets to `i128`.
fn convert_to_signed_integer(
    source: &DataConverter<'_>,
    options: &DataConversionOptions,
    target_type: DataType,
    target: &str,
) -> DataConversionResult<i128> {
    match source {
        DataConverter::Bool(value) => Ok(if *value { 1 } else { 0 }),
        DataConverter::Char(value) => Ok(*value as u32 as i128),
        DataConverter::Int8(value) => Ok(*value as i128),
        DataConverter::Int16(value) => Ok(*value as i128),
        DataConverter::Int32(value) => Ok(*value as i128),
        DataConverter::Int64(value) => Ok(*value as i128),
        DataConverter::Int128(value) => Ok(*value),
        DataConverter::IntSize(value) => Ok(*value as i128),
        DataConverter::UInt8(value) => Ok(*value as i128),
        DataConverter::UInt16(value) => Ok(*value as i128),
        DataConverter::UInt32(value) => Ok(*value as i128),
        DataConverter::UInt64(value) => Ok(*value as i128),
        DataConverter::UInt128(value) => {
            let checked = range_check(*value, 0u128, i128::MAX as u128, target)?;
            Ok(checked as i128)
        }
        DataConverter::UIntSize(value) => Ok(*value as i128),
        DataConverter::Float32(value) => checked_float_to_i128(*value as f64, "f32", target),
        DataConverter::Float64(value) => checked_float_to_i128(*value, "f64", target),
        DataConverter::String(value) => options
            .string
            .normalize(value.as_ref())?
            .parse::<i128>()
            .map_err(|_| {
                DataConversionError::ConversionError(format!(
                    "Cannot convert '{}' to {target}",
                    value.as_ref()
                ))
            }),
        DataConverter::Duration(value) => {
            let units = options.duration.unit.rounded_units(*value);
            let checked = range_check(units, 0u128, i128::MAX as u128, target)?;
            Ok(checked as i128)
        }
        DataConverter::BigInteger(value) => value.as_ref().to_i128().ok_or_else(|| {
            DataConversionError::ConversionError(format!("BigInteger value out of {target} range"))
        }),
        DataConverter::BigDecimal(value) => value.as_ref().to_i128().ok_or_else(|| {
            DataConversionError::ConversionError(format!(
                "BigDecimal value cannot be converted to {target}"
            ))
        }),
        DataConverter::Empty(_) => Err(DataConversionError::NoValue),
        _ => Err(source.unsupported(target_type)),
    }
}

/// Converts source values accepted by unsigned integer targets to `u128`.
fn convert_to_unsigned_integer(
    source: &DataConverter<'_>,
    options: &DataConversionOptions,
    target_type: DataType,
    target: &str,
) -> DataConversionResult<u128> {
    match source {
        DataConverter::Bool(value) => Ok(if *value { 1 } else { 0 }),
        DataConverter::Char(value) => Ok((*value as u32).into()),
        DataConverter::Int8(value) => {
            let checked = range_check(*value, 0i8, i8::MAX, target)?;
            Ok(checked as u128)
        }
        DataConverter::Int16(value) => {
            let checked = range_check(*value, 0i16, i16::MAX, target)?;
            Ok(checked as u128)
        }
        DataConverter::Int32(value) => {
            let checked = range_check(*value, 0i32, i32::MAX, target)?;
            Ok(checked as u128)
        }
        DataConverter::Int64(value) => {
            let checked = range_check(*value, 0i64, i64::MAX, target)?;
            Ok(checked as u128)
        }
        DataConverter::Int128(value) => {
            let checked = range_check(*value, 0i128, i128::MAX, target)?;
            Ok(checked as u128)
        }
        DataConverter::IntSize(value) => {
            let checked = range_check(*value, 0isize, isize::MAX, target)?;
            Ok(checked as u128)
        }
        DataConverter::UInt8(value) => Ok((*value).into()),
        DataConverter::UInt16(value) => Ok((*value).into()),
        DataConverter::UInt32(value) => Ok((*value).into()),
        DataConverter::UInt64(value) => Ok((*value).into()),
        DataConverter::UInt128(value) => Ok(*value),
        DataConverter::UIntSize(value) => Ok(*value as u128),
        DataConverter::String(value) => options
            .string
            .normalize(value.as_ref())?
            .parse::<u128>()
            .map_err(|_| {
                DataConversionError::ConversionError(format!(
                    "Cannot convert '{}' to {target}",
                    value.as_ref()
                ))
            }),
        DataConverter::Duration(value) => Ok(options.duration.unit.rounded_units(*value)),
        DataConverter::Empty(_) => Err(DataConversionError::NoValue),
        _ => Err(source.unsupported(target_type)),
    }
}

impl DataConvertTo<String> for DataConverter<'_> {
    /// Converts a supported source value to `String`.
    fn convert(&self, options: &DataConversionOptions) -> DataConversionResult<String> {
        match self {
            DataConverter::String(value) => options.string.normalize(value.as_ref()),
            DataConverter::Bool(value) => Ok(value.to_string()),
            DataConverter::Char(value) => Ok(value.to_string()),
            DataConverter::Int8(value) => Ok(value.to_string()),
            DataConverter::Int16(value) => Ok(value.to_string()),
            DataConverter::Int32(value) => Ok(value.to_string()),
            DataConverter::Int64(value) => Ok(value.to_string()),
            DataConverter::Int128(value) => Ok(value.to_string()),
            DataConverter::UInt8(value) => Ok(value.to_string()),
            DataConverter::UInt16(value) => Ok(value.to_string()),
            DataConverter::UInt32(value) => Ok(value.to_string()),
            DataConverter::UInt64(value) => Ok(value.to_string()),
            DataConverter::UInt128(value) => Ok(value.to_string()),
            DataConverter::IntSize(value) => Ok(value.to_string()),
            DataConverter::UIntSize(value) => Ok(value.to_string()),
            DataConverter::Float32(value) => Ok(value.to_string()),
            DataConverter::Float64(value) => Ok(value.to_string()),
            DataConverter::BigInteger(value) => Ok(value.to_string()),
            DataConverter::BigDecimal(value) => Ok(value.to_string()),
            DataConverter::Date(value) => Ok(value.to_string()),
            DataConverter::Time(value) => Ok(value.to_string()),
            DataConverter::DateTime(value) => Ok(value.to_string()),
            DataConverter::Instant(value) => Ok(value.to_rfc3339()),
            DataConverter::Duration(value) => Ok(format_duration(*value, options)),
            DataConverter::Url(value) => Ok(value.to_string()),
            DataConverter::StringMap(value) => match serde_json::to_string(value.as_ref()) {
                Ok(value) => Ok(value),
                Err(error) => Err(DataConversionError::JsonSerializationError(
                    error.to_string(),
                )),
            },
            DataConverter::Json(value) => match serde_json::to_string(value.as_ref()) {
                Ok(value) => Ok(value),
                Err(error) => Err(DataConversionError::JsonSerializationError(
                    error.to_string(),
                )),
            },
            DataConverter::Empty(_) => Err(DataConversionError::NoValue),
        }
    }
}

impl DataConvertTo<bool> for DataConverter<'_> {
    /// Converts accepted source values to `bool`.
    fn convert(&self, options: &DataConversionOptions) -> DataConversionResult<bool> {
        match self {
            DataConverter::Bool(value) => Ok(*value),
            DataConverter::Int8(value) => Ok(*value != 0),
            DataConverter::Int16(value) => Ok(*value != 0),
            DataConverter::Int32(value) => Ok(*value != 0),
            DataConverter::Int64(value) => Ok(*value != 0),
            DataConverter::Int128(value) => Ok(*value != 0),
            DataConverter::UInt8(value) => Ok(*value != 0),
            DataConverter::UInt16(value) => Ok(*value != 0),
            DataConverter::UInt32(value) => Ok(*value != 0),
            DataConverter::UInt64(value) => Ok(*value != 0),
            DataConverter::UInt128(value) => Ok(*value != 0),
            DataConverter::String(value) => parse_bool_string(value.as_ref(), options),
            DataConverter::Empty(_) => Err(DataConversionError::NoValue),
            _ => Err(self.unsupported(DataType::Bool)),
        }
    }
}

impl DataConvertTo<char> for DataConverter<'_> {
    /// Converts a character source to `char`.
    fn convert(&self, _options: &DataConversionOptions) -> DataConversionResult<char> {
        match self {
            DataConverter::Char(value) => Ok(*value),
            DataConverter::Empty(_) => Err(DataConversionError::NoValue),
            _ => Err(self.unsupported(DataType::Char)),
        }
    }
}

macro_rules! impl_signed_integer_converter {
    ($target_type:ty, $data_type:expr, $target_name:expr, $min:expr, $max:expr) => {
        impl DataConvertTo<$target_type> for DataConverter<'_> {
            #[inline]
            fn convert(
                &self,
                options: &DataConversionOptions,
            ) -> DataConversionResult<$target_type> {
                let value = convert_to_signed_integer(self, options, $data_type, $target_name)?;
                let checked = range_check(value, $min as i128, $max as i128, $target_name)?;
                Ok(checked as $target_type)
            }
        }
    };
}

impl_signed_integer_converter!(i8, DataType::Int8, "i8", i8::MIN, i8::MAX);
impl_signed_integer_converter!(i16, DataType::Int16, "i16", i16::MIN, i16::MAX);
impl_signed_integer_converter!(i32, DataType::Int32, "i32", i32::MIN, i32::MAX);
impl_signed_integer_converter!(i64, DataType::Int64, "i64", i64::MIN, i64::MAX);
impl_signed_integer_converter!(isize, DataType::IntSize, "isize", isize::MIN, isize::MAX);

impl DataConvertTo<i128> for DataConverter<'_> {
    /// Converts accepted source values to `i128`.
    #[inline]
    fn convert(&self, options: &DataConversionOptions) -> DataConversionResult<i128> {
        convert_to_signed_integer(self, options, DataType::Int128, "i128")
    }
}

macro_rules! impl_unsigned_integer_converter {
    ($target_type:ty, $data_type:expr, $target_name:expr, $max:expr) => {
        impl DataConvertTo<$target_type> for DataConverter<'_> {
            #[inline]
            fn convert(
                &self,
                options: &DataConversionOptions,
            ) -> DataConversionResult<$target_type> {
                let value = convert_to_unsigned_integer(self, options, $data_type, $target_name)?;
                let checked = range_check(
                    value,
                    <$target_type>::MIN as u128,
                    $max as u128,
                    $target_name,
                )?;
                Ok(checked as $target_type)
            }
        }
    };
}

impl_unsigned_integer_converter!(u8, DataType::UInt8, "u8", u8::MAX);
impl_unsigned_integer_converter!(u16, DataType::UInt16, "u16", u16::MAX);
impl_unsigned_integer_converter!(u32, DataType::UInt32, "u32", u32::MAX);
impl_unsigned_integer_converter!(u64, DataType::UInt64, "u64", u64::MAX);
impl_unsigned_integer_converter!(usize, DataType::UIntSize, "usize", usize::MAX);

impl DataConvertTo<u128> for DataConverter<'_> {
    /// Converts accepted source values to `u128`.
    #[inline]
    fn convert(&self, options: &DataConversionOptions) -> DataConversionResult<u128> {
        convert_to_unsigned_integer(self, options, DataType::UInt128, "u128")
    }
}

impl DataConvertTo<f32> for DataConverter<'_> {
    /// Converts accepted source values to `f32`.
    fn convert(&self, options: &DataConversionOptions) -> DataConversionResult<f32> {
        match self {
            DataConverter::Float32(value) => Ok(*value),
            DataConverter::Float64(value) => {
                if value.is_nan() || value.is_infinite() {
                    Ok(*value as f32)
                } else {
                    let checked = range_check(*value, f32::MIN as f64, f32::MAX as f64, "f32")?;
                    Ok(checked as f32)
                }
            }
            DataConverter::Bool(value) => Ok(if *value { 1.0 } else { 0.0 }),
            DataConverter::Char(value) => Ok(*value as u32 as f32),
            DataConverter::Int8(value) => Ok(*value as f32),
            DataConverter::Int16(value) => Ok(*value as f32),
            DataConverter::Int32(value) => Ok(*value as f32),
            DataConverter::Int64(value) => Ok(*value as f32),
            DataConverter::Int128(value) => Ok(*value as f32),
            DataConverter::IntSize(value) => Ok(*value as f32),
            DataConverter::UInt8(value) => Ok(*value as f32),
            DataConverter::UInt16(value) => Ok(*value as f32),
            DataConverter::UInt32(value) => Ok(*value as f32),
            DataConverter::UInt64(value) => Ok(*value as f32),
            DataConverter::UInt128(value) => checked_u128_to_f32(*value),
            DataConverter::UIntSize(value) => Ok(*value as f32),
            DataConverter::String(value) => options
                .string
                .normalize(value.as_ref())?
                .parse::<f32>()
                .map_err(|_| {
                    DataConversionError::ConversionError(format!(
                        "Cannot convert '{}' to f32",
                        value.as_ref()
                    ))
                }),
            DataConverter::BigInteger(value) => checked_bigint_to_f32(value.as_ref()),
            DataConverter::BigDecimal(value) => checked_bigdecimal_to_f32(value.as_ref()),
            DataConverter::Empty(_) => Err(DataConversionError::NoValue),
            _ => Err(self.unsupported(DataType::Float32)),
        }
    }
}

impl DataConvertTo<f64> for DataConverter<'_> {
    /// Converts accepted source values to `f64`.
    fn convert(&self, options: &DataConversionOptions) -> DataConversionResult<f64> {
        match self {
            DataConverter::Float64(value) => Ok(*value),
            DataConverter::Float32(value) => Ok(*value as f64),
            DataConverter::Bool(value) => Ok(if *value { 1.0 } else { 0.0 }),
            DataConverter::Char(value) => Ok(*value as u32 as f64),
            DataConverter::Int8(value) => Ok(*value as f64),
            DataConverter::Int16(value) => Ok(*value as f64),
            DataConverter::Int32(value) => Ok(*value as f64),
            DataConverter::Int64(value) => Ok(*value as f64),
            DataConverter::Int128(value) => Ok(*value as f64),
            DataConverter::IntSize(value) => Ok(*value as f64),
            DataConverter::UInt8(value) => Ok(*value as f64),
            DataConverter::UInt16(value) => Ok(*value as f64),
            DataConverter::UInt32(value) => Ok(*value as f64),
            DataConverter::UInt64(value) => Ok(*value as f64),
            DataConverter::UInt128(value) => Ok(*value as f64),
            DataConverter::UIntSize(value) => Ok(*value as f64),
            DataConverter::String(value) => options
                .string
                .normalize(value.as_ref())?
                .parse::<f64>()
                .map_err(|_| {
                    DataConversionError::ConversionError(format!(
                        "Cannot convert '{}' to f64",
                        value.as_ref()
                    ))
                }),
            DataConverter::BigInteger(value) => checked_bigint_to_f64(value.as_ref()),
            DataConverter::BigDecimal(value) => checked_bigdecimal_to_f64(value.as_ref()),
            DataConverter::Empty(_) => Err(DataConversionError::NoValue),
            _ => Err(self.unsupported(DataType::Float64)),
        }
    }
}

macro_rules! impl_strict_copy_converter {
    ($target_type:ty, $variant:ident, $data_type:expr) => {
        impl DataConvertTo<$target_type> for DataConverter<'_> {
            #[inline]
            fn convert(
                &self,
                _options: &DataConversionOptions,
            ) -> DataConversionResult<$target_type> {
                match self {
                    DataConverter::$variant(value) => Ok(*value),
                    DataConverter::Empty(_) => Err(DataConversionError::NoValue),
                    _ => Err(self.unsupported($data_type)),
                }
            }
        }
    };
}

macro_rules! impl_strict_cow_converter {
    ($target_type:ty, $variant:ident, $data_type:expr) => {
        impl DataConvertTo<$target_type> for DataConverter<'_> {
            #[inline]
            fn convert(
                &self,
                _options: &DataConversionOptions,
            ) -> DataConversionResult<$target_type> {
                match self {
                    DataConverter::$variant(value) => Ok(value.as_ref().clone()),
                    DataConverter::Empty(_) => Err(DataConversionError::NoValue),
                    _ => Err(self.unsupported($data_type)),
                }
            }
        }
    };
}

impl_strict_copy_converter!(NaiveDate, Date, DataType::Date);
impl_strict_copy_converter!(NaiveTime, Time, DataType::Time);
impl_strict_copy_converter!(NaiveDateTime, DateTime, DataType::DateTime);
impl_strict_copy_converter!(DateTime<Utc>, Instant, DataType::Instant);
impl_strict_cow_converter!(BigInt, BigInteger, DataType::BigInteger);
impl_strict_cow_converter!(BigDecimal, BigDecimal, DataType::BigDecimal);
impl_strict_cow_converter!(HashMap<String, String>, StringMap, DataType::StringMap);

impl DataConvertTo<Duration> for DataConverter<'_> {
    /// Converts accepted source values to [`Duration`].
    fn convert(&self, options: &DataConversionOptions) -> DataConversionResult<Duration> {
        match self {
            DataConverter::Duration(value) => Ok(*value),
            DataConverter::Int8(value) => signed_integer_to_duration((*value).into(), options),
            DataConverter::Int16(value) => signed_integer_to_duration((*value).into(), options),
            DataConverter::Int32(value) => signed_integer_to_duration((*value).into(), options),
            DataConverter::Int64(value) => signed_integer_to_duration((*value).into(), options),
            DataConverter::Int128(value) => signed_integer_to_duration(*value, options),
            DataConverter::IntSize(value) => signed_integer_to_duration(*value as i128, options),
            DataConverter::UInt8(value) => unsigned_integer_to_duration((*value).into(), options),
            DataConverter::UInt16(value) => unsigned_integer_to_duration((*value).into(), options),
            DataConverter::UInt32(value) => unsigned_integer_to_duration((*value).into(), options),
            DataConverter::UInt64(value) => unsigned_integer_to_duration((*value).into(), options),
            DataConverter::UInt128(value) => unsigned_integer_to_duration(*value, options),
            DataConverter::UIntSize(value) => unsigned_integer_to_duration(*value as u128, options),
            DataConverter::BigInteger(value) => value.as_ref().to_u64().map_or_else(
                || {
                    Err(DataConversionError::ConversionError(
                        "Cannot convert BigInteger to Duration: value must be a non-negative u64"
                            .to_string(),
                    ))
                },
                |value| {
                    options
                        .duration
                        .unit
                        .duration_from_u64(value)
                        .map_err(|message| {
                            DataConversionError::ConversionError(format!(
                                "Cannot convert BigInteger to Duration: {message}"
                            ))
                        })
                },
            ),
            DataConverter::String(value) => parse_duration_string(value.as_ref(), options),
            DataConverter::Empty(_) => Err(DataConversionError::NoValue),
            _ => Err(self.unsupported(DataType::Duration)),
        }
    }
}

impl DataConvertTo<Url> for DataConverter<'_> {
    /// Converts accepted source values to [`Url`].
    fn convert(&self, options: &DataConversionOptions) -> DataConversionResult<Url> {
        match self {
            DataConverter::Url(value) => Ok(value.as_ref().clone()),
            DataConverter::String(value) => {
                let value = options.string.normalize(value.as_ref())?;
                Url::parse(&value).map_err(|error| {
                    DataConversionError::ConversionError(format!(
                        "Cannot convert '{value}' to Url: {error}"
                    ))
                })
            }
            DataConverter::Empty(_) => Err(DataConversionError::NoValue),
            _ => Err(self.unsupported(DataType::Url)),
        }
    }
}

impl DataConvertTo<serde_json::Value> for DataConverter<'_> {
    /// Converts accepted source values to [`serde_json::Value`].
    fn convert(&self, options: &DataConversionOptions) -> DataConversionResult<serde_json::Value> {
        match self {
            DataConverter::Json(value) => Ok(value.as_ref().clone()),
            DataConverter::String(value) => {
                let value = options.string.normalize(value.as_ref())?;
                serde_json::from_str(&value).map_err(|error| {
                    DataConversionError::JsonDeserializationError(error.to_string())
                })
            }
            DataConverter::StringMap(value) => match serde_json::to_value(value.as_ref()) {
                Ok(value) => Ok(value),
                Err(error) => Err(DataConversionError::JsonSerializationError(
                    error.to_string(),
                )),
            },
            DataConverter::Empty(_) => Err(DataConversionError::NoValue),
            _ => Err(self.unsupported(DataType::Json)),
        }
    }
}
