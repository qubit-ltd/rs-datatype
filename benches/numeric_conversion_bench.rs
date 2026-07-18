// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Numeric text conversion benchmarks.

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use num_bigint::BigUint;
use qubit_datatype::{DataConversionOptions, DataConverter};
use std::hint::black_box;

/// Benchmarks representative exact integer text conversions.
fn benchmark_exact_integer_text(c: &mut Criterion) {
    let mut group = c.benchmark_group("numeric_text_to_i64_exact");
    for (name, source) in [
        ("short_integer", "42"),
        ("signed_integer", "-9223372036854775808"),
        ("scientific_integer", "12345e4"),
        ("exact_decimal", "12345.000"),
    ] {
        group.bench_with_input(BenchmarkId::from_parameter(name), source, |b, source| {
            b.iter(|| black_box(DataConverter::from(black_box(source)).to::<i64>()));
        });
    }
    group.finish();
}

/// Benchmarks representative lossy integer text conversions.
fn benchmark_lossy_integer_text(c: &mut Criterion) {
    let options = DataConversionOptions::lossy();
    let mut group = c.benchmark_group("numeric_text_to_i64_lossy");
    for (name, source) in [
        ("fractional", "12345.6789"),
        ("fractional_scientific", "12345.6789e2"),
        ("small_fraction", "0.000000001"),
    ] {
        group.bench_with_input(BenchmarkId::from_parameter(name), source, |b, source| {
            b.iter(|| {
                black_box(
                    DataConverter::from(black_box(source)).to_with::<i64>(black_box(&options)),
                )
            });
        });
    }
    group.finish();
}

/// Benchmarks exact text-to-f64 conversion paths.
fn benchmark_exact_float_text(c: &mut Criterion) {
    let unbounded_exact = (BigUint::from(1_u8) << 128_u32).to_string();
    let long_coefficient = BigUint::from(5_u8).pow(4096).to_string();
    let long_inexact = format!(
        "{long_coefficient}e-{}",
        long_coefficient.len().saturating_sub(1)
    );
    let sources = [
        ("bounded_exact", "0.5".to_string()),
        ("unbounded_exact", unbounded_exact),
        ("long_inexact_power_of_five", long_inexact),
    ];
    let mut group = c.benchmark_group("numeric_text_to_f64_exact");
    for (name, source) in &sources {
        group.bench_with_input(BenchmarkId::from_parameter(name), source, |b, source| {
            b.iter(|| black_box(DataConverter::from(black_box(source.as_str())).to::<f64>()));
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    benchmark_exact_integer_text,
    benchmark_lossy_integer_text,
    benchmark_exact_float_text,
);
criterion_main!(benches);
