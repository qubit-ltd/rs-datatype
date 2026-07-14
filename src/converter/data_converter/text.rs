// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
// =============================================================================
//! Textual and temporal conversion implementations.

#[cfg(feature = "chrono")]
use chrono::{
    DateTime,
    NaiveDate,
    NaiveDateTime,
    NaiveTime,
    Utc,
};
#[cfg(feature = "url")]
use url::Url;

use super::DataConverter;
use super::duration::format_duration;
use super::string_source::normalize;
use crate::converter::{
    DataConversionError,
    DataConversionOptions,
    DataConvertTo,
    InvalidValueReason,
};
use crate::datatype::DataType;

impl DataConvertTo<char> for DataConverter<'_> {
    fn convert(
        &self,
        options: &DataConversionOptions,
    ) -> Result<char, DataConversionError> {
        match self {
            Self::Char(value) => Ok(*value),
            Self::String(value) => {
                let value = normalize(value, options, DataType::Char)?;
                let mut chars = value.chars();
                match (chars.next(), chars.next()) {
                    (Some(value), None) => Ok(value),
                    _ => Err(self.invalid(
                        DataType::Char,
                        InvalidValueReason::InvalidSyntax {
                            expected: "one Unicode scalar value",
                        },
                    )),
                }
            }
            Self::Empty(_) => Err(self.missing(DataType::Char)),
            _ => Err(self.unsupported(DataType::Char)),
        }
    }
}

impl DataConvertTo<String> for DataConverter<'_> {
    fn convert(
        &self,
        options: &DataConversionOptions,
    ) -> Result<String, DataConversionError> {
        match self {
            Self::Empty(_) => Err(self.missing(DataType::String)),
            Self::String(value) => {
                normalize(value, options, DataType::String).map(str::to_owned)
            }
            Self::Bool(value) => Ok(value.to_string()),
            Self::Char(value) => Ok(value.to_string()),
            Self::Int8(value) => Ok(value.to_string()),
            Self::Int16(value) => Ok(value.to_string()),
            Self::Int32(value) => Ok(value.to_string()),
            Self::Int64(value) => Ok(value.to_string()),
            Self::Int128(value) => Ok(value.to_string()),
            Self::UInt8(value) => Ok(value.to_string()),
            Self::UInt16(value) => Ok(value.to_string()),
            Self::UInt32(value) => Ok(value.to_string()),
            Self::UInt64(value) => Ok(value.to_string()),
            Self::UInt128(value) => Ok(value.to_string()),
            Self::Float32(value) => Ok(value.to_string()),
            Self::Float64(value) => Ok(value.to_string()),
            #[cfg(feature = "big-number")]
            Self::BigInteger(value) => Ok(value.to_string()),
            #[cfg(feature = "big-number")]
            Self::BigDecimal(value) => Ok(value.to_string()),
            #[cfg(feature = "chrono")]
            Self::Date(value) => Ok(value.format("%Y-%m-%d").to_string()),
            #[cfg(feature = "chrono")]
            Self::Time(value) => Ok(value.to_string()),
            #[cfg(feature = "chrono")]
            Self::DateTime(value) => {
                Ok(value.format("%Y-%m-%dT%H:%M:%S%.f").to_string())
            }
            #[cfg(feature = "chrono")]
            Self::Instant(value) => Ok(value.to_rfc3339()),
            Self::Duration(value) => format_duration(*value, options),
            #[cfg(feature = "url")]
            Self::Url(value) => Ok(value.to_string()),
            #[cfg(feature = "json")]
            Self::StringMap(value) => Ok(serde_json::Value::Object(
                value
                    .iter()
                    .map(|(key, value)| {
                        (key.clone(), serde_json::Value::String(value.clone()))
                    })
                    .collect(),
            )
            .to_string()),
            #[cfg(not(feature = "json"))]
            Self::StringMap(_) => Err(self.unsupported(DataType::String)),
            #[cfg(feature = "json")]
            Self::Json(value) => Ok(value.to_string()),
        }
    }
}

#[cfg(feature = "chrono")]
macro_rules! impl_text_or_copy_target {
    ($target:ty, $variant:ident, $data_type:expr, $format:literal, $parser:expr) => {
        impl DataConvertTo<$target> for DataConverter<'_> {
            fn convert(
                &self,
                options: &DataConversionOptions,
            ) -> Result<$target, DataConversionError> {
                match self {
                    Self::$variant(value) => Ok(*value),
                    Self::String(value) => {
                        let value = normalize(value, options, $data_type)?;
                        match ($parser)(value) {
                            Some(value) => Ok(value),
                            None => Err(self.invalid(
                                $data_type,
                                InvalidValueReason::InvalidSyntax {
                                    expected: $format,
                                },
                            )),
                        }
                    }
                    Self::Empty(_) => Err(self.missing($data_type)),
                    _ => Err(self.unsupported($data_type)),
                }
            }
        }
    };
}

