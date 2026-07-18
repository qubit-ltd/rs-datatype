// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Internal numeric representations.

#[cfg(not(any(feature = "big-integer", feature = "big-decimal")))]
use std::convert::Infallible;
#[cfg(not(any(feature = "big-integer", feature = "big-decimal")))]
use std::marker::PhantomData;

#[cfg(feature = "big-decimal")]
use bigdecimal::BigDecimal;
#[cfg(feature = "big-integer")]
use num_bigint::BigInt;

/// Internal numeric representations.
#[derive(Debug, Clone, Copy)]
pub(in crate::numeric) enum NumericValue<'a> {
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
    #[cfg(feature = "big-integer")]
    BigInteger(&'a BigInt),
    /// An arbitrary-precision decimal.
    #[cfg(feature = "big-decimal")]
    BigDecimal(&'a BigDecimal),
    /// Uninhabited branch retaining the borrowing lifetime without big-number
    /// support.
    #[cfg(not(any(feature = "big-integer", feature = "big-decimal")))]
    #[expect(
        dead_code,
        reason = "the Infallible payload makes this lifetime branch uninhabited"
    )]
    Lifetime(PhantomData<&'a ()>, Infallible),
}
