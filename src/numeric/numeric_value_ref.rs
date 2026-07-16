// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Borrowed numeric value representations used by comparison APIs.

use std::marker::PhantomData;

#[cfg(feature = "big-number")]
use bigdecimal::BigDecimal;
#[cfg(feature = "big-number")]
use num_bigint::BigInt;
#[cfg(feature = "big-number")]
use num_traits::ToPrimitive;

/// Borrows or copies a numeric value without depending on a runtime value enum.
#[must_use]
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub enum NumericValueRef<'a> {
    /// An `i8` value.
    Int8(i8),
    /// An `i16` value.
    Int16(i16),
    /// An `i32` value.
    Int32(i32),
    /// An `i64` value.
    Int64(i64),
    /// An `i128` value.
    Int128(i128),
    /// A `u8` value.
    UInt8(u8),
    /// A `u16` value.
    UInt16(u16),
    /// A `u32` value.
    UInt32(u32),
    /// A `u64` value.
    UInt64(u64),
    /// A `u128` value.
    UInt128(u128),
    /// An `f32` value.
    Float32(f32),
    /// An `f64` value.
    Float64(f64),
    /// An arbitrary-precision integer.
    #[cfg(feature = "big-number")]
    BigInteger(&'a BigInt),
    /// An arbitrary-precision decimal.
    #[cfg(feature = "big-number")]
    BigDecimal(&'a BigDecimal),
    /// Retains the borrowing lifetime when arbitrary-precision support is off.
    #[doc(hidden)]
    __Lifetime(PhantomData<&'a ()>),
}

impl NumericValueRef<'_> {
    /// Reports whether this value is a primitive floating-point value.
    ///
    /// # Returns
    ///
    /// `true` for [`Self::Float32`] and [`Self::Float64`].
    #[inline(always)]
    pub(crate) const fn is_float(self) -> bool {
        matches!(self, Self::Float32(_) | Self::Float64(_))
    }

    /// Reports whether this value is NaN.
    ///
    /// # Returns
    ///
    /// `true` only for a primitive NaN payload.
    #[inline(always)]
    pub(crate) fn is_nan(self) -> bool {
        match self {
            Self::Float32(value) => value.is_nan(),
            Self::Float64(value) => value.is_nan(),
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
        match self {
            Self::Float32(value) if value.is_infinite() => {
                Some(value.is_sign_negative())
            }
            Self::Float64(value) if value.is_infinite() => {
                Some(value.is_sign_negative())
            }
            _ => None,
        }
    }

    /// Projects this value to `f64` for approximate comparison.
    ///
    /// # Returns
    ///
    /// The projected numeric value, or `None` for the hidden lifetime marker.
    #[inline]
    pub(crate) fn to_f64(self) -> Option<f64> {
        match self {
            Self::Int8(value) => Some(f64::from(value)),
            Self::Int16(value) => Some(f64::from(value)),
            Self::Int32(value) => Some(f64::from(value)),
            Self::Int64(value) => Some(value as f64),
            Self::Int128(value) => Some(value as f64),
            Self::UInt8(value) => Some(f64::from(value)),
            Self::UInt16(value) => Some(f64::from(value)),
            Self::UInt32(value) => Some(f64::from(value)),
            Self::UInt64(value) => Some(value as f64),
            Self::UInt128(value) => Some(value as f64),
            Self::Float32(value) => Some(f64::from(value)),
            Self::Float64(value) => Some(value),
            #[cfg(feature = "big-number")]
            Self::BigInteger(value) => value.to_f64(),
            #[cfg(feature = "big-number")]
            Self::BigDecimal(value) => value.to_f64(),
            Self::__Lifetime(_) => None,
        }
    }
}
