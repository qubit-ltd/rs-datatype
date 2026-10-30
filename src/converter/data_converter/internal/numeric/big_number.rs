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
use qubit_budget::BigIntegerLimits;
use qubit_budget::BudgetError;
use qubit_budget::MeasuredBudgetError;
use qubit_budget::Observation;
use qubit_budget::ResourceLimit;

use super::super::super::DataConverter;
use super::integer::duration_to_u128;
use super::integer::scalar_integer_magnitude;
#[cfg(feature = "big-decimal")]
use super::parsed_number::ParsedNumber;
use super::syntax::normalize_numeric_text;
#[cfg(feature = "big-decimal")]
use super::syntax::parse_number;
use super::syntax::parse_text_bigint;
use crate::converter::BuiltinDataConversionTarget;
use crate::converter::ConversionContext;
use crate::converter::ConversionPolicy;
use crate::converter::DataConversionError;
use crate::converter::FractionalToIntegerPolicy;
use crate::converter::InvalidValueReason;
use crate::converter::NumericConversionLimits;
use crate::datatype::DataType;

/// Checks BigInteger limits using the BigInt version shared by decimal types.
///
/// # Parameters
///
/// * `limits` - Configured magnitude and significant-digit limits.
/// * `value` - Integer value to measure.
///
/// # Returns
///
/// `Ok(())` when all configured limits accept `value`.
///
/// # Errors
///
/// Returns a measured budget error when a limit rejects the value or a native
/// measurement cannot fit the budget quantity.
#[cfg(any(feature = "big-integer", feature = "big-decimal"))]
#[inline]
fn check_big_integer_limits(
    limits: &BigIntegerLimits<crate::converter::ConversionResource>,
    value: &BigInt,
) -> Result<(), MeasuredBudgetError<crate::converter::ConversionResource>> {
    limits.check(value).map_err(|error| match error {
        MeasuredBudgetError::Budget(BudgetError::LimitExceeded {
            resource,
            observed: Observation::AtLeast(_),
            maximum,
        }) if limits
            .significant_decimal_digits_limit()
            .is_some_and(|limit| *limit.resource() == resource && limit.maximum() == maximum) =>
        {
            // A failing lower bound guarantees maximum is below u64::MAX.
            let observed = maximum
                .checked_add(1)
                .expect("a rejected lower bound exceeds the limit");
            MeasuredBudgetError::Budget(BudgetError::LimitExceeded {
                resource,
                observed: Observation::AtLeast(observed),
                maximum,
            })
        }
        error => error,
    })
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
/// when a BigInteger target would exceed `maximum_digits`. Clearly oversized
/// values report a lower bound without formatting their full decimal expansion.
/// Returns a quantity error if the measurement cannot fit the budget quantity.
#[cfg(any(feature = "big-integer", feature = "big-decimal"))]
#[inline]
fn enforce_big_integer_digit_limit(
    value: &BigInt,
    maximum_digits: u64,
    from: DataType,
    to: DataType,
) -> Result<(), DataConversionError> {
    if to != DataType::BigInteger {
        return Ok(());
    }
    // Since 2^3 < 10, at most three bits per permitted decimal digit always
    // fit. Widen before multiplying so generous u64 limits keep this check
    // allocation-free instead of overflowing into decimal formatting.
    if u128::from(value.bits()) <= u128::from(maximum_digits) * 3 {
        return Ok(());
    }
    let limits = BigIntegerLimits::builder()
        .significant_decimal_digits_limit(ResourceLimit::new(
            crate::converter::ConversionResource::BigIntegerDigits,
            maximum_digits,
        ))
        .build();
    check_big_integer_limits(&limits, value).map_err(|error| match error {
        MeasuredBudgetError::Budget(error) => DataConversionError::limit_exceeded(from, to, error),
        MeasuredBudgetError::Quantity { resource, source } => DataConversionError::quantity(from, to, resource, source),
    })
}

/// Computes a signed coefficient's magnitude remainder without formatting it.
///
/// # Parameters
///
/// * `coefficient` - Integer whose magnitude is reduced.
/// * `divisor` - Nonzero divisor that fits in `u64`.
///
/// # Returns
///
/// The remainder of the coefficient magnitude modulo `divisor`.
///
/// # Errors
///
/// This helper does not return errors. Its caller must provide a nonzero
/// divisor; the widened recurrence stays within `u128` for every `u64` divisor.
#[cfg(feature = "big-decimal")]
fn coefficient_remainder(coefficient: &BigInt, divisor: u64) -> u64 {
    let divisor = u128::from(divisor);
    coefficient.iter_u64_digits().rev().fold(0_u128, |remainder, digit| {
        ((remainder << 64) | u128::from(digit)) % divisor
    }) as u64
}

/// Rejects clearly oversized positive-scale integer results before formatting.
///
/// # Parameters
///
/// * `coefficient` - Nonzero decimal coefficient.
/// * `scale` - Positive decimal scale.
/// * `policy` - Exactness policy for fractional values.
/// * `maximum_digits` - Maximum decimal digits for a `BigInt` target.
/// * `from` - Source type retained in conversion errors.
/// * `to` - Target type retained in conversion errors.
///
/// # Returns
///
/// `Ok(())` when the conservative digit lower bound does not prove that the
/// result exceeds the relevant limit, or when the target is not an integer.
///
/// # Errors
///
/// Returns a BigInteger digit-limit error with a lower-bound observation or a
/// fixed-width out-of-range error when oversize is proven. Under strict policy,
/// scales up to 19 are checked for exact divisibility first. Larger scales
/// check divisibility by 10^19 as a necessary condition; a zero remainder
/// still defers to the original exactness and range checks.
#[cfg(feature = "big-decimal")]
fn precheck_positive_scale_integer(
    coefficient: &BigInt,
    scale: u64,
    policy: FractionalToIntegerPolicy,
    maximum_digits: u64,
    from: DataType,
    to: DataType,
) -> Result<(), DataConversionError> {
    let threshold = if to == DataType::BigInteger {
        maximum_digits
    } else if to.is_integer() {
        39
    } else {
        return Ok(());
    };
    let lower_bound = ((coefficient.bits() - 1) / 4 + 1).saturating_sub(scale);
    if lower_bound <= threshold {
        return Ok(());
    }
    if policy == FractionalToIntegerPolicy::Reject {
        let divisor = 10_u64.pow(scale.min(19) as u32);
        if coefficient_remainder(coefficient, divisor) != 0 {
            return Err(DataConversionError::invalid(
                from,
                to,
                InvalidValueReason::PrecisionLoss,
            ));
        }
        if scale > 19 {
            return Ok(());
        }
    }
    if to == DataType::BigInteger {
        Err(DataConversionError::limit_exceeded(
            from,
            to,
            BudgetError::LimitExceeded {
                resource: crate::converter::ConversionResource::BigIntegerDigits,
                observed: Observation::AtLeast(lower_bound),
                maximum: maximum_digits,
            },
        ))
    } else {
        Err(DataConversionError::invalid(from, to, InvalidValueReason::OutOfRange))
    }
}

/// Enforces coefficient and scale bounds before a decimal is consumed.
#[cfg(feature = "big-decimal")]
#[inline]
pub(super) fn enforce_big_decimal_limits(
    value: &BigDecimal,
    limits: &NumericConversionLimits,
    from: DataType,
    to: DataType,
) -> Result<(), DataConversionError> {
    limits.big_decimal().check(value).map_err(|error| match error {
        MeasuredBudgetError::Budget(error) => DataConversionError::limit_exceeded(from, to, error),
        MeasuredBudgetError::Quantity { resource, source } => DataConversionError::quantity(from, to, resource, source),
    })
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
/// `BigInt` result would exceed `maximum_digits`, or a fixed-width range error
/// before constructing the scale divisor for an oversized result. A `BigInt`
/// digit-limit error from a conservative precheck may report a lower bound
/// rather than an exact digit count; values near the limit use the exact path.
/// For positive scales, a bit-derived lower bound rejects clearly oversized
/// integer results before formatting the coefficient. Under strict policy,
/// scales up to 19 are checked for exact divisibility first. Larger scales
/// check divisibility by 10^19 as a necessary condition; a zero remainder
/// still defers to the original exactness and range checks. This optimization
/// does not bound all allocations for every decimal conversion.
#[cfg(feature = "big-decimal")]
pub(super) fn decimal_to_bigint(
    value: &BigDecimal,
    policy: FractionalToIntegerPolicy,
    maximum_digits: u64,
    from: DataType,
    to: DataType,
) -> Result<BigInt, DataConversionError> {
    let (coefficient, scale) = value.as_bigint_and_scale();
    if coefficient.as_ref() == &BigInt::from(0u8) {
        return Ok(coefficient.into_owned());
    }
    if scale <= 0 {
        let exponent = scale.unsigned_abs();
        // For a nonzero b-bit coefficient, at least floor((b - 1) / 4) + 1
        // decimal digits exist because 2^4 >= 10. Multiplication by 10^exponent
        // adds exponent digits. Saturation keeps this a conservative lower bound.
        let coefficient_digits_lower_bound = (coefficient.bits() - 1) / 4 + 1;
        let result_digits_lower_bound = coefficient_digits_lower_bound.saturating_add(exponent);
        if to == DataType::BigInteger && result_digits_lower_bound > maximum_digits {
            return Err(DataConversionError::limit_exceeded(
                from,
                to,
                BudgetError::LimitExceeded {
                    resource: crate::converter::ConversionResource::BigIntegerDigits,
                    observed: Observation::AtLeast(result_digits_lower_bound),
                    maximum: maximum_digits,
                },
            ));
        }
        if to.is_integer() && to != DataType::BigInteger && result_digits_lower_bound > 39 {
            return Err(DataConversionError::invalid(from, to, InvalidValueReason::OutOfRange));
        }
        let coefficient_digits = coefficient.to_str_radix(10).trim_start_matches('-').len() as u64;
        let result_digits = coefficient_digits.saturating_add(exponent);
        if to == DataType::BigInteger && u128::from(result_digits) > maximum_digits as u128 {
            return Err(DataConversionError::limit_exceeded(
                from,
                to,
                BudgetError::LimitExceeded {
                    resource: crate::converter::ConversionResource::BigIntegerDigits,
                    observed: Observation::Exact(result_digits),
                    maximum: maximum_digits,
                },
            ));
        }
        if (to.is_integer() && to != DataType::BigInteger && result_digits > 39) || exponent > u64::from(u32::MAX) {
            return Err(DataConversionError::invalid(from, to, InvalidValueReason::OutOfRange));
        }
        return Ok(coefficient.as_ref() * BigInt::from(10u8).pow(exponent as u32));
    }

    precheck_positive_scale_integer(coefficient.as_ref(), scale as u64, policy, maximum_digits, from, to)?;
    let coefficient_text = coefficient.as_ref().to_str_radix(10);
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
        && magnitude[magnitude.len() - scale..].bytes().any(|digit| digit != b'0')
    {
        return Err(DataConversionError::invalid(
            from,
            to,
            InvalidValueReason::PrecisionLoss,
        ));
    }
    let result_digits = coefficient_digits - scale;
    if to == DataType::BigInteger && u64::try_from(result_digits).unwrap() > maximum_digits {
        return Err(DataConversionError::limit_exceeded(
            from,
            to,
            BudgetError::LimitExceeded {
                resource: crate::converter::ConversionResource::BigIntegerDigits,
                observed: Observation::Exact(u64::try_from(result_digits).unwrap()),
                maximum: maximum_digits,
            },
        ));
    }
    if to.is_integer() && to != DataType::BigInteger && result_digits > 39 {
        return Err(DataConversionError::invalid(from, to, InvalidValueReason::OutOfRange));
    }
    let divisor = BigInt::from(10u8).pow(scale as u32);
    let quotient = coefficient.as_ref() / &divisor;
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
///
/// # Panics
///
/// Panics only if `BigInt` cannot represent a truncated finite primitive
/// float, which would violate `BigInt`'s finite-float conversion contract.
#[cfg(any(feature = "big-integer", feature = "big-decimal"))]
fn float_to_bigint(
    value: f64,
    policy: FractionalToIntegerPolicy,
    from: DataType,
    to: DataType,
) -> Result<BigInt, DataConversionError> {
    if !value.is_finite() {
        return Err(DataConversionError::invalid(from, to, InvalidValueReason::NonFinite));
    }
    let converted =
        BigInt::from_f64(value.trunc()).expect("finite primitive floats always have a BigInt representation");
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
/// * `limits` - Numeric text, integer digit, and decimal coefficient limits.
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
    options: &ConversionPolicy,
    limits: &NumericConversionLimits,
    to: DataType,
) -> Result<BigInt, DataConversionError> {
    let maximum_digits = limits.max_big_integer_digits();
    #[cfg(feature = "big-integer")]
    if to == DataType::BigInteger
        && let DataConverter::BigInteger(value) = source
    {
        #[cfg(feature = "big-integer")]
        check_big_integer_limits(limits.big_integer(), value.as_ref()).map_err(|error| match error {
            MeasuredBudgetError::Budget(error) => DataConversionError::limit_exceeded(DataType::BigInteger, to, error),
            MeasuredBudgetError::Quantity { resource, source } => {
                DataConversionError::quantity(DataType::BigInteger, to, resource, source)
            }
        })?;
        #[cfg(not(feature = "big-integer"))]
        enforce_big_integer_digit_limit(value.as_ref(), maximum_digits, DataType::BigInteger, to)?;
        return Ok(value.as_ref().clone());
    }

    let result = if let Some((negative, magnitude)) = scalar_integer_magnitude(source) {
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
            DataConverter::Float64(value) => {
                float_to_bigint(*value, options.numeric().fractional_to_integer(), DataType::Float64, to)
            }
            #[cfg(feature = "big-integer")]
            DataConverter::BigInteger(value) => Ok(value.as_ref().clone()),
            #[cfg(feature = "big-decimal")]
            DataConverter::BigDecimal(value) => decimal_to_bigint(
                value.as_ref(),
                options.numeric().fractional_to_integer(),
                limits.max_big_integer_digits(),
                DataType::BigDecimal,
                to,
            ),
            DataConverter::String(value) => {
                let value = normalize_numeric_text(value, options, limits, to)?;
                parse_text_bigint(
                    value,
                    options.numeric().fractional_to_integer(),
                    limits.max_big_integer_digits(),
                    to,
                )
            }
            DataConverter::Duration(value) => duration_to_bigint(*value, options, to),
            DataConverter::Unset(_) => Err(source.missing(to)),
            _ => Err(source.unsupported(to)),
        }
    }?;
    enforce_big_integer_digit_limit(&result, maximum_digits, source.data_type(), to)?;
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
#[inline]
pub(super) fn duration_to_bigint(
    duration: Duration,
    options: &ConversionPolicy,
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
#[inline]
fn non_finite_big_decimal_error(source: &DataConverter<'_>) -> DataConversionError {
    source.invalid(DataType::BigDecimal, InvalidValueReason::NonFinite)
}

/// Parses normalized text into an arbitrary-precision decimal.
///
/// # Parameters
///
/// * `source` - Source converter used to retain error context.
/// * `value` - Raw text to normalize and parse.
/// * `options` - String normalization and numeric parsing policies.
/// * `limits` - Numeric text, digit, and decimal-coefficient limits.
///
/// # Returns
///
/// The exact decimal value represented by `value`.
///
/// # Errors
///
/// Returns a normalization, resource-limit, syntax, or non-finite-value error.
#[cfg(feature = "big-decimal")]
#[inline]
fn parse_big_decimal(
    source: &DataConverter<'_>,
    value: &str,
    options: &ConversionPolicy,
    limits: &NumericConversionLimits,
) -> Result<BigDecimal, DataConversionError> {
    let value = normalize_numeric_text(value, options, limits, DataType::BigDecimal)?;
    let decimal = match parse_number(value, DataType::BigDecimal)? {
        ParsedNumber::Integer(value) => BigDecimal::from(value),
        ParsedNumber::Decimal(value) => value,
        ParsedNumber::NaN | ParsedNumber::PositiveInfinity | ParsedNumber::NegativeInfinity => {
            return Err(non_finite_big_decimal_error(source));
        }
    };
    enforce_big_decimal_limits(&decimal, limits, source.data_type(), DataType::BigDecimal)?;
    Ok(decimal)
}

#[cfg(feature = "big-integer")]
impl BuiltinDataConversionTarget for BigInt {
    /// Converts a borrowed runtime value to an arbitrary-precision integer.
    ///
    /// # Parameters
    ///
    /// * `source` - Borrowed runtime value to convert.
    /// * `context` - Shared conversion session binding numeric policies and
    ///   limits; nested conversions share its operation budget.
    ///
    /// # Returns
    ///
    /// The represented arbitrary-precision integer.
    ///
    /// # Errors
    ///
    /// Returns a missing, unsupported, syntax, non-finite, precision, range,
    /// normalization, or digit-limit error.
    #[inline]
    fn convert_from(
        source: &DataConverter<'_>,
        context: &mut ConversionContext<'_, '_>,
    ) -> Result<Self, DataConversionError> {
        let options = context.policy();
        let limits = context.limits().numeric();
        source_to_bigint(source, options, limits, DataType::BigInteger)
    }

    /// Converts a runtime value to an arbitrary-precision integer, consuming
    /// it.
    ///
    /// # Parameters
    ///
    /// * `source` - Runtime value to consume.
    /// * `context` - Shared conversion session binding numeric policies and
    ///   limits; nested conversions share its operation budget.
    ///
    /// # Returns
    ///
    /// The converted integer; an owned BigInteger reuses its storage.
    ///
    /// # Errors
    ///
    /// Returns the same conversion errors as [`Self::convert_from`].
    #[inline]
    fn convert_owned(
        source: DataConverter<'_>,
        context: &mut ConversionContext<'_, '_>,
    ) -> Result<Self, DataConversionError> {
        let limits = context.limits().numeric();
        match source {
            DataConverter::BigInteger(value) => {
                let maximum_digits = limits.max_big_integer_digits();
                enforce_big_integer_digit_limit(
                    value.as_ref(),
                    maximum_digits,
                    DataType::BigInteger,
                    DataType::BigInteger,
                )?;
                Ok(value.into_owned())
            }
            source => Self::convert_from(&source, context),
        }
    }
}

#[cfg(feature = "big-decimal")]
impl BuiltinDataConversionTarget for BigDecimal {
    /// Converts a borrowed runtime value to an arbitrary-precision decimal.
    ///
    /// # Parameters
    ///
    /// * `source` - Borrowed runtime value to convert.
    /// * `context` - Shared conversion session binding numeric policies and
    ///   limits to this target conversion.
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
        context: &mut ConversionContext<'_, '_>,
    ) -> Result<Self, DataConversionError> {
        let options = context.policy();
        let limits = context.limits().numeric();
        match source {
            DataConverter::BigDecimal(value) => {
                enforce_big_decimal_limits(value.as_ref(), limits, DataType::BigDecimal, DataType::BigDecimal)?;
                Ok(value.as_ref().clone())
            }
            DataConverter::Float32(value) => {
                let decimal = BigDecimal::from_f32(*value).ok_or_else(|| non_finite_big_decimal_error(source))?;
                enforce_big_decimal_limits(&decimal, limits, DataType::Float32, DataType::BigDecimal)?;
                Ok(decimal)
            }
            DataConverter::Float64(value) => {
                let decimal = BigDecimal::from_f64(*value).ok_or_else(|| non_finite_big_decimal_error(source))?;
                enforce_big_decimal_limits(&decimal, limits, DataType::Float64, DataType::BigDecimal)?;
                Ok(decimal)
            }
            DataConverter::String(value) => parse_big_decimal(source, value, options, limits),
            DataConverter::Unset(_) => Err(source.missing(DataType::BigDecimal)),
            DataConverter::Duration(_) | DataConverter::StringMap(_) => Err(source.unsupported(DataType::BigDecimal)),
            #[cfg(feature = "chrono")]
            DataConverter::Date(_)
            | DataConverter::Time(_)
            | DataConverter::DateTime(_)
            | DataConverter::Instant(_) => Err(source.unsupported(DataType::BigDecimal)),
            #[cfg(feature = "url")]
            DataConverter::Url(_) => Err(source.unsupported(DataType::BigDecimal)),
            #[cfg(feature = "json")]
            DataConverter::Json(_) => Err(source.unsupported(DataType::BigDecimal)),
            _ => {
                let decimal = BigDecimal::from(source_to_bigint(source, options, limits, DataType::BigDecimal)?);
                enforce_big_decimal_limits(&decimal, limits, source.data_type(), DataType::BigDecimal)?;
                Ok(decimal)
            }
        }
    }

    /// Converts a runtime value to an arbitrary-precision decimal, consuming
    /// it.
    ///
    /// # Parameters
    ///
    /// * `source` - Runtime value to consume.
    /// * `context` - Shared conversion session binding decimal policies and
    ///   numeric limits to this target conversion.
    ///
    /// # Returns
    ///
    /// The converted decimal; an owned BigDecimal reuses its storage.
    ///
    /// # Errors
    ///
    /// Returns the same conversion errors as [`Self::convert_from`].
    #[inline]
    fn convert_owned(
        source: DataConverter<'_>,
        context: &mut ConversionContext<'_, '_>,
    ) -> Result<Self, DataConversionError> {
        match source {
            DataConverter::BigDecimal(value) => {
                enforce_big_decimal_limits(
                    value.as_ref(),
                    context.limits().numeric(),
                    DataType::BigDecimal,
                    DataType::BigDecimal,
                )?;
                Ok(value.into_owned())
            }
            source => Self::convert_from(&source, context),
        }
    }
}
