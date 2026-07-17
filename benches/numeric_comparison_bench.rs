// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Policy-driven numeric comparison benchmarks.

use std::hint::black_box;
use std::str::FromStr;

use bigdecimal::BigDecimal;
use criterion::{
    Criterion,
    criterion_group,
    criterion_main,
};
use num_bigint::BigInt;
use qubit_datatype::{
    NumericComparisonPolicy,
    NumericValueRef,
    compare_numeric,
};

/// Benchmarks exact and approximate comparison at the `f64` integer boundary.
fn benchmark_u64_f64_boundary(c: &mut Criterion) {
    let integer = NumericValueRef::UInt64((1_u64 << 53) + 1);
    let float = NumericValueRef::Float64((1_u64 << 53) as f64);
    let mut group = c.benchmark_group("u64_f64_2pow53_boundary");

    for policy in [
        NumericComparisonPolicy::Exact,
        NumericComparisonPolicy::Approximate,
    ] {
        group.bench_function(format!("{policy:?}").to_lowercase(), |b| {
            b.iter(|| {
                black_box(compare_numeric(
                    black_box(integer),
                    black_box(float),
                    black_box(policy),
                ))
            });
        });
    }
    group.finish();
}

/// Benchmarks exact comparison across fixed-width signedness boundaries.
fn benchmark_fixed_width_extremes(c: &mut Criterion) {
    let left = NumericValueRef::Int128(i128::MIN);
    let right = NumericValueRef::UInt128(u128::MAX);
    let policy = NumericComparisonPolicy::Exact;
    let mut group = c.benchmark_group("fixed_width_extremes");

    group.bench_function("exact_i128_min_vs_u128_max", |b| {
        b.iter(|| {
            black_box(compare_numeric(
                black_box(left),
                black_box(right),
                black_box(policy),
            ))
        });
    });
    group.finish();
}

/// Benchmarks exact comparison between `BigInt` and `u128`.
fn benchmark_big_integer(c: &mut Criterion) {
    let integer = BigInt::from(u128::MAX);
    let right = NumericValueRef::UInt128(u128::MAX);
    let policy = NumericComparisonPolicy::Exact;
    let mut group = c.benchmark_group("big_integer");

    group.bench_function("exact_bigint_vs_u128_max", |b| {
        b.iter(|| {
            let left = NumericValueRef::BigInteger(black_box(&integer));
            black_box(compare_numeric(
                black_box(left),
                black_box(right),
                black_box(policy),
            ))
        });
    });
    group.finish();
}

/// Benchmarks exact comparison between decimal and binary representations of
/// one tenth.
fn benchmark_big_decimal(c: &mut Criterion) {
    let decimal = BigDecimal::from_str("0.1")
        .expect("the benchmark decimal literal should be valid");
    let right = NumericValueRef::Float64(0.1_f64);
    let policy = NumericComparisonPolicy::Exact;
    let mut group = c.benchmark_group("big_decimal");

    group.bench_function("exact_bigdecimal_0_1_vs_f64_0_1", |b| {
        b.iter(|| {
            let left = NumericValueRef::BigDecimal(black_box(&decimal));
            black_box(compare_numeric(
                black_box(left),
                black_box(right),
                black_box(policy),
            ))
        });
    });
    group.finish();
}

criterion_group!(
    benches,
    benchmark_u64_f64_boundary,
    benchmark_fixed_width_extremes,
    benchmark_big_integer,
    benchmark_big_decimal,
);
criterion_main!(benches);
