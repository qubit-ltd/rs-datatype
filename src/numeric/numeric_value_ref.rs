// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Borrowed numeric value representations used by comparison APIs.

#[cfg(feature = "big-decimal")]
use bigdecimal::BigDecimal;
#[cfg(feature = "big-integer")]
use num_bigint::BigInt;
#[cfg(any(feature = "big-integer", feature = "big-decimal"))]
use num_traits::ToPrimitive;

use super::numeric_value::NumericValue;

/// Borrows or copies a numeric value without depending on a runtime value enum.
///
/// Values are constructed through [`From`] implementations. The internal
/// representation is intentionally private, so callers cannot create marker
/// or otherwise non-numeric states.
///
/// ```compile_fail
/// use qubit_datatype::NumericValueRef;
///
/// let _ = NumericValueRef::Int8(1);
/// ```
#[must_use]
#[derive(Debug, Clone, Copy)]
pub struct NumericValueRef<'a> {
    /// Private numeric representation.
    inner: NumericValue<'a>,
}

impl From<i8> for NumericValueRef<'_> {
    #[inline(always)]
    fn from(value: i8) -> Self {
        Self {
            inner: NumericValue::Int8(value),
        }
    }
}

impl From<i16> for NumericValueRef<'_> {
    #[inline(always)]
    fn from(value: i16) -> Self {
        Self {
            inner: NumericValue::Int16(value),
        }
    }
}

impl From<i32> for NumericValueRef<'_> {
    #[inline(always)]
    fn from(value: i32) -> Self {
        Self {
            inner: NumericValue::Int32(value),
        }
    }
}

impl From<i64> for NumericValueRef<'_> {
    #[inline(always)]
    fn from(value: i64) -> Self {
        Self {
            inner: NumericValue::Int64(value),
        }
    }
}

impl From<i128> for NumericValueRef<'_> {
    #[inline(always)]
    fn from(value: i128) -> Self {
        Self {
            inner: NumericValue::Int128(value),
        }
    }
}

impl From<u8> for NumericValueRef<'_> {
    #[inline(always)]
    fn from(value: u8) -> Self {
        Self {
            inner: NumericValue::UInt8(value),
        }
    }
}

impl From<u16> for NumericValueRef<'_> {
    #[inline(always)]
    fn from(value: u16) -> Self {
        Self {
            inner: NumericValue::UInt16(value),
        }
    }
}

impl From<u32> for NumericValueRef<'_> {
    #[inline(always)]
    fn from(value: u32) -> Self {
        Self {
            inner: NumericValue::UInt32(value),
        }
    }
}

impl From<u64> for NumericValueRef<'_> {
    #[inline(always)]
    fn from(value: u64) -> Self {
        Self {
            inner: NumericValue::UInt64(value),
        }
    }
}

impl From<u128> for NumericValueRef<'_> {
    #[inline(always)]
    fn from(value: u128) -> Self {
        Self {
            inner: NumericValue::UInt128(value),
        }
    }
}

impl From<f32> for NumericValueRef<'_> {
    #[inline(always)]
    fn from(value: f32) -> Self {
        Self {
            inner: NumericValue::Float32(value),
        }
    }
}

impl From<f64> for NumericValueRef<'_> {
    #[inline(always)]
    fn from(value: f64) -> Self {
        Self {
            inner: NumericValue::Float64(value),
        }
    }
}

#[cfg(feature = "big-integer")]
impl<'a> From<&'a BigInt> for NumericValueRef<'a> {
    #[inline(always)]
    fn from(value: &'a BigInt) -> Self {
        Self {
            inner: NumericValue::BigInteger(value),
        }
    }
}

#[cfg(feature = "big-decimal")]
impl<'a> From<&'a BigDecimal> for NumericValueRef<'a> {
    #[inline(always)]
    fn from(value: &'a BigDecimal) -> Self {
        Self {
            inner: NumericValue::BigDecimal(value),
        }
    }
}

impl<'a> NumericValueRef<'a> {
    /// Returns the private representation for numeric algorithms.
    #[inline(always)]
    pub(in crate::numeric) const fn inner(self) -> NumericValue<'a> {
        self.inner
    }

    /// Reports whether this value is a primitive floating-point value.
    ///
    /// # Returns
    ///
    /// `true` for `f32` and `f64` values.
    #[must_use]
    #[inline(always)]
    pub(crate) const fn is_float(self) -> bool {
        matches!(
            self.inner,
            NumericValue::Float32(_) | NumericValue::Float64(_)
        )
    }

    /// Reports whether this value is NaN.
    ///
    /// # Returns
    ///
    /// `true` only for a primitive NaN payload.
    #[must_use]
    #[inline(always)]
    pub fn is_nan(self) -> bool {
        match self.inner {
            NumericValue::Float32(value) => value.is_nan(),
            NumericValue::Float64(value) => value.is_nan(),
            _ => false,
        }
    }

    /// Returns the sign of an infinite primitive value.
    ///
    /// # Returns
    ///
    /// `Some(false)` for positive infinity, `Some(true)` for negative
    /// infinity, and `None` for finite values.
    #[inline(always)]
    pub(crate) fn infinity_is_negative(self) -> Option<bool> {
        match self.inner {
            NumericValue::Float32(value) if value.is_infinite() => {
                Some(value.is_sign_negative())
            }
            NumericValue::Float64(value) if value.is_infinite() => {
                Some(value.is_sign_negative())
            }
            _ => None,
        }
    }

    /// Projects this value to `f64` for approximate comparison.
    ///
    /// # Returns
    ///
    /// The projected numeric value, or `None` when an arbitrary-precision
    /// value cannot be represented as `f64`.
    #[inline]
    pub(crate) fn to_f64(self) -> Option<f64> {
        match self.inner {
            NumericValue::Int8(value) => Some(f64::from(value)),
            NumericValue::Int16(value) => Some(f64::from(value)),
            NumericValue::Int32(value) => Some(f64::from(value)),
            NumericValue::Int64(value) => Some(value as f64),
            NumericValue::Int128(value) => Some(value as f64),
            NumericValue::UInt8(value) => Some(f64::from(value)),
            NumericValue::UInt16(value) => Some(f64::from(value)),
            NumericValue::UInt32(value) => Some(f64::from(value)),
            NumericValue::UInt64(value) => Some(value as f64),
            NumericValue::UInt128(value) => Some(value as f64),
            NumericValue::Float32(value) => Some(f64::from(value)),
            NumericValue::Float64(value) => Some(value),
            #[cfg(feature = "big-integer")]
            NumericValue::BigInteger(value) => value.to_f64(),
            #[cfg(feature = "big-decimal")]
            NumericValue::BigDecimal(value) => value.to_f64(),
            #[cfg(not(any(feature = "big-integer", feature = "big-decimal")))]
            NumericValue::Lifetime(_, impossible) => match impossible {},
        }
    }
}
