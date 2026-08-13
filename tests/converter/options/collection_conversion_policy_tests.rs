// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests for scalar collection conversion policy.

use qubit_datatype::converter::CollectionConversionPolicy;
use qubit_datatype::converter::EmptyItemPolicy;

#[test]
fn test_collection_conversion_policy_profiles_preserve_semantics() {
    let defaults = CollectionConversionPolicy::default();
    assert!(!defaults.split_scalar_strings());
    assert_eq!(defaults.delimiters(), &[',']);
    assert!(!defaults.trim_items());
    assert_eq!(defaults.empty_item_policy(), EmptyItemPolicy::Keep);

    let env = CollectionConversionPolicy::env_friendly();
    assert!(env.split_scalar_strings());
    assert_eq!(env.delimiters(), &[',']);
    assert!(env.trim_items());
    assert_eq!(env.empty_item_policy(), EmptyItemPolicy::Skip);
}

#[test]
fn test_collection_conversion_policy_exact_wire_has_no_limits() {
    let policy = CollectionConversionPolicy::default()
        .with_split_scalar_strings(true)
        .with_delimiters([',', ';'])
        .with_trim_items(true)
        .with_empty_item_policy(EmptyItemPolicy::Skip);
    let wire = serde_json::to_string(&policy).expect("collection policy should serialize");
    assert_eq!(
        wire,
        r#"{"split_scalar_strings":true,"delimiters":[",",";"],"trim_items":true,"empty_item_policy":"skip"}"#
    );
    assert_eq!(
        serde_json::from_str::<CollectionConversionPolicy>(&wire)
            .expect("collection policy should deserialize"),
        policy
    );
}

#[test]
fn test_collection_conversion_policy_rejects_limit_fields() {
    let error = serde_json::from_str::<CollectionConversionPolicy>(r#"{"max_items":3}"#)
        .expect_err("collection limits must not deserialize as policy");
    assert!(error.to_string().contains("unknown field `max_items`"));
}
