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
    DataConvertTo,
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

impl DataConvertTo<bool> for DataConverter<'_> {
    fn convert(
        &self,
        options: &DataConversionOptions,
    ) -> Result<bool, DataConversionError> {
        match self {
            Self::Bool(value) => Ok(*value),
            Self::String(value) => {
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
                    Err(self.invalid(
                        DataType::Bool,
                        InvalidValueReason::InvalidBoolean,
                    ))
                }
            }
            Self::Int8(value) => integer_to_bool(
                *value == 0,
                *value == 1,
                options.boolean.numeric_policy(),
                DataType::Int8,
            ),
            Self::Int16(value) => integer_to_bool(
                *value == 0,
                *value == 1,
                options.boolean.numeric_policy(),
                DataType::Int16,
            ),
            Self::Int32(value) => integer_to_bool(
                *value == 0,
                *value == 1,
                options.boolean.numeric_policy(),
                DataType::Int32,
            ),
            Self::Int64(value) => integer_to_bool(
                *value == 0,
                *value == 1,
                options.boolean.numeric_policy(),
                DataType::Int64,
            ),
            Self::Int128(value) => integer_to_bool(
                *value == 0,
                *value == 1,
                options.boolean.numeric_policy(),
                DataType::Int128,
            ),
            Self::UInt8(value) => integer_to_bool(
                *value == 0,
                *value == 1,
                options.boolean.numeric_policy(),
                DataType::UInt8,
            ),
            Self::UInt16(value) => integer_to_bool(
                *value == 0,
                *value == 1,
                options.boolean.numeric_policy(),
                DataType::UInt16,
            ),
            Self::UInt32(value) => integer_to_bool(
                *value == 0,
                *value == 1,
                options.boolean.numeric_policy(),
                DataType::UInt32,
            ),
            Self::UInt64(value) => integer_to_bool(
                *value == 0,
                *value == 1,
                options.boolean.numeric_policy(),
                DataType::UInt64,
            ),
            Self::UInt128(value) => integer_to_bool(
                *value == 0,
                *value == 1,
                options.boolean.numeric_policy(),
                DataType::UInt128,
            ),
            #[cfg(feature = "big-number")]
            Self::BigInteger(value) => integer_to_bool(
                value.is_zero(),
                value.as_ref() == &BigInt::from(1u8),
                options.boolean.numeric_policy(),
                DataType::BigInteger,
            ),
            Self::Empty(_) => Err(self.missing(DataType::Bool)),
            _ => Err(self.unsupported(DataType::Bool)),
        }
    }
}
