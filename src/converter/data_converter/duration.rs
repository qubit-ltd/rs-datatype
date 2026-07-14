// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
// =============================================================================
//! Duration conversion implementations.

use std::time::Duration;

use num_bigint::BigInt;
use num_traits::{
    ToPrimitive,
    Zero,
};

use super::DataConverter;
use super::numeric::{
    duration_to_bigint,
    source_to_bigint,
};
use super::string_source::normalize;
use crate::converter::{
    DataConversionError,
    DataConversionOptions,
    DataConvertTo,
    DurationUnit,
    InvalidValueReason,
};
use crate::datatype::DataType;

/// Converts a duration unit count to a duration.
///
/// `value` is interpreted using the configured duration unit and `from` is
/// retained as error context. Returns the exact duration, or an invalid-value
/// error for negative or out-of-range counts.
fn integer_to_duration(
    value: &BigInt,
    from: DataType,
    options: &DataConversionOptions,
) -> Result<Duration, DataConversionError> {
    if value < &BigInt::zero() {
        return Err(DataConversionError::InvalidValue {
            from,
            to: DataType::Duration,
            reason: InvalidValueReason::NegativeDuration,
        });
    }
    let Some(value) = value.to_u128() else {
        return Err(DataConversionError::InvalidValue {
            from,
            to: DataType::Duration,
            reason: InvalidValueReason::OutOfRange,
        });
    };
    match options.duration.unit.duration_from_u128(value) {
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
/// with an optional supported unit suffix. A missing suffix uses the configured
/// unit. Returns contextual conversion errors for normalization, syntax, unit,
/// and range failures.
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
                expected: "[0-9]+(ns|us|ms|s|m|h|d)?",
            },
        });
    }
    let unit = if suffix.is_empty() {
        options.duration.unit
    } else {
        if !suffix.bytes().all(|byte| byte.is_ascii_alphabetic()) {
            return Err(DataConversionError::InvalidValue {
                from: DataType::String,
                to,
                reason: InvalidValueReason::InvalidSyntax {
                    expected: "[0-9]+(ns|us|ms|s|m|h|d)?",
                },
            });
        }
        let Some(unit) = DurationUnit::from_suffix(suffix) else {
            return Err(DataConversionError::InvalidValue {
                from: DataType::String,
                to,
                reason: InvalidValueReason::UnsupportedDurationUnit,
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

impl DataConvertTo<Duration> for DataConverter<'_> {
    fn convert(
        &self,
        options: &DataConversionOptions,
    ) -> Result<Duration, DataConversionError> {
        match self {
            Self::Duration(value) => Ok(*value),
            Self::String(value) => parse_duration(value, options),
            Self::BigInteger(value) => {
                integer_to_duration(value, DataType::BigInteger, options)
            }
            Self::Empty(_) => Err(self.missing(DataType::Duration)),
            Self::Int8(_)
            | Self::Int16(_)
            | Self::Int32(_)
            | Self::Int64(_)
            | Self::Int128(_)
            | Self::IntSize(_)
            | Self::UInt8(_)
            | Self::UInt16(_)
            | Self::UInt32(_)
            | Self::UInt64(_)
            | Self::UInt128(_)
            | Self::UIntSize(_) => integer_to_duration(
                &source_to_bigint(self, options, DataType::Duration)?,
                self.data_type(),
                options,
            ),
            _ => Err(self.unsupported(DataType::Duration)),
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
    let units = duration_to_bigint(value, options, DataType::String)?;
    if options.duration.append_unit_suffix {
        Ok(format!("{units}{}", options.duration.unit.suffix()))
    } else {
        Ok(units.to_string())
    }
}
