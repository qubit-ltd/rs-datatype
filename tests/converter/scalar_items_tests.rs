// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests for lazy scalar collection item iteration.

use qubit_datatype::converter::{
    CollectionConversionOptions,
    EmptyItemPolicy,
    ScalarItems,
};

/// Test lazy splitting, trimming, skipping, and original source indices.
#[test]
fn test_scalar_items_lazily_retains_original_indices() {
    let options = CollectionConversionOptions::default()
        .with_split_scalar_strings(true)
        .with_delimiters([',', '、'])
        .with_trim_items(true)
        .with_empty_item_policy(EmptyItemPolicy::Skip);
    let mut items: ScalarItems<'_> = options.scalar_items(" alpha,  、beta ");

    let first = items
        .next()
        .expect("first item should exist")
        .expect("first item should be valid");
    assert_eq!((first.source_index, first.value), (0, "alpha"));

    let second = items
        .next()
        .expect("second retained item should exist")
        .expect("second retained item should be valid");
    assert_eq!((second.source_index, second.value), (2, "beta"));
    assert!(items.next().is_none());
}

/// Test rejection is discovered lazily and iteration can continue afterward.
#[test]
fn test_scalar_items_reports_rejection_when_reached() {
    let options = CollectionConversionOptions::default()
        .with_split_scalar_strings(true)
        .with_trim_items(true)
        .with_empty_item_policy(EmptyItemPolicy::Reject);
    let mut items = options.scalar_items("first, ,third");

    assert_eq!(
        items
            .next()
            .expect("first item should exist")
            .expect("first item should be valid")
            .value,
        "first",
    );
    assert_eq!(
        items
            .next()
            .expect("blank item should be visited")
            .expect_err("blank item should be rejected")
            .source_index(),
        1,
    );
    assert_eq!(
        items
            .next()
            .expect("iteration should continue")
            .expect("third item should be valid")
            .value,
        "third",
    );
    assert!(items.next().is_none());
}
