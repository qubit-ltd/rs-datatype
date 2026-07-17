// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Exact rational comparison helpers for mixed numeric representations.

use std::cmp::Ordering;

use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_rational::BigRational;

use super::exact_decimal::compare_exact_decimal;
use crate::NumericValueRef;

/// Largest decimal scale materialized as an explicit power of ten.
///
/// Larger scales use `BigDecimal`'s allocation-bounded exact comparator.
const MAX_MATERIALIZED_DECIMAL_SCALE: u32 = 4_096;

/// Builds an exact rational from an IEEE-style binary decomposition.
///
/// # Parameters
///
/// * `negative` - Whether the represented number is negative.
/// * `significand` - Unsigned binary significand.
/// * `exponent` - Base-two exponent applied to the significand.
///
/// # Returns
///
/// The exact rational value.
fn binary_rational(
    negative: bool,
    significand: u128,
    exponent: i32,
) -> BigRational {
    let mut numerator = BigInt::from(significand);
    if negative {
        numerator = -numerator;
    }
    if exponent >= 0 {
        BigRational::from_integer(numerator << exponent as usize)
    } else {
        BigRational::new(numerator, BigInt::from(1_u8) << (-exponent) as usize)
    }
}

/// Decodes a finite `f32` into an exact rational.
///
/// # Returns
///
/// The exact represented value. The caller guarantees finiteness.
fn f32_rational(value: f32) -> BigRational {
    let bits = value.to_bits();
    let negative = bits >> 31 != 0;
    let exponent = ((bits >> 23) & 0xff) as i32;
    let fraction = u128::from(bits & 0x007f_ffff);
    if exponent == 0 {
        binary_rational(negative, fraction, -149)
    } else {
        binary_rational(negative, (1_u128 << 23) | fraction, exponent - 150)
    }
}

/// Decodes a finite `f64` into an exact rational.
///
/// # Returns
///
/// The exact represented value. The caller guarantees finiteness.
fn f64_rational(value: f64) -> BigRational {
    let bits = value.to_bits();
    let negative = bits >> 63 != 0;
    let exponent = ((bits >> 52) & 0x7ff) as i32;
    let fraction = u128::from(bits & 0x000f_ffff_ffff_ffff);
    if exponent == 0 {
        binary_rational(negative, fraction, -1074)
    } else {
        binary_rational(negative, (1_u128 << 52) | fraction, exponent - 1075)
    }
}

/// Converts a decimal coefficient and scale into an exact rational.
///
/// # Returns
///
/// The exact decimal value when its scale fits `u32`, otherwise `None` so the
/// caller can use `BigDecimal`'s exact scale-aware comparator without an
/// impractically large power allocation.
fn decimal_rational(value: &BigDecimal) -> Option<BigRational> {
    let (coefficient, scale) = value.as_bigint_and_exponent();
    if scale >= 0 {
        let scale = u32::try_from(scale).ok()?;
        if scale > MAX_MATERIALIZED_DECIMAL_SCALE {
            return None;
        }
        Some(BigRational::new(
            coefficient,
            BigInt::from(10_u8).pow(scale),
        ))
    } else {
        let scale = u32::try_from(scale.unsigned_abs()).ok()?;
        if scale > MAX_MATERIALIZED_DECIMAL_SCALE {
            return None;
        }
        Some(BigRational::from_integer(
            coefficient * BigInt::from(10_u8).pow(scale),
        ))
    }
}

/// Converts a finite numeric value into an exact rational.
///
/// # Returns
///
/// The exact mathematical value, or `None` for non-finite values, the hidden
/// marker, or an impractically large decimal scale.
fn to_exact_rational(value: NumericValueRef<'_>) -> Option<BigRational> {
    match value {
        NumericValueRef::Int8(value) => {
            Some(BigRational::from_integer(BigInt::from(value)))
        }
        NumericValueRef::Int16(value) => {
            Some(BigRational::from_integer(BigInt::from(value)))
        }
        NumericValueRef::Int32(value) => {
            Some(BigRational::from_integer(BigInt::from(value)))
        }
        NumericValueRef::Int64(value) => {
            Some(BigRational::from_integer(BigInt::from(value)))
        }
        NumericValueRef::Int128(value) => {
            Some(BigRational::from_integer(BigInt::from(value)))
        }
        NumericValueRef::UInt8(value) => {
            Some(BigRational::from_integer(BigInt::from(value)))
        }
        NumericValueRef::UInt16(value) => {
            Some(BigRational::from_integer(BigInt::from(value)))
        }
        NumericValueRef::UInt32(value) => {
            Some(BigRational::from_integer(BigInt::from(value)))
        }
        NumericValueRef::UInt64(value) => {
            Some(BigRational::from_integer(BigInt::from(value)))
        }
        NumericValueRef::UInt128(value) => {
            Some(BigRational::from_integer(BigInt::from(value)))
        }
        NumericValueRef::Float32(value) => Some(f32_rational(value)),
        NumericValueRef::Float64(value) => Some(f64_rational(value)),
        NumericValueRef::BigInteger(value) => {
            Some(BigRational::from_integer(value.clone()))
        }
        NumericValueRef::BigDecimal(value) => decimal_rational(value),
        NumericValueRef::__Lifetime(_) => None,
    }
}

/// Compares arbitrary-precision pairs through exact rational values.
///
/// Decimal scales that cannot be materialized as a practical power of ten
/// fall back to `BigDecimal`'s exact scale-aware ordering.
///
/// # Returns
///
/// Their exact mathematical ordering, or `None` for non-finite and marker
/// values.
pub(in crate::numeric) fn compare_exact_rational(
    left: NumericValueRef<'_>,
    right: NumericValueRef<'_>,
) -> Option<Ordering> {
    // TODO: Benchmark and add borrowed or direct fast paths for BigInteger
    // and BigDecimal pairs. The current rational fallback clones BigInt
    // values and materializes powers of ten even when both operands share a
    // representation that may support allocation-free exact comparison.
    match (to_exact_rational(left), to_exact_rational(right)) {
        (Some(left), Some(right)) => Some(left.cmp(&right)),
        _ => compare_exact_decimal(left, right),
    }
}
