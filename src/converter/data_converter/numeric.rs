// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
// =============================================================================
//! Numeric conversion implementations.

use std::str::FromStr;
use std::time::Duration;

use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_traits::{
    FromPrimitive,
    ToPrimitive,
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
    NumericConversionPolicy,
};
use crate::datatype::DataType;

/// Parsed representation shared by textual numeric conversions.
enum ParsedNumber {
    Integer(BigInt),
    Decimal(BigDecimal),
    NaN,
    PositiveInfinity,
    NegativeInfinity,
}

/// Parses a normalized number without selecting a target primitive first.
fn parse_number(
    value: &str,
    to: DataType,
) -> DataConversionResult<ParsedNumber> {
    let lower = value.to_ascii_lowercase();
    match lower.as_str() {
        "nan" => return Ok(ParsedNumber::NaN),
        "inf" | "+inf" | "infinity" | "+infinity" => {
            return Ok(ParsedNumber::PositiveInfinity);
        }
        "-inf" | "-infinity" => return Ok(ParsedNumber::NegativeInfinity),
        _ => {}
    }
    if is_integer_syntax(value) {
        let negative = value.starts_with('-');
        let digits = value.strip_prefix(['+', '-']).unwrap_or(value);
        let integer = digits.bytes().fold(BigInt::from(0u8), |value, digit| {
            value * 10u8 + (digit - b'0')
        });
        return Ok(ParsedNumber::Integer(if negative {
            -integer
        } else {
            integer
        }));
    }
    match BigDecimal::from_str(value) {
        Ok(value) => Ok(ParsedNumber::Decimal(value)),
        Err(_) => Err(invalid(
            DataType::String,
            to,
            DataConversionErrorKind::InvalidSyntax {
                expected: numeric_syntax(to),
            },
        )),
    }
}

/// Tests whether text uses the canonical integer grammar.
pub(super) fn is_integer_syntax(value: &str) -> bool {
    let digits = value.strip_prefix(['+', '-']).unwrap_or(value);
    !digits.is_empty() && digits.bytes().all(|byte| byte.is_ascii_digit())
}

/// Returns the stable syntax label for a numeric target.
fn numeric_syntax(to: DataType) -> &'static str {
    match to {
        DataType::BigDecimal => "decimal number with optional exponent",
        DataType::Float32 | DataType::Float64 => "number",
        _ => "integer",
    }
}

/// Converts parsed text to an integer, applying the numeric policy.
fn parsed_to_bigint(
    parsed: ParsedNumber,
    policy: NumericConversionPolicy,
    to: DataType,
) -> DataConversionResult<BigInt> {
    match parsed {
        ParsedNumber::Integer(value) => Ok(value),
        ParsedNumber::Decimal(value) => {
            decimal_to_bigint(&value, policy, DataType::String, to)
        }
        ParsedNumber::NaN
        | ParsedNumber::PositiveInfinity
        | ParsedNumber::NegativeInfinity => Err(invalid(
            DataType::String,
            to,
            DataConversionErrorKind::NonFinite,
        )),
    }
}

/// Converts a decimal to an integer with exactness checks.
fn decimal_to_bigint(
    value: &BigDecimal,
    policy: NumericConversionPolicy,
    from: DataType,
    to: DataType,
) -> DataConversionResult<BigInt> {
    let (coefficient, scale) = value.as_bigint_and_exponent();
    if coefficient == BigInt::from(0u8) {
        return Ok(coefficient);
    }
    if scale <= 0 {
        let exponent = scale.unsigned_abs();
        let coefficient_digits =
            coefficient.to_str_radix(10).trim_start_matches('-').len() as u64;
        if (to.is_integer()
            && to != DataType::BigInteger
            && coefficient_digits.saturating_add(exponent) > 39)
            || exponent > u64::from(u32::MAX)
        {
            return Err(invalid(from, to, DataConversionErrorKind::OutOfRange));
        }
        return Ok(coefficient * BigInt::from(10u8).pow(exponent as u32));
    }

    let coefficient_digits =
        coefficient.to_str_radix(10).trim_start_matches('-').len() as u64;
    if scale as u64 >= coefficient_digits {
        return if policy == NumericConversionPolicy::Exact {
            Err(invalid(from, to, DataConversionErrorKind::PrecisionLoss))
        } else {
            Ok(BigInt::from(0u8))
        };
    }
    let divisor = BigInt::from(10u8).pow(scale as u32);
    let quotient = &coefficient / &divisor;
    let remainder = coefficient % divisor;
    if policy == NumericConversionPolicy::Exact
        && remainder != BigInt::from(0u8)
    {
        Err(invalid(from, to, DataConversionErrorKind::PrecisionLoss))
    } else {
        Ok(quotient)
    }
}

