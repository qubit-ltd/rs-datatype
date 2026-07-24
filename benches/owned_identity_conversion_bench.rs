// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Owned and borrowed identity conversion benchmarks.

use std::collections::HashMap;
use std::hint::black_box;

#[cfg(feature = "big-decimal")]
use bigdecimal::BigDecimal;
use criterion::{
    BatchSize,
    BenchmarkId,
    Criterion,
    Throughput,
    criterion_group,
    criterion_main,
};
#[cfg(feature = "big-integer")]
use num_bigint::BigInt;
use qubit_datatype::{
    DataConversionError,
    DataConversionTarget,
    DataConverter,
    DataConverters,
};
#[cfg(feature = "json")]
use serde_json::Value as JsonValue;
#[cfg(feature = "url")]
use url::Url;

const TEXT_SIZES: [(&str, usize); 3] =
    [("64B", 64), ("4KiB", 4 * 1024), ("256KiB", 256 * 1024)];
#[cfg(any(feature = "big-integer", feature = "big-decimal"))]
const BIG_NUMBER_DIGITS: [(&str, usize); 3] = [
    ("32_digits", 32),
    ("1024_digits", 1024),
    ("16384_digits", 16 * 1024),
];
const BATCH_ITEM_COUNTS: [usize; 3] = [16, 256, 4096];
const BATCH_ITEM_BYTES: usize = 1024;

/// Converts an owned source through the consuming conversion API.
///
/// # Parameters
///
/// * `value` - Owned identity source.
///
/// # Returns
///
/// The conversion result containing a value of the same type.
fn convert_owned<T>(value: T) -> Result<T, DataConversionError>
where
    T: DataConversionTarget,
    DataConverter<'static>: From<T>,
{
    DataConverter::from(value).into_target::<T>()
}

/// Converts a borrowed source through the current conversion API.
///
/// # Parameters
///
/// * `value` - Borrowed identity source.
///
/// # Returns
///
/// The conversion result containing an owned value of the same type.
fn convert_borrowed<T>(value: &T) -> Result<T, DataConversionError>
where
    T: DataConversionTarget,
    for<'a> DataConverter<'a>: From<&'a T>,
{
    DataConverter::from(value).to::<T>()
}

/// Registers borrowed, owned, and direct-move measurements for one value.
///
/// # Parameters
///
/// * `group` - Criterion benchmark group.
/// * `case_name` - Payload-size label.
/// * `payload_bytes` - Logical payload size used for throughput reporting.
/// * `value` - Representative value cloned by untimed setup closures.
fn benchmark_identity_case<T>(
    group: &mut criterion::BenchmarkGroup<'_, criterion::measurement::WallTime>,
    case_name: &str,
    payload_bytes: usize,
    value: T,
) where
    T: Clone + DataConversionTarget + 'static,
    DataConverter<'static>: From<T>,
    for<'a> DataConverter<'a>: From<&'a T>,
{
    convert_borrowed(&value)
        .expect("borrowed identity benchmark fixture should convert");
    convert_owned(value.clone())
        .expect("owned identity benchmark fixture should convert");

    group.throughput(Throughput::Bytes(payload_bytes as u64));
    group.bench_with_input(
        BenchmarkId::new("borrowed_to_target", case_name),
        &value,
        |b, value| {
            b.iter(|| black_box(convert_borrowed::<T>(black_box(value))))
        },
    );
    group.bench_function(BenchmarkId::new("owned_to_target", case_name), |b| {
        b.iter_batched(
            || value.clone(),
            |value| black_box(convert_owned::<T>(black_box(value))),
            BatchSize::LargeInput,
        );
    });
    group.bench_function(BenchmarkId::new("direct_move", case_name), |b| {
        b.iter_batched(
            || value.clone(),
            |value| black_box(value),
            BatchSize::LargeInput,
        );
    });
}

/// Builds a string map with a predictable logical payload size.
///
/// # Parameters
///
/// * `payload_bytes` - Approximate total bytes stored across values.
///
/// # Returns
///
/// A map containing at least one string entry.
fn string_map_payload(payload_bytes: usize) -> HashMap<String, String> {
    const ENTRY_COUNT: usize = 16;
    let value_bytes = payload_bytes.div_ceil(ENTRY_COUNT);
    (0..ENTRY_COUNT)
        .map(|index| (format!("key-{index}"), "x".repeat(value_bytes)))
        .collect()
}

/// Benchmarks String identity conversion across payload sizes.
fn benchmark_string_identity(c: &mut Criterion) {
    let mut group = c.benchmark_group("identity_string");
    for (name, bytes) in TEXT_SIZES {
        benchmark_identity_case(&mut group, name, bytes, "x".repeat(bytes));
    }
    group.finish();
}

