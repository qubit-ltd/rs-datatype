// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Scalar and materialized collection conversion benchmarks.

use std::hint::black_box;

use criterion::{
    BenchmarkId,
    Criterion,
    criterion_group,
    criterion_main,
};
use qubit_datatype::{
    CollectionConversionOptions,
    DataConversionOptions,
    DataConverters,
    ScalarStringDataConverters,
};

const ITEM_COUNTS: [usize; 4] = [1, 16, 256, 4096];

/// Builds a comma-separated numeric input containing `item_count` values.
///
/// # Parameters
///
/// * `item_count` - Number of scalar values to include.
///
/// # Returns
///
/// A comma-separated string whose values all convert to `u64`.
fn comma_separated_input(item_count: usize) -> String {
    (0..item_count)
        .map(|index| (index % 10).to_string())
        .collect::<Vec<_>>()
        .join(",")
}

/// Benchmarks conversion of the first item from scalar collection text.
fn benchmark_scalar_first(c: &mut Criterion) {
    let options = DataConversionOptions::env_friendly();
    let mut group = c.benchmark_group("scalar_collection_to_first_u64");

    for item_count in ITEM_COUNTS {
        let input = comma_separated_input(item_count);
        group.bench_with_input(
            BenchmarkId::from_parameter(item_count),
            input.as_str(),
            |b, input| {
                b.iter(|| {
                    black_box(
                        ScalarStringDataConverters::from(black_box(input))
                            .to_first_with::<u64>(black_box(&options)),
                    )
                });
            },
        );
    }
    group.finish();
}

/// Benchmarks complete conversion of scalar collection text.
fn benchmark_scalar_complete(c: &mut Criterion) {
    let options = DataConversionOptions::env_friendly();
    let mut group = c.benchmark_group("scalar_collection_to_vec_u64");

    for item_count in ITEM_COUNTS {
        let input = comma_separated_input(item_count);
        group.bench_with_input(
            BenchmarkId::from_parameter(item_count),
            input.as_str(),
            |b, input| {
                b.iter(|| {
                    black_box(
                        ScalarStringDataConverters::from(black_box(input))
                            .to_vec_with::<u64>(black_box(&options)),
                    )
                });
            },
        );
    }
    group.finish();
}

/// Benchmarks conversion of already-materialized string slices.
fn benchmark_materialized_slice(c: &mut Criterion) {
    let options = DataConversionOptions::env_friendly();
    let mut group = c.benchmark_group("materialized_slice_to_vec_u64");

    for item_count in ITEM_COUNTS {
        let input = comma_separated_input(item_count);
        let values = input.split(',').map(str::to_owned).collect::<Vec<_>>();
        group.bench_with_input(
            BenchmarkId::from_parameter(item_count),
            values.as_slice(),
            |b, values| {
                b.iter(|| {
                    black_box(
                        DataConverters::from(black_box(values))
                            .to_vec_with::<u64>(black_box(&options)),
                    )
                });
            },
        );
    }
    group.finish();
}

/// Benchmarks scanning text with a large configured delimiter set.
fn benchmark_large_delimiter_set(c: &mut Criterion) {
    let input = format!("{},tail", "a".repeat(16 * 1024));
    let delimiters =
        std::iter::once(',').chain((0x100..0x13f).filter_map(char::from_u32));
    let options = CollectionConversionOptions::default()
        .with_split_scalar_strings(true)
        .with_delimiters(delimiters);

    c.bench_function("scalar_collection_large_delimiter_set", |b| {
        b.iter(|| {
            black_box(
                black_box(&options)
                    .scalar_items(black_box(input.as_str()))
                    .count(),
            )
        });
    });
}

criterion_group!(
    benches,
    benchmark_scalar_first,
    benchmark_scalar_complete,
    benchmark_materialized_slice,
    benchmark_large_delimiter_set,
);
criterion_main!(benches);
