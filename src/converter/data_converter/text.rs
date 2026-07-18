// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Textual and temporal conversion implementations.

#[cfg(feature = "chrono")]
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
#[cfg(feature = "url")]
use url::Url;

use super::DataConverter;
use super::duration::format_duration;
use super::string_source::normalize;
use crate::converter::{
    DataConversionError, DataConversionOptions, DataConversionTarget, InvalidValueReason,
};
use crate::datatype::DataType;

impl DataConversionTarget for char {
    fn convert_from(
        source: &DataConverter<'_>,
        options: &DataConversionOptions,
    ) -> Result<Self, DataConversionError> {
        match source {
            DataConverter::Char(value) => Ok(*value),
            DataConverter::String(value) => {
                let value = normalize(value, options, DataType::Char)?;
                let mut chars = value.chars();
                match (chars.next(), chars.next()) {
                    (Some(value), None) => Ok(value),
                    _ => Err(source.invalid(
                        DataType::Char,
                        InvalidValueReason::InvalidSyntax {
                            expected: "one Unicode scalar value",
                        },
                    )),
                }
            }
            DataConverter::Empty(_) => Err(source.missing(DataType::Char)),
            _ => Err(source.unsupported(DataType::Char)),
        }
    }
}

impl DataConversionTarget for String {
    fn convert_from(
        source: &DataConverter<'_>,
        options: &DataConversionOptions,
    ) -> Result<Self, DataConversionError> {
        match source {
            DataConverter::Empty(_) => Err(source.missing(DataType::String)),
            DataConverter::String(value) => {
                normalize(value, options, DataType::String).map(str::to_owned)
            }
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
            DataConverter::Float32(value) => Ok(value.to_string()),
            DataConverter::Float64(value) => Ok(value.to_string()),
            #[cfg(feature = "big-integer")]
            DataConverter::BigInteger(value) => Ok(value.to_string()),
            #[cfg(feature = "big-decimal")]
            DataConverter::BigDecimal(value) => Ok(value.to_string()),
            #[cfg(feature = "chrono")]
            DataConverter::Date(value) => Ok(value.format("%Y-%m-%d").to_string()),
            #[cfg(feature = "chrono")]
            DataConverter::Time(value) => Ok(value.to_string()),
            #[cfg(feature = "chrono")]
            DataConverter::DateTime(value) => Ok(value.format("%Y-%m-%dT%H:%M:%S%.f").to_string()),
            #[cfg(feature = "chrono")]
            DataConverter::Instant(value) => Ok(value.to_rfc3339()),
            DataConverter::Duration(value) => format_duration(*value, options),
            #[cfg(feature = "url")]
            DataConverter::Url(value) => Ok(value.to_string()),
            #[cfg(feature = "json")]
            DataConverter::StringMap(value) => Ok(serde_json::Value::Object(
                value
                    .iter()
                    .map(|(key, value)| (key.clone(), serde_json::Value::String(value.clone())))
                    .collect(),
            )
            .to_string()),
            #[cfg(not(feature = "json"))]
            DataConverter::StringMap(_) => Err(source.unsupported(DataType::String)),
            #[cfg(feature = "json")]
            DataConverter::Json(value) => Ok(value.to_string()),
        }
    }
}

#[cfg(feature = "chrono")]
macro_rules! impl_text_or_copy_target {
    ($target:ty, $variant:ident, $data_type:expr, $format:literal, $parser:expr) => {
        impl DataConversionTarget for $target {
            fn convert_from(
                source: &DataConverter<'_>,
                options: &DataConversionOptions,
            ) -> Result<Self, DataConversionError> {
                match source {
                    DataConverter::$variant(value) => Ok(*value),
                    DataConverter::String(value) => {
                        let value = normalize(value, options, $data_type)?;
                        match ($parser)(value) {
                            Some(value) => Ok(value),
                            None => Err(source.invalid(
                                $data_type,
                                InvalidValueReason::InvalidSyntax { expected: $format },
                            )),
                        }
                    }
                    DataConverter::Empty(_) => Err(source.missing($data_type)),
                    _ => Err(source.unsupported($data_type)),
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
impl_text_or_copy_target!(NaiveDate, Date, DataType::Date, "YYYY-MM-DD", parse_date);
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
impl DataConversionTarget for DateTime<Utc> {
    fn convert_from(
        source: &DataConverter<'_>,
        options: &DataConversionOptions,
    ) -> Result<Self, DataConversionError> {
        match source {
            DataConverter::Instant(value) => Ok(*value),
            DataConverter::String(value) => {
                let value = normalize(value, options, DataType::Instant)?;
                match DateTime::parse_from_rfc3339(value) {
                    Ok(value) => Ok(value.with_timezone(&Utc)),
                    Err(_) => Err(source.invalid(
                        DataType::Instant,
                        InvalidValueReason::InvalidSyntax {
                            expected: "RFC 3339 timestamp with offset",
                        },
                    )),
                }
            }
            DataConverter::Empty(_) => Err(source.missing(DataType::Instant)),
            _ => Err(source.unsupported(DataType::Instant)),
        }
    }
}

#[cfg(feature = "url")]
impl DataConversionTarget for Url {
    fn convert_from(
        source: &DataConverter<'_>,
        options: &DataConversionOptions,
    ) -> Result<Self, DataConversionError> {
        match source {
            DataConverter::Url(value) => Ok(value.as_ref().clone()),
            DataConverter::String(value) => {
                let value = normalize(value, options, DataType::Url)?;
                match Url::parse(value) {
                    Ok(value) => Ok(value),
                    Err(_) => Err(source.invalid(
                        DataType::Url,
                        InvalidValueReason::InvalidSyntax {
                            expected: "absolute URL",
                        },
                    )),
                }
            }
            DataConverter::Empty(_) => Err(source.missing(DataType::Url)),
            _ => Err(source.unsupported(DataType::Url)),
        }
    }
}
