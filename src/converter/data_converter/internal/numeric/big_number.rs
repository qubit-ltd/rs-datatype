// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Arbitrary-precision numeric conversion implementations.

use std::time::Duration;

#[cfg(feature = "big-decimal")]
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_traits::FromPrimitive;

use super::super::super::DataConverter;
use super::integer::{
    duration_to_u128,
    scalar_integer_magnitude,
};
#[cfg(feature = "big-decimal")]
use super::parsed_number::ParsedNumber;
#[cfg(feature = "big-decimal")]
use super::syntax::parse_number;
use super::syntax::{
    normalize_numeric_text,
    parse_text_bigint,
};
use crate::converter::{
    ConversionLimit,
    DataConversionError,
    DataConversionOptions,
    DataConversionTarget,
    FractionalToIntegerPolicy,
    InvalidValueReason,
};
use crate::datatype::DataType;

/// Tests whether a non-zero BigInteger exceeds a decimal digit budget.
///
/// The bit-length checks avoid formatting values that are clearly far below or
/// above the limit. Values near the boundary use an exact decimal count whose
/// temporary allocation remains proportional to the configured budget.
///
/// # Parameters
///
/// * `value` - BigInteger value whose magnitude is inspected.
/// * `maximum_digits` - Largest permitted significant decimal digit count.
///
/// # Returns
///
/// `true` when the non-zero magnitude has more than `maximum_digits` decimal
/// digits; zero never exceeds the budget.
#[cfg(any(feature = "big-integer", feature = "big-decimal"))]
#[must_use]
fn exceeds_big_integer_digit_limit(
    value: &BigInt,
    maximum_digits: usize,
) -> bool {
    let bits = u128::from(value.bits());
    if bits == 0 {
        return false;
    }
    let maximum_digits = maximum_digits as u128;
    if bits <= maximum_digits.saturating_mul(3) {
        return false;
    }
    if bits > maximum_digits.saturating_mul(4) {
        return true;
    }
    value.to_str_radix(10).trim_start_matches('-').len() as u128
        > maximum_digits
}

/// Enforces the configured BigInteger result digit limit.
///
/// # Parameters
///
/// * `value` - Candidate BigInteger result.
/// * `maximum_digits` - Largest permitted significant decimal digit count.
/// * `from` - Source type retained in a limit error.
/// * `to` - Target type retained in a limit error.
///
/// # Returns
///
/// `Ok(())` when the target is not BigInteger or the result fits the budget.
///
/// # Errors
///
/// Returns [`DataConversionErrorKind::LimitExceeded`](crate::converter::DataConversionErrorKind::LimitExceeded)
/// when a BigInteger target would exceed `maximum_digits`.
#[cfg(any(feature = "big-integer", feature = "big-decimal"))]
fn enforce_big_integer_digit_limit(
    value: &BigInt,
    maximum_digits: usize,
    from: DataType,
    to: DataType,
) -> Result<(), DataConversionError> {
    if to != DataType::BigInteger
        || !exceeds_big_integer_digit_limit(value, maximum_digits)
    {
        return Ok(());
    }
    Err(DataConversionError::limit_exceeded(
        from,
        to,
        ConversionLimit::BigIntegerDigits {
            maximum: maximum_digits,
        },
    ))
}

