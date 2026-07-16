// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
// =============================================================================
//! Numeric conversion implementations.

use std::time::Duration;

#[cfg(feature = "big-number")]
use bigdecimal::BigDecimal;
#[cfg(feature = "big-number")]
use num_bigint::BigInt;
#[cfg(feature = "big-number")]
use num_traits::{
    FromPrimitive,
    ToPrimitive,
};
#[cfg(feature = "big-number")]
use std::str::FromStr;

use super::DataConverter;
use super::string_source::normalize;
use crate::converter::{
    DataConversionError,
    DataConversionOptions,
    DataConversionTarget,
    InvalidValueReason,
    NumericConversionPolicy,
};
use crate::datatype::DataType;

/// Parsed representation shared by textual numeric conversions.
#[cfg(feature = "big-number")]
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
#[cfg(feature = "big-number")]
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

/// Returns a platform-independent `(negative, magnitude)` representation.
fn signed_magnitude(value: i128) -> (bool, u128) {
    (value.is_negative(), value.unsigned_abs())
}

/// Creates a contextual invalid numeric syntax error.
fn invalid_numeric_syntax(to: DataType) -> DataConversionError {
    DataConversionError::InvalidValue {
        from: DataType::String,
        to,
        reason: InvalidValueReason::InvalidSyntax {
            expected: numeric_syntax(to),
        },
    }
}

/// Parses decimal text into a platform-independent integer intermediate.
///
/// Exact mode rejects a non-zero fractional part. Lossy mode truncates toward
/// zero. Exponents are processed structurally, so extreme values are rejected
/// without allocating an exponent-sized buffer.
fn parse_text_integer(
    value: &str,
    policy: NumericConversionPolicy,
    to: DataType,
) -> Result<(bool, u128), DataConversionError> {
    let lower = value.to_ascii_lowercase();
    if matches!(
        lower.as_str(),
        "nan"
            | "inf"
            | "+inf"
            | "-inf"
            | "infinity"
            | "+infinity"
            | "-infinity"
    ) {
        return Err(DataConversionError::InvalidValue {
            from: DataType::String,
            to,
            reason: InvalidValueReason::NonFinite,
        });
    }

    let (negative, unsigned) = match value.as_bytes().first() {
        Some(b'-') => (true, &value[1..]),
        Some(b'+') => (false, &value[1..]),
        _ => (false, value),
    };
    if unsigned.is_empty() {
        return Err(invalid_numeric_syntax(to));
    }

    let exponent_index = unsigned
        .bytes()
        .position(|byte| matches!(byte, b'e' | b'E'));
    let (mantissa, exponent) = if let Some(index) = exponent_index {
        let mantissa = &unsigned[..index];
        let exponent_text = &unsigned[index + 1..];
        let exponent_bytes = exponent_text.as_bytes();
        if exponent_bytes.is_empty()
            || exponent_bytes[1..]
                .iter()
                .any(|byte| !byte.is_ascii_digit())
            || !matches!(exponent_bytes[0], b'+' | b'-' | b'0'..=b'9')
        {
            return Err(invalid_numeric_syntax(to));
        }
        let digits = exponent_text
            .strip_prefix(['+', '-'])
            .unwrap_or(exponent_text);
        if digits.is_empty()
            || digits.bytes().any(|byte| !byte.is_ascii_digit())
        {
            return Err(invalid_numeric_syntax(to));
        }
        let exponent = match exponent_text.parse::<i64>() {
            Ok(exponent) => exponent,
            Err(_) if exponent_text.starts_with('-') => i64::MIN,
            Err(_) => i64::MAX,
        };
        (mantissa, exponent)
    } else {
        (unsigned, 0)
    };
    if mantissa.is_empty() {
        return Err(invalid_numeric_syntax(to));
    }

    let mut digits = String::with_capacity(mantissa.len());
    let mut decimal_seen = false;
    let mut fractional_digits = 0usize;
    for byte in mantissa.bytes() {
        match byte {
            b'0'..=b'9' => {
                digits.push(char::from(byte));
                if decimal_seen {
                    fractional_digits += 1;
                }
            }
            b'.' if !decimal_seen => decimal_seen = true,
            _ => return Err(invalid_numeric_syntax(to)),
        }
    }
    if digits.is_empty() {
        return Err(invalid_numeric_syntax(to));
    }

    let decimal_position =
        (digits.len() - fractional_digits) as i128 + i128::from(exponent);
    let integer_digit_count = if decimal_position <= 0 {
        0
    } else {
        usize::try_from(decimal_position)
            .unwrap_or(usize::MAX)
            .min(digits.len())
    };
    let fractional_non_zero = if decimal_position <= 0 {
        digits.bytes().any(|byte| byte != b'0')
    } else if decimal_position < digits.len() as i128 {
        digits.as_bytes()[integer_digit_count..]
            .iter()
            .any(|byte| *byte != b'0')
    } else {
        false
    };
    if policy == NumericConversionPolicy::Exact && fractional_non_zero {
        return Err(DataConversionError::InvalidValue {
            from: DataType::String,
            to,
            reason: InvalidValueReason::PrecisionLoss,
        });
    }

    let mut magnitude = 0u128;
    for byte in digits.bytes().take(integer_digit_count) {
        let Some(next) = magnitude.checked_mul(10) else {
            return Err(DataConversionError::InvalidValue {
                from: DataType::String,
                to,
                reason: InvalidValueReason::OutOfRange,
            });
        };
        let Some(next) = next.checked_add(u128::from(byte - b'0')) else {
            return Err(DataConversionError::InvalidValue {
                from: DataType::String,
                to,
                reason: InvalidValueReason::OutOfRange,
            });
        };
        magnitude = next;
    }
    if decimal_position > digits.len() as i128 && magnitude != 0 {
        let zero_count =
            match u32::try_from(decimal_position - digits.len() as i128) {
                Ok(zero_count) => zero_count,
                Err(_) => {
                    return Err(DataConversionError::InvalidValue {
                        from: DataType::String,
                        to,
                        reason: InvalidValueReason::OutOfRange,
                    });
                }
            };
        let multiplier = 10u128.checked_pow(zero_count).ok_or(
            DataConversionError::InvalidValue {
                from: DataType::String,
                to,
                reason: InvalidValueReason::OutOfRange,
            },
        )?;
        magnitude = magnitude.checked_mul(multiplier).ok_or(
            DataConversionError::InvalidValue {
                from: DataType::String,
                to,
                reason: InvalidValueReason::OutOfRange,
            },
        )?;
    }

    Ok((negative && magnitude != 0, magnitude))
}

