// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
// =============================================================================
//! Duration conversion implementations.

use std::time::Duration;

#[cfg(feature = "big-number")]
use num_bigint::Sign;
#[cfg(feature = "big-number")]
use num_traits::ToPrimitive;

use super::DataConverter;
use super::numeric::{
    duration_to_u128,
    source_to_integer,
};
use super::string_source::normalize;
use crate::converter::{
    DataConversionError,
    DataConversionOptions,
    DataConversionTarget,
    DurationUnit,
    InvalidValueReason,
    SuffixlessDurationPolicy,
};
use crate::datatype::DataType;

/// Converts a duration unit count to a duration.
///
/// `value` is interpreted using the configured duration unit and `from` is
/// retained as error context. Returns the exact duration, or an invalid-value
/// error for negative or out-of-range counts.
fn integer_to_duration(
    value: (bool, u128),
    from: DataType,
    options: &DataConversionOptions,
) -> Result<Duration, DataConversionError> {
    let (negative, value) = value;
    if negative {
        return Err(DataConversionError::InvalidValue {
            from,
            to: DataType::Duration,
            reason: InvalidValueReason::NegativeDuration,
        });
    }
    match options
        .duration
        .numeric_input_unit
        .duration_from_u128(value)
    {
        Ok(duration) => Ok(duration),
        Err(_) => Err(DataConversionError::InvalidValue {
            from,
            to: DataType::Duration,
            reason: InvalidValueReason::OutOfRange,
        }),
    }
}

/// Parses the canonical duration grammar.
///
/// `value` is normalized using `options`, then parsed as a non-negative integer
/// with an optional supported unit suffix. For a missing suffix, the configured
/// policy either rejects the input or supplies an assumed unit. Returns
/// contextual conversion errors for normalization, syntax, unit, and range
/// failures.
fn parse_duration(
    value: &str,
    options: &DataConversionOptions,
) -> Result<Duration, DataConversionError> {
    let to = DataType::Duration;
    let value = normalize(value, options, to)?;
    let split_at = value
        .bytes()
        .position(|byte| !byte.is_ascii_digit())
        .unwrap_or(value.len());
    let (digits, suffix) = value.split_at(split_at);
    if digits.is_empty() || !digits.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(DataConversionError::InvalidValue {
            from: DataType::String,
            to,
            reason: InvalidValueReason::InvalidSyntax {
                expected: "[0-9]+(ns|us|µs|μs|ms|s|m|h|d)?",
            },
        });
    }
    let unit = if suffix.is_empty() {
        match options.duration.suffixless_string_policy {
            SuffixlessDurationPolicy::Reject => {
                return Err(DataConversionError::InvalidValue {
                    from: DataType::String,
                    to,
                    reason: InvalidValueReason::InvalidSyntax {
                        expected: "[0-9]+(ns|us|µs|μs|ms|s|m|h|d)",
                    },
                });
            }
            SuffixlessDurationPolicy::Assume(unit) => unit,
        }
    } else {
        let Some(unit) = DurationUnit::from_suffix(suffix) else {
            if suffix.chars().all(char::is_alphabetic) {
                return Err(DataConversionError::InvalidValue {
                    from: DataType::String,
                    to,
                    reason: InvalidValueReason::UnsupportedDurationUnit,
                });
            }
            return Err(DataConversionError::InvalidValue {
                from: DataType::String,
                to,
                reason: InvalidValueReason::InvalidSyntax {
                    expected: "[0-9]+(ns|us|µs|μs|ms|s|m|h|d)?",
                },
            });
        };
        unit
    };
    let value = match digits.parse::<u128>() {
        Ok(value) => value,
        Err(_) => {
            return Err(DataConversionError::InvalidValue {
                from: DataType::String,
                to,
                reason: InvalidValueReason::OutOfRange,
            });
        }
    };
    match unit.duration_from_u128(value) {
        Ok(duration) => Ok(duration),
        Err(_) => Err(DataConversionError::InvalidValue {
            from: DataType::String,
            to,
            reason: InvalidValueReason::OutOfRange,
        }),
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
            DataConverter::Empty(_) => Err(source.missing(DataType::Duration)),
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
            #[cfg(feature = "big-number")]
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
/// Returns an exact unit count under [`NumericConversionPolicy::Exact`], or a
/// half-up rounded count under the lossy policy. A precision-losing exact
/// conversion returns [`DataConversionError::InvalidValue`].
pub(super) fn format_duration(
    value: Duration,
    options: &DataConversionOptions,
) -> Result<String, DataConversionError> {
    let units = duration_to_u128(value, options, DataType::String)?;
    if options.duration.append_unit_suffix {
        Ok(format!("{units}{}", options.duration.output_unit.suffix()))
    } else {
        Ok(units.to_string())
    }
}
