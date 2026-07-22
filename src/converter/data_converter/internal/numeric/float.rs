// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Primitive floating-point target dispatch.

use super::super::super::DataConverter;
#[cfg(feature = "big-integer")]
use super::float_big_number::{
    bigint_to_f32,
    bigint_to_f64,
};
#[cfg(feature = "big-decimal")]
use super::float_big_number::{
    decimal_to_f32,
    decimal_to_f64,
};
use super::float_text::{
    parse_text_f32,
    parse_text_f64,
};
use super::integer::scalar_integer_magnitude;
use super::syntax::normalize_numeric_text;
use crate::converter::{
    DataConversionError,
    DataConversionOptions,
    DataConversionTarget,
    FloatRoundingPolicy,
    InvalidValueReason,
};
use crate::datatype::DataType;

/// Tests whether an unsigned integer fits a floating-point mantissa exactly.
///
/// # Parameters
///
/// * `value` - Unsigned integer to inspect.
/// * `mantissa_digits` - Number of significant binary digits in the target.
///
/// # Returns
///
/// `true` when the target mantissa represents `value` exactly.
#[must_use]
#[inline]
fn unsigned_integer_is_exact(value: u128, mantissa_digits: u32) -> bool {
    if value == 0 {
        return true;
    }
    let significant_bits = u128::BITS - value.leading_zeros();
    significant_bits <= mantissa_digits
        || value.trailing_zeros() >= significant_bits - mantissa_digits
}

/// Converts an integer intermediate to `f64` under a float rounding policy.
///
/// # Parameters
///
/// * `value` - Sign and magnitude of the integer.
/// * `policy` - Exact or nearest-even float rounding policy.
/// * `from` - Source type retained in conversion errors.
/// * `to` - Target type retained in conversion errors.
///
/// # Returns
///
/// The represented `f64` value.
///
/// # Errors
///
/// Returns a precision error when exact conversion would round the integer.
fn integer_to_f64(
    value: (bool, u128),
    policy: FloatRoundingPolicy,
    from: DataType,
    to: DataType,
) -> Result<f64, DataConversionError> {
    let (negative, magnitude) = value;
    let converted = if negative {
        -(magnitude as f64)
    } else {
        magnitude as f64
    };
    let exact = unsigned_integer_is_exact(magnitude, f64::MANTISSA_DIGITS);
    if policy == FloatRoundingPolicy::Exact && !exact {
        Err(DataConversionError::invalid(
            from,
            to,
            InvalidValueReason::PrecisionLoss,
        ))
    } else {
        Ok(converted)
    }
}

/// Converts an integer intermediate to `f32` under a float rounding policy.
///
/// # Parameters
///
/// * `value` - Sign and magnitude of the integer.
/// * `policy` - Exact or nearest-even float rounding policy.
/// * `from` - Source type retained in conversion errors.
/// * `to` - Target type retained in conversion errors.
///
/// # Returns
///
/// The represented finite `f32` value.
///
/// # Errors
///
/// Returns a range error for overflow or a precision error when exact
/// conversion would round the integer.
fn integer_to_f32(
    value: (bool, u128),
    policy: FloatRoundingPolicy,
    from: DataType,
    to: DataType,
) -> Result<f32, DataConversionError> {
    let (negative, magnitude) = value;
    let converted = if negative {
        -(magnitude as f32)
    } else {
        magnitude as f32
    };
    if !converted.is_finite() {
        return Err(DataConversionError::invalid(
            from,
            to,
            InvalidValueReason::OutOfRange,
        ));
    }
    let exact = unsigned_integer_is_exact(magnitude, f32::MANTISSA_DIGITS);
    if policy == FloatRoundingPolicy::Exact && !exact {
        Err(DataConversionError::invalid(
            from,
            to,
            InvalidValueReason::PrecisionLoss,
        ))
    } else {
        Ok(converted)
    }
}

