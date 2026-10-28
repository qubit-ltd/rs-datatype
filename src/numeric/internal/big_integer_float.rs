// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Allocation-free gates for exact BigInteger and float comparisons.

use std::cmp::Ordering;

use num_bigint::BigInt;
use num_traits::Signed;
use num_traits::Zero;

use crate::NumberRef;
use crate::numeric::internal::NumberRepr;
use crate::numeric::internal::finite_parts;

/// Compares a `BigInt` with a finite fixed-width value when the result follows
/// from zero, sign, or highest-bit ordering alone.
///
/// # Parameters
///
/// * `integer` - Arbitrary-precision integer on the left.
/// * `number` - Numeric value on the right.
///
/// # Returns
///
/// An exact ordering when the comparison can be decided without constructing
/// an arbitrary-precision rational; `None` when an exact fallback is needed.
pub(in crate::numeric) fn compare_big_integer_float(integer: &BigInt, number: NumberRef<'_>) -> Option<Ordering> {
    if !matches!(number.inner(), NumberRepr::Float32(_) | NumberRepr::Float64(_)) {
        return None;
    }
    let (float_negative, significand, exponent) = finite_parts(number)?;
    let integer_zero = integer.is_zero();
    if integer_zero || significand == 0 {
        return Some(match (integer_zero, significand == 0) {
            (true, true) => Ordering::Equal,
            (true, false) => {
                if float_negative {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            }
            (false, true) => {
                if integer.is_negative() {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            }
            (false, false) => unreachable!(),
        });
    }

    let integer_negative = integer.is_negative();
    if integer_negative != float_negative {
        return Some(if integer_negative {
            Ordering::Less
        } else {
            Ordering::Greater
        });
    }

    let integer_high_bit = i64::try_from(integer.bits()).ok()? - 1;
    let float_high_bit = i64::from(exponent) + i64::from(128 - significand.leading_zeros()) - 1;
    match integer_high_bit.cmp(&float_high_bit) {
        Ordering::Equal => None,
        ordering if integer_negative => Some(ordering.reverse()),
        ordering => Some(ordering),
    }
}
