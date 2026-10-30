// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
use std::borrow::Cow;
use std::collections::HashMap;
use std::time::Duration;

#[cfg(feature = "big-decimal")]
use bigdecimal::BigDecimal;
#[cfg(feature = "chrono")]
use chrono::DateTime;
#[cfg(feature = "chrono")]
use chrono::NaiveDate;
#[cfg(feature = "chrono")]
use chrono::NaiveDateTime;
#[cfg(feature = "chrono")]
use chrono::NaiveTime;
#[cfg(feature = "chrono")]
use chrono::Utc;
#[cfg(feature = "big-integer")]
use num_bigint::BigInt;
#[cfg(feature = "json")]
use serde_json::Value;
#[cfg(feature = "url")]
use url::Url;

use super::super::admitted_conversion::AdmittedConversion;
use super::super::conversion_context::ConversionContext;
use super::super::conversion_session::ConversionSession;
use super::super::data_conversion_target::DataConversionTarget;
use super::super::error::DataConversionError;
use super::super::error::InvalidValueReason;
use super::super::options::ConversionLimits;
use super::super::options::ConversionPolicy;
#[cfg(feature = "json")]
use super::structured::account_json_structure;
use super::structured::account_string_map_structure;
use crate::datatype::DataType;
use crate::datatype::for_each_data_type_mapping;