/// Converts a finite float to an integer with exactness checks.
fn float_to_bigint(
    value: f64,
    policy: NumericConversionPolicy,
    from: DataType,
    to: DataType,
) -> DataConversionResult<BigInt> {
    if !value.is_finite() {
        return Err(invalid(from, to, DataConversionErrorKind::NonFinite));
    }
    let converted = BigInt::from_f64(value.trunc())
        .expect("finite primitive floats always have a BigInt representation");
    if policy == NumericConversionPolicy::Exact && value.fract() != 0.0 {
        Err(invalid(from, to, DataConversionErrorKind::PrecisionLoss))
    } else {
        Ok(converted)
    }
}

/// Extracts an arbitrary-precision integer from a supported source.
pub(super) fn source_to_bigint(
    source: &DataConverter<'_>,
    options: &DataConversionOptions,
    to: DataType,
) -> DataConversionResult<BigInt> {
    match source {
        DataConverter::Bool(value) => Ok(BigInt::from(u8::from(*value))),
        DataConverter::Char(value) => Ok(BigInt::from(*value as u32)),
        DataConverter::Int8(value) => Ok(BigInt::from(*value)),
        DataConverter::Int16(value) => Ok(BigInt::from(*value)),
        DataConverter::Int32(value) => Ok(BigInt::from(*value)),
        DataConverter::Int64(value) => Ok(BigInt::from(*value)),
        DataConverter::Int128(value) => Ok(BigInt::from(*value)),
        DataConverter::IntSize(value) => Ok(BigInt::from(*value)),
        DataConverter::UInt8(value) => Ok(BigInt::from(*value)),
        DataConverter::UInt16(value) => Ok(BigInt::from(*value)),
        DataConverter::UInt32(value) => Ok(BigInt::from(*value)),
        DataConverter::UInt64(value) => Ok(BigInt::from(*value)),
        DataConverter::UInt128(value) => Ok(BigInt::from(*value)),
        DataConverter::UIntSize(value) => Ok(BigInt::from(*value)),
        DataConverter::Float32(value) => float_to_bigint(
            f64::from(*value),
            options.numeric_policy,
            DataType::Float32,
            to,
        ),
        DataConverter::Float64(value) => float_to_bigint(
            *value,
            options.numeric_policy,
            DataType::Float64,
            to,
        ),
        DataConverter::BigInteger(value) => Ok(value.as_ref().clone()),
        DataConverter::BigDecimal(value) => decimal_to_bigint(
            value.as_ref(),
            options.numeric_policy,
            DataType::BigDecimal,
            to,
        ),
        DataConverter::String(value) => {
            let value = normalize(value, options, to)?;
            if to == DataType::BigInteger && !is_integer_syntax(value) {
                return Err(invalid(
                    DataType::String,
                    to,
                    DataConversionErrorKind::InvalidSyntax {
                        expected: "[+-]?[0-9]+",
                    },
                ));
            }
            parsed_to_bigint(
                parse_number(value, to)?,
                options.numeric_policy,
                to,
            )
        }
        DataConverter::Duration(value) => {
            duration_to_bigint(*value, options, to)
        }
        DataConverter::Empty(_) => Err(source.missing(to)),
        _ => Err(source.unsupported(to)),
    }
}

