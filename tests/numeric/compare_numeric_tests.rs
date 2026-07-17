// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use std::cmp::Ordering;
use std::marker::PhantomData;

#[cfg(feature = "big-number")]
use bigdecimal::BigDecimal;
#[cfg(feature = "big-number")]
use num_bigint::BigInt;
use qubit_datatype::{
    NumericComparisonPolicy,
    NumericValueRef,
    compare_numeric,
};
#[cfg(feature = "big-number")]
use std::str::FromStr;

/// Asserts an exact ordering and its reverse-direction symmetry.
fn assert_exact(
    left: NumericValueRef<'_>,
    right: NumericValueRef<'_>,
    expected: Option<Ordering>,
) {
    assert_eq!(
        compare_numeric(left, right, NumericComparisonPolicy::Exact),
        expected
    );
    assert_eq!(
        compare_numeric(right, left, NumericComparisonPolicy::Exact),
        expected.map(Ordering::reverse)
    );
}

/// Covers exact fixed-width boundaries without integer-to-float projection.
#[test]
fn test_compare_numeric_exact_fixed_boundaries() {
    assert_exact(
        NumericValueRef::Int128(i128::MIN),
        NumericValueRef::UInt128(u128::MAX),
        Some(Ordering::Less),
    );
    assert_exact(
        NumericValueRef::UInt64((1_u64 << 53) + 1),
        NumericValueRef::Float64((1_u64 << 53) as f64),
        Some(Ordering::Greater),
    );
    assert_exact(
        NumericValueRef::UInt32((1_u32 << 24) + 1),
        NumericValueRef::Float32((1_u32 << 24) as f32),
        Some(Ordering::Greater),
    );
    assert_exact(
        NumericValueRef::Int32(1),
        NumericValueRef::Float64(1.5),
        Some(Ordering::Less),
    );
    assert_exact(
        NumericValueRef::Float64(-0.0),
        NumericValueRef::Float64(0.0),
        Some(Ordering::Equal),
    );
    assert_exact(
        NumericValueRef::Float64(f64::NEG_INFINITY),
        NumericValueRef::Int8(i8::MIN),
        Some(Ordering::Less),
    );
    assert_exact(
        NumericValueRef::Float32(f32::INFINITY),
        NumericValueRef::UInt128(u128::MAX),
        Some(Ordering::Greater),
    );
    assert_exact(
        NumericValueRef::Float32(f32::NEG_INFINITY),
        NumericValueRef::Float64(f64::INFINITY),
        Some(Ordering::Less),
    );
    assert_exact(
        NumericValueRef::Float32(f32::INFINITY),
        NumericValueRef::Float64(f64::INFINITY),
        Some(Ordering::Equal),
    );

    for value in [
        NumericValueRef::Int8(1),
        NumericValueRef::Int16(1),
        NumericValueRef::Int32(1),
        NumericValueRef::Int64(1),
        NumericValueRef::Int128(1),
        NumericValueRef::UInt8(1),
        NumericValueRef::UInt16(1),
        NumericValueRef::UInt32(1),
        NumericValueRef::UInt64(1),
        NumericValueRef::UInt128(1),
        NumericValueRef::Float32(1.0),
        NumericValueRef::Float64(1.0),
    ] {
        assert_exact(value, NumericValueRef::Int8(1), Some(Ordering::Equal));
    }
    assert_exact(
        NumericValueRef::Float32(f32::from_bits(1)),
        NumericValueRef::Int8(0),
        Some(Ordering::Greater),
    );
    assert_exact(
        NumericValueRef::Float64(-f64::from_bits(1)),
        NumericValueRef::Int8(0),
        Some(Ordering::Less),
    );
    assert_exact(
        NumericValueRef::Float64(4.0),
        NumericValueRef::Float64(2.0),
        Some(Ordering::Greater),
    );
    assert_exact(
        NumericValueRef::Float64(-4.0),
        NumericValueRef::Float64(-2.0),
        Some(Ordering::Less),
    );
}

/// Verifies that every NaN payload is unordered.
#[test]
fn test_compare_numeric_rejects_nan() {
    for nan in [
        f64::from_bits(0x7ff8_0000_0000_0001),
        f64::from_bits(0x7fff_ffff_ffff_ffff),
    ] {
        assert_exact(
            NumericValueRef::Float64(nan),
            NumericValueRef::Float64(nan),
            None,
        );
        assert_exact(
            NumericValueRef::Float64(nan),
            NumericValueRef::Int8(0),
            None,
        );
    }
}

/// Verifies approximate projection only changes float-participating pairs.
#[test]
fn test_compare_numeric_approximate_policy() {
    assert_eq!(
        compare_numeric(
            NumericValueRef::UInt64((1_u64 << 53) + 1),
            NumericValueRef::Float64((1_u64 << 53) as f64),
            NumericComparisonPolicy::Approximate,
        ),
        Some(Ordering::Equal)
    );
    assert_eq!(
        compare_numeric(
            NumericValueRef::UInt128(u128::MAX - 1),
            NumericValueRef::UInt128(u128::MAX),
            NumericComparisonPolicy::Approximate,
        ),
        Some(Ordering::Less)
    );

    for value in [
        NumericValueRef::Int8(0),
        NumericValueRef::Int16(0),
        NumericValueRef::Int32(0),
        NumericValueRef::Int64(0),
        NumericValueRef::Int128(0),
        NumericValueRef::UInt8(0),
        NumericValueRef::UInt16(0),
        NumericValueRef::UInt32(0),
        NumericValueRef::UInt64(0),
        NumericValueRef::UInt128(0),
        NumericValueRef::Float32(0.0),
        NumericValueRef::Float64(0.0),
    ] {
        assert_eq!(
            compare_numeric(
                value,
                NumericValueRef::Float64(0.0),
                NumericComparisonPolicy::Approximate,
            ),
            Some(Ordering::Equal)
        );
    }
    assert_eq!(
        compare_numeric(
            NumericValueRef::__Lifetime(PhantomData),
            NumericValueRef::Float64(0.0),
            NumericComparisonPolicy::Approximate,
        ),
        None
    );
}

