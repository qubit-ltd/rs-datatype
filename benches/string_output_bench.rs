// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Benchmarks single-pass String rendering under a cumulative output budget.

use std::collections::HashMap;
use std::hint::black_box;

use criterion::BatchSize;
use criterion::BenchmarkId;
use criterion::Criterion;
use criterion::criterion_group;
use criterion::criterion_main;
use qubit_datatype::ConversionLimits;
use qubit_datatype::ConversionOperationLimits;
use qubit_datatype::ConversionPolicy;
use qubit_datatype::ConversionSession;
use qubit_datatype::DataConverter;

const TEXT_SIZES: [usize; 3] = [64, 4 * 1024, 256 * 1024];

/// Benchmarks integer and Unicode text rendering at representative budgets.
fn benchmark_scalar_string_output(c: &mut Criterion) {
    let mut group = c.benchmark_group("string_output_scalar");
    for size in TEXT_SIZES {
        let source = "7".repeat(size);
        let policy = ConversionPolicy::strict();
        let limits = ConversionLimits::builder()
            .operation_limits(
                ConversionOperationLimits::builder()
                    .max_output_bytes(u64::try_from(source.len()).unwrap()),
            )
            .build();
        group.bench_with_input(
            BenchmarkId::new("borrowed_identity", size),
            &source,
            |bench, source| {
                bench.iter(|| {
                    let mut session = ConversionSession::new(&policy, &limits);
                    black_box(
                        DataConverter::from(black_box(source.as_str()))
                            .to_in::<String>(&mut session),
                    )
                });
            },
        );
        group.bench_with_input(
            BenchmarkId::new("owned_identity", size),
            &source,
            |bench, source| {
                bench.iter_batched(
                    || source.clone(),
                    |source| {
                        let mut session =
                            ConversionSession::new(&policy, &limits);
                        black_box(
                            DataConverter::from(black_box(source))
                                .into_target_in::<String>(&mut session),
                        )
                    },
                    BatchSize::LargeInput,
                );
            },
        );
    }
    group.finish();
}

/// Benchmarks canonical JSON StringMap rendering through the same writer.
fn benchmark_string_map_output(c: &mut Criterion) {
    let mut group = c.benchmark_group("string_output_json_map");
    let map = HashMap::from([
        ("a".to_owned(), "1".to_owned()),
        ("b".to_owned(), "2".to_owned()),
        ("c".to_owned(), "3".to_owned()),
    ]);
    let policy = ConversionPolicy::strict();
    let limits = ConversionLimits::builder()
        .operation_limits(
            ConversionOperationLimits::builder().max_output_bytes(32),
        )
        .build();
    group.bench_function("borrowed", |bench| {
        bench.iter(|| {
            let mut session = ConversionSession::new(&policy, &limits);
            black_box(
                DataConverter::from(black_box(&map))
                    .to_in::<String>(&mut session),
            )
        });
    });
    group.bench_function("owned", |bench| {
        bench.iter_batched(
            || map.clone(),
            |map| {
                let mut session = ConversionSession::new(&policy, &limits);
                black_box(
                    DataConverter::from(black_box(map))
                        .into_target_in::<String>(&mut session),
                )
            },
            BatchSize::SmallInput,
        );
    });
    group.finish();
}

criterion_group!(
    string_output_benches,
    benchmark_scalar_string_output,
    benchmark_string_map_output,
);
criterion_main!(string_output_benches);
