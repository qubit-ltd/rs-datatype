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

use super::data_conversion_error::DataConversionError;
use super::data_conversion_error_kind::InvalidValueReason;
use super::data_conversion_options::DataConversionOptions;
use super::data_convert_to::DataConvertTo;
use super::string_normalization_error::StringNormalizationError;
use crate::datatype::DataType;

mod boolean;
mod duration;
mod numeric;
mod source;
mod structured;
mod text;

/// A lightweight wrapper around every supported source value.
#[derive(Debug, Clone, PartialEq)]
pub enum DataConverter<'a> {
    /// Empty source with a known type.
    Empty(DataType),
    /// Boolean source.
    Bool(bool),
    /// Character source.
    Char(char),
    /// Signed integer sources.
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Int128(i128),
    /// Unsigned integer sources.
    UInt8(u8),
    UInt16(u16),
    UInt32(u32),
    UInt64(u64),
    UInt128(u128),
    /// Platform-width integer sources.
    IntSize(isize),
    UIntSize(usize),
    /// Floating-point sources.
    Float32(f32),
    Float64(f64),
    /// Arbitrary-precision numeric sources.
    BigInteger(Cow<'a, BigInt>),
    BigDecimal(Cow<'a, BigDecimal>),
    /// Text source.
    String(Cow<'a, str>),
    /// Temporal sources.
    Date(NaiveDate),
    Time(NaiveTime),
    DateTime(NaiveDateTime),
    Instant(DateTime<Utc>),
    Duration(Duration),
    /// Structured sources.
    Url(Cow<'a, Url>),
    StringMap(Cow<'a, HashMap<String, String>>),
    Json(Cow<'a, serde_json::Value>),
}

impl DataConverter<'_> {
    /// Converts this source using the shared default options.
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
    #[inline(always)]
    fn missing(&self, to: DataType) -> DataConversionError {
        DataConversionError::Missing {
            from: self.data_type(),
            to,
        }
    }

    /// Builds an unsupported-pair error for this source and target.
    #[inline(always)]
    fn unsupported(&self, to: DataType) -> DataConversionError {
        DataConversionError::Unsupported {
            from: self.data_type(),
            to,
        }
    }

    /// Builds an invalid-value error for this source and target.
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

/// Normalizes a textual source and attaches target context to policy errors.
fn normalize<'a>(
    value: &'a str,
    options: &DataConversionOptions,
    to: DataType,
) -> Result<&'a str, DataConversionError> {
    options
        .string
        .normalize(value)
        .map_err(|error| match error {
            StringNormalizationError::Missing => DataConversionError::Missing {
                from: DataType::String,
                to,
            },
            StringNormalizationError::BlankRejected => {
                DataConversionError::InvalidValue {
                    from: DataType::String,
                    to,
                    reason: InvalidValueReason::BlankRejected,
                }
            }
        })
}
