// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Borrowed numeric value representations used by comparison APIs.

use std::cmp::Ordering;

#[cfg(feature = "big-decimal")]
use bigdecimal::BigDecimal;
#[cfg(any(feature = "big-integer", feature = "big-decimal"))]
use num_bigint::BigInt;
#[cfg(any(feature = "big-integer", feature = "big-decimal"))]
use num_rational::BigRational;
#[cfg(any(feature = "big-integer", feature = "big-decimal"))]
use num_traits::{
    Signed,
    ToPrimitive,
    Zero,
};

use super::NumericComparisonPolicy;
#[cfg(feature = "big-decimal")]
use super::internal::decimal_rational;
use super::internal::{
    NumberRepr,
    compare_magnitude,
    finite_parts,
};
#[cfg(any(feature = "big-integer", feature = "big-decimal"))]
use super::internal::{
    f32_rational,
    f64_rational,
};

/// Borrows or copies a numeric value without depending on a runtime value enum.
///
/// Values are constructed through [`From`] implementations. The internal
/// representation is intentionally private, so callers cannot create marker
/// or otherwise non-numeric states.
///
/// ```compile_fail
/// use qubit_datatype::NumberRef;
///
/// let _ = NumberRef::Int8(1);
/// ```
#[must_use]
#[derive(Debug, Clone, Copy)]
pub struct NumberRef<'a> {
    /// Private numeric representation.
    inner: NumberRepr<'a>,
}

macro_rules! impl_from_copy {
    ($source:ty, $variant:ident) => {
        impl From<$source> for NumberRef<'_> {
            #[inline(always)]
            fn from(value: $source) -> Self {
                Self {
                    inner: NumberRepr::$variant(value),
                }
            }
        }
    };
}

#[cfg(any(feature = "big-integer", feature = "big-decimal"))]
macro_rules! impl_from_ref {
    ($source:ty, $variant:ident) => {
        impl<'a> From<&'a $source> for NumberRef<'a> {
            #[inline(always)]
            fn from(value: &'a $source) -> Self {
                Self {
                    inner: NumberRepr::$variant(value),
                }
            }
        }
    };
}

impl_from_copy!(i8, Int8);
impl_from_copy!(i16, Int16);
impl_from_copy!(i32, Int32);
impl_from_copy!(i64, Int64);
impl_from_copy!(i128, Int128);
impl_from_copy!(u8, UInt8);
impl_from_copy!(u16, UInt16);
impl_from_copy!(u32, UInt32);
impl_from_copy!(u64, UInt64);
impl_from_copy!(u128, UInt128);
impl_from_copy!(f32, Float32);
impl_from_copy!(f64, Float64);
#[cfg(feature = "big-integer")]
impl_from_ref!(BigInt, BigInteger);
#[cfg(feature = "big-decimal")]
impl_from_ref!(BigDecimal, BigDecimal);

impl<'a> NumberRef<'a> {
    /// Reports whether this value is an integer representation.
    ///
    /// # Returns
    ///
    /// `true` for fixed-width integers and `BigInt` values.
    #[must_use]
    pub fn is_integer(self) -> bool {
        match self.inner {
            NumberRepr::Int8(_)
            | NumberRepr::Int16(_)
            | NumberRepr::Int32(_)
            | NumberRepr::Int64(_)
            | NumberRepr::Int128(_)
            | NumberRepr::UInt8(_)
            | NumberRepr::UInt16(_)
            | NumberRepr::UInt32(_)
            | NumberRepr::UInt64(_)
            | NumberRepr::UInt128(_) => true,
            #[cfg(feature = "big-integer")]
            NumberRepr::BigInteger(_) => true,
            _ => false,
        }
    }

    /// Reports whether this value is a primitive floating-point value.
    ///
    /// # Returns
    ///
    /// `true` for `f32` and `f64` values.
    #[inline(always)]
    #[must_use]
    pub fn is_float(self) -> bool {
        matches!(self.inner, NumberRepr::Float32(_) | NumberRepr::Float64(_))
    }

