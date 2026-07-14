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

use super::DataConverter;
use super::string_source::normalize;
use crate::converter::{
    DataConversionError,
    DataConversionOptions,
    DataConvertTo,
    InvalidValueReason,
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
///
/// `value` must already have passed string normalization; `to` supplies target
/// context and selects the expected syntax label on failure. Returns an exact
/// integer/decimal representation or a non-finite marker. Invalid decimal text
/// returns [`DataConversionError::InvalidValue`].
fn parse_number(
    value: &str,
    to: DataType,
) -> Result<ParsedNumber, DataConversionError> {
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
        Err(_) => Err(DataConversionError::InvalidValue {
            from: DataType::String,
            to,
            reason: InvalidValueReason::InvalidSyntax {
                expected: numeric_syntax(to),
            },
        }),
    }
}

/// Tests whether text uses the canonical integer grammar.
///
/// Returns `true` for one or more ASCII digits with an optional leading sign,
/// and `false` for empty, whitespace-containing, or otherwise invalid text.
pub(super) fn is_integer_syntax(value: &str) -> bool {
    let digits = value.strip_prefix(['+', '-']).unwrap_or(value);
    !digits.is_empty() && digits.bytes().all(|byte| byte.is_ascii_digit())
}

/// Returns the stable syntax label for a numeric target.
///
/// `to` selects the label embedded in invalid-syntax errors. The return value
/// is static and contains no source data.
fn numeric_syntax(to: DataType) -> &'static str {
    match to {
        DataType::BigDecimal => "decimal number with optional exponent",
        DataType::Float32 | DataType::Float64 => "number",
        _ => "integer",
    }
}

/// Converts parsed text to an integer, applying the numeric policy.
///
/// Decimal input is exact only when it has no fractional remainder. Lossy mode
/// truncates toward zero. Non-finite markers always return an invalid-value
/// error whose source type is [`DataType::String`].
fn parsed_to_bigint(
    parsed: ParsedNumber,
    policy: NumericConversionPolicy,
    to: DataType,
) -> Result<BigInt, DataConversionError> {
    match parsed {
        ParsedNumber::Integer(value) => Ok(value),
        ParsedNumber::Decimal(value) => {
            decimal_to_bigint(&value, policy, DataType::String, to)
        }
        ParsedNumber::NaN
        | ParsedNumber::PositiveInfinity
        | ParsedNumber::NegativeInfinity => {
            Err(DataConversionError::InvalidValue {
                from: DataType::String,
                to,
                reason: InvalidValueReason::NonFinite,
            })
        }
    }
}

/// Converts a decimal to an integer with exactness checks.
///
/// `from` and `to` are used only as error context. Returns the integral value;
/// exact mode rejects any fractional remainder, while lossy mode truncates
/// toward zero. Values that cannot reasonably fit a primitive target are
/// rejected before constructing an impractically large power of ten.
fn decimal_to_bigint(
    value: &BigDecimal,
    policy: NumericConversionPolicy,
    from: DataType,
    to: DataType,
) -> Result<BigInt, DataConversionError> {
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
            return Err(DataConversionError::InvalidValue {
                from,
                to,
                reason: InvalidValueReason::OutOfRange,
            });
        }
        return Ok(coefficient * BigInt::from(10u8).pow(exponent as u32));
    }

    let coefficient_digits =
        coefficient.to_str_radix(10).trim_start_matches('-').len() as u64;
    if scale as u64 >= coefficient_digits {
        return if policy == NumericConversionPolicy::Exact {
            Err(DataConversionError::InvalidValue {
                from,
                to,
                reason: InvalidValueReason::PrecisionLoss,
            })
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
        Err(DataConversionError::InvalidValue {
            from,
            to,
            reason: InvalidValueReason::PrecisionLoss,
        })
    } else {
        Ok(quotient)
    }
}

/// Converts a finite float to an integer with exactness checks.
///
/// Returns a `BigInt` after truncation toward zero. Exact mode rejects a
/// fractional source, and every policy rejects non-finite values. `from` and
/// `to` are retained in those errors.
fn float_to_bigint(
    value: f64,
    policy: NumericConversionPolicy,
    from: DataType,
    to: DataType,
) -> Result<BigInt, DataConversionError> {
    if !value.is_finite() {
        return Err(DataConversionError::InvalidValue {
            from,
            to,
            reason: InvalidValueReason::NonFinite,
        });
    }
    let converted = BigInt::from_f64(value.trunc())
        .expect("finite primitive floats always have a BigInt representation");
    if policy == NumericConversionPolicy::Exact && value.fract() != 0.0 {
        Err(DataConversionError::InvalidValue {
            from,
            to,
            reason: InvalidValueReason::PrecisionLoss,
        })
    } else {
        Ok(converted)
    }
}