/// Converts a duration to integer units under the numeric policy.
pub(super) fn duration_to_bigint(
    duration: Duration,
    options: &DataConversionOptions,
    to: DataType,
) -> DataConversionResult<BigInt> {
    let unit_nanos = options.duration.unit.nanos_per_unit();
    let total_nanos = duration.as_nanos();
    if options.numeric_policy == NumericConversionPolicy::Exact
        && !total_nanos.is_multiple_of(unit_nanos)
    {
        return Err(invalid(
            DataType::Duration,
            to,
            DataConversionErrorKind::PrecisionLoss,
        ));
    }
    Ok(BigInt::from(
        if options.numeric_policy == NumericConversionPolicy::Exact {
            total_nanos / unit_nanos
        } else {
            options.duration.unit.rounded_units(duration)
        },
    ))
}

/// Converts a supported source to a signed primitive range.
fn to_i128(
    source: &DataConverter<'_>,
    options: &DataConversionOptions,
    to: DataType,
) -> DataConversionResult<i128> {
    match source_to_bigint(source, options, to)?.to_i128() {
        Some(value) => Ok(value),
        None => Err(invalid(
            source.data_type(),
            to,
            DataConversionErrorKind::OutOfRange,
        )),
    }
}

/// Converts a supported source to an unsigned primitive range.
fn to_u128(
    source: &DataConverter<'_>,
    options: &DataConversionOptions,
    to: DataType,
) -> DataConversionResult<u128> {
    match source_to_bigint(source, options, to)?.to_u128() {
        Some(value) => Ok(value),
        None => Err(invalid(
            source.data_type(),
            to,
            DataConversionErrorKind::OutOfRange,
        )),
    }
}

/// Checks a signed target range.
fn checked_signed<T>(
    value: i128,
    source: &DataConverter<'_>,
    to: DataType,
) -> DataConversionResult<T>
where
    T: TryFrom<i128>,
{
    match T::try_from(value) {
        Ok(value) => Ok(value),
        Err(_) => Err(invalid(
            source.data_type(),
            to,
            DataConversionErrorKind::OutOfRange,
        )),
    }
}

/// Checks an unsigned target range.
fn checked_unsigned<T>(
    value: u128,
    source: &DataConverter<'_>,
    to: DataType,
) -> DataConversionResult<T>
where
    T: TryFrom<u128>,
{
    match T::try_from(value) {
        Ok(value) => Ok(value),
        Err(_) => Err(invalid(
            source.data_type(),
            to,
            DataConversionErrorKind::OutOfRange,
        )),
    }
}

macro_rules! impl_signed_target {
    ($target:ty, $data_type:expr) => {
        impl DataConvertTo<$target> for DataConverter<'_> {
            fn convert(
                &self,
                options: &DataConversionOptions,
            ) -> DataConversionResult<$target> {
                checked_signed(
                    to_i128(self, options, $data_type)?,
                    self,
                    $data_type,
                )
            }
        }
    };
}

macro_rules! impl_unsigned_target {
    ($target:ty, $data_type:expr) => {
        impl DataConvertTo<$target> for DataConverter<'_> {
            fn convert(
                &self,
                options: &DataConversionOptions,
            ) -> DataConversionResult<$target> {
                checked_unsigned(
                    to_u128(self, options, $data_type)?,
                    self,
                    $data_type,
                )
            }
        }
    };
}

impl_signed_target!(i8, DataType::Int8);
impl_signed_target!(i16, DataType::Int16);
impl_signed_target!(i32, DataType::Int32);
impl_signed_target!(i64, DataType::Int64);
impl_signed_target!(i128, DataType::Int128);
impl_signed_target!(isize, DataType::IntSize);
impl_unsigned_target!(u8, DataType::UInt8);
impl_unsigned_target!(u16, DataType::UInt16);
impl_unsigned_target!(u32, DataType::UInt32);
impl_unsigned_target!(u64, DataType::UInt64);
impl_unsigned_target!(u128, DataType::UInt128);
impl_unsigned_target!(usize, DataType::UIntSize);

