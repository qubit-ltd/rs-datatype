// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests for lazy scalar collection item iteration.

use qubit_datatype::converter::{CollectionConversionOptions, EmptyItemPolicy, ScalarItems};

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

/// Test retained-item limits after empty-item filtering.
#[test]
fn test_scalar_items_enforces_retained_item_limit() {
    let options = CollectionConversionOptions::default()
        .with_split_scalar_strings(true)
        .with_trim_items(true)
        .with_empty_item_policy(EmptyItemPolicy::Skip)
        .with_max_items(2);
    let mut items = options.scalar_items("a,  ,b,c,d");

    assert_eq!(items.next().expect("first item").expect("valid").value, "a");
    assert_eq!(
        items.next().expect("second item").expect("valid").value,
        "b"
    );
    let error = items
        .next()
        .expect("limit error")
        .expect_err("third retained item must exceed the limit");
    assert_eq!(error.source_index(), 3);
    assert_eq!(error.maximum_items(), Some(2));
    assert!(items.next().is_none());
}

/// Test rejected empty items take precedence and do not consume quota.
#[test]
fn test_scalar_items_rejection_precedes_item_limit() {
    let options = CollectionConversionOptions::default()
        .with_split_scalar_strings(true)
        .with_empty_item_policy(EmptyItemPolicy::Reject)
        .with_max_items(1);
    let mut items = options.scalar_items("a,,b");

    assert_eq!(items.next().expect("first item").expect("valid").value, "a");
    let blank = items
        .next()
        .expect("blank rejection")
        .expect_err("blank item must be rejected before limit checking");
    assert_eq!(blank.source_index(), 1);
    assert_eq!(blank.maximum_items(), None);
    let limit = items
        .next()
        .expect("limit error")
        .expect_err("next retained item must exceed the limit");
    assert_eq!(limit.source_index(), 2);
    assert_eq!(limit.maximum_items(), Some(1));
    assert!(items.next().is_none());
}

/// Test zero permits only an empty retained result.
#[test]
fn test_scalar_items_zero_limit_allows_only_empty_result() {
    let retained = CollectionConversionOptions::default()
        .with_split_scalar_strings(true)
        .with_max_items(0);
    let error = retained
        .scalar_items("a")
        .next()
        .expect("limit error")
        .expect_err("zero limit must reject the first retained item");
    assert_eq!(error.source_index(), 0);
    assert_eq!(error.maximum_items(), Some(0));

    let skipped = CollectionConversionOptions::default()
        .with_split_scalar_strings(true)
        .with_empty_item_policy(EmptyItemPolicy::Skip)
        .with_max_items(0);
    assert!(skipped.scalar_items(",,").next().is_none());
}

/// Test a large delimiter set preserves Unicode splitting semantics.
#[test]
fn test_scalar_items_supports_large_delimiter_sets() {
    let delimiters = std::iter::once(',').chain((0x100..0x140).filter_map(char::from_u32));
    let options = CollectionConversionOptions::default()
        .with_split_scalar_strings(true)
        .with_delimiters(delimiters);
    let values = options
        .scalar_items("alpha,bravoĀcharlie")
        .map(|item| item.expect("all split items should be valid").value)
        .collect::<Vec<_>>();

    assert_eq!(values, ["alpha", "bravo", "charlie"]);
}
