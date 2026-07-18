// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Public policy-driven numeric comparison entry point.

use std::cmp::Ordering;

#[cfg(any(feature = "big-integer", feature = "big-decimal"))]
use super::internal::compare_exact_rational;
use super::internal::compare_fixed;
use super::{NumericComparisonPolicy, NumericValueRef};

/// Compares two numeric representations under an explicit policy.
///
/// [`NumericComparisonPolicy::Approximate`] first orders primitive infinities.
/// When a finite primitive float participates, it attempts to project both
/// operands to finite `f64` values. If either operand cannot be projected that
/// way, comparison falls back to the exact path. The projected comparison is
/// pair-dependent and not transitive, so it must not be used to implement
/// [`Ord`], sort or group values, or construct ordered-map or ordered-set keys.
/// Use [`NumericComparisonPolicy::Exact`] whenever a deterministic ordering is
/// required.
///
/// # Parameters
///
/// * `left` - Left numeric operand.
/// * `right` - Right numeric operand.
/// * `policy` - Exact or approximate comparison policy.
///
/// # Returns
///
/// The numeric ordering, or `None` when either operand is NaN or is the hidden
/// lifetime marker.
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
    #[cfg(any(feature = "big-integer", feature = "big-decimal"))]
    {
        #[cfg(feature = "big-integer")]
        let has_big_integer = matches!(left, NumericValueRef::BigInteger(_))
            || matches!(right, NumericValueRef::BigInteger(_));
        #[cfg(not(feature = "big-integer"))]
        let has_big_integer = false;
        #[cfg(feature = "big-decimal")]
        let has_big_decimal = matches!(left, NumericValueRef::BigDecimal(_))
            || matches!(right, NumericValueRef::BigDecimal(_));
        #[cfg(not(feature = "big-decimal"))]
        let has_big_decimal = false;
        if has_big_integer || has_big_decimal {
            return compare_exact_rational(left, right);
        }
    }
    compare_fixed(left, right)
}