/// Converts an integer exactly or lossily to a float.
fn bigint_to_f64(
    value: &BigInt,
    policy: NumericConversionPolicy,
    from: DataType,
    to: DataType,
) -> DataConversionResult<f64> {
    let converted = value.to_f64().unwrap_or(f64::INFINITY);
    if !converted.is_finite() {
        return Err(invalid(from, to, DataConversionErrorKind::OutOfRange));
    }
    if policy == NumericConversionPolicy::Exact
        && BigInt::from_f64(converted).as_ref() != Some(value)
    {
        Err(invalid(from, to, DataConversionErrorKind::PrecisionLoss))
    } else {
        Ok(converted)
    }
}

/// Converts a decimal exactly or lossily to a float.
fn decimal_to_f64(
    value: &BigDecimal,
    policy: NumericConversionPolicy,
    from: DataType,
    to: DataType,
) -> DataConversionResult<f64> {
    let converted = value.to_f64().unwrap_or(f64::INFINITY);
    if !converted.is_finite() {
        return Err(invalid(from, to, DataConversionErrorKind::OutOfRange));
    }
    if policy == NumericConversionPolicy::Exact
        && BigDecimal::from_f64(converted).as_ref() != Some(value)
    {
        Err(invalid(from, to, DataConversionErrorKind::PrecisionLoss))
    } else {
        Ok(converted)
    }
}

/// Converts a source to f64 before target-width validation.
fn source_to_f64(
    source: &DataConverter<'_>,
    options: &DataConversionOptions,
    to: DataType,
) -> DataConversionResult<f64> {
    match source {
        DataConverter::Float64(value) => Ok(*value),
        DataConverter::Float32(value) => Ok(f64::from(*value)),
        DataConverter::Bool(value) => Ok(if *value { 1.0 } else { 0.0 }),
        DataConverter::Char(value) => Ok(f64::from(*value as u32)),
        DataConverter::Int8(value) => bigint_to_f64(
            &BigInt::from(*value),
            options.numeric_policy,
            DataType::Int8,
            to,
        ),
        DataConverter::Int16(value) => bigint_to_f64(
            &BigInt::from(*value),
            options.numeric_policy,
            DataType::Int16,
            to,
        ),
        DataConverter::Int32(value) => bigint_to_f64(
            &BigInt::from(*value),
            options.numeric_policy,
            DataType::Int32,
            to,
        ),
        DataConverter::Int64(value) => bigint_to_f64(
            &BigInt::from(*value),
            options.numeric_policy,
            DataType::Int64,
            to,
        ),
        DataConverter::Int128(value) => bigint_to_f64(
            &BigInt::from(*value),
            options.numeric_policy,
            DataType::Int128,
            to,
        ),
        DataConverter::IntSize(value) => bigint_to_f64(
            &BigInt::from(*value),
            options.numeric_policy,
            DataType::IntSize,
            to,
        ),
        DataConverter::UInt8(value) => bigint_to_f64(
            &BigInt::from(*value),
            options.numeric_policy,
            DataType::UInt8,
            to,
        ),
        DataConverter::UInt16(value) => bigint_to_f64(
            &BigInt::from(*value),
            options.numeric_policy,
            DataType::UInt16,
            to,
        ),
        DataConverter::UInt32(value) => bigint_to_f64(
            &BigInt::from(*value),
            options.numeric_policy,
            DataType::UInt32,
            to,
        ),
        DataConverter::UInt64(value) => bigint_to_f64(
            &BigInt::from(*value),
            options.numeric_policy,
            DataType::UInt64,
            to,
        ),
        DataConverter::UInt128(value) => bigint_to_f64(
            &BigInt::from(*value),
            options.numeric_policy,
            DataType::UInt128,
            to,
        ),
        DataConverter::UIntSize(value) => bigint_to_f64(
            &BigInt::from(*value),
            options.numeric_policy,
            DataType::UIntSize,
            to,
        ),
        DataConverter::BigInteger(value) => bigint_to_f64(
            value,
            options.numeric_policy,
            DataType::BigInteger,
            to,
        ),
        DataConverter::BigDecimal(value) => decimal_to_f64(
            value,
            options.numeric_policy,
            DataType::BigDecimal,
            to,
        ),
        DataConverter::String(value) => {
            let value = normalize(value, options, to)?;
            match parse_number(value, to)? {
                ParsedNumber::Integer(value) => bigint_to_f64(
                    &value,
                    options.numeric_policy,
                    DataType::String,
                    to,
                ),
                ParsedNumber::Decimal(value) => decimal_to_f64(
                    &value,
                    options.numeric_policy,
                    DataType::String,
                    to,
                ),
                ParsedNumber::NaN => Ok(f64::NAN),
                ParsedNumber::PositiveInfinity => Ok(f64::INFINITY),
                ParsedNumber::NegativeInfinity => Ok(f64::NEG_INFINITY),
            }
        }
        DataConverter::Empty(_) => Err(source.missing(to)),
        _ => Err(source.unsupported(to)),
    }
}

