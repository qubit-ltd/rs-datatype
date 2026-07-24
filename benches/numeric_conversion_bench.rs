// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Numeric text conversion benchmarks.

use criterion::{
    BenchmarkId,
    Criterion,
    criterion_group,
    criterion_main,
};
use qubit_datatype::{
    DataConversionOptions,
    DataConverter,
};
#[cfg(any(feature = "big-integer", feature = "big-decimal"))]
use qubit_datatype::{
    NumericConversionLimits,
    NumericConversionOptions,
};
use std::hint::black_box;

#[cfg(any(feature = "big-integer", feature = "big-decimal"))]
const BIG_NUMBER_TEXT_SIZES: [(&str, usize); 4] = [
    ("1KiB", 1024),
    ("16KiB", 16 * 1024),
    ("64KiB", 64 * 1024),
    ("256KiB", 256 * 1024),
];

/// Benchmarks representative exact integer text conversions.
fn benchmark_exact_integer_text(c: &mut Criterion) {
    let mut group = c.benchmark_group("numeric_text_to_i64_exact");
    for (name, source) in [
        ("short_integer", "42"),
        ("signed_integer", "-9223372036854775808"),
        ("scientific_integer", "12345e4"),
        ("exact_decimal", "12345.000"),
    ] {
        DataConverter::from(source)
            .to::<i64>()
            .expect("exact integer benchmark fixture should convert");
        group.bench_with_input(
            BenchmarkId::from_parameter(name),
            source,
            |b, source| {
                b.iter(|| {
                    black_box(
                        DataConverter::from(black_box(source)).to::<i64>(),
                    )
                });
            },
        );
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
        DataConverter::from(source)
            .to_with::<i64>(&options)
            .expect("lossy integer benchmark fixture should convert");
        group.bench_with_input(
            BenchmarkId::from_parameter(name),
            source,
            |b, source| {
                b.iter(|| {
                    black_box(
                        DataConverter::from(black_box(source))
                            .to_with::<i64>(black_box(&options)),
                    )
                });
            },
        );
    }
    group.finish();
}

/// Benchmarks exact text-to-f64 conversion paths.
fn benchmark_exact_float_text(c: &mut Criterion) {
    let unbounded_exact = "340282366920938463463374607431768211456";
    let long_coefficient = "5".repeat(4096);
    let long_inexact = format!(
        "{long_coefficient}e-{}",
        long_coefficient.len().saturating_sub(1)
    );
    let sources = [
        ("bounded_exact", "0.5".to_string()),
        ("unbounded_exact", unbounded_exact.to_string()),
        ("long_inexact_coefficient", long_inexact),
    ];
    let mut group = c.benchmark_group("numeric_text_to_f64_exact");
    for (name, source) in &sources {
        DataConverter::from(source.as_str())
            .to::<f64>()
            .expect("exact float benchmark fixture should convert");
        group.bench_with_input(
            BenchmarkId::from_parameter(name),
            source,
            |b, source| {
                b.iter(|| {
                    black_box(
                        DataConverter::from(black_box(source.as_str()))
                            .to::<f64>(),
                    )
                });
            },
        );
    }
    group.finish();
}

/// Benchmarks text-to-BigInt parsing across representative input sizes.
fn benchmark_big_integer_text(c: &mut Criterion) {
    #[cfg(feature = "big-integer")]
    {
        let mut group = c.benchmark_group("numeric_text_to_big_integer");
        for (name, digits) in BIG_NUMBER_TEXT_SIZES {
            let source = "9".repeat(digits);
            let options = DataConversionOptions::strict().with_numeric_options(
                NumericConversionOptions::strict().with_limits(
                    NumericConversionLimits::default()
                        .with_max_text_bytes(digits)
                        .with_max_big_integer_digits(digits),
                ),
            );
            DataConverter::from(source.as_str())
                .to_with::<num_bigint::BigInt>(&options)
                .expect("integer benchmark fixture should parse");
            group.bench_with_input(
                BenchmarkId::from_parameter(name),
                &source,
                |b, source| {
                    b.iter(|| {
                        black_box(
                            DataConverter::from(black_box(source.as_str()))
                                .to_with::<num_bigint::BigInt>(black_box(
                                    &options,
                                )),
                        )
                    });
                },
            );
        }
        group.finish();
    }
    #[cfg(not(feature = "big-integer"))]
    let _ = c;
}

/// Benchmarks text-to-BigDecimal parsing across representative input sizes.
fn benchmark_big_decimal_text(c: &mut Criterion) {
    #[cfg(feature = "big-decimal")]
    {
        let mut group = c.benchmark_group("numeric_text_to_big_decimal");
        for (name, digits) in BIG_NUMBER_TEXT_SIZES {
            let source = "9".repeat(digits);
            let options = DataConversionOptions::strict().with_numeric_options(
                NumericConversionOptions::strict().with_limits(
                    NumericConversionLimits::default()
                        .with_max_text_bytes(digits)
                        .with_max_big_integer_digits(digits),
                ),
            );
            DataConverter::from(source.as_str())
                .to_with::<bigdecimal::BigDecimal>(&options)
                .expect("decimal benchmark fixture should parse");
            group.bench_with_input(
                BenchmarkId::from_parameter(name),
                &source,
                |b, source| {
                    b.iter(|| {
                        black_box(
                            DataConverter::from(black_box(source.as_str()))
                                .to_with::<bigdecimal::BigDecimal>(black_box(
                                &options,
                            )),
                        )
                    });
                },
            );
        }
        group.finish();
    }
    #[cfg(not(feature = "big-decimal"))]
    let _ = c;
}

criterion_group!(
    benches,
    benchmark_exact_integer_text,
    benchmark_lossy_integer_text,
    benchmark_exact_float_text,
    benchmark_big_integer_text,
    benchmark_big_decimal_text,
);
criterion_main!(benches);