/// Expands the central data-type mapping table into a source-variant match.
macro_rules! data_converter_data_type_match {
    ($value:expr; $( $(#[$meta:meta])* ($variant:ident, $source:ty, $strategy:ident) ),+ $(,)?) => {
        match $value {
            Self::Unset(data_type) => *data_type,
            $(
                $(#[$meta])*
                Self::$variant(_) => DataType::$variant,
            )+
        }
    };
}

/// A borrowed-or-owned runtime source value for policy-driven conversion.
///
/// `DataConverter` erases the concrete Rust source type while preserving its
/// [`DataType`]. Construct one with a standard [`From`] conversion, then call
/// [`Self::to`] for the default profile or [`Self::to_with`] for an explicit
/// [`ConversionPolicy`] profile. `Cow`-backed variants borrow large values
/// when possible, so wrapping a string, big number, URL, map, or JSON value
/// does not require cloning it.
///
/// When the wrapper does not need to be reused, [`Self::into_target`] and
/// [`Self::into_target_with`] can move owned storage into targets that can
/// preserve it instead of cloning its allocation.
///
/// Use this type when the source type is known at run time, such as values read
/// from configuration, command-line arguments, or heterogeneous metadata. For
/// homogeneous collections, [`crate::DataConverters`] provides indexed batch
/// errors on top of the same conversion rules.
///
/// Conversion profiles control normalization and conversion choices; they do
/// not make every target use the same numeric lexer. In particular,
/// `BigDecimal` preserves the spellings accepted by its parser, including
/// `"12_34"`, `"1__2"`, `"12_"`, and `"1_2.5"`, under strict, lossy, and
/// environment-friendly profiles. Fixed-width integer targets and `BigInt`
/// reject these underscore spellings, but accept decimal and exponent notation.
/// Acceptance depends on the fractional-to-integer policy and the target
/// range: exact integral values are accepted by the strict profile, while
/// truncation requires an explicit policy. Successful conversion is not an
/// integer-only lexical validation. See the user guide for the complete
/// text-conversion contract.
///
/// # Type Parameters
///
/// * `'a` - Lifetime of borrowed string, big-number, URL, map, and JSON sources
///   stored by the converter.
///
/// # Examples
///
/// ```
/// use qubit_datatype::{
///     ConversionLimits,
///     ConversionPolicy,
///     DataConverter,
/// };
///
/// let source = DataConverter::from("42");
/// assert_eq!(source.to::<u16>(), Ok(42));
///
/// let lossy = ConversionPolicy::lossy();
/// let limits = ConversionLimits::default();
/// let value: u16 = DataConverter::from("3.9").to_with(&lossy, &limits).unwrap();
/// assert_eq!(value, 3);
/// ```
///
/// Integer results can come from decimal or exponent notation when the
/// represented value satisfies the selected fractional-to-integer policy:
///
/// ```
/// use qubit_datatype::{ConversionLimits, ConversionPolicy, DataConverter};
///
/// assert_eq!(DataConverter::from("1.0").to::<i32>(), Ok(1));
/// assert_eq!(DataConverter::from("1e2").to::<i32>(), Ok(100));
/// assert!(DataConverter::from("1.9").to::<i32>().is_err());
/// assert_eq!(
///     DataConverter::from("1.9").to_with::<i32>(
///         &ConversionPolicy::lossy(),
///         &ConversionLimits::default(),
///     ),
///     Ok(1),
/// );
/// ```
///
/// Platform-sized integers are intentionally not conversion sources:
///
/// ```compile_fail
/// use qubit_datatype::DataConverter;
///
/// let _ = DataConverter::from(1_usize);
/// ```
#[must_use]
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum DataConverter<'a> {
    /// Missing source whose declared type remains known.
    Unset(
        /// Declared type retained for the missing source.
        DataType,
    ),
    /// Boolean source value.
    Bool(
        /// Boolean source value.
        bool,
    ),
    /// Unicode scalar source value.
    Char(
        /// Unicode scalar source value.
        char,
    ),
    /// 8-bit signed integer source value.
    Int8(
        /// 8-bit signed integer source value.
        i8,
    ),
    /// 16-bit signed integer source value.
    Int16(
        /// 16-bit signed integer source value.
        i16,
    ),
    /// 32-bit signed integer source value.
    Int32(
        /// 32-bit signed integer source value.
        i32,
    ),
    /// 64-bit signed integer source value.
    Int64(
        /// 64-bit signed integer source value.
        i64,
    ),
    /// 128-bit signed integer source value.
    Int128(
        /// 128-bit signed integer source value.
        i128,
    ),
    /// 8-bit unsigned integer source value.
    UInt8(
        /// 8-bit unsigned integer source value.
        u8,
    ),
    /// 16-bit unsigned integer source value.
    UInt16(
        /// 16-bit unsigned integer source value.
        u16,
    ),
    /// 32-bit unsigned integer source value.
    UInt32(
        /// 32-bit unsigned integer source value.
        u32,
    ),
    /// 64-bit unsigned integer source value.
    UInt64(
        /// 64-bit unsigned integer source value.
        u64,
    ),
    /// 128-bit unsigned integer source value.
    UInt128(
        /// 128-bit unsigned integer source value.
        u128,
    ),
    /// 32-bit floating-point source value.
    Float32(
        /// 32-bit floating-point source value.
        f32,
    ),
    /// 64-bit floating-point source value.
    Float64(
        /// 64-bit floating-point source value.
        f64,
    ),
    /// Borrowed or owned arbitrary-precision integer source value.
    #[cfg(feature = "big-integer")]
    BigInteger(
        /// Borrowed or owned arbitrary-precision integer source value.
        Cow<'a, BigInt>,
    ),
    /// Borrowed or owned arbitrary-precision decimal source value.
    #[cfg(feature = "big-decimal")]
    BigDecimal(
        /// Borrowed or owned arbitrary-precision decimal source value.
        Cow<'a, BigDecimal>,
    ),
    /// Borrowed or owned UTF-8 text source value.
    String(
        /// Borrowed or owned UTF-8 text source value.
        Cow<'a, str>,
    ),
    /// Calendar date without a time zone.
    #[cfg(feature = "chrono")]
    Date(
        /// Calendar date without a time zone.
        NaiveDate,
    ),
    /// Clock time without a date or time zone.
    #[cfg(feature = "chrono")]
    Time(
        /// Clock time without a date or time zone.
        NaiveTime,
    ),
    /// Local date and time without a time zone.
    #[cfg(feature = "chrono")]
    DateTime(
        /// Local date and time without a time zone.
        NaiveDateTime,
    ),
    /// UTC instant.
    #[cfg(feature = "chrono")]
    Instant(
        /// UTC instant.
        DateTime<Utc>,
    ),
    /// Non-negative span represented by [`Duration`].
    Duration(
        /// Non-negative span source value.
        Duration,
    ),
    /// Borrowed or owned absolute URL source value.
    #[cfg(feature = "url")]
    Url(
        /// Borrowed or owned absolute URL source value.
        Cow<'a, Url>,
    ),
    /// Borrowed or owned string-to-string map source value.
    StringMap(
        /// Borrowed or owned string-to-string map source value.
        Cow<'a, HashMap<String, String>>,
    ),
    /// Borrowed or owned JSON source value.
    #[cfg(feature = "json")]
    Json(
        /// Borrowed or owned JSON source value.
        Cow<'a, Value>,
    ),
}

impl<'source> DataConverter<'source> {
    /// Returns the runtime type of the wrapped source.
    ///
    /// For [`Self::Unset`], this returns the declared type stored in the
    /// variant rather than a generic missing-value type.
    ///
    /// # Returns
    ///
    /// Returns the [`DataType`] corresponding to this enum variant.
    #[inline]
    pub const fn data_type(&self) -> DataType {
        for_each_data_type_mapping!(data_converter_data_type_match, self)
    }

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
    #[inline]
    pub fn to<T>(&self) -> Result<T, DataConversionError>
    where
        T: DataConversionTarget,
    {
        self.to_with(ConversionPolicy::default_ref(), ConversionLimits::default_ref())
    }

    /// Converts this source using an explicit policy and limits.
    ///
    /// # Type Parameters
    ///
    /// * `T` - Requested target type.
    ///
    /// # Parameters
    ///
    /// * `policy` - Policies for string normalization, numeric precision,
    ///   booleans, collections, and durations.
    /// * `limits` - Value-level and operation-level resource limits.
    ///
    /// # Returns
    ///
    /// Returns the converted target value.
    ///
    /// # Errors
    ///
    /// Returns a structured error containing source type, target type, and a
    /// value-free rejection reason.
    #[inline]
    pub fn to_with<T>(&self, policy: &ConversionPolicy, limits: &ConversionLimits) -> Result<T, DataConversionError>
    where
        T: DataConversionTarget,
    {
        let mut session = ConversionSession::new(policy, limits);
        self.to_in(&mut session)
    }

    /// Converts this source using an existing conversion session.
    ///
    /// # Type Parameters
    ///
    /// * `T` - Requested target type.
    ///
    /// # Parameters
    ///
    /// * `session` - Session whose policy and cumulative budgets are reused.
    ///
    /// The item budget is charged before input checks and target conversion.
    /// A successful item charge and any successfully charged input bytes remain
    /// consumed if a later input check or target conversion fails. A failed
    /// item-budget check adds no item; a failed individual input-byte charge
    /// adds no bytes and does not undo the preceding item charge.
    ///
    /// # Returns
    ///
    /// Returns the converted target value.
    ///
    /// # Errors
    ///
    /// Returns a structured error under the same conditions as [`Self::to`].
    #[inline]
    pub fn to_in<T>(&self, session: &mut ConversionSession<'_>) -> Result<T, DataConversionError>
    where
        T: DataConversionTarget,
    {
        session
            .consume_item()
            .map_err(|limit| DataConversionError::limit_exceeded(self.data_type(), T::DATA_TYPE, limit))?;
        self.charge_input_for_target(T::DATA_TYPE, session)?;

        let context = ConversionContext::new(session, self.data_type(), T::DATA_TYPE);
        T::convert(AdmittedConversion::new(self.borrowed(), context))
    }

    /// Consumes this source and converts it using the shared default options.
    ///
    /// Consuming the wrapper allows targets to reuse owned source allocations
    /// when the conversion can preserve them. Borrowed sources retain the same
    /// behavior as [`Self::to`].
    ///
    /// # Type Parameters
    ///
    /// * `T` - Requested target type.
    ///
    /// # Returns
    ///
    /// Returns the converted target value.
    ///
    /// # Errors
    ///
    /// Returns a structured error under the same conditions as [`Self::to`].
    #[inline]
    pub fn into_target<T>(self) -> Result<T, DataConversionError>
    where
        T: DataConversionTarget,
    {
        self.into_target_with(ConversionPolicy::default_ref(), ConversionLimits::default_ref())
    }

    /// Consumes this source and converts it using an explicit policy and
    /// limits.
    ///
    /// # Type Parameters
    ///
    /// * `T` - Requested target type.
    ///
    /// # Parameters
    ///
    /// * `policy` - Policies for string normalization, numeric precision,
    ///   booleans, collections, and durations.
    /// * `limits` - Value-level and operation-level resource limits.
    ///
    /// # Returns
    ///
    /// Returns the converted target value.
    ///
    /// # Errors
    ///
    /// Returns a structured error containing source type, target type, and a
    /// value-free rejection reason.
    #[inline]
    pub fn into_target_with<T>(
        self,
        policy: &ConversionPolicy,
        limits: &ConversionLimits,
    ) -> Result<T, DataConversionError>
    where
        T: DataConversionTarget,
    {
        let mut session = ConversionSession::new(policy, limits);
        self.into_target_in(&mut session)
    }

    /// Consumes this source and converts it using an existing session.
    ///
    /// # Type Parameters
    ///
    /// * `T` - Requested target type.
    ///
    /// # Parameters
    ///
    /// * `session` - Session whose policy and cumulative budgets are reused.
    ///
    /// The item budget is charged before input checks and target conversion.
    /// A successful item charge and any successfully charged input bytes remain
    /// consumed if a later input check or target conversion fails. A failed
    /// item-budget check adds no item; a failed individual input-byte charge
    /// adds no bytes and does not undo the preceding item charge.
    ///
    /// # Returns
    ///
    /// Returns the converted target value.
    ///
    /// # Errors
    ///
    /// Returns a structured error under the same conditions as [`Self::to`].
    #[inline]
    pub fn into_target_in<T>(self, session: &mut ConversionSession<'_>) -> Result<T, DataConversionError>
    where
        T: DataConversionTarget,
    {
        let from = self.data_type();
        session
            .consume_item()
            .map_err(|limit| DataConversionError::limit_exceeded(from, T::DATA_TYPE, limit))?;
        self.charge_input_for_target(T::DATA_TYPE, session)?;
        let context = ConversionContext::new(session, from, T::DATA_TYPE);
        T::convert(AdmittedConversion::new(self, context))
    }

    /// Charges textual input and supported structured values before a target
    /// parser can allocate or recurse.
    fn charge_input_for_target(
        &self,
        target: DataType,
        session: &mut ConversionSession<'_>,
    ) -> Result<(), DataConversionError> {
        match self {
            Self::String(value) => {
                session
                    .consume_input_bytes_usize(value.len())
                    .map_err(|limit| DataConversionError::measured_limit(self.data_type(), target, limit))?;
            }
            #[cfg(feature = "json")]
            Self::Json(value) if target == DataType::Json => {
                account_json_structure(value, self.data_type(), target, session)?;
            }
            Self::StringMap(value) if target == DataType::StringMap => {
                account_string_map_structure(value, 1, session)
                    .map_err(|limit| DataConversionError::measured_limit(self.data_type(), target, limit))?;
            }
            #[cfg(feature = "json")]
            Self::StringMap(value) if target == DataType::Json => {
                account_string_map_structure(value, 1, session)
                    .map_err(|limit| DataConversionError::measured_limit(self.data_type(), target, limit))?;
            }
            _ => {}
        }
        Ok(())
    }

    /// Reborrows heap-backed storage while copying only scalar values.
    #[inline]
    fn borrowed<'borrowed>(&'borrowed self) -> DataConverter<'borrowed>
    where
        'source: 'borrowed,
    {
        match self {
            Self::Unset(value) => Self::Unset(*value),
            Self::Bool(value) => Self::Bool(*value),
            Self::Char(value) => Self::Char(*value),
            Self::Int8(value) => Self::Int8(*value),
            Self::Int16(value) => Self::Int16(*value),
            Self::Int32(value) => Self::Int32(*value),
            Self::Int64(value) => Self::Int64(*value),
            Self::Int128(value) => Self::Int128(*value),
            Self::UInt8(value) => Self::UInt8(*value),
            Self::UInt16(value) => Self::UInt16(*value),
            Self::UInt32(value) => Self::UInt32(*value),
            Self::UInt64(value) => Self::UInt64(*value),
            Self::UInt128(value) => Self::UInt128(*value),
            Self::Float32(value) => Self::Float32(*value),
            Self::Float64(value) => Self::Float64(*value),
            Self::String(value) => DataConverter::String(Cow::Borrowed(value.as_ref())),
            #[cfg(feature = "big-integer")]
            Self::BigInteger(value) => DataConverter::BigInteger(Cow::Borrowed(value.as_ref())),
            #[cfg(feature = "big-decimal")]
            Self::BigDecimal(value) => DataConverter::BigDecimal(Cow::Borrowed(value.as_ref())),
            #[cfg(feature = "chrono")]
            Self::Date(value) => Self::Date(*value),
            #[cfg(feature = "chrono")]
            Self::Time(value) => Self::Time(*value),
            #[cfg(feature = "chrono")]
            Self::DateTime(value) => Self::DateTime(*value),
            #[cfg(feature = "chrono")]
            Self::Instant(value) => Self::Instant(*value),
            Self::Duration(value) => Self::Duration(*value),
            #[cfg(feature = "url")]
            Self::Url(value) => DataConverter::Url(Cow::Borrowed(value.as_ref())),
            Self::StringMap(value) => DataConverter::StringMap(Cow::Borrowed(value.as_ref())),
            #[cfg(feature = "json")]
            Self::Json(value) => DataConverter::Json(Cow::Borrowed(value.as_ref())),
        }
    }

    /// Builds a missing-value error for this source and target.
    ///
    /// # Parameters
    ///
    /// * `to` - Requested target type.
    ///
    /// # Returns
    ///
    /// A missing-value error whose source type comes from [`Self::data_type`].
    #[inline]
    pub(super) fn missing(&self, to: DataType) -> DataConversionError {
        DataConversionError::missing(self.data_type(), to)
    }

    /// Builds an unsupported-pair error for this source and target.
    ///
    /// # Parameters
    ///
    /// * `to` - Requested target type.
    ///
    /// # Returns
    ///
    /// An unsupported-pair error whose source type comes from
    /// [`Self::data_type`].
    #[inline]
    pub(super) fn unsupported(&self, to: DataType) -> DataConversionError {
        DataConversionError::unsupported(self.data_type(), to)
    }

    /// Builds an invalid-value error for this source and target.
    ///
    /// # Parameters
    ///
    /// * `to` - Requested target type.
    /// * `reason` - Stable, value-independent rejection reason.
    ///
    /// # Returns
    ///
    /// An invalid-value error recording this source's runtime type.
    #[inline]
    pub(super) fn invalid(&self, to: DataType, reason: InvalidValueReason) -> DataConversionError {
        DataConversionError::invalid(self.data_type(), to, reason)
    }
}