    /// Reports whether this value is an arbitrary-precision integer.
    ///
    /// # Returns
    ///
    /// `true` for `BigInt` values when the `big-integer` feature is enabled;
    /// otherwise, `false`.
    #[inline(always)]
    #[must_use]
    pub fn is_big_integer(self) -> bool {
        #[cfg(feature = "big-integer")]
        {
            matches!(self.inner, NumberRepr::BigInteger(_))
        }
        #[cfg(not(feature = "big-integer"))]
        {
            false
        }
    }

    /// Reports whether this value is an arbitrary-precision decimal.
    ///
    /// # Returns
    ///
    /// `true` for `BigDecimal` values when the `big-decimal` feature is
    /// enabled; otherwise, `false`.
    #[inline(always)]
    #[must_use]
    pub fn is_big_decimal(self) -> bool {
        #[cfg(feature = "big-decimal")]
        {
            matches!(self.inner, NumberRepr::BigDecimal(_))
        }
        #[cfg(not(feature = "big-decimal"))]
        {
            false
        }
    }

    /// Reports whether this value is NaN.
    ///
    /// # Returns
    ///
    /// `true` only for a primitive NaN payload.
    #[inline]
    #[must_use]
    pub fn is_nan(self) -> bool {
        match self.inner {
            NumberRepr::Float32(value) => value.is_nan(),
            NumberRepr::Float64(value) => value.is_nan(),
            _ => false,
        }
    }

    /// Reports whether this value is an infinite primitive float.
    ///
    /// # Returns
    ///
    /// `true` for positive or negative infinity; otherwise, `false`.
    #[inline]
    #[must_use]
    pub fn is_infinite(self) -> bool {
        match self.inner {
            NumberRepr::Float32(value) => value.is_infinite(),
            NumberRepr::Float64(value) => value.is_infinite(),
            _ => false,
        }
    }

    /// Reports whether this value is finite.
    ///
    /// # Returns
    ///
    /// `true` for all integers, arbitrary-precision values, and finite
    /// primitive floats; otherwise, `false`.
    #[inline(always)]
    #[must_use]
    pub fn is_finite(self) -> bool {
        !self.is_nan() && !self.is_infinite()
    }

    /// Reports whether this value is mathematically zero.
    ///
    /// # Returns
    ///
    /// `true` for integer zero and both positive and negative floating-point
    /// zero; otherwise, `false`.
    #[must_use]
    pub fn is_zero(self) -> bool {
        match self.inner {
            NumberRepr::Int8(value) => value == 0,
            NumberRepr::Int16(value) => value == 0,
            NumberRepr::Int32(value) => value == 0,
            NumberRepr::Int64(value) => value == 0,
            NumberRepr::Int128(value) => value == 0,
            NumberRepr::UInt8(value) => value == 0,
            NumberRepr::UInt16(value) => value == 0,
            NumberRepr::UInt32(value) => value == 0,
            NumberRepr::UInt64(value) => value == 0,
            NumberRepr::UInt128(value) => value == 0,
            NumberRepr::Float32(value) => value == 0.0,
            NumberRepr::Float64(value) => value == 0.0,
            #[cfg(feature = "big-integer")]
            NumberRepr::BigInteger(value) => value.is_zero(),
            #[cfg(feature = "big-decimal")]
            NumberRepr::BigDecimal(value) => value.is_zero(),
            #[cfg(not(any(feature = "big-integer", feature = "big-decimal")))]
            NumberRepr::Lifetime(_, impossible) => match impossible {},
        }
    }

