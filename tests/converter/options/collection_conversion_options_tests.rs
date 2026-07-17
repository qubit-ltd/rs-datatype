// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # CollectionConversionOptions Unit Tests
//!
//! Tests for scalar string collection conversion options.

use qubit_datatype::converter::{
    CollectionConversionOptions,
    EmptyItemPolicy,
    ScalarItem,
    ScalarItems,
};

/// Test the environment-variable collection profile.
#[test]
fn test_collection_conversion_options_env_friendly_profile() {
    let options = CollectionConversionOptions::env_friendly();
    assert!(options.split_scalar_strings);
    assert_eq!(options.delimiters, vec![',']);
    assert!(options.trim_items);
    assert_eq!(options.empty_item_policy, EmptyItemPolicy::Skip);
}

/// Test that misspelled collection option fields are rejected.
#[test]
fn test_collection_conversion_options_reject_unknown_fields() {
    let error = serde_json::from_str::<CollectionConversionOptions>(
        r#"{"split_scalar_strings":true,"unexpected":false}"#,
    )
    .expect_err("unknown collection option fields must be rejected");

    assert!(error.to_string().contains("unknown field `unexpected`"));
}

/// Test collection option split and empty-item branches.
#[test]
fn test_collection_conversion_options_cover_policy_branches() {
    let scalar_options = CollectionConversionOptions::default();
    let scalar_items = scalar_options
        .scalar_items(" a,b ")
        .collect::<Result<Vec<_>, _>>()
        .expect("non-split scalar should remain one item");
    assert_eq!(scalar_items.len(), 1);
    assert_eq!(scalar_items[0].source_index, 0);
    assert_eq!(scalar_items[0].value, " a,b ");

    let kept_options = CollectionConversionOptions::default()
        .with_split_scalar_strings(true)
        .with_delimiters([',', ';']);
    let kept_items = kept_options
        .scalar_items("a,,;b")
        .collect::<Result<Vec<_>, _>>()
        .expect("empty items should be kept by default");
    assert_eq!(
        kept_items
            .iter()
            .map(|item| (item.source_index, item.value))
            .collect::<Vec<_>>(),
        vec![(0, "a"), (1, ""), (2, ""), (3, "b")],
    );

    let skipped_options = CollectionConversionOptions::default()
        .with_split_scalar_strings(true)
        .with_delimiters([','])
        .with_trim_items(true)
        .with_empty_item_policy(EmptyItemPolicy::Skip);
    let skipped_items = skipped_options
        .scalar_items("a,  ,b")
        .collect::<Result<Vec<_>, _>>()
        .expect("blank collection item should be skipped");
    assert_eq!(
        skipped_items
            .iter()
            .map(|item| (item.source_index, item.value))
            .collect::<Vec<_>>(),
        vec![(0, "a"), (2, "b")],
    );

    let rejected_options = CollectionConversionOptions::default()
        .with_split_scalar_strings(true)
        .with_delimiters([','])
        .with_trim_items(true)
        .with_empty_item_policy(EmptyItemPolicy::Reject);
    let mut rejected_items: ScalarItems<'_> =
        rejected_options.scalar_items("a,  ,b");
    let first: ScalarItem<'_> = rejected_items
        .next()
        .expect("first item should exist")
        .expect("first item should be valid");
    assert_eq!((first.source_index, first.value), (0, "a"));
    let rejected_item = rejected_items
        .next()
        .expect("rejected item should exist")
        .expect_err("blank collection item should be rejected");
    assert_eq!(rejected_item.source_index(), 1);
    assert_eq!(
        rejected_item.to_string(),
        "blank scalar item rejected at source index 1",
    );
    let third = rejected_items
        .next()
        .expect("iteration should continue after the rejected item")
        .expect("third item should be valid");
    assert_eq!((third.source_index, third.value), (2, "b"));
    assert!(rejected_items.next().is_none());
}

/// Test empty scalar input and Unicode delimiters without allocating values.
#[test]
fn test_collection_conversion_options_scalar_items_handles_boundaries() {
    let empty_options = CollectionConversionOptions::default();
    let empty = empty_options
        .scalar_items("")
        .collect::<Result<Vec<_>, _>>()
        .expect("empty unsplit scalar should be retained");
    assert_eq!(empty.len(), 1);
    assert_eq!((empty[0].source_index, empty[0].value), (0, ""));

    let unicode_options = CollectionConversionOptions::default()
        .with_split_scalar_strings(true)
        .with_delimiters(['、', '，']);
    let unicode = unicode_options
        .scalar_items("甲、乙，，丙")
        .collect::<Result<Vec<_>, _>>()
        .expect("Unicode delimiters should split on character boundaries");
    assert_eq!(
        unicode
            .iter()
            .map(|item| (item.source_index, item.value))
            .collect::<Vec<_>>(),
        vec![(0, "甲"), (1, "乙"), (2, ""), (3, "丙")],
    );
}
