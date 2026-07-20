// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Private numeric representation used by [`crate::NumberRef`].

#[cfg(not(any(feature = "big-integer", feature = "big-decimal")))]
use std::convert::Infallible;
#[cfg(not(any(feature = "big-integer", feature = "big-decimal")))]
use std::marker::PhantomData;

#[cfg(feature = "big-decimal")]
use bigdecimal::BigDecimal;
#[cfg(feature = "big-integer")]
use num_bigint::BigInt;

/// Private storage representation for [`crate::NumberRef`].
#[derive(Debug, Clone, Copy)]
pub(in crate::numeric) enum NumberRepr<'a> {
    /// An `i8` value.
    Int8(
        /// Stored 8-bit signed integer value.
        i8,
    ),
    /// An `i16` value.
    Int16(
        /// Stored 16-bit signed integer value.
        i16,
    ),
    /// An `i32` value.
    Int32(
        /// Stored 32-bit signed integer value.
        i32,
    ),
    /// An `i64` value.
    Int64(
        /// Stored 64-bit signed integer value.
        i64,
    ),
    /// An `i128` value.
    Int128(
        /// Stored 128-bit signed integer value.
        i128,
    ),
    /// A `u8` value.
    UInt8(
        /// Stored 8-bit unsigned integer value.
        u8,
    ),
    /// A `u16` value.
    UInt16(
        /// Stored 16-bit unsigned integer value.
        u16,
    ),
    /// A `u32` value.
    UInt32(
        /// Stored 32-bit unsigned integer value.
        u32,
    ),
    /// A `u64` value.
    UInt64(
        /// Stored 64-bit unsigned integer value.
        u64,
    ),
    /// A `u128` value.
    UInt128(
        /// Stored 128-bit unsigned integer value.
        u128,
    ),
    /// An `f32` value.
    Float32(
        /// Stored 32-bit floating-point value.
        f32,
    ),
    /// An `f64` value.
    Float64(
        /// Stored 64-bit floating-point value.
        f64,
    ),
    /// An arbitrary-precision integer.
    #[cfg(feature = "big-integer")]
    BigInteger(
        /// Borrowed arbitrary-precision integer value.
        &'a BigInt,
    ),
    /// An arbitrary-precision decimal.
    #[cfg(feature = "big-decimal")]
    BigDecimal(
        /// Borrowed arbitrary-precision decimal value.
        &'a BigDecimal,
    ),
    /// Uninhabited branch retaining the borrowing lifetime without big-number
    /// support.
    #[cfg(not(any(feature = "big-integer", feature = "big-decimal")))]
    #[expect(
        dead_code,
        reason = "the Infallible payload makes this lifetime branch uninhabited"
    )]
    Lifetime(
        /// Marker retaining the borrow lifetime without big-number support.
        PhantomData<&'a ()>,
        /// Uninhabited payload preventing this variant from being constructed.
        Infallible,
    ),
}
