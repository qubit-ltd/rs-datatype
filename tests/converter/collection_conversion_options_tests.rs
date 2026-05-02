/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! # CollectionConversionOptions Unit Tests
//!
//! Tests for scalar string collection conversion options.
//!

use qubit_datatype::converter::{
    CollectionConversionOptions,
    EmptyItemPolicy,
};

/// Test collection option split and empty-item branches.
#[test]
fn test_collection_conversion_options_cover_policy_branches() {
    let scalar_items = CollectionConversionOptions::default()
        .scalar_items(" a,b ")
        .expect("non-split scalar should remain one item");
    assert_eq!(scalar_items, vec![" a,b ".to_string()]);

    let kept_items = CollectionConversionOptions::default()
        .with_split_scalar_strings(true)
        .with_delimiters([',', ';'])
        .scalar_items("a,,;b")
        .expect("empty items should be kept by default");
    assert_eq!(
        kept_items,
        vec![
            "a".to_string(),
            "".to_string(),
            "".to_string(),
            "b".to_string(),
        ],
    );

    let skipped_items = CollectionConversionOptions::default()
        .with_split_scalar_strings(true)
        .with_delimiters([','])
        .with_trim_items(true)
        .with_empty_item_policy(EmptyItemPolicy::Skip)
        .scalar_items("a,  ,b")
        .expect("blank collection item should be skipped");
    assert_eq!(skipped_items, vec!["a".to_string(), "b".to_string()]);

    let rejected_items = CollectionConversionOptions::default()
        .with_split_scalar_strings(true)
        .with_delimiters([','])
        .with_trim_items(true)
        .with_empty_item_policy(EmptyItemPolicy::Reject)
        .scalar_items("a,  ,b")
        .expect_err("blank collection item should be rejected");
    assert_eq!(rejected_items.index, 1);
}
