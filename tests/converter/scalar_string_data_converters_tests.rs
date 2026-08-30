// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # ScalarStringDataConverters Unit Tests
//!
//! Tests for scalar string collection conversion behavior.

use qubit_datatype::ConversionResource;
use qubit_datatype::DataType;
use qubit_datatype::converter::BlankStringPolicy;
use qubit_datatype::converter::CollectionConversionLimits;
use qubit_datatype::converter::CollectionConversionPolicy;
use qubit_datatype::converter::ConversionLimits;
use qubit_datatype::converter::ConversionPolicy;
use qubit_datatype::converter::DataConversionError;
use qubit_datatype::converter::DataConversionErrorKind;
use qubit_datatype::converter::EmptyItemPolicy;
use qubit_datatype::converter::InvalidValueReason;
use qubit_datatype::converter::ScalarStringDataConverters;
use qubit_datatype::converter::StringConversionPolicy;

/// Test default scalar string conversion without splitting.
#[test]
fn test_scalar_string_data_converters_to_vec_converts_unsplit_scalar() {
    let ports: Vec<u16> = ScalarStringDataConverters::new("8080")
        .to_vec()
        .expect("unsplit scalar string should convert as one item");

    assert_eq!(ports, vec![8080]);
}

/// Test default first-value conversion without splitting.
#[test]
fn test_scalar_string_data_converters_to_first_converts_unsplit_scalar() {
    let port: u16 = ScalarStringDataConverters::from("8080")
        .to_first()
        .expect("unsplit scalar string should convert as first item");

    assert_eq!(port, 8080);
}

/// Test configurable scalar string splitting for vector conversion.
#[test]
fn test_scalar_string_data_converters_to_vec_with_splits_items() {
    let options = ConversionPolicy::builder()
        .string_policy(StringConversionPolicy::builder().trim(true).build())
        .collection_policy(
            CollectionConversionPolicy::builder()
                .split_scalar_strings(true)
                .delimiters([',', ';'])
                .trim_items(true)
                .empty_item_policy(EmptyItemPolicy::Skip)
                .build(),
        )
        .build();

    let ports: Vec<u16> = ScalarStringDataConverters::from(" 8080, 8081;; 8082 ")
        .to_vec_with(&options, ConversionLimits::default_ref())
        .expect("scalar string should split and parse into ports");

    assert_eq!(ports, vec![8080, 8081, 8082]);
}

/// Test configurable scalar string splitting for first-value conversion.
#[test]
fn test_scalar_string_data_converters_to_first_with_splits_items() {
    let options = ConversionPolicy::env_friendly();

    let first: u16 = ScalarStringDataConverters::from(" 8080, 8081 ")
        .to_first_with(&options, ConversionLimits::default_ref())
        .expect("first split scalar item should parse");

    assert_eq!(first, 8080);
}

/// Test scalar string normalization errors.
#[test]
fn test_scalar_string_data_converters_to_vec_with_reports_missing_scalar() {
    let options = ConversionPolicy::builder()
        .string_policy(
            StringConversionPolicy::builder()
                .trim(true)
                .blank_string_policy(BlankStringPolicy::TreatAsMissing)
                .build(),
        )
        .build();

    let error = ScalarStringDataConverters::from("   ")
        .to_vec_with::<u16>(&options, ConversionLimits::default_ref())
        .expect_err("blank scalar string should be treated as missing");

    assert_eq!(error.source_index(), 0);
    assert_eq!(
        error.conversion_error(),
        &DataConversionError::missing(DataType::String, DataType::UInt16),
    );
}

/// Test scalar string first-value normalization errors.
#[test]
fn test_scalar_string_data_converters_to_first_with_reports_missing_scalar() {
    let options = ConversionPolicy::builder()
        .string_policy(
            StringConversionPolicy::builder()
                .trim(true)
                .blank_string_policy(BlankStringPolicy::TreatAsMissing)
                .build(),
        )
        .build();

    assert_eq!(
        ScalarStringDataConverters::from("   ").to_first_with::<u16>(&options, ConversionLimits::default_ref()),
        Err(DataConversionError::missing(DataType::String, DataType::UInt16)),
    );
}

