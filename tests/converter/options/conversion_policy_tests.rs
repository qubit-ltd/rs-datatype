// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests for the aggregate conversion policy.

use qubit_datatype::converter::BlankStringPolicy;
use qubit_datatype::converter::BooleanConversionPolicy;
use qubit_datatype::converter::CollectionConversionPolicy;
use qubit_datatype::converter::ConversionPolicy;
use qubit_datatype::converter::DurationConversionPolicy;
use qubit_datatype::converter::DurationRoundingPolicy;
use qubit_datatype::converter::EmptyItemPolicy;
use qubit_datatype::converter::NumericConversionPolicy;
use qubit_datatype::converter::StringConversionPolicy;
use serde_json::from_str;

/// Verifies the policy wire contains semantic fields but no resource limits.
#[test]
fn test_conversion_policy_wire_contains_no_resource_limits() {
    let value =
        serde_json::to_value(ConversionPolicy::default()).expect("the default conversion policy should serialize");
    let object = value
        .as_object()
        .expect("the conversion policy wire should be an object");

    assert_eq!(
        object.keys().map(String::as_str).collect::<Vec<_>>(),
        ["string", "boolean", "numeric", "collection", "duration"],
    );
    let text = value.to_string();
    for forbidden in [
        "max_items",
        "max_input_bytes",
        "max_output_bytes",
        "max_text_bytes",
        "budget",
        "operation",
    ] {
        assert!(!text.contains(forbidden), "unexpected policy field: {forbidden}",);
    }
}

#[test]
fn test_conversion_policy_profiles_preserve_semantic_defaults() {
    assert_eq!(ConversionPolicy::strict(), ConversionPolicy::default());
    assert_eq!(ConversionPolicy::lossy().numeric(), &NumericConversionPolicy::lossy());
    assert!(ConversionPolicy::lossy().string().trim());
    assert_eq!(
        ConversionPolicy::lossy().duration().rounding_policy(),
        DurationRoundingPolicy::HalfUp
    );
    assert!(ConversionPolicy::env_friendly().collection().split_scalar_strings());
    assert!(std::ptr::eq(
        ConversionPolicy::default_ref(),
        ConversionPolicy::default_ref()
    ));
}

#[test]
fn test_conversion_policy_builders_replace_each_policy_group() {
    let policy = ConversionPolicy::builder()
        .numeric_policy(NumericConversionPolicy::lossy())
        .string_policy(StringConversionPolicy::builder().trim(true).build())
        .blank_string_policy(BlankStringPolicy::TreatAsMissing)
        .boolean_policy(BooleanConversionPolicy::env_friendly())
        .collection_policy(CollectionConversionPolicy::env_friendly())
        .empty_item_policy(EmptyItemPolicy::Skip)
        .duration_policy(DurationConversionPolicy::default())
        .build();

    assert_eq!(policy.numeric(), &NumericConversionPolicy::lossy());
    assert!(policy.string().trim());
    assert_eq!(policy.string().blank_string_policy(), BlankStringPolicy::TreatAsMissing);
    assert_eq!(policy.boolean(), &BooleanConversionPolicy::env_friendly());
    assert!(policy.collection().split_scalar_strings());
    assert_eq!(policy.collection().empty_item_policy(), EmptyItemPolicy::Skip);
    assert_eq!(policy.duration(), &DurationConversionPolicy::default());
}

/// Verifies that an existing profile can be customized through its builder.
#[test]
fn test_conversion_policy_profile_into_builder() {
    let policy = ConversionPolicy::env_friendly()
        .into_builder()
        .collection_policy(
            CollectionConversionPolicy::env_friendly()
                .into_builder()
                .delimiters([';', ','])
                .build(),
        )
        .build();

    assert_eq!(policy.collection().delimiters(), &[';', ',']);
}

/// Verifies that each aggregate builder override is independently observable.
#[test]
fn test_conversion_policy_individual_builder_overrides() {
    let boolean = std::hint::black_box(BooleanConversionPolicy::env_friendly());
    let policy = ConversionPolicy::builder().boolean_policy(boolean.clone()).build();
    assert_eq!(policy.boolean(), &boolean);

    let policy = ConversionPolicy::builder()
        .blank_string_policy(BlankStringPolicy::TreatAsMissing)
        .build();
    assert_eq!(policy.string().blank_string_policy(), BlankStringPolicy::TreatAsMissing);

    let policy = ConversionPolicy::builder()
        .empty_item_policy(EmptyItemPolicy::Skip)
        .build();
    assert_eq!(policy.collection().empty_item_policy(), EmptyItemPolicy::Skip);
}

#[test]
fn test_conversion_policy_string_builder_override_order() {
    let leaf_last = ConversionPolicy::builder()
        .string_policy(StringConversionPolicy::default())
        .blank_string_policy(BlankStringPolicy::Reject)
        .build();
    let group_last = ConversionPolicy::builder()
        .blank_string_policy(BlankStringPolicy::Reject)
        .string_policy(StringConversionPolicy::default())
        .build();

    assert_eq!(leaf_last.string().blank_string_policy(), BlankStringPolicy::Reject);
    assert_eq!(group_last.string().blank_string_policy(), BlankStringPolicy::Preserve);
}

#[test]
fn test_conversion_policy_collection_builder_override_order() {
    let leaf_last = ConversionPolicy::builder()
        .collection_policy(CollectionConversionPolicy::default())
        .empty_item_policy(EmptyItemPolicy::Reject)
        .build();
    let group_last = ConversionPolicy::builder()
        .empty_item_policy(EmptyItemPolicy::Reject)
        .collection_policy(CollectionConversionPolicy::default())
        .build();

    assert_eq!(leaf_last.collection().empty_item_policy(), EmptyItemPolicy::Reject);
    assert_eq!(group_last.collection().empty_item_policy(), EmptyItemPolicy::Keep);
}

#[test]
fn test_conversion_policy_deserialize_defaults() {
    assert_eq!(
        from_str::<ConversionPolicy>("{}").expect("omitted aggregate policies should use defaults"),
        ConversionPolicy::default(),
    );
}

#[test]
fn test_conversion_policy_deserialize_partial_string_override() {
    let policy = from_str::<ConversionPolicy>(r#"{"string":{"trim":true}}"#)
        .expect("partial aggregate policies should deserialize");
    let expected = ConversionPolicy::default()
        .into_builder()
        .string_policy(StringConversionPolicy::default().into_builder().trim(true).build())
        .build();
    assert_eq!(policy, expected);
}

#[test]
fn test_conversion_policy_deserialize_rejects_unknown_fields() {
    for wire in [
        r#"{"unexpected":true}"#,
        r#"{"string":{"trim":true,"unexpected":true}}"#,
    ] {
        assert!(from_str::<ConversionPolicy>(wire).is_err());
    }
}
