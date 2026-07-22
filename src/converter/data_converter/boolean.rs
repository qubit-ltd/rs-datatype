// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Boolean conversion implementations.

#[cfg(feature = "big-integer")]
use num_bigint::BigInt;
#[cfg(feature = "big-integer")]
use num_traits::Zero;

use super::DataConverter;
use super::numeric::{
    check_numeric_text_limit,
    is_integer_syntax,
};
use super::string_source::normalize;
use crate::converter::{
    BooleanNumericPolicy,
    DataConversionError,
    DataConversionOptions,
    DataConversionTarget,
    InvalidValueReason,
};
use crate::datatype::DataType;

/// Applies the configured integer-to-boolean policy.
///
/// # Parameters
///
/// * `zero` - Whether the parsed integer is zero.
/// * `one` - Whether the parsed integer is positive one.
/// * `policy` - Accepted numeric domain for boolean conversion.
/// * `from` - Source type retained in conversion errors.
///
/// # Returns
///
/// The boolean selected by `policy`.
///
/// # Errors
///
/// Returns an invalid-value [`DataConversionError`] when `policy` rejects the
/// integer.
fn integer_to_bool(
    zero: bool,
    one: bool,
    policy: BooleanNumericPolicy,
    from: DataType,
) -> Result<bool, DataConversionError> {
    match policy {
        BooleanNumericPolicy::ZeroOrOne if zero => Ok(false),
        BooleanNumericPolicy::ZeroOrOne if one => Ok(true),
        BooleanNumericPolicy::NonZero => Ok(!zero),
        BooleanNumericPolicy::ZeroOrOne | BooleanNumericPolicy::Reject => {
            Err(DataConversionError::invalid(
                from,
                DataType::Bool,
                InvalidValueReason::InvalidBoolean,
            ))
        }
    }
}

/// Converts string input to a boolean under the configured policies.
///
/// # Parameters
///
/// * `value` - Source string before configured normalization.
/// * `options` - String, Boolean literal, numeric policy, and text limits.
///
/// # Returns
///
/// The represented Boolean value.
///
/// # Errors
///
/// Returns a normalization or numeric-text limit error when those policies
/// reject the input, or an invalid-Boolean error for unknown literals and
/// rejected numeric values.
fn string_to_bool(
    value: &str,
    options: &DataConversionOptions,
) -> Result<bool, DataConversionError> {
    let value = normalize(value, options, DataType::Bool)?;
    if let Some(value) = options.boolean().parse(value) {
        return Ok(value);
    }
    if !is_integer_syntax(value) {
        return Err(DataConversionError::invalid(
            DataType::String,
            DataType::Bool,
            InvalidValueReason::InvalidBoolean,
        ));
    }
    check_numeric_text_limit(value, options, DataType::Bool)?;
    let digits = value.strip_prefix(['+', '-']).unwrap_or(value);
    let zero = digits.bytes().all(|byte| byte == b'0');
    let one = !value.starts_with('-') && digits.trim_start_matches('0') == "1";
    integer_to_bool(
        zero,
        one,
        options.boolean().numeric_policy(),
        DataType::String,
    )
}

/// Converts an arbitrary-precision integer to a boolean.
///
/// # Parameters
///
/// * `value` - Arbitrary-precision integer source.
/// * `options` - Boolean numeric policy.
///
/// # Returns
///
/// The Boolean selected by the numeric policy.
///
/// # Errors
///
/// Returns an invalid-Boolean error when the policy rejects `value`.
#[cfg(feature = "big-integer")]
fn big_integer_to_bool(
    value: &BigInt,
    options: &DataConversionOptions,
) -> Result<bool, DataConversionError> {
    integer_to_bool(
        value.is_zero(),
        value == &BigInt::from(1u8),
        options.boolean().numeric_policy(),
        DataType::BigInteger,
    )
}

impl DataConversionTarget for bool {
    /// Converts a borrowed runtime value to a Boolean.
    ///
    /// # Parameters
    ///
    /// * `source` - Borrowed runtime value to convert.
    /// * `options` - String normalization and Boolean conversion policies.
    ///
    /// # Returns
    ///
    /// The converted Boolean.
    ///
    /// # Errors
    ///
    /// Returns a missing, unsupported, invalid-Boolean, normalization, or
    /// numeric-text-limit error as applicable to the source.
    fn convert_from(
        source: &DataConverter<'_>,
        options: &DataConversionOptions,
    ) -> Result<Self, DataConversionError> {
        match source {
            DataConverter::Bool(value) => Ok(*value),
            DataConverter::String(value) => string_to_bool(value, options),
            DataConverter::Int8(value) => integer_to_bool(
                *value == 0,
                *value == 1,
                options.boolean().numeric_policy(),
                DataType::Int8,
            ),
            DataConverter::Int16(value) => integer_to_bool(
                *value == 0,
                *value == 1,
                options.boolean().numeric_policy(),
                DataType::Int16,
            ),
            DataConverter::Int32(value) => integer_to_bool(
                *value == 0,
                *value == 1,
                options.boolean().numeric_policy(),
                DataType::Int32,
            ),
            DataConverter::Int64(value) => integer_to_bool(
                *value == 0,
                *value == 1,
                options.boolean().numeric_policy(),
                DataType::Int64,
            ),
            DataConverter::Int128(value) => integer_to_bool(
                *value == 0,
                *value == 1,
                options.boolean().numeric_policy(),
                DataType::Int128,
            ),
            DataConverter::UInt8(value) => integer_to_bool(
                *value == 0,
                *value == 1,
                options.boolean().numeric_policy(),
                DataType::UInt8,
            ),
            DataConverter::UInt16(value) => integer_to_bool(
                *value == 0,
                *value == 1,
                options.boolean().numeric_policy(),
                DataType::UInt16,
            ),
            DataConverter::UInt32(value) => integer_to_bool(
                *value == 0,
                *value == 1,
                options.boolean().numeric_policy(),
                DataType::UInt32,
            ),
            DataConverter::UInt64(value) => integer_to_bool(
                *value == 0,
                *value == 1,
                options.boolean().numeric_policy(),
                DataType::UInt64,
            ),
            DataConverter::UInt128(value) => integer_to_bool(
                *value == 0,
                *value == 1,
                options.boolean().numeric_policy(),
                DataType::UInt128,
            ),
            #[cfg(feature = "big-integer")]
            DataConverter::BigInteger(value) => {
                big_integer_to_bool(value.as_ref(), options)
            }
            DataConverter::Unset(_) => Err(source.missing(DataType::Bool)),
            _ => Err(source.unsupported(DataType::Bool)),
        }
    }
}