/// Extracts an arbitrary-precision integer from a supported source.
///
/// `options` controls decimal/float exactness and duration units; `to` supplies
/// the final target context. Returns missing, unsupported, syntax, range, or
/// precision errors with the original source type when extraction fails.
pub(super) fn source_to_bigint(
    source: &DataConverter<'_>,
    options: &DataConversionOptions,
    to: DataType,
) -> Result<BigInt, DataConversionError> {
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
                return Err(DataConversionError::InvalidValue {
                    from: DataType::String,
                    to,
                    reason: InvalidValueReason::InvalidSyntax {
                        expected: "[+-]?[0-9]+",
                    },
                });
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
///
/// The duration unit comes from `options`. Exact mode rejects a remainder;
/// lossy mode uses half-up rounding. `to` is retained as target context.
pub(super) fn duration_to_bigint(
    duration: Duration,
    options: &DataConversionOptions,
    to: DataType,
) -> Result<BigInt, DataConversionError> {
    let unit_nanos = options.duration.output_unit.nanos_per_unit();
    let total_nanos = duration.as_nanos();
    if options.numeric_policy == NumericConversionPolicy::Exact
        && !total_nanos.is_multiple_of(unit_nanos)
    {
        return Err(DataConversionError::InvalidValue {
            from: DataType::Duration,
            to,
            reason: InvalidValueReason::PrecisionLoss,
        });
    }
    Ok(BigInt::from(
        if options.numeric_policy == NumericConversionPolicy::Exact {
            total_nanos / unit_nanos
        } else {
            options.duration.output_unit.rounded_units(duration)
        },
    ))
}

/// Converts a supported source to a signed primitive range.
///
/// Returns an `i128` intermediate or an out-of-range error associated with
/// `to`. Source parsing and policy errors are propagated unchanged.
fn to_i128(
    source: &DataConverter<'_>,
    options: &DataConversionOptions,
    to: DataType,
) -> Result<i128, DataConversionError> {
    match source_to_bigint(source, options, to)?.to_i128() {
        Some(value) => Ok(value),
        None => Err(DataConversionError::InvalidValue {
            from: source.data_type(),
            to,
            reason: InvalidValueReason::OutOfRange,
        }),
    }
}

/// Converts a supported source to an unsigned primitive range.
///
/// Returns a `u128` intermediate or an out-of-range error associated with
/// `to`. Negative and otherwise unrepresentable values are rejected.
fn to_u128(
    source: &DataConverter<'_>,
    options: &DataConversionOptions,
    to: DataType,
) -> Result<u128, DataConversionError> {
    match source_to_bigint(source, options, to)?.to_u128() {
        Some(value) => Ok(value),
        None => Err(DataConversionError::InvalidValue {
            from: source.data_type(),
            to,
            reason: InvalidValueReason::OutOfRange,
        }),
    }
}

/// Checks a signed target range.
///
/// `T` must support checked conversion from `i128`. Returns the converted value
/// or an out-of-range error containing `source` and `to` type context.
fn checked_signed<T>(
    value: i128,
    source: &DataConverter<'_>,
    to: DataType,
) -> Result<T, DataConversionError>
where
    T: TryFrom<i128>,
{
    match T::try_from(value) {
        Ok(value) => Ok(value),
        Err(_) => Err(DataConversionError::InvalidValue {
            from: source.data_type(),
            to,
            reason: InvalidValueReason::OutOfRange,
        }),
    }
}

/// Checks an unsigned target range.
///
/// `T` must support checked conversion from `u128`. Returns the converted value
/// or an out-of-range error containing `source` and `to` type context.
fn checked_unsigned<T>(
    value: u128,
    source: &DataConverter<'_>,
    to: DataType,
) -> Result<T, DataConversionError>
where
    T: TryFrom<u128>,
{
    match T::try_from(value) {
        Ok(value) => Ok(value),
        Err(_) => Err(DataConversionError::InvalidValue {
            from: source.data_type(),
            to,
            reason: InvalidValueReason::OutOfRange,
        }),
    }
}

macro_rules! impl_signed_target {
    ($target:ty, $data_type:expr) => {
        impl DataConvertTo<$target> for DataConverter<'_> {
            fn convert(
                &self,
                options: &DataConversionOptions,
            ) -> Result<$target, DataConversionError> {
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
            ) -> Result<$target, DataConversionError> {
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
///
/// Lossy mode accepts finite IEEE rounding. Exact mode additionally requires
/// converting the result back to reproduce `value`. Non-finite results are
/// reported as out of range using `from` and `to`.
fn bigint_to_f64(
    value: &BigInt,
    policy: NumericConversionPolicy,
    from: DataType,
    to: DataType,
) -> Result<f64, DataConversionError> {
    let converted = value.to_f64().unwrap_or(f64::INFINITY);
    if !converted.is_finite() {
        return Err(DataConversionError::InvalidValue {
            from,
            to,
            reason: InvalidValueReason::OutOfRange,
        });
    }
    if policy == NumericConversionPolicy::Exact
        && BigInt::from_f64(converted).as_ref() != Some(value)
    {
        Err(DataConversionError::InvalidValue {
            from,
            to,
            reason: InvalidValueReason::PrecisionLoss,
        })
    } else {
        Ok(converted)
    }
}

/// Converts a decimal exactly or lossily to a float.
///
/// Lossy mode accepts finite IEEE rounding. Exact mode additionally requires
/// converting the result back to reproduce `value`. Non-finite results are
/// reported as out of range using `from` and `to`.
fn decimal_to_f64(
    value: &BigDecimal,
    policy: NumericConversionPolicy,
    from: DataType,
    to: DataType,
) -> Result<f64, DataConversionError> {
    let converted = value.to_f64().unwrap_or(f64::INFINITY);
    if !converted.is_finite() {
        return Err(DataConversionError::InvalidValue {
            from,
            to,
            reason: InvalidValueReason::OutOfRange,
        });
    }
    if policy == NumericConversionPolicy::Exact
        && BigDecimal::from_f64(converted).as_ref() != Some(value)
    {
        Err(DataConversionError::InvalidValue {
            from,
            to,
            reason: InvalidValueReason::PrecisionLoss,
        })
    } else {
        Ok(converted)
    }
}

/// Converts a source to f64 before target-width validation.
///
/// `options` controls exactness and `to` identifies the eventual float target.
/// Returns contextual missing, unsupported, syntax, range, non-finite, or
/// precision errors when an `f64` intermediate cannot be produced.
fn source_to_f64(
    source: &DataConverter<'_>,
    options: &DataConversionOptions,
    to: DataType,
) -> Result<f64, DataConversionError> {
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
    ) -> Result<f64, DataConversionError> {
        source_to_f64(self, options, DataType::Float64)
    }
}

impl DataConvertTo<f32> for DataConverter<'_> {
    fn convert(
        &self,
        options: &DataConversionOptions,
    ) -> Result<f32, DataConversionError> {
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
            return Err(
                self.invalid(DataType::Float32, InvalidValueReason::OutOfRange)
            );
        }
        if options.numeric_policy == NumericConversionPolicy::Exact
            && f64::from(converted) != value
        {
            Err(self
                .invalid(DataType::Float32, InvalidValueReason::PrecisionLoss))
        } else {
            Ok(converted)
        }
    }
}

impl DataConvertTo<BigInt> for DataConverter<'_> {
    fn convert(
        &self,
        options: &DataConversionOptions,
    ) -> Result<BigInt, DataConversionError> {
        source_to_bigint(self, options, DataType::BigInteger)
    }
}

impl DataConvertTo<BigDecimal> for DataConverter<'_> {
    fn convert(
        &self,
        options: &DataConversionOptions,
    ) -> Result<BigDecimal, DataConversionError> {
        match self {
            Self::BigDecimal(value) => Ok(value.as_ref().clone()),
            Self::Float32(value) => match BigDecimal::from_f32(*value) {
                Some(value) => Ok(value),
                None => Err(self.invalid(
                    DataType::BigDecimal,
                    InvalidValueReason::NonFinite,
                )),
            },
            Self::Float64(value) => match BigDecimal::from_f64(*value) {
                Some(value) => Ok(value),
                None => Err(self.invalid(
                    DataType::BigDecimal,
                    InvalidValueReason::NonFinite,
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
                        InvalidValueReason::NonFinite,
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
