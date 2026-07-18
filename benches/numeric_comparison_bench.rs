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
    NumberRef,
    NumericComparisonPolicy,
};

/// Benchmarks exact and approximate comparison at the `f64` integer boundary.
fn benchmark_u64_f64_boundary(c: &mut Criterion) {
    let integer = NumberRef::from((1_u64 << 53) + 1);
    let float = NumberRef::from((1_u64 << 53) as f64);
    let mut group = c.benchmark_group("u64_f64_2pow53_boundary");

    for policy in [
        NumericComparisonPolicy::Exact,
        NumericComparisonPolicy::Approximate,
    ] {
        group.bench_function(format!("{policy:?}").to_lowercase(), |b| {
            b.iter(|| {
                black_box(
                    black_box(integer)
                        .compare_to(black_box(float), black_box(policy)),
                )
            });
        });
    }
    group.finish();
}

/// Benchmarks exact comparison across fixed-width signedness boundaries.
fn benchmark_fixed_width_extremes(c: &mut Criterion) {
    let left = NumberRef::from(i128::MIN);
    let right = NumberRef::from(u128::MAX);
    let policy = NumericComparisonPolicy::Exact;
    let mut group = c.benchmark_group("fixed_width_extremes");

    group.bench_function("exact_i128_min_vs_u128_max", |b| {
        b.iter(|| {
            black_box(
                black_box(left).compare_to(black_box(right), black_box(policy)),
            )
        });
    });
    group.finish();
}

/// Benchmarks exact comparison between `BigInt` and `u128`.
fn benchmark_big_integer(c: &mut Criterion) {
    let integer = BigInt::from(u128::MAX);
    let same_integer = BigInt::from(u128::MAX);
    let right = NumberRef::from(u128::MAX);
    let policy = NumericComparisonPolicy::Exact;
    let mut group = c.benchmark_group("big_integer");

    group.bench_function("exact_bigint_vs_u128_max", |b| {
        b.iter(|| {
            let left = NumberRef::from(black_box(&integer));
            black_box(
                black_box(left).compare_to(black_box(right), black_box(policy)),
            )
        });
    });
    group.bench_function("exact_bigint_vs_bigint_equal", |b| {
        b.iter(|| {
            let left = NumberRef::from(black_box(&integer));
            let right = NumberRef::from(black_box(&same_integer));
            black_box(
                black_box(left).compare_to(black_box(right), black_box(policy)),
            )
        });
    });
    group.finish();
}

/// Benchmarks exact comparison between decimal and binary representations of
/// one tenth.
fn benchmark_big_decimal(c: &mut Criterion) {
    let decimal = BigDecimal::from_str("0.1")
        .expect("the benchmark decimal literal should be valid");
    let same_scale = BigDecimal::from_str("0.1")
        .expect("the benchmark decimal literal should be valid");
    let different_scale = BigDecimal::from_str("0.10")
        .expect("the benchmark decimal literal should be valid");
    let right = NumberRef::from(0.1_f64);
    let policy = NumericComparisonPolicy::Exact;
    let mut group = c.benchmark_group("big_decimal");

    group.bench_function("exact_bigdecimal_0_1_vs_f64_0_1", |b| {
        b.iter(|| {
            let left = NumberRef::from(black_box(&decimal));
            black_box(
                black_box(left).compare_to(black_box(right), black_box(policy)),
            )
        });
    });
    group.bench_function("exact_bigdecimal_same_scale_equal", |b| {
        b.iter(|| {
            let left = NumberRef::from(black_box(&decimal));
            let right = NumberRef::from(black_box(&same_scale));
            black_box(
                black_box(left).compare_to(black_box(right), black_box(policy)),
            )
        });
    });
    group.bench_function("exact_bigdecimal_different_scale_equal", |b| {
        b.iter(|| {
            let left = NumberRef::from(black_box(&decimal));
            let right = NumberRef::from(black_box(&different_scale));
            black_box(
                black_box(left).compare_to(black_box(right), black_box(policy)),
            )
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
