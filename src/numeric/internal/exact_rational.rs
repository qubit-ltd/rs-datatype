// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Exact rational conversion helpers for mixed numeric representations.

#[cfg(feature = "big-decimal")]
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_rational::BigRational;

/// Largest decimal scale materialized as an explicit power of ten.
///
/// Larger scales use `BigDecimal`'s allocation-bounded exact comparator.
#[cfg(feature = "big-decimal")]
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
#[must_use]
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
/// # Parameters
///
/// * `value` - Finite `f32` to decode.
///
/// # Returns
///
/// The exact represented value. The caller guarantees finiteness.
#[must_use]
pub(in crate::numeric) fn f32_rational(value: f32) -> BigRational {
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
/// # Parameters
///
/// * `value` - Finite `f64` to decode.
///
/// # Returns
///
/// The exact represented value. The caller guarantees finiteness.
#[must_use]
pub(in crate::numeric) fn f64_rational(value: f64) -> BigRational {
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
/// # Parameters
///
/// * `value` - Arbitrary-precision decimal to convert.
///
/// # Returns
///
/// The exact decimal value when its scale fits `u32`, otherwise `None` so the
/// caller can use `BigDecimal`'s exact scale-aware comparator without an
/// impractically large power allocation.
#[cfg(feature = "big-decimal")]
pub(in crate::numeric) fn decimal_rational(
    value: &BigDecimal,
) -> Option<BigRational> {
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