/// Converts a finite primitive float to an integer intermediate.
fn float_to_integer(
    value: f64,
    policy: NumericConversionPolicy,
    from: DataType,
    to: DataType,
) -> Result<(bool, u128), DataConversionError> {
    if !value.is_finite() {
        return Err(DataConversionError::InvalidValue {
            from,
            to,
            reason: InvalidValueReason::NonFinite,
        });
    }
    if policy == NumericConversionPolicy::Exact && value.fract() != 0.0 {
        return Err(DataConversionError::InvalidValue {
            from,
            to,
            reason: InvalidValueReason::PrecisionLoss,
        });
    }
    match parse_text_integer(
        &value.trunc().to_string(),
        NumericConversionPolicy::Lossy,
        to,
    ) {
        Ok(value) => Ok(value),
        Err(DataConversionError::InvalidValue { to, reason, .. }) => {
            Err(DataConversionError::InvalidValue { from, to, reason })
        }
        Err(other) => Err(other),
    }
}

/// Extracts an integer intermediate from a supported source.
pub(super) fn source_to_integer(
    source: &DataConverter<'_>,
    options: &DataConversionOptions,
    to: DataType,
) -> Result<(bool, u128), DataConversionError> {
    match source {
        DataConverter::Bool(value) => Ok((false, u128::from(*value))),
        DataConverter::Char(value) => Ok((false, u128::from(*value as u32))),
        DataConverter::Int8(value) => Ok(signed_magnitude(i128::from(*value))),
        DataConverter::Int16(value) => Ok(signed_magnitude(i128::from(*value))),
        DataConverter::Int32(value) => Ok(signed_magnitude(i128::from(*value))),
        DataConverter::Int64(value) => Ok(signed_magnitude(i128::from(*value))),
        DataConverter::Int128(value) => Ok(signed_magnitude(*value)),
        DataConverter::UInt8(value) => Ok((false, u128::from(*value))),
        DataConverter::UInt16(value) => Ok((false, u128::from(*value))),
        DataConverter::UInt32(value) => Ok((false, u128::from(*value))),
        DataConverter::UInt64(value) => Ok((false, u128::from(*value))),
        DataConverter::UInt128(value) => Ok((false, *value)),
        DataConverter::Float32(value) => float_to_integer(
            f64::from(*value),
            options.numeric_policy,
            DataType::Float32,
            to,
        ),
        DataConverter::Float64(value) => float_to_integer(
            *value,
            options.numeric_policy,
            DataType::Float64,
            to,
        ),
        #[cfg(feature = "big-number")]
        DataConverter::BigInteger(value) => {
            if let Some(value) = value.to_i128() {
                Ok(signed_magnitude(value))
            } else if let Some(value) = value.to_u128() {
                Ok((false, value))
            } else {
                Err(DataConversionError::InvalidValue {
                    from: DataType::BigInteger,
                    to,
                    reason: InvalidValueReason::OutOfRange,
                })
            }
        }
        #[cfg(feature = "big-number")]
        DataConverter::BigDecimal(value) => {
            let integer = decimal_to_bigint(
                value,
                options.numeric_policy,
                DataType::BigDecimal,
                to,
            )?;
            if let Some(value) = integer.to_i128() {
                Ok(signed_magnitude(value))
            } else if let Some(value) = integer.to_u128() {
                Ok((false, value))
            } else {
                Err(DataConversionError::InvalidValue {
                    from: DataType::BigDecimal,
                    to,
                    reason: InvalidValueReason::OutOfRange,
                })
            }
        }
        DataConverter::String(value) => {
            let value = normalize(value, options, to)?;
            parse_text_integer(value, options.numeric_policy, to)
        }
        DataConverter::Duration(value) => {
            Ok((false, duration_to_u128(*value, options, to)?))
        }
        DataConverter::Empty(_) => Err(source.missing(to)),
        _ => Err(source.unsupported(to)),
    }
}