    /// Reports whether this value is greater than zero.
    ///
    /// # Returns
    ///
    /// `true` for positive finite or infinite values; zero, negative values,
    /// and NaN return `false`.
    #[must_use]
    pub fn is_positive(self) -> bool {
        match self.inner {
            NumberRepr::Int8(value) => value > 0,
            NumberRepr::Int16(value) => value > 0,
            NumberRepr::Int32(value) => value > 0,
            NumberRepr::Int64(value) => value > 0,
            NumberRepr::Int128(value) => value > 0,
            NumberRepr::UInt8(value) => value > 0,
            NumberRepr::UInt16(value) => value > 0,
            NumberRepr::UInt32(value) => value > 0,
            NumberRepr::UInt64(value) => value > 0,
            NumberRepr::UInt128(value) => value > 0,
            NumberRepr::Float32(value) => value > 0.0,
            NumberRepr::Float64(value) => value > 0.0,
            #[cfg(feature = "big-integer")]
            NumberRepr::BigInteger(value) => value.is_positive(),
            #[cfg(feature = "big-decimal")]
            NumberRepr::BigDecimal(value) => value.is_positive(),
            #[cfg(not(any(feature = "big-integer", feature = "big-decimal")))]
            NumberRepr::Lifetime(_, impossible) => match impossible {},
        }
    }

    /// Reports whether this value is less than zero.
    ///
    /// # Returns
    ///
    /// `true` for negative finite or infinite values; zero, positive values,
    /// and NaN return `false`.
    #[must_use]
    pub fn is_negative(self) -> bool {
        match self.inner {
            NumberRepr::Int8(value) => value < 0,
            NumberRepr::Int16(value) => value < 0,
            NumberRepr::Int32(value) => value < 0,
            NumberRepr::Int64(value) => value < 0,
            NumberRepr::Int128(value) => value < 0,
            NumberRepr::UInt8(_) => false,
            NumberRepr::UInt16(_) => false,
            NumberRepr::UInt32(_) => false,
            NumberRepr::UInt64(_) => false,
            NumberRepr::UInt128(_) => false,
            NumberRepr::Float32(value) => value < 0.0,
            NumberRepr::Float64(value) => value < 0.0,
            #[cfg(feature = "big-integer")]
            NumberRepr::BigInteger(value) => value.is_negative(),
            #[cfg(feature = "big-decimal")]
            NumberRepr::BigDecimal(value) => value.is_negative(),
            #[cfg(not(any(feature = "big-integer", feature = "big-decimal")))]
            NumberRepr::Lifetime(_, impossible) => match impossible {},
        }
    }

    /// Compares this value with `right` under an explicit numeric policy.
    ///
    /// # Parameters
    ///
    /// * `right` - Right numeric operand.
    /// * `policy` - Exact or approximate comparison policy.
    ///
    /// # Returns
    ///
    /// The numeric ordering, or `None` when either operand is NaN.
    #[must_use]
    pub fn compare(
        self,
        right: NumberRef<'_>,
        policy: NumericComparisonPolicy,
    ) -> Option<Ordering> {
        if self.is_nan() || right.is_nan() {
            return None;
        }
        match (self.is_infinite(), right.is_infinite()) {
            (true, true) => {
                return Some(if self.is_negative() == right.is_negative() {
                    Ordering::Equal
                } else if self.is_negative() {
                    Ordering::Less
                } else {
                    Ordering::Greater
                });
            }
            (true, false) => {
                return Some(if self.is_negative() {
                    Ordering::Less
                } else {
                    Ordering::Greater
                });
            }
            (false, true) => {
                return Some(if right.is_negative() {
                    Ordering::Greater
                } else {
                    Ordering::Less
                });
            }
            (false, false) => {}
        }
        if policy == NumericComparisonPolicy::Approximate
            && (self.is_float() || right.is_float())
            && let (Some(left), Some(right)) =
                (self.to_approximate_f64(), right.to_approximate_f64())
            && left.is_finite()
            && right.is_finite()
        {
            return left.partial_cmp(&right);
        }
        #[cfg(any(feature = "big-integer", feature = "big-decimal"))]
        {
            #[cfg(feature = "big-integer")]
            let has_big_integer =
                self.is_big_integer() || right.is_big_integer();
            #[cfg(not(feature = "big-integer"))]
            let has_big_integer = false;
            #[cfg(feature = "big-decimal")]
            let has_big_decimal =
                self.is_big_decimal() || right.is_big_decimal();
            #[cfg(not(feature = "big-decimal"))]
            let has_big_decimal = false;
            if has_big_integer || has_big_decimal {
                return self.compare_exact_rational(right);
            }
        }
        self.compare_fixed(right)
    }

