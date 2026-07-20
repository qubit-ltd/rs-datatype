// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Arbitrary-precision source conversions to primitive floating-point targets.

#[cfg(feature = "big-decimal")]
use bigdecimal::BigDecimal;
#[cfg(feature = "big-integer")]
use num_bigint::BigInt;
use num_traits::{FromPrimitive, ToPrimitive};

use crate::converter::{DataConversionError, FloatRoundingPolicy, InvalidValueReason};
use crate::datatype::DataType;

/// Converts an integer to a float under an explicit rounding policy.
///
/// [`FloatRoundingPolicy::NearestEven`] accepts finite IEEE rounding.
/// [`FloatRoundingPolicy::Exact`] additionally requires converting the result
/// back to reproduce `value`. Non-finite results are reported as out of range
/// using `from` and `to`.
///
/// # Parameters
///
/// * `value` - Arbitrary-precision integer to convert.
/// * `policy` - Exact or nearest-even float rounding policy.
/// * `from` - Source type retained in conversion errors.
/// * `to` - Target type retained in conversion errors.
///
/// # Returns
///
/// The finite `f64` representation.
///
/// # Errors
///
/// Returns a range error for overflow or a precision error when exact
/// conversion would round the integer.
#[cfg(feature = "big-integer")]
pub(super) fn bigint_to_f64(
    value: &BigInt,
    policy: FloatRoundingPolicy,
    from: DataType,
    to: DataType,
) -> Result<f64, DataConversionError> {
    let converted = value.to_f64().unwrap_or(f64::INFINITY);
    if !converted.is_finite() {
        return Err(DataConversionError::invalid(
            from,
            to,
            InvalidValueReason::OutOfRange,
        ));
    }
    if policy == FloatRoundingPolicy::Exact && BigInt::from_f64(converted).as_ref() != Some(value) {
        Err(DataConversionError::invalid(
            from,
            to,
            InvalidValueReason::PrecisionLoss,
        ))
    } else {
        Ok(converted)
    }
}

/// Converts an integer exactly or lossily to an `f32`.
///
/// # Parameters
///
/// * `value` - Arbitrary-precision integer to convert.
/// * `policy` - Exact or nearest-even float rounding policy.
/// * `from` - Source type retained in conversion errors.
/// * `to` - Target type retained in conversion errors.
///
/// # Returns
///
/// The finite `f32` representation.
///
/// # Errors
///
/// Returns a range error for overflow or a precision error when exact
/// conversion would round the integer.
#[cfg(feature = "big-integer")]
pub(super) fn bigint_to_f32(
    value: &BigInt,
    policy: FloatRoundingPolicy,
    from: DataType,
    to: DataType,
) -> Result<f32, DataConversionError> {
    let converted = value.to_f32().unwrap_or(f32::INFINITY);
    if !converted.is_finite() {
        return Err(DataConversionError::invalid(
            from,
            to,
            InvalidValueReason::OutOfRange,
        ));
    }
    if policy == FloatRoundingPolicy::Exact && BigInt::from_f32(converted).as_ref() != Some(value) {
        Err(DataConversionError::invalid(
            from,
            to,
            InvalidValueReason::PrecisionLoss,
        ))
    } else {
        Ok(converted)
    }
}

/// Converts a decimal to a float under an explicit rounding policy.
///
/// [`FloatRoundingPolicy::NearestEven`] accepts finite IEEE rounding.
/// [`FloatRoundingPolicy::Exact`] additionally requires converting the result
/// back to reproduce `value`. Non-finite results are reported as out of range
/// using `from` and `to`.
///
/// # Parameters
///
/// * `value` - Arbitrary-precision decimal to convert.
/// * `policy` - Exact or nearest-even float rounding policy.
/// * `from` - Source type retained in conversion errors.
/// * `to` - Target type retained in conversion errors.
///
/// # Returns
///
/// The finite `f64` representation.
///
/// # Errors
///
/// Returns a range error for overflow or a precision error when exact
/// conversion would round the decimal.
#[cfg(feature = "big-decimal")]
pub(super) fn decimal_to_f64(
    value: &BigDecimal,
    policy: FloatRoundingPolicy,
    from: DataType,
    to: DataType,
) -> Result<f64, DataConversionError> {
    let converted = value.to_f64().unwrap_or(f64::INFINITY);
    if !converted.is_finite() {
        return Err(DataConversionError::invalid(
            from,
            to,
            InvalidValueReason::OutOfRange,
        ));
    }
    if policy == FloatRoundingPolicy::Exact
        && BigDecimal::from_f64(converted).as_ref() != Some(value)
    {
        Err(DataConversionError::invalid(
            from,
            to,
            InvalidValueReason::PrecisionLoss,
        ))
    } else {
        Ok(converted)
    }
}

/// Converts a decimal exactly or lossily to an `f32`.
///
/// # Parameters
///
/// * `value` - Arbitrary-precision decimal to convert.
/// * `policy` - Exact or nearest-even float rounding policy.
/// * `from` - Source type retained in conversion errors.
/// * `to` - Target type retained in conversion errors.
///
/// # Returns
///
/// The finite `f32` representation.
///
/// # Errors
///
/// Returns a range error for overflow or a precision error when exact
/// conversion would round the decimal.
#[cfg(feature = "big-decimal")]
pub(super) fn decimal_to_f32(
    value: &BigDecimal,
    policy: FloatRoundingPolicy,
    from: DataType,
    to: DataType,
) -> Result<f32, DataConversionError> {
    let converted = value.to_f32().unwrap_or(f32::INFINITY);
    if !converted.is_finite() {
        return Err(DataConversionError::invalid(
            from,
            to,
            InvalidValueReason::OutOfRange,
        ));
    }
    if policy == FloatRoundingPolicy::Exact
        && BigDecimal::from_f32(converted).as_ref() != Some(value)
    {
        Err(DataConversionError::invalid(
            from,
            to,
            InvalidValueReason::PrecisionLoss,
        ))
    } else {
        Ok(converted)
    }
}