impl DataConvertTo<f64> for DataConverter<'_> {
    fn convert(
        &self,
        options: &DataConversionOptions,
    ) -> DataConversionResult<f64> {
        source_to_f64(self, options, DataType::Float64)
    }
}

impl DataConvertTo<f32> for DataConverter<'_> {
    fn convert(
        &self,
        options: &DataConversionOptions,
    ) -> DataConversionResult<f32> {
        if let Self::Float32(value) = self {
            return Ok(*value);
        }
        let value = source_to_f64(self, options, DataType::Float32)?;
        if value.is_nan() {
            return Ok(f32::NAN);
        }
        if value == f64::INFINITY {
            return Ok(f32::INFINITY);
        }
        if value == f64::NEG_INFINITY {
            return Ok(f32::NEG_INFINITY);
        }
        let converted = value as f32;
        if !converted.is_finite() {
            return Err(self.invalid(
                DataType::Float32,
                DataConversionErrorKind::OutOfRange,
            ));
        }
        if options.numeric_policy == NumericConversionPolicy::Exact
            && f64::from(converted) != value
        {
            Err(self.invalid(
                DataType::Float32,
                DataConversionErrorKind::PrecisionLoss,
            ))
        } else {
            Ok(converted)
        }
    }
}

impl DataConvertTo<BigInt> for DataConverter<'_> {
    fn convert(
        &self,
        options: &DataConversionOptions,
    ) -> DataConversionResult<BigInt> {
        source_to_bigint(self, options, DataType::BigInteger)
    }
}

impl DataConvertTo<BigDecimal> for DataConverter<'_> {
    fn convert(
        &self,
        options: &DataConversionOptions,
    ) -> DataConversionResult<BigDecimal> {
        match self {
            Self::BigDecimal(value) => Ok(value.as_ref().clone()),
            Self::Float32(value) => match BigDecimal::from_f32(*value) {
                Some(value) => Ok(value),
                None => Err(self.invalid(
                    DataType::BigDecimal,
                    DataConversionErrorKind::NonFinite,
                )),
            },
            Self::Float64(value) => match BigDecimal::from_f64(*value) {
                Some(value) => Ok(value),
                None => Err(self.invalid(
                    DataType::BigDecimal,
                    DataConversionErrorKind::NonFinite,
                )),
            },
            Self::String(value) => {
                let value = normalize(value, options, DataType::BigDecimal)?;
                match parse_number(value, DataType::BigDecimal)? {
                    ParsedNumber::Integer(value) => Ok(BigDecimal::from(value)),
                    ParsedNumber::Decimal(value) => Ok(value),
                    ParsedNumber::NaN
                    | ParsedNumber::PositiveInfinity
                    | ParsedNumber::NegativeInfinity => Err(self.invalid(
                        DataType::BigDecimal,
                        DataConversionErrorKind::NonFinite,
                    )),
                }
            }
            Self::Empty(_) => Err(self.missing(DataType::BigDecimal)),
            Self::Date(_)
            | Self::Time(_)
            | Self::DateTime(_)
            | Self::Instant(_)
            | Self::Duration(_)
            | Self::Url(_)
            | Self::StringMap(_)
            | Self::Json(_) => Err(self.unsupported(DataType::BigDecimal)),
            _ => Ok(BigDecimal::from(source_to_bigint(
                self,
                options,
                DataType::BigDecimal,
            )?)),
        }
    }
}