/// Converts a decimal to an integer with exactness checks.
///
/// # Parameters
///
/// * `value` - Decimal value to convert.
/// * `policy` - Exactness policy for fractional values.
/// * `maximum_digits` - Maximum decimal digits for a `BigInt` target.
/// * `from` - Source type retained in conversion errors.
/// * `to` - Target type retained in conversion errors.
///
/// # Returns
///
/// The integral value; [`FractionalToIntegerPolicy::Truncate`] truncates toward
/// zero.
///
/// # Errors
///
/// Returns a precision error for a fractional value under
/// [`FractionalToIntegerPolicy::Reject`], a resource limit error when a
/// `BigInt` result would exceed `maximum_digits`, or a range error before
/// constructing an impractically large fixed-width intermediate.
#[cfg(feature = "big-decimal")]
pub(super) fn decimal_to_bigint(
    value: &BigDecimal,
    policy: FractionalToIntegerPolicy,
    maximum_digits: usize,
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
        let result_digits = coefficient_digits.saturating_add(exponent);
        if to == DataType::BigInteger
            && u128::from(result_digits) > maximum_digits as u128
        {
            return Err(DataConversionError::limit_exceeded(
                from,
                to,
                ConversionLimit::BigIntegerDigits {
                    maximum: maximum_digits,
                },
            ));
        }
        if (to.is_integer() && to != DataType::BigInteger && result_digits > 39)
            || exponent > u64::from(u32::MAX)
        {
            return Err(DataConversionError::invalid(
                from,
                to,
                InvalidValueReason::OutOfRange,
            ));
        }
        return Ok(coefficient * BigInt::from(10u8).pow(exponent as u32));
    }

    let coefficient_text = coefficient.to_str_radix(10);
    let coefficient_digits = coefficient_text.trim_start_matches('-').len();
    if scale as u64 >= coefficient_digits as u64 {
        return if policy == FractionalToIntegerPolicy::Reject {
            Err(DataConversionError::invalid(
                from,
                to,
                InvalidValueReason::PrecisionLoss,
            ))
        } else {
            Ok(BigInt::from(0u8))
        };
    }
    let scale = scale as usize;
    let magnitude = coefficient_text.trim_start_matches('-');
    if policy == FractionalToIntegerPolicy::Reject
        && magnitude[magnitude.len() - scale..]
            .bytes()
            .any(|digit| digit != b'0')
    {
        return Err(DataConversionError::invalid(
            from,
            to,
            InvalidValueReason::PrecisionLoss,
        ));
    }
    let result_digits = coefficient_digits - scale;
    if to == DataType::BigInteger && result_digits > maximum_digits {
        return Err(DataConversionError::limit_exceeded(
            from,
            to,
            ConversionLimit::BigIntegerDigits {
                maximum: maximum_digits,
            },
        ));
    }
    let divisor = BigInt::from(10u8).pow(scale as u32);
    let quotient = &coefficient / &divisor;
    Ok(quotient)
}

/// Converts a finite float to an integer with exactness checks.
///
/// # Parameters
///
/// * `value` - Floating-point value to convert.
/// * `policy` - Exactness policy for fractional values.
/// * `from` - Source type retained in conversion errors.
/// * `to` - Target type retained in conversion errors.
///
/// # Returns
///
/// A `BigInt` after truncation toward zero.
///
/// # Errors
///
/// Returns a non-finite error for NaN or infinity, or a precision error for a
/// fractional value under [`FractionalToIntegerPolicy::Reject`].
#[cfg(any(feature = "big-integer", feature = "big-decimal"))]
fn float_to_bigint(
    value: f64,
    policy: FractionalToIntegerPolicy,
    from: DataType,
    to: DataType,
) -> Result<BigInt, DataConversionError> {
    if !value.is_finite() {
        return Err(DataConversionError::invalid(
            from,
            to,
            InvalidValueReason::NonFinite,
        ));
    }
    let converted = BigInt::from_f64(value.trunc())
        .expect("finite primitive floats always have a BigInt representation");
    if policy == FractionalToIntegerPolicy::Reject && value.fract() != 0.0 {
        Err(DataConversionError::invalid(
            from,
            to,
            InvalidValueReason::PrecisionLoss,
        ))
    } else {
        Ok(converted)
    }
}