    /// Returns the private representation for numeric algorithms.
    ///
    /// # Returns
    ///
    /// The crate-private `NumberRepr` payload borrowed or copied by this value.
    #[inline(always)]
    #[must_use]
    pub(in crate::numeric) const fn inner(self) -> NumberRepr<'a> {
        self.inner
    }

    /// Compares arbitrary-precision pairs through exact rational values.
    ///
    /// Decimal scales that cannot be materialized as a practical power of ten
    /// fall back to `BigDecimal`'s exact scale-aware ordering.
    ///
    /// # Parameters
    ///
    /// * `right` - Right arbitrary-precision operand.
    ///
    /// # Returns
    ///
    /// Their exact mathematical ordering, or `None` for non-finite values.
    #[cfg(any(feature = "big-integer", feature = "big-decimal"))]
    #[must_use]
    fn compare_exact_rational(self, right: NumberRef<'_>) -> Option<Ordering> {
        #[cfg(feature = "big-integer")]
        if let (NumberRepr::BigInteger(left), NumberRepr::BigInteger(right)) =
            (self.inner, right.inner)
        {
            return Some(left.cmp(right));
        }
        #[cfg(feature = "big-decimal")]
        if let (NumberRepr::BigDecimal(left), NumberRepr::BigDecimal(right)) =
            (self.inner, right.inner)
        {
            return Some(left.cmp(right));
        }
        match (self.to_exact_rational(), right.to_exact_rational()) {
            (Some(left), Some(right)) => Some(left.cmp(&right)),
            #[cfg(feature = "big-decimal")]
            _ => self.compare_exact_decimal(right),
            #[cfg(not(feature = "big-decimal"))]
            _ => None,
        }
    }

    /// Compares values through exact arbitrary-precision decimal
    /// representations.
    ///
    /// # Parameters
    ///
    /// * `right` - Right numeric operand.
    ///
    /// # Returns
    ///
    /// Their mathematical ordering, or `None` when either operand is
    /// non-finite.
    #[cfg(feature = "big-decimal")]
    #[inline(always)]
    #[must_use]
    fn compare_exact_decimal(self, right: NumberRef<'_>) -> Option<Ordering> {
        Some(self.to_exact_decimal()?.cmp(&right.to_exact_decimal()?))
    }

    /// Compares two finite fixed-width numeric values exactly.
    ///
    /// # Parameters
    ///
    /// * `right` - Right fixed-width operand.
    ///
    /// # Returns
    ///
    /// Their mathematical ordering, or `None` for a non-finite or unsupported
    /// input.
    #[must_use]
    fn compare_fixed(self, right: NumberRef<'_>) -> Option<Ordering> {
        let (left_negative, left_significand, left_exponent) =
            finite_parts(self)?;
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

    /// Projects this value to `f64` for approximate comparison.
    ///
    /// Fixed-width integers and primitive floats convert by widening or
    /// casting. Arbitrary-precision values use `ToPrimitive::to_f64`, which
    /// may lose precision or return `None` when the magnitude cannot be
    /// represented.
    ///
    /// # Returns
    ///
    /// The approximate `f64` projection, or `None` when an arbitrary-precision
    /// value cannot be represented as `f64`.
    #[must_use]
    fn to_approximate_f64(self) -> Option<f64> {
        match self.inner {
            NumberRepr::Int8(value) => Some(f64::from(value)),
            NumberRepr::Int16(value) => Some(f64::from(value)),
            NumberRepr::Int32(value) => Some(f64::from(value)),
            NumberRepr::Int64(value) => Some(value as f64),
            NumberRepr::Int128(value) => Some(value as f64),
            NumberRepr::UInt8(value) => Some(f64::from(value)),
            NumberRepr::UInt16(value) => Some(f64::from(value)),
            NumberRepr::UInt32(value) => Some(f64::from(value)),
            NumberRepr::UInt64(value) => Some(value as f64),
            NumberRepr::UInt128(value) => Some(value as f64),
            NumberRepr::Float32(value) => Some(f64::from(value)),
            NumberRepr::Float64(value) => Some(value),
            #[cfg(feature = "big-integer")]
            NumberRepr::BigInteger(value) => value.to_f64(),
            #[cfg(feature = "big-decimal")]
            NumberRepr::BigDecimal(value) => value.to_f64(),
            #[cfg(not(any(feature = "big-integer", feature = "big-decimal")))]
            NumberRepr::Lifetime(_, impossible) => match impossible {},
        }
    }

    /// Converts this finite numeric value into an exact rational.
    ///
    /// # Returns
    ///
    /// The exact mathematical value, or `None` for non-finite values or an
    /// impractically large decimal scale.
    #[cfg(any(feature = "big-integer", feature = "big-decimal"))]
    #[must_use]
    fn to_exact_rational(self) -> Option<BigRational> {
        match self.inner {
            NumberRepr::Int8(value) => {
                Some(BigRational::from_integer(BigInt::from(value)))
            }
            NumberRepr::Int16(value) => {
                Some(BigRational::from_integer(BigInt::from(value)))
            }
            NumberRepr::Int32(value) => {
                Some(BigRational::from_integer(BigInt::from(value)))
            }
            NumberRepr::Int64(value) => {
                Some(BigRational::from_integer(BigInt::from(value)))
            }
            NumberRepr::Int128(value) => {
                Some(BigRational::from_integer(BigInt::from(value)))
            }
            NumberRepr::UInt8(value) => {
                Some(BigRational::from_integer(BigInt::from(value)))
            }
            NumberRepr::UInt16(value) => {
                Some(BigRational::from_integer(BigInt::from(value)))
            }
            NumberRepr::UInt32(value) => {
                Some(BigRational::from_integer(BigInt::from(value)))
            }
            NumberRepr::UInt64(value) => {
                Some(BigRational::from_integer(BigInt::from(value)))
            }
            NumberRepr::UInt128(value) => {
                Some(BigRational::from_integer(BigInt::from(value)))
            }
            NumberRepr::Float32(value) => Some(f32_rational(value)),
            NumberRepr::Float64(value) => Some(f64_rational(value)),
            #[cfg(feature = "big-integer")]
            NumberRepr::BigInteger(value) => {
                Some(BigRational::from_integer(value.clone()))
            }
            #[cfg(feature = "big-decimal")]
            NumberRepr::BigDecimal(value) => decimal_rational(value),
        }
    }

    /// Converts this finite numeric value into an exact arbitrary-precision
    /// decimal.
    ///
    /// `BigDecimal`'s primitive-float conversion decodes the IEEE significand
    /// and exponent exactly; it does not use display formatting.
    ///
    /// # Returns
    ///
    /// The exact decimal value, or `None` for a non-finite primitive float.
    #[cfg(feature = "big-decimal")]
    #[must_use]
    fn to_exact_decimal(self) -> Option<BigDecimal> {
        match self.inner {
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
            NumberRepr::BigInteger(value) => {
                Some(BigDecimal::from(value.clone()))
            }
            NumberRepr::BigDecimal(value) => Some(value.clone()),
        }
    }
}
