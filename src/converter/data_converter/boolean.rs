// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
// =============================================================================
//! Boolean conversion implementations.

#[cfg(feature = "big-number")]
use num_bigint::BigInt;
#[cfg(feature = "big-number")]
use num_traits::Zero;

use super::DataConverter;
use super::numeric::is_integer_syntax;
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
/// `zero` and `one` classify the already parsed integer, `policy` selects the
/// accepted numeric domain, and `from` is retained in any error. Returns the
/// mapped boolean, or [`DataConversionError::InvalidValue`] when the policy
/// rejects the value.
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
            Err(DataConversionError::InvalidValue {
                from,
                to: DataType::Bool,
                reason: InvalidValueReason::InvalidBoolean,
            })
        }
    }
}

impl DataConversionTarget for bool {
    fn convert_from(
        source: &DataConverter<'_>,
        options: &DataConversionOptions,
    ) -> Result<Self, DataConversionError> {
        match source {
            DataConverter::Bool(value) => Ok(*value),
            DataConverter::String(value) => {
                let value = normalize(value, options, DataType::Bool)?;
                if let Some(value) = options.boolean.parse(value) {
                    return Ok(value);
                }
                if is_integer_syntax(value) {
                    let digits =
                        value.strip_prefix(['+', '-']).unwrap_or(value);
                    let zero = digits.bytes().all(|byte| byte == b'0');
                    let one = !value.starts_with('-')
                        && digits.trim_start_matches('0') == "1";
                    integer_to_bool(
                        zero,
                        one,
                        options.boolean.numeric_policy(),
                        DataType::String,
                    )
                } else {
                    Err(source.invalid(
                        DataType::Bool,
                        InvalidValueReason::InvalidBoolean,
                    ))
                }
            }
            DataConverter::Int8(value) => integer_to_bool(
                *value == 0,
                *value == 1,
                options.boolean.numeric_policy(),
                DataType::Int8,
            ),
            DataConverter::Int16(value) => integer_to_bool(
                *value == 0,
                *value == 1,
                options.boolean.numeric_policy(),
                DataType::Int16,
            ),
            DataConverter::Int32(value) => integer_to_bool(
                *value == 0,
                *value == 1,
                options.boolean.numeric_policy(),
                DataType::Int32,
            ),
            DataConverter::Int64(value) => integer_to_bool(
                *value == 0,
                *value == 1,
                options.boolean.numeric_policy(),
                DataType::Int64,
            ),
            DataConverter::Int128(value) => integer_to_bool(
                *value == 0,
                *value == 1,
                options.boolean.numeric_policy(),
                DataType::Int128,
            ),
            DataConverter::UInt8(value) => integer_to_bool(
                *value == 0,
                *value == 1,
                options.boolean.numeric_policy(),
                DataType::UInt8,
            ),
            DataConverter::UInt16(value) => integer_to_bool(
                *value == 0,
                *value == 1,
                options.boolean.numeric_policy(),
                DataType::UInt16,
            ),
            DataConverter::UInt32(value) => integer_to_bool(
                *value == 0,
                *value == 1,
                options.boolean.numeric_policy(),
                DataType::UInt32,
            ),
            DataConverter::UInt64(value) => integer_to_bool(
                *value == 0,
                *value == 1,
                options.boolean.numeric_policy(),
                DataType::UInt64,
            ),
            DataConverter::UInt128(value) => integer_to_bool(
                *value == 0,
                *value == 1,
                options.boolean.numeric_policy(),
                DataType::UInt128,
            ),
            #[cfg(feature = "big-number")]
            DataConverter::BigInteger(value) => integer_to_bool(
                value.is_zero(),
                value.as_ref() == &BigInt::from(1u8),
                options.boolean.numeric_policy(),
                DataType::BigInteger,
            ),
            DataConverter::Empty(_) => Err(source.missing(DataType::Bool)),
            _ => Err(source.unsupported(DataType::Bool)),
        }
    }
}
