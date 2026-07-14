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

use super::numeric::{
    duration_to_bigint,
    source_to_bigint,
};
use super::{
    DataConverter,
    invalid,
    normalize,
};
use crate::converter::{
    DataConversionErrorKind,
    DataConversionOptions,
    DataConversionResult,
    DataConvertTo,
    DurationUnit,
};
use crate::datatype::DataType;

/// Converts a duration unit count to a duration.
fn integer_to_duration(
    value: &BigInt,
    from: DataType,
    options: &DataConversionOptions,
) -> DataConversionResult<Duration> {
    if value < &BigInt::zero() {
        return Err(invalid(
            from,
            DataType::Duration,
            DataConversionErrorKind::NegativeDuration,
        ));
    }
    let Some(value) = value.to_u128() else {
        return Err(invalid(
            from,
            DataType::Duration,
            DataConversionErrorKind::OutOfRange,
        ));
    };
    match options.duration.unit.duration_from_u128(value) {
        Ok(duration) => Ok(duration),
        Err(_) => Err(invalid(
            from,
            DataType::Duration,
            DataConversionErrorKind::OutOfRange,
        )),
    }
}

/// Parses the canonical duration grammar.
fn parse_duration(
    value: &str,
    options: &DataConversionOptions,
) -> DataConversionResult<Duration> {
    let to = DataType::Duration;
    let value = normalize(value, options, to)?;
    let split_at = value
        .bytes()
        .position(|byte| !byte.is_ascii_digit())
        .unwrap_or(value.len());
    let (digits, suffix) = value.split_at(split_at);
    if digits.is_empty() || !digits.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(invalid(
            DataType::String,
            to,
            DataConversionErrorKind::InvalidSyntax {
                expected: "[0-9]+(ns|us|ms|s|m|h|d)?",
            },
        ));
    }
    let unit = if suffix.is_empty() {
        options.duration.unit
    } else {
        if !suffix.bytes().all(|byte| byte.is_ascii_alphabetic()) {
            return Err(invalid(
                DataType::String,
                to,
                DataConversionErrorKind::InvalidSyntax {
                    expected: "[0-9]+(ns|us|ms|s|m|h|d)?",
                },
            ));
        }
        let Some(unit) = DurationUnit::from_suffix(suffix) else {
            return Err(invalid(
                DataType::String,
                to,
                DataConversionErrorKind::UnsupportedDurationUnit,
            ));
        };
        unit
    };
    let value = match digits.parse::<u128>() {
        Ok(value) => value,
        Err(_) => {
            return Err(invalid(
                DataType::String,
                to,
                DataConversionErrorKind::OutOfRange,
            ));
        }
    };
    match unit.duration_from_u128(value) {
        Ok(duration) => Ok(duration),
        Err(_) => Err(invalid(
            DataType::String,
            to,
            DataConversionErrorKind::OutOfRange,
        )),
    }
}

impl DataConvertTo<Duration> for DataConverter<'_> {
    fn convert(
        &self,
        options: &DataConversionOptions,
    ) -> DataConversionResult<Duration> {
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
pub(super) fn format_duration(
    value: Duration,
    options: &DataConversionOptions,
) -> DataConversionResult<String> {
    let units = duration_to_bigint(value, options, DataType::String)?;
    if options.duration.append_unit_suffix {
        Ok(format!("{units}{}", options.duration.unit.suffix()))
    } else {
        Ok(units.to_string())
    }
}