/// Converts a decimal to an integer with exactness checks.
///
/// `from` and `to` are used only as error context. Returns the integral value;
/// exact mode rejects any fractional remainder, while lossy mode truncates
/// toward zero. Values that cannot reasonably fit a primitive target are
/// rejected before constructing an impractically large power of ten.
#[cfg(feature = "big-number")]
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
#[cfg(feature = "big-number")]
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
#[cfg(feature = "big-number")]
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
        DataConverter::UInt8(value) => Ok(BigInt::from(*value)),
        DataConverter::UInt16(value) => Ok(BigInt::from(*value)),
        DataConverter::UInt32(value) => Ok(BigInt::from(*value)),
        DataConverter::UInt64(value) => Ok(BigInt::from(*value)),
        DataConverter::UInt128(value) => Ok(BigInt::from(*value)),
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
            if !is_integer_syntax(value) {
                return Err(DataConversionError::InvalidValue {
                    from: DataType::String,
                    to,
                    reason: InvalidValueReason::InvalidSyntax {
                        expected: "[+-]?[0-9]+",
                    },
                });
            }
            match BigInt::from_str(value) {
                Ok(value) => Ok(value),
                Err(_) => Err(DataConversionError::InvalidValue {
                    from: DataType::String,
                    to,
                    reason: InvalidValueReason::InvalidSyntax {
                        expected: "[+-]?[0-9]+",
                    },
                }),
            }
        }
        DataConverter::Duration(value) => {
            duration_to_bigint(*value, options, to)
        }
        DataConverter::Empty(_) => Err(source.missing(to)),
        _ => Err(source.unsupported(to)),
    }
}

/// Converts a duration to unsigned integer units under the numeric policy.
///
/// The duration unit comes from `options`. Exact mode rejects a remainder;
/// lossy mode uses half-up rounding. `to` is retained as target context.
pub(super) fn duration_to_u128(
    duration: Duration,
    options: &DataConversionOptions,
    to: DataType,
) -> Result<u128, DataConversionError> {
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
    Ok(
        if options.numeric_policy == NumericConversionPolicy::Exact {
            total_nanos / unit_nanos
        } else {
            options.duration.output_unit.rounded_units(duration)
        },
    )
}

