// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Exact comparison helpers for fixed-width numeric representations.

use std::cmp::Ordering;

use crate::NumberRef;
use crate::numeric::internal::NumberRepr;

/// Converts a finite fixed-width value into sign, significand, and power of
/// two.
///
/// # Parameters
///
/// * `value` - Fixed-width numeric representation to decompose.
///
/// # Returns
///
/// A tuple representing `(-1)^sign * significand * 2^exponent`, or `None`
/// for non-finite values or arbitrary-precision representations.
fn finite_parts(value: NumberRef<'_>) -> Option<(bool, u128, i32)> {
    match value.inner() {
        NumberRepr::Int8(value) => signed_parts(i128::from(value)),
        NumberRepr::Int16(value) => signed_parts(i128::from(value)),
        NumberRepr::Int32(value) => signed_parts(i128::from(value)),
        NumberRepr::Int64(value) => signed_parts(i128::from(value)),
        NumberRepr::Int128(value) => signed_parts(value),
        NumberRepr::UInt8(value) => Some((false, u128::from(value), 0)),
        NumberRepr::UInt16(value) => Some((false, u128::from(value), 0)),
        NumberRepr::UInt32(value) => Some((false, u128::from(value), 0)),
        NumberRepr::UInt64(value) => Some((false, u128::from(value), 0)),
        NumberRepr::UInt128(value) => Some((false, value, 0)),
        NumberRepr::Float32(value) if value.is_finite() => {
            Some(f32_parts(value))
        }
        NumberRepr::Float64(value) if value.is_finite() => {
            Some(f64_parts(value))
        }
        _ => None,
    }
}

/// Converts a signed integer into finite binary parts.
///
/// # Parameters
///
/// * `value` - Signed integer to decompose.
///
/// # Returns
///
/// Its sign, unsigned magnitude, and zero binary exponent.
#[inline(always)]
fn signed_parts(value: i128) -> Option<(bool, u128, i32)> {
    Some((value.is_negative(), value.unsigned_abs(), 0))
}

/// Decodes a finite `f32` into exact binary parts.
///
/// # Parameters
///
/// * `value` - Finite `f32` to decode.
///
/// # Returns
///
/// Its sign, integer significand, and signed binary exponent.
fn f32_parts(value: f32) -> (bool, u128, i32) {
    let bits = value.to_bits();
    let negative = bits >> 31 != 0;
    let exponent = ((bits >> 23) & 0xff) as i32;
    let fraction = u128::from(bits & 0x007f_ffff);
    if exponent == 0 {
        (negative, fraction, -149)
    } else {
        (negative, (1_u128 << 23) | fraction, exponent - 150)
    }
}

/// Decodes a finite `f64` into exact binary parts.
///
/// # Parameters
///
/// * `value` - Finite `f64` to decode.
///
/// # Returns
///
/// Its sign, integer significand, and signed binary exponent.
fn f64_parts(value: f64) -> (bool, u128, i32) {
    let bits = value.to_bits();
    let negative = bits >> 63 != 0;
    let exponent = ((bits >> 52) & 0x7ff) as i32;
    let fraction = u128::from(bits & 0x000f_ffff_ffff_ffff);
    if exponent == 0 {
        (negative, fraction, -1074)
    } else {
        (negative, (1_u128 << 52) | fraction, exponent - 1075)
    }
}

/// Compares two non-negative binary rationals without overflowing `u128`.
///
/// # Parameters
///
/// * `left_significand` - Left integer significand.
/// * `left_exponent` - Left signed binary exponent.
/// * `right_significand` - Right integer significand.
/// * `right_exponent` - Right signed binary exponent.
///
/// # Returns
///
/// The exact magnitude ordering.
fn compare_magnitude(
    left_significand: u128,
    left_exponent: i32,
    right_significand: u128,
    right_exponent: i32,
) -> Ordering {
    if left_significand == 0 || right_significand == 0 {
        return left_significand.cmp(&right_significand);
    }
    let left_high_bit =
        128 - left_significand.leading_zeros() as i32 + left_exponent;
    let right_high_bit =
        128 - right_significand.leading_zeros() as i32 + right_exponent;
    match left_high_bit.cmp(&right_high_bit) {
        Ordering::Equal => match left_exponent.cmp(&right_exponent) {
            Ordering::Equal => left_significand.cmp(&right_significand),
            Ordering::Greater => (left_significand
                << (left_exponent - right_exponent))
                .cmp(&right_significand),
            Ordering::Less => left_significand
                .cmp(&(right_significand << (right_exponent - left_exponent))),
        },
        ordering => ordering,
    }
}

/// Compares two finite fixed-width numeric values exactly.
///
/// # Parameters
///
/// * `left` - Left fixed-width operand.
/// * `right` - Right fixed-width operand.
///
/// # Returns
///
/// Their mathematical ordering, or `None` for a non-finite or unsupported
/// input.
pub(in crate::numeric) fn compare_fixed(
    left: NumberRef<'_>,
    right: NumberRef<'_>,
) -> Option<Ordering> {
    let (left_negative, left_significand, left_exponent) = finite_parts(left)?;
    let (right_negative, right_significand, right_exponent) =
        finite_parts(right)?;
    if left_significand == 0 && right_significand == 0 {
        return Some(Ordering::Equal);
    }
    match left_negative.cmp(&right_negative) {
        Ordering::Less => Some(Ordering::Greater),
        Ordering::Greater => Some(Ordering::Less),
        Ordering::Equal => {
            let ordering = compare_magnitude(
                left_significand,
                left_exponent,
                right_significand,
                right_exponent,
            );
            Some(if left_negative {
                ordering.reverse()
            } else {
                ordering
            })
        }
    }
}
