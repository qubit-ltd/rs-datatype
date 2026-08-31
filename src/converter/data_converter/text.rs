// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Textual and temporal conversion implementations.

#[cfg(feature = "chrono")]
use chrono::DateTime;
#[cfg(feature = "chrono")]
use chrono::Datelike;
#[cfg(feature = "chrono")]
use chrono::NaiveDate;
#[cfg(feature = "chrono")]
use chrono::NaiveDateTime;
#[cfg(feature = "chrono")]
use chrono::NaiveTime;
#[cfg(feature = "chrono")]
use chrono::Utc;
#[cfg(feature = "url")]
use url::Url;

use super::DataConverter;
use super::duration::duration_text_parts;
#[cfg(feature = "json")]
use super::internal::CanonicalStringMap;
use super::string_source::normalize;
#[cfg(feature = "url")]
use super::structured::check_structured_text_limit;
use crate::converter::ConversionContext;
use crate::converter::ConversionStringWriter;
use crate::converter::DataConversionError;
use crate::converter::DataConversionTarget;
#[cfg(feature = "json")]
use crate::converter::DataFormat;
use crate::converter::InvalidValueReason;
use crate::datatype::DataType;

/// Validates the four-digit year required by canonical temporal text.
///
/// # Parameters
///
/// * `source` - Temporal source retained in a conversion error.
/// * `year` - Proleptic Gregorian year to validate.
///
/// # Returns
///
/// `Ok(())` when `year` is in the canonical `0000..=9999` range.
///
/// # Errors
///
/// Returns an out-of-range conversion error for a negative year or a year
/// greater than 9999.
#[cfg(feature = "chrono")]
#[inline]
fn validate_canonical_temporal_year(source: &DataConverter<'_>, year: i32) -> Result<(), DataConversionError> {
    if (0..=9_999).contains(&year) {
        Ok(())
    } else {
        Err(source.invalid(DataType::String, InvalidValueReason::OutOfRange))
    }
}

/// Formats source variants whose canonical string form is their `Display`
/// representation.
///
/// Sources requiring normalization, validation, or structured conversion are
/// left to the target implementation.
///
/// # Parameters
///
/// * `source` - Borrowed source representation to inspect.
///
/// # Returns
///
/// The formatted value for a directly displayable source, otherwise `None`.
fn format_display_source(
    source: &DataConverter<'_>,
    writer: &mut ConversionStringWriter<'_>,
) -> Result<bool, DataConversionError> {
    match source {
        DataConverter::Bool(value) => writer.write_display(value).map(|()| true),
        DataConverter::Char(value) => writer.write_display(value).map(|()| true),
        DataConverter::Int8(value) => writer.write_display(value).map(|()| true),
        DataConverter::Int16(value) => writer.write_display(value).map(|()| true),
        DataConverter::Int32(value) => writer.write_display(value).map(|()| true),
        DataConverter::Int64(value) => writer.write_display(value).map(|()| true),
        DataConverter::Int128(value) => writer.write_display(value).map(|()| true),
        DataConverter::UInt8(value) => writer.write_display(value).map(|()| true),
        DataConverter::UInt16(value) => writer.write_display(value).map(|()| true),
        DataConverter::UInt32(value) => writer.write_display(value).map(|()| true),
        DataConverter::UInt64(value) => writer.write_display(value).map(|()| true),
        DataConverter::UInt128(value) => writer.write_display(value).map(|()| true),
        DataConverter::Float32(value) => writer.write_display(value).map(|()| true),
        DataConverter::Float64(value) => writer.write_display(value).map(|()| true),
        #[cfg(feature = "big-integer")]
        DataConverter::BigInteger(value) => writer.write_display(value).map(|()| true),
        #[cfg(feature = "big-decimal")]
        DataConverter::BigDecimal(value) => writer.write_display(value).map(|()| true),
        #[cfg(feature = "chrono")]
        DataConverter::Time(value) => writer.write_display(value).map(|()| true),
        #[cfg(feature = "url")]
        DataConverter::Url(value) => writer.write_display(value).map(|()| true),
        #[cfg(feature = "json")]
        DataConverter::Json(value) => writer.write_display(value).map(|()| true),
        _ => Ok(false),
    }
}