/// Benchmarks StringMap identity conversion across payload sizes.
fn benchmark_string_map_identity(c: &mut Criterion) {
    let mut group = c.benchmark_group("identity_string_map");
    for (name, bytes) in TEXT_SIZES {
        benchmark_identity_case(
            &mut group,
            name,
            bytes,
            string_map_payload(bytes),
        );
    }
    group.finish();
}

/// Benchmarks owned and borrowed String batch identity conversion.
fn benchmark_string_batch_identity(c: &mut Criterion) {
    let mut group = c.benchmark_group("identity_string_batch");
    for item_count in BATCH_ITEM_COUNTS {
        let values = vec!["x".repeat(BATCH_ITEM_BYTES); item_count];
        DataConverters::from(values.as_slice())
            .to_vec::<String>()
            .expect("borrowed batch identity benchmark fixture should convert");
        DataConverters::from(values.clone())
            .to_vec::<String>()
            .expect("owned batch identity benchmark fixture should convert");

        group.throughput(Throughput::Elements(item_count as u64));
        group.bench_with_input(
            BenchmarkId::new("borrowed_to_target", item_count),
            values.as_slice(),
            |b, values| {
                b.iter(|| {
                    black_box(
                        DataConverters::from(black_box(values))
                            .to_vec::<String>(),
                    )
                });
            },
        );
        group.bench_function(
            BenchmarkId::new("owned_to_target", item_count),
            |b| {
                b.iter_batched(
                    || values.clone(),
                    |values| {
                        black_box(
                            DataConverters::from(black_box(values))
                                .to_vec::<String>(),
                        )
                    },
                    BatchSize::LargeInput,
                );
            },
        );
        group.bench_function(
            BenchmarkId::new("direct_move", item_count),
            |b| {
                b.iter_batched(
                    || values.clone(),
                    black_box,
                    BatchSize::LargeInput,
                );
            },
        );
    }
    group.finish();
}

/// Benchmarks JSON identity conversion when JSON support is enabled.
#[cfg(feature = "json")]
fn benchmark_json_identity(c: &mut Criterion) {
    let mut group = c.benchmark_group("identity_json");
    for (name, bytes) in TEXT_SIZES {
        benchmark_identity_case(
            &mut group,
            name,
            bytes,
            JsonValue::String("x".repeat(bytes)),
        );
    }
    group.finish();
}

/// Benchmarks URL identity conversion when URL support is enabled.
#[cfg(feature = "url")]
fn benchmark_url_identity(c: &mut Criterion) {
    let mut group = c.benchmark_group("identity_url");
    for (name, bytes) in TEXT_SIZES {
        let source = format!("https://example.com/{}", "x".repeat(bytes));
        let value = Url::parse(&source).expect("benchmark URL should parse");
        benchmark_identity_case(&mut group, name, source.len(), value);
    }
    group.finish();
}

/// Benchmarks BigInt identity conversion when big-integer support is enabled.
#[cfg(feature = "big-integer")]
fn benchmark_big_integer_identity(c: &mut Criterion) {
    let mut group = c.benchmark_group("identity_big_integer");
    for (name, digits) in BIG_NUMBER_DIGITS {
        let value = BigInt::parse_bytes("9".repeat(digits).as_bytes(), 10)
            .expect("benchmark BigInt should parse");
        benchmark_identity_case(&mut group, name, digits, value);
    }
    group.finish();
}

/// Benchmarks BigDecimal identity conversion when big-decimal support is
/// enabled.
#[cfg(feature = "big-decimal")]
fn benchmark_big_decimal_identity(c: &mut Criterion) {
    let mut group = c.benchmark_group("identity_big_decimal");
    for (name, digits) in BIG_NUMBER_DIGITS {
        let source = format!("{}.{}", "9".repeat(digits), "5".repeat(digits));
        let value = source
            .parse::<BigDecimal>()
            .expect("benchmark BigDecimal should parse");
        benchmark_identity_case(&mut group, name, source.len(), value);
    }
    group.finish();
}

/// Runs every rich-type identity benchmark enabled by Cargo features.
fn benchmark_rich_identity(c: &mut Criterion) {
    #[cfg(feature = "json")]
    benchmark_json_identity(c);
    #[cfg(feature = "url")]
    benchmark_url_identity(c);
    #[cfg(feature = "big-integer")]
    benchmark_big_integer_identity(c);
    #[cfg(feature = "big-decimal")]
    benchmark_big_decimal_identity(c);
    #[cfg(not(any(
        feature = "json",
        feature = "url",
        feature = "big-integer",
        feature = "big-decimal",
    )))]
    let _ = c;
}

criterion_group!(
    benches,
    benchmark_string_identity,
    benchmark_string_map_identity,
    benchmark_string_batch_identity,
    benchmark_rich_identity,
);
criterion_main!(benches);
