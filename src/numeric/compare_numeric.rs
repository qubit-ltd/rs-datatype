// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use std::cmp::Ordering;

#[cfg(feature = "big-number")]
use super::internal::compare_exact_rational;
use super::internal::compare_fixed;
use super::{
    NumericComparisonPolicy,
    NumericValueRef,
};

/// Compares two numeric representations under an explicit policy.
///
/// # Arguments
///
/// * `left` - Left numeric operand.
/// * `right` - Right numeric operand.
/// * `policy` - Exact or approximate comparison policy.
///
/// # Returns
///
/// The numeric ordering, or `None` when either operand is NaN or is the hidden
/// lifetime marker.
#[must_use]
pub fn compare_numeric(
    left: NumericValueRef<'_>,
    right: NumericValueRef<'_>,
    policy: NumericComparisonPolicy,
) -> Option<Ordering> {
    if left.is_nan() || right.is_nan() {
        return None;
    }
    match (left.infinity_is_negative(), right.infinity_is_negative()) {
        (Some(left_negative), Some(right_negative)) => {
            return Some(right_negative.cmp(&left_negative));
        }
        (Some(true), None) | (None, Some(false)) => {
            return Some(Ordering::Less);
        }
        (Some(false), None) | (None, Some(true)) => {
            return Some(Ordering::Greater);
        }
        (None, None) => {}
    }
    if policy == NumericComparisonPolicy::Approximate
        && (left.is_float() || right.is_float())
        && let (Some(left), Some(right)) = (left.to_f64(), right.to_f64())
        && left.is_finite()
        && right.is_finite()
    {
        return left.partial_cmp(&right);
    }
    #[cfg(feature = "big-number")]
    if matches!(
        left,
        NumericValueRef::BigInteger(_) | NumericValueRef::BigDecimal(_)
    ) || matches!(
        right,
        NumericValueRef::BigInteger(_) | NumericValueRef::BigDecimal(_)
    ) {
        return compare_exact_rational(left, right);
    }
    compare_fixed(left, right)
}
