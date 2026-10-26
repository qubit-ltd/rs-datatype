// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Policy-driven numeric comparison benchmarks.

use std::cmp::Ordering;
use std::hint::black_box;
use std::str::FromStr;

use bigdecimal::BigDecimal;
use criterion::BenchmarkId;
use criterion::Criterion;
use criterion::criterion_group;
use criterion::criterion_main;
use num_bigint::BigInt;
use qubit_datatype::NumberRef;
use qubit_datatype::NumericComparisonPolicy;

/// Benchmarks exact and approximate comparison at the `f64` integer boundary.
fn benchmark_u64_f64_boundary(c: &mut Criterion) {
    let integer = NumberRef::from((1_u64 << 53) + 1);
    let float = NumberRef::from((1_u64 << 53) as f64);
    let mut group = c.benchmark_group("u64_f64_2pow53_boundary");

    for policy in [NumericComparisonPolicy::Exact, NumericComparisonPolicy::Approximate] {
        group.bench_function(format!("{policy:?}").to_lowercase(), |b| {
            b.iter(|| black_box(black_box(integer).compare(black_box(float), black_box(policy))));
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
        b.iter(|| black_box(black_box(left).compare(black_box(right), black_box(policy))));
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
            black_box(black_box(left).compare(black_box(right), black_box(policy)))
        });
    });
    group.bench_function("exact_bigint_vs_bigint_equal", |b| {
        b.iter(|| {
            let left = NumberRef::from(black_box(&integer));
            let right = NumberRef::from(black_box(&same_integer));
            black_box(black_box(left).compare(black_box(right), black_box(policy)))
        });
    });
    group.finish();
}

/// Benchmarks exact comparisons between large integers and finite floats.
fn benchmark_big_integer_float(c: &mut Criterion) {
    let policy = NumericComparisonPolicy::Exact;
    let floats = [
        ("f32_one", NumberRef::from(1.0_f32)),
        ("f32_negative_one", NumberRef::from(-1.0_f32)),
        ("f64_one", NumberRef::from(1.0_f64)),
        ("f64_negative_one", NumberRef::from(-1.0_f64)),
        ("f32_zero", NumberRef::from(0.0_f32)),
        ("f64_zero", NumberRef::from(-0.0_f64)),
    ];
    let positive_infinity = NumberRef::from(f64::INFINITY);
    let negative_infinity = NumberRef::from(f64::NEG_INFINITY);
    let nan = NumberRef::from(f64::NAN);
    let mut group = c.benchmark_group("bigint_float_exact");
    group.sample_size(30);
    group.warm_up_time(std::time::Duration::from_millis(500));
    group.measurement_time(std::time::Duration::from_secs(1));

    for bits in [63_u64, 1023, 4095, 65535] {
        let positive = BigInt::from(1_u8) << bits;
        let negative = -&positive;
        assert_eq!(positive.bits(), bits + 1);
        assert_eq!(negative.bits(), bits + 1);

        for (sign, integer, expected) in [
            ("positive", &positive, Ordering::Greater),
            ("negative", &negative, Ordering::Less),
        ] {
            for (float_name, float) in floats {
                let integer_ref = NumberRef::from(integer);
                assert_eq!(integer_ref.compare(float, policy), Some(expected));
                assert_eq!(float.compare(integer_ref, policy), Some(expected.reverse()));
                assert_eq!(integer_ref.compare(positive_infinity, policy), Some(Ordering::Less));
                assert_eq!(positive_infinity.compare(integer_ref, policy), Some(Ordering::Greater));
                assert_eq!(integer_ref.compare(negative_infinity, policy), Some(Ordering::Greater));
                assert_eq!(negative_infinity.compare(integer_ref, policy), Some(Ordering::Less));
                assert_eq!(integer_ref.compare(nan, policy), None);
                assert_eq!(nan.compare(integer_ref, policy), None);

                for (direction, left, right, ordering) in [
                    ("bigint_vs_float", integer_ref, float, expected),
                    ("float_vs_bigint", float, integer_ref, expected.reverse()),
                ] {
                    group.bench_function(format!("{bits}_bits/{sign}/{float_name}/{direction}"), |b| {
                        b.iter(|| black_box(black_box(left).compare(black_box(right), black_box(policy))));
                    });
                    assert_eq!(left.compare(right, policy), Some(ordering));
                }
            }
        }
    }
    group.finish();
}

/// Benchmarks exact comparison between decimal and binary representations of
/// one tenth.
fn benchmark_big_decimal(c: &mut Criterion) {
    let decimal = BigDecimal::from_str("0.1").expect("the benchmark decimal literal should be valid");
    let same_scale = BigDecimal::from_str("0.1").expect("the benchmark decimal literal should be valid");
    let different_scale = BigDecimal::from_str("0.10").expect("the benchmark decimal literal should be valid");
    let right = NumberRef::from(0.1_f64);
    let policy = NumericComparisonPolicy::Exact;
    let mut group = c.benchmark_group("big_decimal");

    group.bench_function("exact_bigdecimal_0_1_vs_f64_0_1", |b| {
        b.iter(|| {
            let left = NumberRef::from(black_box(&decimal));
            black_box(black_box(left).compare(black_box(right), black_box(policy)))
        });
    });
    group.bench_function("exact_bigdecimal_same_scale_equal", |b| {
        b.iter(|| {
            let left = NumberRef::from(black_box(&decimal));
            let right = NumberRef::from(black_box(&same_scale));
            black_box(black_box(left).compare(black_box(right), black_box(policy)))
        });
    });
    group.bench_function("exact_bigdecimal_different_scale_equal", |b| {
        b.iter(|| {
            let left = NumberRef::from(black_box(&decimal));
            let right = NumberRef::from(black_box(&different_scale));
            black_box(black_box(left).compare(black_box(right), black_box(policy)))
        });
    });
    group.finish();
}