/// Covers exact and approximate arbitrary-precision comparisons.
#[cfg(feature = "big-number")]
#[test]
fn test_compare_numeric_big_number_paths() {
    let integer = BigInt::from(u128::MAX) + BigInt::from(1_u8);
    assert_exact(
        NumericValueRef::BigInteger(&integer),
        NumericValueRef::UInt128(u128::MAX),
        Some(Ordering::Greater),
    );

    let decimal = BigDecimal::from_str("0.1").unwrap();
    assert_eq!(
        compare_numeric(
            NumericValueRef::BigDecimal(&decimal),
            NumericValueRef::Float64(0.1),
            NumericComparisonPolicy::Exact,
        ),
        Some(Ordering::Less)
    );
    assert_eq!(
        compare_numeric(
            NumericValueRef::BigDecimal(&decimal),
            NumericValueRef::Float64(0.1),
            NumericComparisonPolicy::Approximate,
        ),
        Some(Ordering::Equal)
    );

    for value in [
        NumericValueRef::Int8(1),
        NumericValueRef::Int16(1),
        NumericValueRef::Int32(1),
        NumericValueRef::Int64(1),
        NumericValueRef::Int128(1),
        NumericValueRef::UInt8(1),
        NumericValueRef::UInt16(1),
        NumericValueRef::UInt32(1),
        NumericValueRef::UInt64(1),
        NumericValueRef::UInt128(1),
        NumericValueRef::Float32(1.0),
        NumericValueRef::Float64(1.0),
    ] {
        assert_exact(
            NumericValueRef::BigInteger(&BigInt::from(1)),
            value,
            Some(Ordering::Equal),
        );
    }

    assert_exact(
        NumericValueRef::BigInteger(&BigInt::from(0)),
        NumericValueRef::Float32(-f32::from_bits(1)),
        Some(Ordering::Greater),
    );
    for value in [
        NumericValueRef::Float64(f64::from_bits(1)),
        NumericValueRef::Float64(f64::MAX),
    ] {
        assert_exact(
            NumericValueRef::BigInteger(&BigInt::from(0)),
            value,
            Some(Ordering::Less),
        );
    }

    let negative_scale = BigDecimal::new(BigInt::from(12), -2);
    assert_exact(
        NumericValueRef::BigDecimal(&negative_scale),
        NumericValueRef::BigInteger(&BigInt::from(1_200)),
        Some(Ordering::Equal),
    );

    let extreme_scale =
        BigDecimal::new(BigInt::from(0), i64::from(u32::MAX) + 1);
    let extreme_negative_scale =
        BigDecimal::new(BigInt::from(0), -(i64::from(u32::MAX) + 1));
    assert_exact(
        NumericValueRef::BigDecimal(&extreme_negative_scale),
        NumericValueRef::BigDecimal(&extreme_scale),
        Some(Ordering::Equal),
    );
    let large_scale = BigDecimal::new(BigInt::from(1), 1_000_000);
    let large_negative_scale = BigDecimal::new(BigInt::from(1), -1_000_000);
    assert_exact(
        NumericValueRef::BigDecimal(&large_scale),
        NumericValueRef::BigInteger(&BigInt::from(0)),
        Some(Ordering::Greater),
    );
    assert_exact(
        NumericValueRef::BigDecimal(&large_negative_scale),
        NumericValueRef::BigInteger(&BigInt::from(0)),
        Some(Ordering::Greater),
    );
    for value in [
        NumericValueRef::Int8(0),
        NumericValueRef::Int16(0),
        NumericValueRef::Int32(0),
        NumericValueRef::Int64(0),
        NumericValueRef::Int128(0),
        NumericValueRef::UInt8(0),
        NumericValueRef::UInt16(0),
        NumericValueRef::UInt32(0),
        NumericValueRef::UInt64(0),
        NumericValueRef::UInt128(0),
        NumericValueRef::Float32(0.0),
        NumericValueRef::Float64(0.0),
        NumericValueRef::BigInteger(&integer),
        NumericValueRef::BigDecimal(&decimal),
    ] {
        let expected = if matches!(
            value,
            NumericValueRef::BigInteger(_) | NumericValueRef::BigDecimal(_)
        ) {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Equal)
        };
        assert_exact(
            NumericValueRef::BigDecimal(&extreme_scale),
            value,
            expected,
        );
    }
    assert_exact(
        NumericValueRef::BigInteger(&BigInt::from(0)),
        NumericValueRef::__Lifetime(PhantomData),
        None,
    );

    let too_large_for_f64 = BigInt::from(1_u8) << 20_000;
    assert_eq!(
        compare_numeric(
            NumericValueRef::BigInteger(&too_large_for_f64),
            NumericValueRef::Float64(1.0),
            NumericComparisonPolicy::Approximate,
        ),
        Some(Ordering::Greater)
    );
}