/// Converts a duration to arbitrary-precision integer units.
#[cfg(feature = "big-number")]
pub(super) fn duration_to_bigint(
    duration: Duration,
    options: &DataConversionOptions,
    to: DataType,
) -> Result<BigInt, DataConversionError> {
    duration_to_u128(duration, options, to).map(BigInt::from)
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
    let (negative, magnitude) = source_to_integer(source, options, to)?;
    if negative && magnitude == 1u128 << 127 {
        return Ok(i128::MIN);
    }
    let value = match i128::try_from(magnitude) {
        Ok(value) => value,
        Err(_) => {
            return Err(DataConversionError::InvalidValue {
                from: source.data_type(),
                to,
                reason: InvalidValueReason::OutOfRange,
            });
        }
    };
    Ok(if negative { -value } else { value })
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
    let (negative, magnitude) = source_to_integer(source, options, to)?;
    if negative {
        Err(DataConversionError::InvalidValue {
            from: source.data_type(),
            to,
            reason: InvalidValueReason::OutOfRange,
        })
    } else {
        Ok(magnitude)
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
        impl DataConversionTarget for $target {
            fn convert_from(
                source: &DataConverter<'_>,
                options: &DataConversionOptions,
            ) -> Result<Self, DataConversionError> {
                checked_signed(
                    to_i128(source, options, $data_type)?,
                    source,
                    $data_type,
                )
            }
        }
    };
}