/// Converts a source to f64 before target-width validation.
///
/// # Parameters
///
/// * `source` - Borrowed source representation to convert.
/// * `options` - Numeric and string conversion policies.
/// * `to` - Eventual floating-point target type.
///
/// # Returns
///
/// An `f64` intermediate for target-width validation.
///
/// # Errors
///
/// Returns contextual missing, unsupported, syntax, range, non-finite, or
/// precision errors when an intermediate cannot be produced.
fn source_to_f64(
    source: &DataConverter<'_>,
    options: &DataConversionOptions,
    to: DataType,
) -> Result<f64, DataConversionError> {
    if let Some(value) = scalar_integer_magnitude(source) {
        return integer_to_f64(
            value,
            options.numeric().numeric_to_float(),
            source.data_type(),
            to,
        );
    }
    match source {
        DataConverter::Float64(value) => Ok(*value),
        DataConverter::Float32(value) => Ok(f64::from(*value)),
        #[cfg(feature = "big-integer")]
        DataConverter::BigInteger(value) => bigint_to_f64(
            value,
            options.numeric().numeric_to_float(),
            DataType::BigInteger,
            to,
        ),
        #[cfg(feature = "big-decimal")]
        DataConverter::BigDecimal(value) => decimal_to_f64(
            value,
            options.numeric().numeric_to_float(),
            DataType::BigDecimal,
            to,
        ),
        DataConverter::String(value) => {
            let value = normalize_numeric_text(value, options, to)?;
            parse_text_f64(value, options, to)
        }
        DataConverter::Unset(_) => Err(source.missing(to)),
        _ => Err(source.unsupported(to)),
    }
}

impl DataConversionTarget for f64 {
    /// Converts a borrowed runtime value to `f64`.
    ///
    /// # Parameters
    ///
    /// * `source` - Borrowed runtime value to convert.
    /// * `options` - Numeric syntax, rounding, and resource policies.
    ///
    /// # Returns
    ///
    /// The represented `f64` value.
    ///
    /// # Errors
    ///
    /// Returns a missing, unsupported, syntax, precision, range, normalization,
    /// or resource-limit error.
    #[inline(always)]
    fn convert_from(
        source: &DataConverter<'_>,
        options: &DataConversionOptions,
    ) -> Result<Self, DataConversionError> {
        source_to_f64(source, options, DataType::Float64)
    }
}

impl DataConversionTarget for f32 {
    /// Converts a borrowed runtime value to `f32`.
    ///
    /// # Parameters
    ///
    /// * `source` - Borrowed runtime value to convert.
    /// * `options` - Numeric syntax, rounding, and resource policies.
    ///
    /// # Returns
    ///
    /// The represented `f32` value.
    ///
    /// # Errors
    ///
    /// Returns a missing, unsupported, syntax, precision, range, normalization,
    /// or resource-limit error.
    fn convert_from(
        source: &DataConverter<'_>,
        options: &DataConversionOptions,
    ) -> Result<Self, DataConversionError> {
        let to = DataType::Float32;
        if let Some(value) = scalar_integer_magnitude(source) {
            return integer_to_f32(
                value,
                options.numeric().numeric_to_float(),
                source.data_type(),
                to,
            );
        }
        match source {
            DataConverter::Float32(value) => Ok(*value),
            DataConverter::Float64(value) => {
                if value.is_nan() {
                    return Ok(f32::NAN);
                }
                if *value == f64::INFINITY {
                    return Ok(f32::INFINITY);
                }
                if *value == f64::NEG_INFINITY {
                    return Ok(f32::NEG_INFINITY);
                }
                let converted = *value as f32;
                if !converted.is_finite() {
                    return Err(
                        source.invalid(to, InvalidValueReason::OutOfRange)
                    );
                }
                if options.numeric().numeric_to_float()
                    == FloatRoundingPolicy::Exact
                    && f64::from(converted) != *value
                {
                    Err(source.invalid(to, InvalidValueReason::PrecisionLoss))
                } else {
                    Ok(converted)
                }
            }
            #[cfg(feature = "big-integer")]
            DataConverter::BigInteger(value) => bigint_to_f32(
                value,
                options.numeric().numeric_to_float(),
                DataType::BigInteger,
                to,
            ),
            #[cfg(feature = "big-decimal")]
            DataConverter::BigDecimal(value) => decimal_to_f32(
                value,
                options.numeric().numeric_to_float(),
                DataType::BigDecimal,
                to,
            ),
            DataConverter::String(value) => {
                let value = normalize_numeric_text(value, options, to)?;
                parse_text_f32(value, options, to)
            }
            DataConverter::Unset(_) => Err(source.missing(to)),
            _ => Err(source.unsupported(to)),
        }
    }
}