/// Builds decimal candidates for membership comparisons against `42`.
fn decimal_candidates(count: usize, includes_match: bool) -> Vec<BigDecimal> {
    (0..count)
        .map(|index| {
            if includes_match && index + 1 == count {
                BigDecimal::from(42)
            } else {
                BigDecimal::from(43_u64 + index as u64)
            }
        })
        .collect()
}

/// Builds integer candidates for membership comparisons against `42`.
fn integer_candidates(count: usize, includes_match: bool) -> Vec<BigInt> {
    (0..count)
        .map(|index| {
            if includes_match && index + 1 == count {
                BigInt::from(42)
            } else {
                BigInt::from(43_u64 + index as u64)
            }
        })
        .collect()
}

/// Benchmarks exact `BigInt` and `BigDecimal` comparisons in both directions.
fn benchmark_big_integer_decimal_pairs(c: &mut Criterion) {
    let integer = BigInt::from(42);
    let equal_decimal = BigDecimal::from(42);
    let fractional_decimal = BigDecimal::from_str("42.5").expect("the benchmark decimal literal should be valid");
    let policy = NumericComparisonPolicy::Exact;
    let mut group = c.benchmark_group("big_integer_decimal_pairs");

    group.bench_function("bigint_vs_bigdecimal_equal", |b| {
        b.iter(|| {
            let left = NumberRef::from(black_box(&integer));
            let right = NumberRef::from(black_box(&equal_decimal));
            black_box(black_box(left).compare(black_box(right), black_box(policy)))
        });
    });
    group.bench_function("bigdecimal_vs_bigint_equal", |b| {
        b.iter(|| {
            let left = NumberRef::from(black_box(&equal_decimal));
            let right = NumberRef::from(black_box(&integer));
            black_box(black_box(left).compare(black_box(right), black_box(policy)))
        });
    });
    group.bench_function("bigint_vs_bigdecimal_fractional", |b| {
        b.iter(|| {
            let left = NumberRef::from(black_box(&integer));
            let right = NumberRef::from(black_box(&fractional_decimal));
            black_box(black_box(left).compare(black_box(right), black_box(policy)))
        });
    });
    group.bench_function("bigdecimal_vs_bigint_fractional", |b| {
        b.iter(|| {
            let left = NumberRef::from(black_box(&fractional_decimal));
            let right = NumberRef::from(black_box(&integer));
            black_box(black_box(left).compare(black_box(right), black_box(policy)))
        });
    });
    group.finish();
}

/// Benchmarks `In` and `NotIn`-style scans across big numeric representations.
fn benchmark_big_numeric_membership(c: &mut Criterion) {
    let integer = BigInt::from(42);
    let decimal = BigDecimal::from(42);
    let policy = NumericComparisonPolicy::Exact;
    let mut group = c.benchmark_group("big_numeric_membership");

    for count in [1_usize, 16, 64] {
        for (case, candidates) in [
            ("last_match", decimal_candidates(count, true)),
            ("no_match", decimal_candidates(count, false)),
        ] {
            group.bench_with_input(
                BenchmarkId::new(format!("bigint_any_{case}"), count),
                &candidates,
                |b, candidates| {
                    b.iter(|| {
                        let stored = NumberRef::from(black_box(&integer));
                        black_box(candidates.iter().any(|candidate| {
                            stored.compare(NumberRef::from(black_box(candidate)), black_box(policy))
                                == Some(Ordering::Equal)
                        }))
                    });
                },
            );
            group.bench_with_input(
                BenchmarkId::new(format!("bigint_all_{case}"), count),
                &candidates,
                |b, candidates| {
                    b.iter(|| {
                        let stored = NumberRef::from(black_box(&integer));
                        black_box(candidates.iter().all(|candidate| {
                            stored.compare(NumberRef::from(black_box(candidate)), black_box(policy))
                                != Some(Ordering::Equal)
                        }))
                    });
                },
            );
        }
        for (case, candidates) in [
            ("last_match", integer_candidates(count, true)),
            ("no_match", integer_candidates(count, false)),
        ] {
            group.bench_with_input(
                BenchmarkId::new(format!("bigdecimal_any_{case}"), count),
                &candidates,
                |b, candidates| {
                    b.iter(|| {
                        let stored = NumberRef::from(black_box(&decimal));
                        black_box(candidates.iter().any(|candidate| {
                            stored.compare(NumberRef::from(black_box(candidate)), black_box(policy))
                                == Some(Ordering::Equal)
                        }))
                    });
                },
            );
            group.bench_with_input(
                BenchmarkId::new(format!("bigdecimal_all_{case}"), count),
                &candidates,
                |b, candidates| {
                    b.iter(|| {
                        let stored = NumberRef::from(black_box(&decimal));
                        black_box(candidates.iter().all(|candidate| {
                            stored.compare(NumberRef::from(black_box(candidate)), black_box(policy))
                                != Some(Ordering::Equal)
                        }))
                    });
                },
            );
        }
    }
    group.finish();
}

criterion_group!(
    benches,
    benchmark_u64_f64_boundary,
    benchmark_fixed_width_extremes,
    benchmark_big_integer,
    benchmark_big_integer_float,
    benchmark_big_decimal,
    benchmark_big_integer_decimal_pairs,
    benchmark_big_numeric_membership,
);
criterion_main!(benches);
