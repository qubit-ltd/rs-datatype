// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Exact arbitrary-precision decimal comparison helpers.

use std::cmp::Ordering;

use bigdecimal::BigDecimal;

use crate::NumberRef;
use crate::numeric::internal::NumberRepr;

/// Converts a finite numeric value into an exact arbitrary-precision decimal.
///
/// `BigDecimal`'s primitive-float conversion decodes the IEEE significand and
/// exponent exactly; it does not use display formatting.
///
/// # Parameters
///
/// * `value` - Finite numeric representation to convert.
///
/// # Returns
///
/// The exact decimal value, or `None` for a non-finite value.
fn to_exact_decimal(value: NumberRef<'_>) -> Option<BigDecimal> {
    match value.inner() {
        NumberRepr::Int8(value) => Some(BigDecimal::from(value)),
        NumberRepr::Int16(value) => Some(BigDecimal::from(value)),
        NumberRepr::Int32(value) => Some(BigDecimal::from(value)),
        NumberRepr::Int64(value) => Some(BigDecimal::from(value)),
        NumberRepr::Int128(value) => Some(BigDecimal::from(value)),
        NumberRepr::UInt8(value) => Some(BigDecimal::from(value)),
        NumberRepr::UInt16(value) => Some(BigDecimal::from(value)),
        NumberRepr::UInt32(value) => Some(BigDecimal::from(value)),
        NumberRepr::UInt64(value) => Some(BigDecimal::from(value)),
        NumberRepr::UInt128(value) => Some(BigDecimal::from(value)),
        NumberRepr::Float32(value) => BigDecimal::try_from(value).ok(),
        NumberRepr::Float64(value) => BigDecimal::try_from(value).ok(),
        #[cfg(feature = "big-integer")]
        NumberRepr::BigInteger(value) => Some(BigDecimal::from(value.clone())),
        NumberRepr::BigDecimal(value) => Some(value.clone()),
    }
}

/// Compares values through exact arbitrary-precision decimal representations.
///
/// # Parameters
///
/// * `left` - Left numeric operand.
/// * `right` - Right numeric operand.
///
/// # Returns
///
/// Their mathematical ordering, or `None` for a non-finite value.
#[inline(always)]
pub(in crate::numeric) fn compare_exact_decimal(
    left: NumberRef<'_>,
    right: NumberRef<'_>,
) -> Option<Ordering> {
    Some(to_exact_decimal(left)?.cmp(&to_exact_decimal(right)?))
}