/// Extracts an arbitrary-precision integer from a supported source.
///
/// # Parameters
///
/// * `source` - Borrowed source representation to convert.
/// * `options` - Decimal, float, string, and duration conversion policies.
/// * `to` - Final target type retained in conversion errors.
///
/// # Returns
///
/// The source represented as an arbitrary-precision integer.
///
/// # Errors
///
/// Returns missing, unsupported, syntax, range, or precision errors with the
/// original source type when extraction fails.
#[cfg(any(feature = "big-integer", feature = "big-decimal"))]
pub(super) fn source_to_bigint(
    source: &DataConverter<'_>,
    options: &DataConversionOptions,
    to: DataType,
) -> Result<BigInt, DataConversionError> {
    let maximum_digits = options.numeric().limits().max_big_integer_digits();
    #[cfg(feature = "big-integer")]
    if to == DataType::BigInteger
        && let DataConverter::BigInteger(value) = source
    {
        enforce_big_integer_digit_limit(
            value.as_ref(),
            maximum_digits,
            DataType::BigInteger,
            to,
        )?;
        return Ok(value.as_ref().clone());
    }

    let result =
        if let Some((negative, magnitude)) = scalar_integer_magnitude(source) {
            let value = BigInt::from(magnitude);
            Ok(if negative { -value } else { value })
        } else {
            match source {
                DataConverter::Float32(value) => float_to_bigint(
                    f64::from(*value),
                    options.numeric().fractional_to_integer(),
                    DataType::Float32,
                    to,
                ),
                DataConverter::Float64(value) => float_to_bigint(
                    *value,
                    options.numeric().fractional_to_integer(),
                    DataType::Float64,
                    to,
                ),
                #[cfg(feature = "big-integer")]
                DataConverter::BigInteger(value) => Ok(value.as_ref().clone()),
                #[cfg(feature = "big-decimal")]
                DataConverter::BigDecimal(value) => decimal_to_bigint(
                    value.as_ref(),
                    options.numeric().fractional_to_integer(),
                    options.numeric().limits().max_big_integer_digits(),
                    DataType::BigDecimal,
                    to,
                ),
                DataConverter::String(value) => {
                    let value = normalize_numeric_text(value, options, to)?;
                    parse_text_bigint(
                        value,
                        options.numeric().fractional_to_integer(),
                        options.numeric().limits().max_big_integer_digits(),
                        to,
                    )
                }
                DataConverter::Duration(value) => {
                    duration_to_bigint(*value, options, to)
                }
                DataConverter::Unset(_) => Err(source.missing(to)),
                _ => Err(source.unsupported(to)),
            }
        }?;
    enforce_big_integer_digit_limit(
        &result,
        maximum_digits,
        source.data_type(),
        to,
    )?;
    Ok(result)
}

/// Converts a duration to arbitrary-precision integer units.
///
/// # Parameters
///
/// * `duration` - Duration to convert to configured units.
/// * `options` - Duration output unit and rounding policy.
/// * `to` - Target type retained in conversion errors.
///
/// # Returns
///
/// The configured unit count as a `BigInt`.
///
/// # Errors
///
/// Returns a precision error when exact conversion would lose a sub-unit
/// remainder.
#[cfg(any(feature = "big-integer", feature = "big-decimal"))]
#[inline(always)]
pub(super) fn duration_to_bigint(
    duration: Duration,
    options: &DataConversionOptions,
    to: DataType,
) -> Result<BigInt, DataConversionError> {
    duration_to_u128(duration, options, to).map(BigInt::from)
}

/// Creates the conversion error used for non-finite decimal sources.
///
/// # Parameters
///
/// * `source` - Source converter whose type is retained in the error.
///
/// # Returns
///
/// An invalid-value error targeting [`DataType::BigDecimal`].
#[cfg(feature = "big-decimal")]
#[inline(always)]
fn non_finite_big_decimal_error(
    source: &DataConverter<'_>,
) -> DataConversionError {
    source.invalid(DataType::BigDecimal, InvalidValueReason::NonFinite)
}

/// Parses normalized text into an arbitrary-precision decimal.
///
/// # Parameters
///
/// * `source` - Source converter used to retain error context.
/// * `value` - Raw text to normalize and parse.
/// * `options` - Conversion policies and resource limits.
///
/// # Returns
///
/// The exact decimal value represented by `value`.
///
/// # Errors
///
/// Returns a normalization, resource-limit, syntax, or non-finite-value error.
#[cfg(feature = "big-decimal")]
fn parse_big_decimal(
    source: &DataConverter<'_>,
    value: &str,
    options: &DataConversionOptions,
) -> Result<BigDecimal, DataConversionError> {
    let value = normalize_numeric_text(value, options, DataType::BigDecimal)?;
    match parse_number(value, DataType::BigDecimal)? {
        ParsedNumber::Integer(value) => Ok(BigDecimal::from(value)),
        ParsedNumber::Decimal(value) => Ok(value),
        ParsedNumber::NaN
        | ParsedNumber::PositiveInfinity
        | ParsedNumber::NegativeInfinity => {
            Err(non_finite_big_decimal_error(source))
        }
    }
}

