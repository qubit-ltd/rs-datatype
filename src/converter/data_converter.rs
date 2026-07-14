// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
// =============================================================================
//! # Reusable Data Conversion
//!
//! Defines the public source wrapper and delegates target families to focused
//! conversion modules.

use std::borrow::Cow;
use std::collections::HashMap;
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
use url::Url;

use super::data_convert_to::DataConvertTo;
use super::error::{
    DataConversionError,
    InvalidValueReason,
};
use super::options::DataConversionOptions;
use crate::datatype::DataType;

mod boolean;
mod duration;
mod numeric;
mod source;
mod string_source;
mod structured;
mod text;

/// A borrowed-or-owned runtime source value for policy-driven conversion.
///
/// `DataConverter` erases the concrete Rust source type while preserving its
/// [`DataType`]. Construct one with a standard [`From`] conversion, then call
/// [`Self::to`] for the default profile or [`Self::to_with`] for an explicit
/// [`DataConversionOptions`] profile. `Cow`-backed variants borrow large values
/// when possible, so wrapping a string, big number, URL, map, or JSON value
/// does not require cloning it.
///
/// Use this type when the source type is known at run time, such as values read
/// from configuration, command-line arguments, or heterogeneous metadata. For
/// homogeneous collections, [`super::DataConverters`] provides indexed batch
/// errors on top of the same conversion rules.
///
/// # Examples
///
/// ```
/// use qubit_datatype::{
///     DataConversionOptions,
///     DataConverter,
///     NumericConversionPolicy,
/// };
///
/// let source = DataConverter::from("42");
/// assert_eq!(source.to::<u16>(), Ok(42));
///
/// let lossy = DataConversionOptions::default()
///     .with_numeric_policy(NumericConversionPolicy::Lossy);
/// assert_eq!(DataConverter::from("3.9").to_with::<u16>(&lossy), Ok(3));
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum DataConverter<'a> {
    /// Missing source whose declared type remains known.
    Empty(DataType),
    /// Boolean source value.
    Bool(bool),
    /// Unicode scalar source value.
    Char(char),
    /// 8-bit signed integer source value.
    Int8(i8),
    /// 16-bit signed integer source value.
    Int16(i16),
    /// 32-bit signed integer source value.
    Int32(i32),
    /// 64-bit signed integer source value.
    Int64(i64),
    /// 128-bit signed integer source value.
    Int128(i128),
    /// 8-bit unsigned integer source value.
    UInt8(u8),
    /// 16-bit unsigned integer source value.
    UInt16(u16),
    /// 32-bit unsigned integer source value.
    UInt32(u32),
    /// 64-bit unsigned integer source value.
    UInt64(u64),
    /// 128-bit unsigned integer source value.
    UInt128(u128),
    /// Pointer-width signed integer source value.
    IntSize(isize),
    /// Pointer-width unsigned integer source value.
    UIntSize(usize),
    /// 32-bit floating-point source value.
    Float32(f32),
    /// 64-bit floating-point source value.
    Float64(f64),
    /// Borrowed or owned arbitrary-precision integer source value.
    BigInteger(Cow<'a, BigInt>),
    /// Borrowed or owned arbitrary-precision decimal source value.
    BigDecimal(Cow<'a, BigDecimal>),
    /// Borrowed or owned UTF-8 text source value.
    String(Cow<'a, str>),
    /// Calendar date without a time zone.
    Date(NaiveDate),
    /// Clock time without a date or time zone.
    Time(NaiveTime),
    /// Local date and time without a time zone.
    DateTime(NaiveDateTime),
    /// UTC instant.
    Instant(DateTime<Utc>),
    /// Non-negative span represented by [`Duration`].
    Duration(Duration),
    /// Borrowed or owned absolute URL source value.
    Url(Cow<'a, Url>),
    /// Borrowed or owned string-to-string map source value.
    StringMap(Cow<'a, HashMap<String, String>>),
    /// Borrowed or owned JSON source value.
    Json(Cow<'a, serde_json::Value>),
}

impl DataConverter<'_> {
    /// Converts this source using the shared default options.
    ///
    /// # Type Parameters
    ///
    /// * `T` - Requested target type. A conversion implementation must exist
    ///   for the source wrapper and `T`.
    ///
    /// # Returns
    ///
    /// Returns the converted target value.
    ///
    /// # Errors
    ///
    /// Returns a structured error when the value is missing, the type pair is
    /// unsupported, or the source value violates the target contract.
    #[inline(always)]
    pub fn to<T>(&self) -> Result<T, DataConversionError>
    where
        Self: DataConvertTo<T>,
    {
        self.to_with(DataConversionOptions::default_ref())
    }

    /// Converts this source using explicit options.
    ///
    /// # Type Parameters
    ///
    /// * `T` - Requested target type.
    ///
    /// # Parameters
    ///
    /// * `options` - Policies for string normalization, numeric precision,
    ///   booleans, collections, and durations.
    ///
    /// # Returns
    ///
    /// Returns the converted target value.
    ///
    /// # Errors
    ///
    /// Returns a structured error containing source type, target type, and a
    /// value-free rejection reason.
    #[inline(always)]
    pub fn to_with<T>(
        &self,
        options: &DataConversionOptions,
    ) -> Result<T, DataConversionError>
    where
        Self: DataConvertTo<T>,
    {
        <Self as DataConvertTo<T>>::convert(self, options)
    }

    /// Returns the runtime type of the wrapped source.
    ///
    /// For [`Self::Empty`], this returns the declared type stored in the
    /// variant rather than a generic missing-value type.
    ///
    /// # Returns
    ///
    /// Returns the [`DataType`] corresponding to this enum variant.
    #[inline]
    pub const fn data_type(&self) -> DataType {
        match self {
            Self::Empty(data_type) => *data_type,
            Self::Bool(_) => DataType::Bool,
            Self::Char(_) => DataType::Char,
            Self::Int8(_) => DataType::Int8,
            Self::Int16(_) => DataType::Int16,
            Self::Int32(_) => DataType::Int32,
            Self::Int64(_) => DataType::Int64,
            Self::Int128(_) => DataType::Int128,
            Self::UInt8(_) => DataType::UInt8,
            Self::UInt16(_) => DataType::UInt16,
            Self::UInt32(_) => DataType::UInt32,
            Self::UInt64(_) => DataType::UInt64,
            Self::UInt128(_) => DataType::UInt128,
            Self::IntSize(_) => DataType::IntSize,
            Self::UIntSize(_) => DataType::UIntSize,
            Self::Float32(_) => DataType::Float32,
            Self::Float64(_) => DataType::Float64,
            Self::BigInteger(_) => DataType::BigInteger,
            Self::BigDecimal(_) => DataType::BigDecimal,
            Self::String(_) => DataType::String,
            Self::Date(_) => DataType::Date,
            Self::Time(_) => DataType::Time,
            Self::DateTime(_) => DataType::DateTime,
            Self::Instant(_) => DataType::Instant,
            Self::Duration(_) => DataType::Duration,
            Self::Url(_) => DataType::Url,
            Self::StringMap(_) => DataType::StringMap,
            Self::Json(_) => DataType::Json,
        }
    }

    /// Builds a missing-value error for this source and target.
    ///
    /// `to` is the requested target type. The returned error derives its
    /// source type from [`Self::data_type`].
    #[inline(always)]
    fn missing(&self, to: DataType) -> DataConversionError {
        DataConversionError::Missing {
            from: self.data_type(),
            to,
        }
    }

    /// Builds an unsupported-pair error for this source and target.
    ///
    /// `to` is the requested target type. The returned error derives its
    /// source type from [`Self::data_type`].
    #[inline(always)]
    fn unsupported(&self, to: DataType) -> DataConversionError {
        DataConversionError::Unsupported {
            from: self.data_type(),
            to,
        }
    }

    /// Builds an invalid-value error for this source and target.
    ///
    /// `to` identifies the requested target, while `reason` explains the
    /// value-independent rejection. The returned error records this source's
    /// runtime type.
    #[inline(always)]
    fn invalid(
        &self,
        to: DataType,
        reason: InvalidValueReason,
    ) -> DataConversionError {
        DataConversionError::InvalidValue {
            from: self.data_type(),
            to,
            reason,
        }
    }
}