/// Reports whether a source can be rendered directly through `Display`.
///
/// # Parameters
///
/// * `source` - Borrowed source representation to inspect.
///
/// # Returns
///
/// `true` when the source has a canonical direct display representation;
/// otherwise, `false` when conversion requires normalization or validation.
fn is_display_source(source: &DataConverter<'_>) -> bool {
    match source {
        DataConverter::Bool(_)
        | DataConverter::Char(_)
        | DataConverter::Int8(_)
        | DataConverter::Int16(_)
        | DataConverter::Int32(_)
        | DataConverter::Int64(_)
        | DataConverter::Int128(_)
        | DataConverter::UInt8(_)
        | DataConverter::UInt16(_)
        | DataConverter::UInt32(_)
        | DataConverter::UInt64(_)
        | DataConverter::UInt128(_)
        | DataConverter::Float32(_)
        | DataConverter::Float64(_) => true,
        #[cfg(feature = "big-integer")]
        DataConverter::BigInteger(_) => true,
        #[cfg(feature = "big-decimal")]
        DataConverter::BigDecimal(_) => true,
        #[cfg(feature = "chrono")]
        DataConverter::Time(_) => true,
        #[cfg(feature = "url")]
        DataConverter::Url(_) => true,
        #[cfg(feature = "json")]
        DataConverter::Json(_) => true,
        _ => false,
    }
}

/// Converts serialized JSON bytes into a UTF-8 String result.
///
/// # Parameters
///
/// * `from` - Runtime source type retained in the error.
/// * `bytes` - Serialized JSON bytes to decode as UTF-8.
///
/// # Returns
///
/// The decoded string when the bytes are valid UTF-8.
///
/// # Errors
///
/// Returns a JSON serialization invalid-value error when the bytes are not
/// valid UTF-8.
#[cfg(feature = "json")]
fn json_bytes_to_string(from: DataType, bytes: Vec<u8>) -> Result<String, DataConversionError> {
    String::from_utf8(bytes).map_err(|_| {
        DataConversionError::invalid(
            from,
            DataType::String,
            InvalidValueReason::Serialization {
                format: DataFormat::Json,
            },
        )
    })
}

impl DataConversionTarget for char {
    /// Converts a borrowed runtime value to one Unicode scalar value.
    ///
    /// # Parameters
    ///
    /// * `source` - Borrowed runtime value to convert.
    /// * `options` - String normalization policies.
    ///
    /// # Returns
    ///
    /// The source character or the only scalar in normalized string input.
    ///
    /// # Errors
    ///
    /// Returns a missing, unsupported, normalization, or invalid-syntax error.
    fn convert_from(
        source: &DataConverter<'_>,
        context: &mut ConversionContext<'_, '_>,
    ) -> Result<Self, DataConversionError> {
        let options = context.policy();
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
            DataConverter::Unset(_) => Err(source.missing(DataType::Char)),
            _ => Err(source.unsupported(DataType::Char)),
        }
    }
}

impl DataConversionTarget for String {
    /// Formats a borrowed runtime value as canonical text.
    ///
    /// # Parameters
    ///
    /// * `source` - Borrowed runtime value to format.
    /// * `session` - Shared conversion policies and output budget.
    ///
    /// # Returns
    ///
    /// The canonical string representation.
    ///
    /// # Errors
    ///
    /// Returns a missing, unsupported, normalization, range, precision, or
    /// serialization error as applicable to the source.
    fn convert_from(
        source: &DataConverter<'_>,
        context: &mut ConversionContext<'_, '_>,
    ) -> Result<Self, DataConversionError> {
        let options = context.policy();
        #[cfg(feature = "json")]
        if let DataConverter::Json(value) = source {
            return context
                .encode_json(value.as_ref())
                .and_then(|bytes| json_bytes_to_string(source.data_type(), bytes));
        }
        if let DataConverter::String(value) = source {
            let normalized = normalize(value, options, DataType::String)?;
            return context.write_string(|writer| writer.write_str(normalized));
        }
        if is_display_source(source) {
            return context.write_string(|writer| format_display_source(source, writer).map(|_| ()));
        }
        match source {
            DataConverter::Unset(_) => Err(source.missing(DataType::String)),
            #[cfg(feature = "chrono")]
            DataConverter::Date(value) => {
                validate_canonical_temporal_year(source, value.year())?;
                context.write_string(|writer| writer.write_display(&value.format("%Y-%m-%d")))
            }
            #[cfg(feature = "chrono")]
            DataConverter::DateTime(value) => {
                validate_canonical_temporal_year(source, value.year())?;
                context.write_string(|writer| writer.write_display(&value.format("%Y-%m-%dT%H:%M:%S%.f")))
            }
            #[cfg(feature = "chrono")]
            DataConverter::Instant(value) => {
                validate_canonical_temporal_year(source, value.year())?;
                context.write_string(|writer| writer.write_display(&value.format("%+")))
            }
            DataConverter::Duration(value) => {
                let (units, suffix) = duration_text_parts(*value, options)?;
                context.write_string(|writer| match suffix {
                    Some(suffix) => writer.write_fmt(format_args!("{units}{suffix}")),
                    None => writer.write_display(&units),
                })
            }
            #[cfg(feature = "json")]
            DataConverter::StringMap(value) => context
                .encode_json(&CanonicalStringMap { value })
                .and_then(|bytes| json_bytes_to_string(source.data_type(), bytes)),
            #[cfg(not(feature = "json"))]
            DataConverter::StringMap(_) => Err(source.unsupported(DataType::String)),
            _ => Err(source.unsupported(DataType::String)),
        }
    }