/// Parses the canonical date grammar without alternate padding.
///
/// Returns `Some` only for a valid `YYYY-MM-DD` value of exactly ten bytes;
/// otherwise returns `None`.
#[cfg(feature = "chrono")]
fn parse_date(value: &str) -> Option<NaiveDate> {
    if value.len() == 10 {
        NaiveDate::parse_from_str(value, "%Y-%m-%d").ok()
    } else {
        None
    }
}

/// Parses a canonical time with at most nine fractional digits.
///
/// Returns `Some` for `HH:MM:SS` with an optional non-empty decimal fraction,
/// and `None` for invalid syntax, invalid clock values, or excess precision.
#[cfg(feature = "chrono")]
fn parse_time(value: &str) -> Option<NaiveTime> {
    let (whole, fraction) = value
        .split_once('.')
        .map_or((value, None), |(whole, fraction)| (whole, Some(fraction)));
    if whole.len() != 8
        || fraction.is_some_and(|fraction| {
            fraction.is_empty()
                || fraction.len() > 9
                || !fraction.bytes().all(|byte| byte.is_ascii_digit())
        })
    {
        return None;
    }
    NaiveTime::parse_from_str(value, "%H:%M:%S%.f").ok()
}

/// Parses a canonical local date-time with a required `T` separator.
///
/// Returns `Some` when both the date and time components use their canonical
/// grammars and form a valid local date-time; otherwise returns `None`.
#[cfg(feature = "chrono")]
fn parse_datetime(value: &str) -> Option<NaiveDateTime> {
    let (date, time) = value.split_once('T')?;
    parse_date(date)?;
    parse_time(time)?;
    NaiveDateTime::parse_from_str(value, "%Y-%m-%dT%H:%M:%S%.f").ok()
}

#[cfg(feature = "chrono")]
impl_text_or_copy_target!(
    NaiveDate,
    Date,
    DataType::Date,
    "YYYY-MM-DD",
    parse_date
);
#[cfg(feature = "chrono")]
impl_text_or_copy_target!(
    NaiveTime,
    Time,
    DataType::Time,
    "HH:MM:SS[.fraction]",
    parse_time
);
#[cfg(feature = "chrono")]
impl_text_or_copy_target!(
    NaiveDateTime,
    DateTime,
    DataType::DateTime,
    "YYYY-MM-DDTHH:MM:SS[.fraction]",
    parse_datetime
);

#[cfg(feature = "chrono")]
impl DataConvertTo<DateTime<Utc>> for DataConverter<'_> {
    fn convert(
        &self,
        options: &DataConversionOptions,
    ) -> Result<DateTime<Utc>, DataConversionError> {
        match self {
            Self::Instant(value) => Ok(*value),
            Self::String(value) => {
                let value = normalize(value, options, DataType::Instant)?;
                match DateTime::parse_from_rfc3339(value) {
                    Ok(value) => Ok(value.with_timezone(&Utc)),
                    Err(_) => Err(self.invalid(
                        DataType::Instant,
                        InvalidValueReason::InvalidSyntax {
                            expected: "RFC 3339 timestamp with offset",
                        },
                    )),
                }
            }
            Self::Empty(_) => Err(self.missing(DataType::Instant)),
            _ => Err(self.unsupported(DataType::Instant)),
        }
    }
}

#[cfg(feature = "url")]
impl DataConvertTo<Url> for DataConverter<'_> {
    fn convert(
        &self,
        options: &DataConversionOptions,
    ) -> Result<Url, DataConversionError> {
        match self {
            Self::Url(value) => Ok(value.as_ref().clone()),
            Self::String(value) => {
                let value = normalize(value, options, DataType::Url)?;
                match Url::parse(value) {
                    Ok(value) => Ok(value),
                    Err(_) => Err(self.invalid(
                        DataType::Url,
                        InvalidValueReason::InvalidSyntax {
                            expected: "absolute URL",
                        },
                    )),
                }
            }
            Self::Empty(_) => Err(self.missing(DataType::Url)),
            _ => Err(self.unsupported(DataType::Url)),
        }
    }
}
