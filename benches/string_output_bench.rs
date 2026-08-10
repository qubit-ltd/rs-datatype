// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0 (the "License");
//    you may not use this file except in compliance with the License.
//    You may obtain a copy of the License at
//
//        http://www.apache.org/licenses/LICENSE-2.0
//
//    Unless required by applicable law or agreed to in writing, software
//    distributed under the License is distributed on an "AS IS" BASIS,
//    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//    See the License for the specific language governing permissions and
//    limitations under the License.
// =============================================================================
//! Benchmarks single-pass String rendering under a cumulative output budget.

use std::collections::HashMap;
use std::hint::black_box;

use criterion::BenchmarkId;
use criterion::Criterion;
use criterion::criterion_group;
use criterion::criterion_main;
use qubit_datatype::ConversionBudgetLimits;
use qubit_datatype::DataConversionOptions;
use qubit_datatype::DataConverter;

const TEXT_SIZES: [usize; 3] = [64, 4 * 1024, 256 * 1024];

/// Benchmarks integer and Unicode text rendering at representative budgets.
fn benchmark_scalar_string_output(c: &mut Criterion) {
    let mut group = c.benchmark_group("string_output_scalar");
    for size in TEXT_SIZES {
        let source = "7".repeat(size);
        let options = DataConversionOptions::strict().with_budget_limits(
            ConversionBudgetLimits::default()
                .with_max_output_bytes(source.len()),
        );
        group.bench_with_input(
            BenchmarkId::new("borrowed_identity", size),
            &source,
            |bench, source| {
                bench.iter(|| {
                    let mut session = options.session();
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
                bench.iter(|| {
                    let mut session = options.session();
                    black_box(
                        DataConverter::from(black_box(source.clone()))
                            .into_target_in::<String>(&mut session),
                    )
                });
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
    let options = DataConversionOptions::strict().with_budget_limits(
        ConversionBudgetLimits::default().with_max_output_bytes(32),
    );
    group.bench_function("borrowed", |bench| {
        bench.iter(|| {
            let mut session = options.session();
            black_box(
                DataConverter::from(black_box(&map))
                    .to_in::<String>(&mut session),
            )
        });
    });
    group.bench_function("owned", |bench| {
        bench.iter(|| {
            let mut session = options.session();
            black_box(
                DataConverter::from(black_box(map.clone()))
                    .into_target_in::<String>(&mut session),
            )
        });
    });
    group.finish();
}

criterion_group!(
    string_output_benches,
    benchmark_scalar_string_output,
    benchmark_string_map_output,
);
criterion_main!(string_output_benches);