#[cfg(feature = "big-integer")]
impl DataConversionTarget for BigInt {
    /// Converts a borrowed runtime value to an arbitrary-precision integer.
    ///
    /// # Parameters
    ///
    /// * `source` - Borrowed runtime value to convert.
    /// * `options` - Numeric rounding and BigInteger digit-limit policies.
    ///
    /// # Returns
    ///
    /// The represented arbitrary-precision integer.
    ///
    /// # Errors
    ///
    /// Returns a missing, unsupported, syntax, non-finite, precision, range,
    /// normalization, or digit-limit error.
    #[inline(always)]
    fn convert_from(
        source: &DataConverter<'_>,
        options: &DataConversionOptions,
    ) -> Result<Self, DataConversionError> {
        source_to_bigint(source, options, DataType::BigInteger)
    }

    /// Converts a runtime value to an arbitrary-precision integer, consuming
    /// it.
    ///
    /// # Parameters
    ///
    /// * `source` - Runtime value to consume.
    /// * `options` - Numeric rounding and BigInteger digit-limit policies.
    ///
    /// # Returns
    ///
    /// The converted integer; an owned BigInteger reuses its storage.
    ///
    /// # Errors
    ///
    /// Returns the same conversion errors as [`Self::convert_from`].
    fn convert_owned(
        source: DataConverter<'_>,
        options: &DataConversionOptions,
    ) -> Result<Self, DataConversionError> {
        match source {
            DataConverter::BigInteger(value) => {
                let maximum_digits =
                    options.numeric().limits().max_big_integer_digits();
                enforce_big_integer_digit_limit(
                    value.as_ref(),
                    maximum_digits,
                    DataType::BigInteger,
                    DataType::BigInteger,
                )?;
                Ok(value.into_owned())
            }
            source => Self::convert_from(&source, options),
        }
    }
}

#[cfg(feature = "big-decimal")]
impl DataConversionTarget for BigDecimal {
    /// Converts a borrowed runtime value to an arbitrary-precision decimal.
    ///
    /// # Parameters
    ///
    /// * `source` - Borrowed runtime value to convert.
    /// * `options` - Numeric syntax, rounding, and resource policies.
    ///
    /// # Returns
    ///
    /// The represented arbitrary-precision decimal.
    ///
    /// # Errors
    ///
    /// Returns a missing, unsupported, syntax, non-finite, normalization, or
    /// resource-limit error.
    fn convert_from(
        source: &DataConverter<'_>,
        options: &DataConversionOptions,
    ) -> Result<Self, DataConversionError> {
        match source {
            DataConverter::BigDecimal(value) => Ok(value.as_ref().clone()),
            DataConverter::Float32(value) => BigDecimal::from_f32(*value)
                .ok_or_else(|| non_finite_big_decimal_error(source)),
            DataConverter::Float64(value) => BigDecimal::from_f64(*value)
                .ok_or_else(|| non_finite_big_decimal_error(source)),
            DataConverter::String(value) => {
                parse_big_decimal(source, value, options)
            }
            DataConverter::Unset(_) => {
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

    /// Converts a runtime value to an arbitrary-precision decimal, consuming
    /// it.
    ///
    /// # Parameters
    ///
    /// * `source` - Runtime value to consume.
    /// * `options` - Numeric syntax, rounding, and resource policies.
    ///
    /// # Returns
    ///
    /// The converted decimal; an owned BigDecimal reuses its storage.
    ///
    /// # Errors
    ///
    /// Returns the same conversion errors as [`Self::convert_from`].
    fn convert_owned(
        source: DataConverter<'_>,
        options: &DataConversionOptions,
    ) -> Result<Self, DataConversionError> {
        match source {
            DataConverter::BigDecimal(value) => Ok(value.into_owned()),
            source => Self::convert_from(&source, options),
        }
    }
}
