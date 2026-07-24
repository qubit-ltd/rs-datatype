// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Fuzzes bounded UTF-8 text conversion across supported target families.
//!
//! Every input is exercised under the strict, lossy, and environment-friendly
//! profiles. Successful values and structured conversion errors are both valid;
//! the invariant is that no target conversion panics for arbitrary bounded
//! text.

#![no_main]

use std::collections::HashMap;
use std::time::Duration;

use bigdecimal::BigDecimal;
use chrono::{
    DateTime,
    NaiveDate,
    NaiveDateTime,
    NaiveTime,
    Utc,
};
use libfuzzer_sys::fuzz_target;
use num_bigint::BigInt;
use qubit_datatype::{
    DataConversionOptions,
    DataConversionTarget,
    DataConverter,
};
use serde_json::Value;
use url::Url;

/// Caps parser work and derived allocations while retaining long-limit cases.
const MAX_INPUT_SIZE: usize = 16 * 1024;

fuzz_target!(|data: &[u8]| {
    if data.len() > MAX_INPUT_SIZE {
        return;
    }
    let Ok(text) = std::str::from_utf8(data) else {
        return;
    };
    let converter = DataConverter::from(text);
    let options = [
        DataConversionOptions::strict(),
        DataConversionOptions::lossy(),
        DataConversionOptions::env_friendly(),
    ];

    exercise_conversion::<bool>(&converter, &options);
    exercise_conversion::<i128>(&converter, &options);
    exercise_conversion::<u128>(&converter, &options);
    exercise_conversion::<f32>(&converter, &options);
    exercise_conversion::<f64>(&converter, &options);
    exercise_conversion::<BigInt>(&converter, &options);
    exercise_conversion::<BigDecimal>(&converter, &options);
    exercise_conversion::<NaiveDate>(&converter, &options);
    exercise_conversion::<NaiveTime>(&converter, &options);
    exercise_conversion::<NaiveDateTime>(&converter, &options);
    exercise_conversion::<DateTime<Utc>>(&converter, &options);
    exercise_conversion::<Duration>(&converter, &options);
    exercise_conversion::<Url>(&converter, &options);
    exercise_conversion::<Value>(&converter, &options);
    exercise_conversion::<HashMap<String, String>>(&converter, &options);
});

/// Exercises one target type under every supplied conversion profile.
///
/// Conversion errors are accepted as normal fuzz outcomes and discarded.
///
/// # Parameters
///
/// * `converter` - Valid UTF-8 source to convert.
/// * `options` - Bounded set of conversion profiles to exercise.
fn exercise_conversion<T>(
    converter: &DataConverter<'_>,
    options: &[DataConversionOptions],
) where
    T: DataConversionTarget,
{
    for option in options {
        let _ = converter.to_with::<T>(option);
    }
}
