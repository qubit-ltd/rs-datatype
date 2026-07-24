// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Duration conversion implementations.

use std::time::Duration;

#[cfg(feature = "big-integer")]
use num_bigint::Sign;
#[cfg(feature = "big-integer")]
use num_traits::ToPrimitive;

use super::DataConverter;
use super::numeric::{
    duration_to_u128,
    source_to_integer,
};
use super::string_source::normalize;
use crate::converter::{
    ConversionLimit,
    DataConversionError,
    DataConversionOptions,
    DataConversionTarget,
    InvalidValueReason,
};
use crate::datatype::DataType;
use crate::duration::{
    DurationParseError,
    DurationTextOptions,
    DurationUnitParseMode,
    SuffixlessDurationPolicy,
    parse_duration_text,
};

/// Converts a duration unit count to a duration.
///
/// # Parameters
///
/// * `value` - Sign and magnitude of the duration unit count.
/// * `from` - Source type retained in conversion errors.
/// * `options` - Duration unit and numeric conversion policies.
///
/// # Returns
///
/// The exact represented duration.
///
/// # Errors
///
/// Returns an invalid-value error for negative or out-of-range counts.
fn integer_to_duration(
    value: (bool, u128),
    from: DataType,
    options: &DataConversionOptions,
) -> Result<Duration, DataConversionError> {
    let (negative, value) = value;
    if negative {
        return Err(DataConversionError::invalid(
            from,
            DataType::Duration,
            InvalidValueReason::NegativeDuration,
        ));
    }
    match options
        .duration()
        .numeric_input_unit()
        .duration_from_u128(value)
    {
        Ok(duration) => Ok(duration),
        Err(_) => Err(DataConversionError::invalid(
            from,
            DataType::Duration,
            InvalidValueReason::OutOfRange,
        )),
    }
}

/// Builds the public duration parser options used by conversion.
///
/// # Parameters
///
/// * `options` - Conversion policies supplying suffix and text-limit rules.
///
/// # Returns
///
/// Duration text options equivalent to the converter configuration.
#[inline(always)]
fn duration_text_options(
    options: &DataConversionOptions,
) -> DurationTextOptions {
    DurationTextOptions::new(
        options.duration().suffixless_string_policy(),
        options.duration().unit_parse_mode(),
    )
    .with_max_text_bytes(options.duration().max_text_bytes())
}

/// Selects the expected duration grammar for an invalid source value.
///
/// # Parameters
///
/// * `value` - Normalized duration text that failed parsing.
/// * `options` - Duration suffix and unit parsing policies.
///
/// # Returns
///
/// A static grammar distinguishing required and optional suffixes as well as
/// strict and lenient unit vocabularies.
fn expected_duration_syntax(
    value: &str,
    options: &DataConversionOptions,
) -> &'static str {
    let suffix_required = !value.is_empty()
        && value.bytes().all(|byte| byte.is_ascii_digit())
        && options.duration().suffixless_string_policy()
            == SuffixlessDurationPolicy::Reject;
    match (suffix_required, options.duration().unit_parse_mode()) {
        (true, DurationUnitParseMode::Strict) => {
            "[0-9]+(ns|us|µs|μs|ms|s|min|h|d)"
        }
        (true, DurationUnitParseMode::Lenient) => {
            "[0-9]+(ns|us|µs|μs|ms|s|min|m|h|d)"
        }
        (false, DurationUnitParseMode::Strict) => {
            "[0-9]+(ns|us|µs|μs|ms|s|min|h|d)?"
        }
        (false, DurationUnitParseMode::Lenient) => {
            "[0-9]+(ns|us|µs|μs|ms|s|min|m|h|d)?"
        }
    }
}

