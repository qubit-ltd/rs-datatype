// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Benchmarks single-pass String rendering under a cumulative output budget.
//!
//! `iter_batched` keeps borrowed fixture lookup and owned fixture cloning in
//! setup, outside the measured routine. Session construction and conversion
//! are measured on both sides; Criterion drops each returned conversion
//! result after the routine, outside the measured interval.

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

/// Benchmarks borrowed and owned ASCII identity conversion at representative
/// budgets.
fn benchmark_scalar_string_output(c: &mut Criterion) {
    let mut group = c.benchmark_group("string_output_scalar");
    for size in TEXT_SIZES {
        let source = "7".repeat(size);
        let policy = ConversionPolicy::strict();
        let limits = ConversionLimits::builder()
            .operation_limits(
                ConversionOperationLimits::builder()
                    .max_output_bytes(u64::try_from(source.len()).unwrap())
                    .build(),
            )
            .build();
        let (borrowed, borrowed_used) = {
            let mut session = ConversionSession::new(&policy, &limits);
            let output = DataConverter::from(source.as_str()).to_in::<String>(&mut session);
            (output, session.output_bytes_used())
        };
        let borrowed = borrowed.expect("borrowed ASCII identity benchmark fixture should convert");
        assert_eq!(borrowed, source);
        assert_eq!(borrowed_used, u64::try_from(source.len()).unwrap());
        let (owned, owned_used) = {
            let mut session = ConversionSession::new(&policy, &limits);
            let output = DataConverter::from(source.clone()).into_target_in::<String>(&mut session);
            (output, session.output_bytes_used())
        };
        let owned = owned.expect("owned ASCII identity benchmark fixture should convert");
        assert_eq!(owned, source);
        assert_eq!(owned_used, u64::try_from(source.len()).unwrap());
        group.bench_with_input(BenchmarkId::new("borrowed_identity", size), &source, |bench, source| {
            bench.iter_batched(
                || source.as_str(),
                |source| {
                    let mut session = ConversionSession::new(&policy, &limits);
                    black_box(DataConverter::from(black_box(source)).to_in::<String>(&mut session))
                },
                BatchSize::LargeInput,
            );
        });
        group.bench_with_input(BenchmarkId::new("owned_identity", size), &source, |bench, source| {
            bench.iter_batched(
                || source.clone(),
                |source| {
                    let mut session = ConversionSession::new(&policy, &limits);
                    black_box(DataConverter::from(black_box(source)).into_target_in::<String>(&mut session))
                },
                BatchSize::LargeInput,
            );
        });
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
        .operation_limits(ConversionOperationLimits::builder().max_output_bytes(32).build())
        .build();
    let expected = r#"{"a":"1","b":"2","c":"3"}"#;
    let borrowed = {
        let mut session = ConversionSession::new(&policy, &limits);
        DataConverter::from(&map).to_in::<String>(&mut session)
    }
    .expect("borrowed string map output benchmark fixture should convert");
    assert_eq!(borrowed, expected);
    let owned = {
        let mut session = ConversionSession::new(&policy, &limits);
        DataConverter::from(map.clone()).into_target_in::<String>(&mut session)
    }
    .expect("owned string map output benchmark fixture should convert");
    assert_eq!(owned, expected);
    group.bench_function("borrowed", |bench| {
        bench.iter_batched(
            || &map,
            |map| {
                let mut session = ConversionSession::new(&policy, &limits);
                black_box(DataConverter::from(black_box(map)).to_in::<String>(&mut session))
            },
            BatchSize::SmallInput,
        );
    });
    group.bench_function("owned", |bench| {
        bench.iter_batched(
            || map.clone(),
            |map| {
                let mut session = ConversionSession::new(&policy, &limits);
                black_box(DataConverter::from(black_box(map)).into_target_in::<String>(&mut session))
            },
            BatchSize::SmallInput,
        );
    });
    group.finish();
}

/// Benchmarks integer and floating-point formatting through DataConverter.
fn benchmark_numeric_string_output(c: &mut Criterion) {
    let mut group = c.benchmark_group("string_output_numeric");
    let policy = ConversionPolicy::strict();

    let number = -1_234_567_890_123_456_789_i128;
    let expected = number.to_string();
    let limits = ConversionLimits::builder()
        .operation_limits(
            ConversionOperationLimits::builder()
                .max_output_bytes(u64::try_from(expected.len()).unwrap())
                .build(),
        )
        .build();
    let mut session = ConversionSession::new(&policy, &limits);
    let output = DataConverter::from(number)
        .to_in::<String>(&mut session)
        .expect("i128 string output benchmark fixture should convert");
    assert_eq!(output, expected);
    assert_eq!(session.output_bytes_used(), u64::try_from(expected.len()).unwrap());
    group.bench_function("i128", |bench| {
        bench.iter_batched(
            || number,
            |number| {
                let mut session = ConversionSession::new(&policy, &limits);
                black_box(DataConverter::from(black_box(number)).to_in::<String>(&mut session))
            },
            BatchSize::SmallInput,
        );
    });

    let number = 12345.5_f64;
    let expected = number.to_string();
    let limits = ConversionLimits::builder()
        .operation_limits(
            ConversionOperationLimits::builder()
                .max_output_bytes(u64::try_from(expected.len()).unwrap())
                .build(),
        )
        .build();
    let mut session = ConversionSession::new(&policy, &limits);
    let output = DataConverter::from(number)
        .to_in::<String>(&mut session)
        .expect("f64 string output benchmark fixture should convert");
    assert_eq!(output, expected);
    assert_eq!(session.output_bytes_used(), u64::try_from(expected.len()).unwrap());
    group.bench_function("f64", |bench| {
        bench.iter_batched(
            || number,
            |number| {
                let mut session = ConversionSession::new(&policy, &limits);
                black_box(DataConverter::from(black_box(number)).to_in::<String>(&mut session))
            },
            BatchSize::SmallInput,
        );
    });
    group.finish();
}

/// Benchmarks borrowed and owned identity conversion for multibyte UTF-8 text.
fn benchmark_utf8_string_output(c: &mut Criterion) {
    let mut group = c.benchmark_group("string_output_utf8");
    let policy = ConversionPolicy::strict();
    for size in TEXT_SIZES {
        let source = "é汉🙂".repeat(size / "é汉🙂".len());
        let bytes = source.len();
        let output_budget = u64::try_from(bytes).unwrap();
        let limits = ConversionLimits::builder()
            .operation_limits(
                ConversionOperationLimits::builder()
                    .max_output_bytes(output_budget)
                    .build(),
            )
            .build();

        let mut session = ConversionSession::new(&policy, &limits);
        let output = DataConverter::from(source.as_str())
            .to_in::<String>(&mut session)
            .expect("borrowed UTF-8 identity benchmark fixture should convert");
        assert_eq!(output, source);
        assert_eq!(session.output_bytes_used(), output_budget);

        let mut session = ConversionSession::new(&policy, &limits);
        let output = DataConverter::from(source.clone())
            .into_target_in::<String>(&mut session)
            .expect("owned UTF-8 identity benchmark fixture should convert");
        assert_eq!(output, source);
        assert_eq!(session.output_bytes_used(), output_budget);

        group.bench_with_input(
            BenchmarkId::new("borrowed_identity", bytes),
            &source,
            |bench, source| {
                bench.iter_batched(
                    || source.as_str(),
                    |source| {
                        let mut session = ConversionSession::new(&policy, &limits);
                        black_box(DataConverter::from(black_box(source)).to_in::<String>(&mut session))
                    },
                    BatchSize::LargeInput,
                );
            },
        );
        group.bench_with_input(BenchmarkId::new("owned_identity", bytes), &source, |bench, source| {
            bench.iter_batched(
                || source.clone(),
                |source| {
                    let mut session = ConversionSession::new(&policy, &limits);
                    black_box(DataConverter::from(black_box(source)).into_target_in::<String>(&mut session))
                },
                BatchSize::LargeInput,
            );
        });
    }
    group.finish();
}

criterion_group!(
    string_output_benches,
    benchmark_scalar_string_output,
    benchmark_string_map_output,
    benchmark_numeric_string_output,
    benchmark_utf8_string_output,
);
criterion_main!(string_output_benches);