macro_rules! impl_unsigned_target {
    ($target:ty, $data_type:expr) => {
        impl DataConversionTarget for $target {
            fn convert_from(
                source: &DataConverter<'_>,
                options: &DataConversionOptions,
            ) -> Result<Self, DataConversionError> {
                checked_unsigned(
                    to_u128(source, options, $data_type)?,
                    source,
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
impl_unsigned_target!(u8, DataType::UInt8);
impl_unsigned_target!(u16, DataType::UInt16);
impl_unsigned_target!(u32, DataType::UInt32);
impl_unsigned_target!(u64, DataType::UInt64);
impl_unsigned_target!(u128, DataType::UInt128);

/// Converts an integer exactly or lossily to a float.
///
/// Lossy mode accepts finite IEEE rounding. Exact mode additionally requires
/// converting the result back to reproduce `value`. Non-finite results are
/// reported as out of range using `from` and `to`.
#[cfg(feature = "big-number")]
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
#[cfg(feature = "big-number")]
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

/// Tests whether an unsigned integer is represented exactly by `f64`.
fn unsigned_integer_is_exact_f64(value: u128) -> bool {
    if value == 0 {
        return true;
    }
    let significant_bits = u128::BITS - value.leading_zeros();
    significant_bits <= f64::MANTISSA_DIGITS
        || value.trailing_zeros() >= significant_bits - f64::MANTISSA_DIGITS
}

/// Converts an integer intermediate to `f64` under the numeric policy.
fn integer_to_f64(
    value: (bool, u128),
    policy: NumericConversionPolicy,
    from: DataType,
    to: DataType,
) -> Result<f64, DataConversionError> {
    let (negative, magnitude) = value;
    let converted = if negative {
        -(magnitude as f64)
    } else {
        magnitude as f64
    };
    let exact = unsigned_integer_is_exact_f64(magnitude);
    if policy == NumericConversionPolicy::Exact && !exact {
        Err(DataConversionError::InvalidValue {
            from,
            to,
            reason: InvalidValueReason::PrecisionLoss,
        })
    } else {
        Ok(converted)
    }
}

/// Tests whether decimal text denotes an exactly representable finite `f64`.
fn text_is_exact_f64(value: &str, converted: f64, to: DataType) -> bool {
    let unsigned = value.strip_prefix(['+', '-']).unwrap_or(value);
    let exponent_index = unsigned
        .bytes()
        .position(|byte| matches!(byte, b'e' | b'E'));
    let (mantissa, exponent) = if let Some(index) = exponent_index {
        let Ok(exponent) = unsigned[index + 1..].parse::<i64>() else {
            return false;
        };
        (&unsigned[..index], exponent)
    } else {
        (unsigned, 0)
    };

    let mut coefficient = 0u128;
    let mut decimal_seen = false;
    let mut fractional_digits = 0i128;
    for byte in mantissa.bytes() {
        match byte {
            b'0'..=b'9' => {
                let Some(next) = coefficient.checked_mul(10) else {
                    return false;
                };
                let Some(next) = next.checked_add(u128::from(byte - b'0'))
                else {
                    return false;
                };
                coefficient = next;
                if decimal_seen {
                    fractional_digits += 1;
                }
            }
            b'.' if !decimal_seen => decimal_seen = true,
            _ => return false,
        }
    }
    if coefficient == 0 {
        return converted == 0.0;
    }

    let scale = fractional_digits - i128::from(exponent);
    if scale <= 0 {
        let Ok(integer) =
            parse_text_integer(value, NumericConversionPolicy::Exact, to)
        else {
            return false;
        };
        return integer_to_f64(
            integer,
            NumericConversionPolicy::Exact,
            DataType::String,
            to,
        )
        .is_ok();
    }
    let Ok(scale) = u32::try_from(scale) else {
        return false;
    };
    for _ in 0..scale {
        if !coefficient.is_multiple_of(5) {
            return false;
        }
        coefficient /= 5;
    }
    let cancelled_twos = coefficient.trailing_zeros().min(scale);
    coefficient >>= cancelled_twos;
    let significant_bits = u128::BITS - coefficient.leading_zeros();
    converted != 0.0 && significant_bits <= f64::MANTISSA_DIGITS
}

/// Parses a text source as `f64` and enforces exactness when requested.
fn parse_text_f64(
    value: &str,
    options: &DataConversionOptions,
    to: DataType,
) -> Result<f64, DataConversionError> {
    let lower = value.to_ascii_lowercase();
    let explicit_non_finite = matches!(
        lower.as_str(),
        "nan"
            | "inf"
            | "+inf"
            | "-inf"
            | "infinity"
            | "+infinity"
            | "-infinity"
    );
    let converted = match lower.as_str() {
        "nan" => f64::NAN,
        "inf" | "+inf" | "infinity" | "+infinity" => f64::INFINITY,
        "-inf" | "-infinity" => f64::NEG_INFINITY,
        _ => value
            .parse::<f64>()
            .map_err(|_| invalid_numeric_syntax(to))?,
    };
    if !explicit_non_finite && !converted.is_finite() {
        return Err(DataConversionError::InvalidValue {
            from: DataType::String,
            to,
            reason: InvalidValueReason::OutOfRange,
        });
    }
    if options.numeric_policy == NumericConversionPolicy::Exact
        && converted.is_finite()
        && !text_is_exact_f64(value, converted, to)
    {
        Err(DataConversionError::InvalidValue {
            from: DataType::String,
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
        DataConverter::Int8(value) => integer_to_f64(
            signed_magnitude(i128::from(*value)),
            options.numeric_policy,
            DataType::Int8,
            to,
        ),
        DataConverter::Int16(value) => integer_to_f64(
            signed_magnitude(i128::from(*value)),
            options.numeric_policy,
            DataType::Int16,
            to,
        ),
        DataConverter::Int32(value) => integer_to_f64(
            signed_magnitude(i128::from(*value)),
            options.numeric_policy,
            DataType::Int32,
            to,
        ),
        DataConverter::Int64(value) => integer_to_f64(
            signed_magnitude(i128::from(*value)),
            options.numeric_policy,
            DataType::Int64,
            to,
        ),
        DataConverter::Int128(value) => integer_to_f64(
            signed_magnitude(*value),
            options.numeric_policy,
            DataType::Int128,
            to,
        ),
        DataConverter::UInt8(value) => integer_to_f64(
            (false, u128::from(*value)),
            options.numeric_policy,
            DataType::UInt8,
            to,
        ),
        DataConverter::UInt16(value) => integer_to_f64(
            (false, u128::from(*value)),
            options.numeric_policy,
            DataType::UInt16,
            to,
        ),
        DataConverter::UInt32(value) => integer_to_f64(
            (false, u128::from(*value)),
            options.numeric_policy,
            DataType::UInt32,
            to,
        ),
        DataConverter::UInt64(value) => integer_to_f64(
            (false, u128::from(*value)),
            options.numeric_policy,
            DataType::UInt64,
            to,
        ),
        DataConverter::UInt128(value) => integer_to_f64(
            (false, *value),
            options.numeric_policy,
            DataType::UInt128,
            to,
        ),
        #[cfg(feature = "big-number")]
        DataConverter::BigInteger(value) => bigint_to_f64(
            value,
            options.numeric_policy,
            DataType::BigInteger,
            to,
        ),
        #[cfg(feature = "big-number")]
        DataConverter::BigDecimal(value) => decimal_to_f64(
            value,
            options.numeric_policy,
            DataType::BigDecimal,
            to,
        ),
        DataConverter::String(value) => {
            let value = normalize(value, options, to)?;
            parse_text_f64(value, options, to)
        }
        DataConverter::Empty(_) => Err(source.missing(to)),
        _ => Err(source.unsupported(to)),
    }
}

impl DataConversionTarget for f64 {
    fn convert_from(
        source: &DataConverter<'_>,
        options: &DataConversionOptions,
    ) -> Result<Self, DataConversionError> {
        source_to_f64(source, options, DataType::Float64)
    }
}

impl DataConversionTarget for f32 {
    fn convert_from(
        source: &DataConverter<'_>,
        options: &DataConversionOptions,
    ) -> Result<Self, DataConversionError> {
        if let DataConverter::Float32(value) = source {
            return Ok(*value);
        }
        let value = source_to_f64(source, options, DataType::Float32)?;
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
            return Err(source
                .invalid(DataType::Float32, InvalidValueReason::OutOfRange));
        }
        if options.numeric_policy == NumericConversionPolicy::Exact
            && f64::from(converted) != value
        {
            Err(source
                .invalid(DataType::Float32, InvalidValueReason::PrecisionLoss))
        } else {
            Ok(converted)
        }
    }
}

#[cfg(feature = "big-number")]
impl DataConversionTarget for BigInt {
    fn convert_from(
        source: &DataConverter<'_>,
        options: &DataConversionOptions,
    ) -> Result<Self, DataConversionError> {
        source_to_bigint(source, options, DataType::BigInteger)
    }
}

#[cfg(feature = "big-number")]
impl DataConversionTarget for BigDecimal {
    fn convert_from(
        source: &DataConverter<'_>,
        options: &DataConversionOptions,
    ) -> Result<Self, DataConversionError> {
        match source {
            DataConverter::BigDecimal(value) => Ok(value.as_ref().clone()),
            DataConverter::Float32(value) => match BigDecimal::from_f32(*value)
            {
                Some(value) => Ok(value),
                None => Err(source.invalid(
                    DataType::BigDecimal,
                    InvalidValueReason::NonFinite,
                )),
            },
            DataConverter::Float64(value) => match BigDecimal::from_f64(*value)
            {
                Some(value) => Ok(value),
                None => Err(source.invalid(
                    DataType::BigDecimal,
                    InvalidValueReason::NonFinite,
                )),
            },
            DataConverter::String(value) => {
                let value = normalize(value, options, DataType::BigDecimal)?;
                match parse_number(value, DataType::BigDecimal)? {
                    ParsedNumber::Integer(value) => Ok(BigDecimal::from(value)),
                    ParsedNumber::Decimal(value) => Ok(value),
                    ParsedNumber::NaN
                    | ParsedNumber::PositiveInfinity
                    | ParsedNumber::NegativeInfinity => Err(source.invalid(
                        DataType::BigDecimal,
                        InvalidValueReason::NonFinite,
                    )),
                }
            }
            DataConverter::Empty(_) => {
                Err(source.missing(DataType::BigDecimal))
            }
            DataConverter::Duration(_) | DataConverter::StringMap(_) => {
                Err(source.unsupported(DataType::BigDecimal))
            }
            #[cfg(feature = "chrono")]
            DataConverter::Date(_)
            | DataConverter::Time(_)
            | DataConverter::DateTime(_)
            | DataConverter::Instant(_) => {
                Err(source.unsupported(DataType::BigDecimal))
            }
            #[cfg(feature = "url")]
            DataConverter::Url(_) => {
                Err(source.unsupported(DataType::BigDecimal))
            }
            #[cfg(feature = "json")]
            DataConverter::Json(_) => {
                Err(source.unsupported(DataType::BigDecimal))
            }
            _ => Ok(BigDecimal::from(source_to_bigint(
                source,
                options,
                DataType::BigDecimal,
            )?)),
        }
    }
}