/// Test scalar string empty item rejection.
#[test]
fn test_scalar_string_data_converters_to_vec_with_rejects_empty_item() {
    let options = ConversionPolicy::builder()
        .collection_policy(
            CollectionConversionPolicy::builder()
                .split_scalar_strings(true)
                .empty_item_policy(EmptyItemPolicy::Reject)
                .build(),
        )
        .build();

    let error = ScalarStringDataConverters::from("1,,2")
        .to_vec_with::<u16>(&options, ConversionLimits::default_ref())
        .expect_err("empty scalar item should be rejected");

    assert_eq!(error.source_index(), 1);
    assert_eq!(
        error.conversion_error(),
        &DataConversionError::invalid(DataType::String, DataType::UInt16, InvalidValueReason::BlankRejected,),
    );
}

/// Test vector conversion reports the first retained item over the limit.
#[test]
fn test_scalar_string_data_converters_to_vec_with_enforces_item_limit() {
    let options = ConversionPolicy::builder()
        .collection_policy(
            CollectionConversionPolicy::builder()
                .split_scalar_strings(true)
                .empty_item_policy(EmptyItemPolicy::Skip)
                .build(),
        )
        .build();
    let limits = ConversionLimits::builder()
        .collection_limits(CollectionConversionLimits::builder().max_items(2).build())
        .build();
    let error = ScalarStringDataConverters::from("1,,2,3")
        .to_vec_with::<u16>(&options, &limits)
        .expect_err("third retained item must exceed the limit");

    assert_eq!(error.source_index(), 3);
    assert_eq!(error.conversion_error().kind(), DataConversionErrorKind::LimitExceeded,);
    assert_eq!(error.conversion_error().from_type(), Some(DataType::String));
    assert_eq!(error.conversion_error().to_type(), DataType::UInt16);
    assert_eq!(
        error.conversion_error().resource(),
        Some(ConversionResource::CollectionItems),
    );
    assert_eq!(error.conversion_error().configured_limit(), Some(2));
    assert_eq!(error.conversion_error().observed_lower_bound(), Some(3));
    assert_eq!(error.conversion_error().observation_is_exact(), Some(true));
}

/// Test first conversion remains lazy and honors a zero item limit.
#[test]
fn test_scalar_string_data_converters_to_first_with_item_limit() {
    let one = ConversionPolicy::builder()
        .collection_policy(CollectionConversionPolicy::builder().split_scalar_strings(true).build())
        .build();
    let one_limit = ConversionLimits::builder()
        .collection_limits(CollectionConversionLimits::builder().max_items(1).build())
        .build();
    assert_eq!(
        ScalarStringDataConverters::from("1,2,3").to_first_with::<u16>(&one, &one_limit),
        Ok(1),
    );

    let zero = ConversionPolicy::builder()
        .collection_policy(CollectionConversionPolicy::builder().split_scalar_strings(true).build())
        .build();
    let zero_limit = ConversionLimits::builder()
        .collection_limits(CollectionConversionLimits::builder().max_items(0).build())
        .build();
    let error = ScalarStringDataConverters::from("1,2,3")
        .to_first_with::<u16>(&zero, &zero_limit)
        .expect_err("zero limit must reject the first retained item");
    assert_eq!(error.kind(), DataConversionErrorKind::LimitExceeded);
    assert_eq!(error.resource(), Some(ConversionResource::CollectionItems));
    assert_eq!(error.configured_limit(), Some(0));
    assert_eq!(error.observed_lower_bound(), Some(1));
}

/// Test scalar string first-value empty item rejection.
#[test]
fn test_scalar_string_data_converters_to_first_with_rejects_empty_item() {
    let options = ConversionPolicy::builder()
        .collection_policy(
            CollectionConversionPolicy::builder()
                .split_scalar_strings(true)
                .empty_item_policy(EmptyItemPolicy::Reject)
                .build(),
        )
        .build();

    assert_eq!(
        ScalarStringDataConverters::from(",1,2").to_first_with::<u16>(&options, ConversionLimits::default_ref()),
        Err(DataConversionError::invalid(
            DataType::String,
            DataType::UInt16,
            InvalidValueReason::BlankRejected,
        )),
    );
}

/// Test scalar string first-value behavior when all items are skipped.
#[test]
fn test_scalar_string_data_converters_to_first_with_reports_empty_after_skip() {
    let options = ConversionPolicy::builder()
        .collection_policy(
            CollectionConversionPolicy::builder()
                .split_scalar_strings(true)
                .empty_item_policy(EmptyItemPolicy::Skip)
                .build(),
        )
        .build();

    assert_eq!(
        ScalarStringDataConverters::from(",,").to_first_with::<u16>(&options, ConversionLimits::default_ref()),
        Err(DataConversionError::empty_collection(DataType::UInt16)),
    );
}