/// Maps a duration parser error into the converter error model.
///
/// # Parameters
///
/// * `error` - Structured parsing failure to translate.
/// * `value` - Normalized source text used only to select syntax guidance.
/// * `options` - Conversion policies that determine expected syntax.
///
/// # Returns
///
/// A value-free conversion error preserving the limit, syntax, unit, or range
/// category. Arbitrary source suffixes are never retained.
fn map_duration_parse_error(
    error: DurationParseError,
    value: &str,
    options: &DataConversionOptions,
) -> DataConversionError {
    let to = DataType::Duration;
    match error {
        DurationParseError::LimitExceeded { maximum } => {
            DataConversionError::limit_exceeded(
                DataType::String,
                to,
                ConversionLimit::DurationTextBytes { maximum },
            )
        }
        DurationParseError::InvalidSyntax => DataConversionError::invalid(
            DataType::String,
            to,
            InvalidValueReason::InvalidSyntax {
                expected: expected_duration_syntax(value, options),
            },
        ),
        DurationParseError::NonCanonicalUnit { canonical } => {
            DataConversionError::invalid(
                DataType::String,
                to,
                InvalidValueReason::NonCanonicalDurationUnit {
                    canonical: canonical.to_owned(),
                },
            )
        }
        DurationParseError::UnsupportedUnit => DataConversionError::invalid(
            DataType::String,
            to,
            InvalidValueReason::UnsupportedDurationUnit,
        ),
        DurationParseError::OutOfRange => DataConversionError::invalid(
            DataType::String,
            to,
            InvalidValueReason::OutOfRange,
        ),
    }
}

/// Parses the canonical duration grammar.
///
/// # Parameters
///
/// * `value` - Duration text to normalize and parse.
/// * `options` - String normalization and duration parsing policies.
///
/// # Returns
///
/// The represented non-negative duration.
///
/// # Errors
///
/// Returns contextual conversion errors for normalization, syntax, unit, and
/// range failures.
#[inline(always)]
fn parse_duration(
    value: &str,
    options: &DataConversionOptions,
) -> Result<Duration, DataConversionError> {
    let value = normalize(value, options, DataType::Duration)?;
    parse_duration_text(value, &duration_text_options(options))
        .map_err(|error| map_duration_parse_error(error, value, options))
}

impl DataConversionTarget for Duration {
    /// Converts a borrowed runtime value to a duration.
    ///
    /// # Parameters
    ///
    /// * `source` - Borrowed runtime value to convert.
    /// * `options` - String, numeric-unit, and duration parsing policies.
    ///
    /// # Returns
    ///
    /// The represented non-negative duration.
    ///
    /// # Errors
    ///
    /// Returns a missing, unsupported, negative, syntax, unit, range, or
    /// configured resource-limit error as applicable to the source.
    fn convert_from(
        source: &DataConverter<'_>,
        options: &DataConversionOptions,
    ) -> Result<Self, DataConversionError> {
        match source {
            DataConverter::Duration(value) => Ok(*value),
            DataConverter::String(value) => parse_duration(value, options),
            DataConverter::Unset(_) => Err(source.missing(DataType::Duration)),
            DataConverter::Int8(_)
            | DataConverter::Int16(_)
            | DataConverter::Int32(_)
            | DataConverter::Int64(_)
            | DataConverter::Int128(_)
            | DataConverter::UInt8(_)
            | DataConverter::UInt16(_)
            | DataConverter::UInt32(_)
            | DataConverter::UInt64(_)
            | DataConverter::UInt128(_) => integer_to_duration(
                source_to_integer(source, options, DataType::Duration)?,
                source.data_type(),
                options,
            ),
            #[cfg(feature = "big-integer")]
            DataConverter::BigInteger(value) => {
                if value.sign() == Sign::Minus {
                    return Err(source.invalid(
                        DataType::Duration,
                        InvalidValueReason::NegativeDuration,
                    ));
                }
                let Some(value) = value.to_u128() else {
                    return Err(source.invalid(
                        DataType::Duration,
                        InvalidValueReason::OutOfRange,
                    ));
                };
                integer_to_duration(
                    (false, value),
                    DataType::BigInteger,
                    options,
                )
            }
            _ => Err(source.unsupported(DataType::Duration)),
        }
    }
}

/// Formats a duration using the configured unit and suffix policy.
///
/// # Parameters
///
/// * `value` - Duration to format.
/// * `options` - Output unit, suffix, and numeric conversion policies.
///
/// # Returns
///
/// An exact unit count under the reject policy, or a half-up rounded count
/// when Duration rounding permits it.
///
/// # Errors
///
/// Returns an invalid-value [`DataConversionError`] when exact conversion
/// would lose precision.
#[inline]
pub(super) fn format_duration(
    value: Duration,
    options: &DataConversionOptions,
) -> Result<String, DataConversionError> {
    let units = duration_to_u128(value, options, DataType::String)?;
    if options.duration().append_unit_suffix() {
        Ok(format!(
            "{units}{}",
            options.duration().output_unit().symbol()
        ))
    } else {
        Ok(units.to_string())
    }
}