    /// Converts an owned source while reusing unchanged string storage.
    fn convert_owned(
        source: DataConverter<'_>,
        context: &mut ConversionContext<'_, '_>,
    ) -> Result<Self, DataConversionError> {
        if let DataConverter::String(value) = source {
            let normalized = normalize(value.as_ref(), context.policy(), DataType::String)?;
            if normalized.len() == value.len() {
                let result = value.into_owned();
                context.admit_output_bytes(result.len())?;
                return Ok(result);
            }
            return context.write_string(|writer| writer.write_str(normalized));
        }
        #[cfg(feature = "url")]
        if let DataConverter::Url(value) = source {
            let result: String = value.into_owned().into();
            context.admit_output_bytes(result.len())?;
            return Ok(result);
        }
        Self::convert_from(&source, context)
    }
}

/// Implements a canonical temporal target from text or an identical source.
#[cfg(feature = "chrono")]
macro_rules! impl_text_or_copy_target {
    ($target:ty, $variant:ident, $data_type:expr, $format:literal, $parser:expr) => {
        impl DataConversionTarget for $target {
            /// Converts a borrowed runtime value to the canonical temporal
            /// target.
            ///
            /// # Parameters
            ///
            /// * `source` - Borrowed runtime value to convert.
            /// * `options` - String normalization policies.
            ///
            /// # Returns
            ///
            /// The parsed or copied temporal value.
            ///
            /// # Errors
            ///
            /// Returns a missing, unsupported, normalization, or canonical
            /// temporal-syntax error.
            fn convert_from(
                source: &DataConverter<'_>,
                context: &mut ConversionContext<'_, '_>,
            ) -> Result<Self, DataConversionError> {
                let options = context.policy();
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
                    DataConverter::Unset(_) => Err(source.missing($data_type)),
                    _ => Err(source.unsupported($data_type)),
                }
            }
        }
    };
}

/// Reports whether a value has the canonical `YYYY-MM-DD` byte shape.
///
/// # Parameters
///
/// * `value` - Candidate date text.
///
/// # Returns
///
/// `true` when every field contains only ASCII digits and both separators are
/// in their canonical positions.
#[cfg(feature = "chrono")]
#[must_use]
fn has_canonical_date_shape(value: &str) -> bool {
    let bytes = value.as_bytes();
    bytes.len() == 10
        && bytes[4] == b'-'
        && bytes[7] == b'-'
        && bytes[..4]
            .iter()
            .chain(bytes[5..7].iter())
            .chain(bytes[8..].iter())
            .all(u8::is_ascii_digit)
}

/// Reports whether a value has the canonical `HH:MM:SS` byte shape.
///
/// # Parameters
///
/// * `value` - Candidate whole-second time text.
///
/// # Returns
///
/// `true` when every field contains only ASCII digits and both separators are
/// in their canonical positions.
#[cfg(feature = "chrono")]
#[must_use]
fn has_canonical_time_shape(value: &str) -> bool {
    let bytes = value.as_bytes();
    bytes.len() == 8
        && bytes[2] == b':'
        && bytes[5] == b':'
        && bytes[..2]
            .iter()
            .chain(bytes[3..5].iter())
            .chain(bytes[6..].iter())
            .all(u8::is_ascii_digit)
}

/// Parses the canonical date grammar without alternate padding.
///
/// # Parameters
///
/// * `value` - Date text to parse without normalization.
///
/// # Returns
///
/// `Some` for a valid ten-byte `YYYY-MM-DD` value, otherwise `None`.
#[cfg(feature = "chrono")]
#[inline]
fn parse_date(value: &str) -> Option<NaiveDate> {
    if !has_canonical_date_shape(value) {
        return None;
    }
    NaiveDate::parse_from_str(value, "%Y-%m-%d").ok()
}

