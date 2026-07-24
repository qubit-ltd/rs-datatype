// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Fuzzes bounded scalar-string splitting and collection conversion.
//!
//! Successful vector conversion must respect the configured retained-item
//! limit, and first-item conversion must agree with the vector's first value.
//! Policy failures are accepted outcomes and must not panic.

#![no_main]

use libfuzzer_sys::fuzz_target;
use qubit_datatype::{
    CollectionConversionOptions,
    DataConversionOptions,
    EmptyItemPolicy,
    ScalarStringDataConverters,
};

/// Caps text scanning and item materialization for each fuzz iteration.
const MAX_INPUT_SIZE: usize = 16 * 1024;
/// Bounds delimiter-set construction while covering the large-set lookup path.
const MAX_DELIMITERS: usize = 64;

fuzz_target!(|data: &[u8]| {
    if data.len() > MAX_INPUT_SIZE {
        return;
    }
    let [policy_control, delimiter_control, payload @ ..] = data else {
        return;
    };
    let max_items = usize::from(*policy_control >> 3);
    let delimiter_count =
        usize::from(*delimiter_control) % (MAX_DELIMITERS + 1);
    let delimiter_count = delimiter_count.min(payload.len());
    let (delimiter_bytes, text_bytes) = payload.split_at(delimiter_count);
    let delimiters = delimiter_bytes.iter().copied().map(char::from);
    let Ok(text) = std::str::from_utf8(text_bytes) else {
        return;
    };
    let empty_item_policy = match policy_control % 3 {
        0 => EmptyItemPolicy::Keep,
        1 => EmptyItemPolicy::Skip,
        _ => EmptyItemPolicy::Reject,
    };
    let collection = CollectionConversionOptions::default()
        .with_split_scalar_strings(true)
        .with_delimiters(delimiters)
        .with_trim_items(policy_control & 0b100 != 0)
        .with_empty_item_policy(empty_item_policy)
        .with_max_items(max_items);

    for item in collection.scalar_items(text) {
        let _ = item;
    }

    let options = DataConversionOptions::env_friendly()
        .with_collection_options(collection);
    let converter = ScalarStringDataConverters::from(text);
    let first = converter.to_first_with::<String>(&options);
    let values = converter.to_vec_with::<String>(&options);
    if let Ok(values) = values {
        assert!(
            values.len() <= max_items,
            "successful collection conversion exceeded its item limit"
        );
        match values.first() {
            Some(expected) => {
                assert_eq!(
                    first.as_ref().ok(),
                    Some(expected),
                    "successful first-item conversion disagreed with the vector"
                );
            }
            None => {
                assert!(
                    first.is_err(),
                    "empty vector conversion must not produce a first item"
                );
            }
        }
    }
});
