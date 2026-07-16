// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use std::cmp::Ordering;

use bigdecimal::BigDecimal;

use crate::NumericValueRef;

/// Converts a finite numeric value into an exact arbitrary-precision decimal.
///
/// `BigDecimal`'s primitive-float conversion decodes the IEEE significand and
/// exponent exactly; it does not use display formatting.
///
/// # Returns
///
/// The exact decimal value, or `None` for a non-finite or marker value.
fn to_exact_decimal(value: NumericValueRef<'_>) -> Option<BigDecimal> {
    match value {
        NumericValueRef::Int8(value) => Some(BigDecimal::from(value)),
        NumericValueRef::Int16(value) => Some(BigDecimal::from(value)),
        NumericValueRef::Int32(value) => Some(BigDecimal::from(value)),
        NumericValueRef::Int64(value) => Some(BigDecimal::from(value)),
        NumericValueRef::Int128(value) => Some(BigDecimal::from(value)),
        NumericValueRef::UInt8(value) => Some(BigDecimal::from(value)),
        NumericValueRef::UInt16(value) => Some(BigDecimal::from(value)),
        NumericValueRef::UInt32(value) => Some(BigDecimal::from(value)),
        NumericValueRef::UInt64(value) => Some(BigDecimal::from(value)),
        NumericValueRef::UInt128(value) => Some(BigDecimal::from(value)),
        NumericValueRef::Float32(value) => BigDecimal::try_from(value).ok(),
        NumericValueRef::Float64(value) => BigDecimal::try_from(value).ok(),
        NumericValueRef::BigInteger(value) => {
            Some(BigDecimal::from(value.clone()))
        }
        NumericValueRef::BigDecimal(value) => Some(value.clone()),
        NumericValueRef::__Lifetime(_) => None,
    }
}

/// Compares values through exact arbitrary-precision decimal representations.
///
/// # Returns
///
/// Their mathematical ordering, or `None` for a non-finite or marker value.
pub(in crate::numeric) fn compare_exact_decimal(
    left: NumericValueRef<'_>,
    right: NumericValueRef<'_>,
) -> Option<Ordering> {
    Some(to_exact_decimal(left)?.cmp(&to_exact_decimal(right)?))
}