/// Test that skipped empty items do not renumber later source failures.
#[test]
fn test_scalar_string_data_converters_preserves_original_source_index() {
    let options = ConversionPolicy::builder()
        .collection_policy(
            CollectionConversionPolicy::builder()
                .split_scalar_strings(true)
                .empty_item_policy(EmptyItemPolicy::Skip)
                .build(),
        )
        .build();

    let error = ScalarStringDataConverters::from("1,,bad")
        .to_vec_with::<u16>(&options, ConversionLimits::default_ref())
        .expect_err("invalid third source item should fail");

    assert_eq!(error.source_index(), 2);
}

/// Test that first-value conversion does not inspect a rejected tail item.
#[test]
fn test_scalar_string_data_converters_to_first_short_circuits_tail() {
    let options = ConversionPolicy::builder()
        .collection_policy(
            CollectionConversionPolicy::builder()
                .split_scalar_strings(true)
                .empty_item_policy(EmptyItemPolicy::Reject)
                .build(),
        )
        .build();

    let first = ScalarStringDataConverters::from("1,,")
        .to_first_with::<u16>(&options, ConversionLimits::default_ref())
        .expect("valid first item should short-circuit the rejected tail");

    assert_eq!(first, 1);
}

/// Test whole-scalar blank rejection is reported as a conversion failure.
#[test]
fn test_scalar_string_data_converters_rejects_blank_scalar() {
    let options = ConversionPolicy::builder()
        .string_policy(
            StringConversionPolicy::builder()
                .trim(true)
                .blank_string_policy(BlankStringPolicy::Reject)
                .build(),
        )
        .build();

    let error = ScalarStringDataConverters::from("   ")
        .to_vec_with::<u16>(&options, ConversionLimits::default_ref())
        .expect_err("blank scalar should be rejected");
    assert_eq!(error.source_index(), 0);
    assert!(matches!(
        error.conversion_error(),
        conversion_error if matches!(conversion_error.reason(), Some(InvalidValueReason::BlankRejected)),
    ));
}

/// Test collection empty-item policy runs before string blank policy.
#[test]
fn test_scalar_string_data_converters_env_blank_list_is_empty() {
    let options = ConversionPolicy::env_friendly();

    assert_eq!(
        ScalarStringDataConverters::from("   ").to_vec_with::<u16>(&options, ConversionLimits::default_ref()),
        Ok(Vec::new()),
    );
    assert_eq!(
        ScalarStringDataConverters::from(" , \t,").to_vec_with::<u16>(&options, ConversionLimits::default_ref()),
        Ok(Vec::new()),
    );
    assert_eq!(
        ScalarStringDataConverters::from("   ").to_first_with::<u16>(&options, ConversionLimits::default_ref()),
        Err(DataConversionError::empty_collection(DataType::UInt16)),
    );
}

/// Test the complete scalar source is checked before delimiter scanning.
#[test]
fn test_scalar_string_data_converters_checks_delimiter_only_source_before_scan() {
    let policy = ConversionPolicy::env_friendly();
    let limits = ConversionLimits::builder()
        .collection_limits(CollectionConversionLimits::builder().max_source_bytes(3).build())
        .build();

    let error = ScalarStringDataConverters::from(",,,,")
        .to_vec_with::<String>(&policy, &limits)
        .expect_err("the complete source should be checked before empty items are skipped");

    assert_eq!(
        error.conversion_error().resource(),
        Some(ConversionResource::CollectionSourceBytes),
    );
}

/// Test first-item conversion checks the complete source before
/// short-circuiting.
#[test]
fn test_scalar_string_data_converters_to_first_checks_complete_source() {
    let policy = ConversionPolicy::env_friendly();
    let limits = ConversionLimits::builder()
        .collection_limits(CollectionConversionLimits::builder().max_source_bytes(3).build())
        .build();

    let error = ScalarStringDataConverters::from("1,long-tail")
        .to_first_with::<u16>(&policy, &limits)
        .expect_err("the complete source should be checked before returning its first item");

    assert_eq!(error.resource(), Some(ConversionResource::CollectionSourceBytes),);
}