/// Parses a canonical time with at most nine fractional digits.
///
/// # Parameters
///
/// * `value` - Time text to parse without normalization.
///
/// # Returns
///
/// `Some` for `HH:MM:SS` with an optional non-empty fraction of at most nine
/// digits, otherwise `None`.
#[cfg(feature = "chrono")]
fn parse_time(value: &str) -> Option<NaiveTime> {
    let (whole, fraction) = value
        .split_once('.')
        .map_or((value, None), |(whole, fraction)| (whole, Some(fraction)));
    if !has_canonical_time_shape(whole)
        || fraction.is_some_and(|fraction| {
            fraction.is_empty() || fraction.len() > 9 || !fraction.bytes().all(|byte| byte.is_ascii_digit())
        })
    {
        return None;
    }
    NaiveTime::parse_from_str(value, "%H:%M:%S%.f").ok()
}

/// Parses a canonical local date-time with a required `T` separator.
///
/// # Parameters
///
/// * `value` - Local date-time text to parse without normalization.
///
/// # Returns
///
/// `Some` when canonical date and time components form a valid local
/// date-time, otherwise `None`.
#[cfg(feature = "chrono")]
#[inline]
fn parse_datetime(value: &str) -> Option<NaiveDateTime> {
    let (date, time) = value.split_once('T')?;
    Some(NaiveDateTime::new(parse_date(date)?, parse_time(time)?))
}

#[cfg(feature = "chrono")]
impl_text_or_copy_target!(NaiveDate, Date, DataType::Date, "YYYY-MM-DD", parse_date);
#[cfg(feature = "chrono")]
impl_text_or_copy_target!(NaiveTime, Time, DataType::Time, "HH:MM:SS[.fraction]", parse_time);
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
    /// Converts a borrowed runtime value to a UTC instant.
    ///
    /// # Parameters
    ///
    /// * `source` - Borrowed runtime value to convert.
    /// * `options` - String normalization policies.
    ///
    /// # Returns
    ///
    /// The copied instant or parsed RFC 3339 value normalized to UTC.
    ///
    /// # Errors
    ///
    /// Returns a missing, unsupported, normalization, or RFC 3339 syntax error.
    fn convert_from(
        source: &DataConverter<'_>,
        context: &mut ConversionContext<'_, '_>,
    ) -> Result<Self, DataConversionError> {
        let options = context.policy();
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
            DataConverter::Unset(_) => Err(source.missing(DataType::Instant)),
            _ => Err(source.unsupported(DataType::Instant)),
        }
    }
}

#[cfg(feature = "url")]
impl DataConversionTarget for Url {
    /// Converts a borrowed runtime value to a URL.
    ///
    /// # Parameters
    ///
    /// * `source` - Borrowed runtime value to convert.
    /// * `options` - String normalization and structured text-limit policies.
    ///
    /// # Returns
    ///
    /// A cloned URL or one parsed from normalized string input.
    ///
    /// # Errors
    ///
    /// Returns a missing, unsupported, normalization, text-limit, or URL syntax
    /// error.
    fn convert_from(
        source: &DataConverter<'_>,
        context: &mut ConversionContext<'_, '_>,
    ) -> Result<Self, DataConversionError> {
        let options = context.policy();
        let limits = context.limits();
        match source {
            DataConverter::Url(value) => Ok(value.as_ref().clone()),
            DataConverter::String(value) => {
                let value = normalize(value, options, DataType::Url)?;
                check_structured_text_limit(value, DataType::String, DataType::Url, limits.structured())?;
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
            DataConverter::Unset(_) => Err(source.missing(DataType::Url)),
            _ => Err(source.unsupported(DataType::Url)),
        }
    }

    /// Converts a runtime value to a URL, consuming it when possible.
    ///
    /// # Parameters
    ///
    /// * `source` - Runtime value to consume.
    /// * `options` - String normalization and structured text-limit policies.
    ///
    /// # Returns
    ///
    /// The converted URL; an owned URL reuses its storage.
    ///
    /// # Errors
    ///
    /// Returns the same conversion errors as [`Self::convert_from`].
    #[inline(always)]
    fn convert_owned(
        source: DataConverter<'_>,
        context: &mut ConversionContext<'_, '_>,
    ) -> Result<Self, DataConversionError> {
        match source {
            DataConverter::Url(value) => Ok(value.into_owned()),
            source => Self::convert_from(&source, context),
        }
    }
}
