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
    DurationUnitSuffixSet,
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
fn parse_duration(
    value: &str,
    options: &DataConversionOptions,
) -> Result<Duration, DataConversionError> {
    let to = DataType::Duration;
    let value = normalize(value, options, to)?;
    let text_options = DurationTextOptions::new(
        options.duration().suffixless_string_policy(),
        options.duration().unit_suffix_set(),
    )
    .with_max_text_bytes(options.duration().max_text_bytes());
    match parse_duration_text(value, &text_options) {
        Ok(duration) => Ok(duration),
        Err(DurationParseError::LimitExceeded { maximum }) => {
            Err(DataConversionError::limit_exceeded(
                DataType::String,
                to,
                ConversionLimit::DurationTextBytes { maximum },
            ))
        }
        Err(DurationParseError::InvalidSyntax) => {
            let suffix_required = !value.is_empty()
                && value.bytes().all(|byte| byte.is_ascii_digit())
                && options.duration().suffixless_string_policy()
                    == SuffixlessDurationPolicy::Reject;
            Err(DataConversionError::invalid(
                DataType::String,
                to,
                InvalidValueReason::InvalidSyntax {
                    expected: if suffix_required {
                        match options.duration().unit_suffix_set() {
                            DurationUnitSuffixSet::Ascii => {
                                "[0-9]+(ns|us|ms|s|m|h|d)"
                            }
                            DurationUnitSuffixSet::Extended => {
                                "[0-9]+(ns|us|µs|μs|ms|s|m|h|d)"
                            }
                        }
                    } else {
                        match options.duration().unit_suffix_set() {
                            DurationUnitSuffixSet::Ascii => {
                                "[0-9]+(ns|us|ms|s|m|h|d)?"
                            }
                            DurationUnitSuffixSet::Extended => {
                                "[0-9]+(ns|us|µs|μs|ms|s|m|h|d)?"
                            }
                        }
                    },
                },
            ))
        }
        Err(DurationParseError::UnsupportedUnit { .. }) => {
            Err(DataConversionError::invalid(
                DataType::String,
                to,
                InvalidValueReason::UnsupportedDurationUnit,
            ))
        }
        Err(DurationParseError::OutOfRange) => {
            Err(DataConversionError::invalid(
                DataType::String,
                to,
                InvalidValueReason::OutOfRange,
            ))
        }
    }
}

impl DataConversionTarget for Duration {
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
pub(super) fn format_duration(
    value: Duration,
    options: &DataConversionOptions,
) -> Result<String, DataConversionError> {
    let units = duration_to_u128(value, options, DataType::String)?;
    if options.duration().append_unit_suffix() {
        Ok(format!(
            "{units}{}",
            options.duration().output_unit().suffix()
        ))
    } else {
        Ok(units.to_string())
    }
}
