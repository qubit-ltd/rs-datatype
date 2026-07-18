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

use qubit_datatype::DataType;
use qubit_datatype::converter::{
    BlankStringPolicy, CollectionConversionOptions, DataConversionError, DataConversionOptions,
    EmptyItemPolicy, InvalidValueReason, ScalarStringDataConverters, StringConversionOptions,
};

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
    let options = DataConversionOptions::default()
        .with_string_options(StringConversionOptions::default().with_trim(true))
        .with_collection_options(
            CollectionConversionOptions::default()
                .with_split_scalar_strings(true)
                .with_delimiters([',', ';'])
                .with_trim_items(true)
                .with_empty_item_policy(EmptyItemPolicy::Skip),
        );

    let ports: Vec<u16> = ScalarStringDataConverters::from(" 8080, 8081;; 8082 ")
        .to_vec_with(&options)
        .expect("scalar string should split and parse into ports");

    assert_eq!(ports, vec![8080, 8081, 8082]);
}

/// Test configurable scalar string splitting for first-value conversion.
#[test]
fn test_scalar_string_data_converters_to_first_with_splits_items() {
    let options = DataConversionOptions::env_friendly();

    let first: u16 = ScalarStringDataConverters::from(" 8080, 8081 ")
        .to_first_with(&options)
        .expect("first split scalar item should parse");

    assert_eq!(first, 8080);
}

/// Test scalar string normalization errors.
#[test]
fn test_scalar_string_data_converters_to_vec_with_reports_missing_scalar() {
    let options = DataConversionOptions::default().with_string_options(
        StringConversionOptions::default()
            .with_trim(true)
            .with_blank_string_policy(BlankStringPolicy::TreatAsMissing),
    );

    let error = ScalarStringDataConverters::from("   ")
        .to_vec_with::<u16>(&options)
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
    let options = DataConversionOptions::default().with_string_options(
        StringConversionOptions::default()
            .with_trim(true)
            .with_blank_string_policy(BlankStringPolicy::TreatAsMissing),
    );

    assert_eq!(
        ScalarStringDataConverters::from("   ").to_first_with::<u16>(&options),
        Err(DataConversionError::missing(
            DataType::String,
            DataType::UInt16
        )),
    );
}

/// Test scalar string empty item rejection.
#[test]
fn test_scalar_string_data_converters_to_vec_with_rejects_empty_item() {
    let options = DataConversionOptions::default().with_collection_options(
        CollectionConversionOptions::default()
            .with_split_scalar_strings(true)
            .with_empty_item_policy(EmptyItemPolicy::Reject),
    );

    let error = ScalarStringDataConverters::from("1,,2")
        .to_vec_with::<u16>(&options)
        .expect_err("empty scalar item should be rejected");

    assert_eq!(error.source_index(), 1);
    assert_eq!(
        error.conversion_error(),
        &DataConversionError::invalid(
            DataType::String,
            DataType::UInt16,
            InvalidValueReason::BlankRejected,
        ),
    );
}

/// Test scalar string first-value empty item rejection.
#[test]
fn test_scalar_string_data_converters_to_first_with_rejects_empty_item() {
    let options = DataConversionOptions::default().with_collection_options(
        CollectionConversionOptions::default()
            .with_split_scalar_strings(true)
            .with_empty_item_policy(EmptyItemPolicy::Reject),
    );

    assert_eq!(
        ScalarStringDataConverters::from(",1,2").to_first_with::<u16>(&options),
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
    let options = DataConversionOptions::default().with_collection_options(
        CollectionConversionOptions::default()
            .with_split_scalar_strings(true)
            .with_empty_item_policy(EmptyItemPolicy::Skip),
    );

    assert_eq!(
        ScalarStringDataConverters::from(",,").to_first_with::<u16>(&options),
        Err(DataConversionError::empty_collection(DataType::UInt16)),
    );
}

/// Test that skipped empty items do not renumber later source failures.
#[test]
fn test_scalar_string_data_converters_preserves_original_source_index() {
    let options = DataConversionOptions::default().with_collection_options(
        CollectionConversionOptions::default()
            .with_split_scalar_strings(true)
            .with_empty_item_policy(EmptyItemPolicy::Skip),
    );

    let error = ScalarStringDataConverters::from("1,,bad")
        .to_vec_with::<u16>(&options)
        .expect_err("invalid third source item should fail");

    assert_eq!(error.source_index(), 2);
}

/// Test that first-value conversion does not inspect a rejected tail item.
#[test]
fn test_scalar_string_data_converters_to_first_short_circuits_tail() {
    let options = DataConversionOptions::default().with_collection_options(
        CollectionConversionOptions::default()
            .with_split_scalar_strings(true)
            .with_empty_item_policy(EmptyItemPolicy::Reject),
    );

    let first = ScalarStringDataConverters::from("1,,")
        .to_first_with::<u16>(&options)
        .expect("valid first item should short-circuit the rejected tail");

    assert_eq!(first, 1);
}

/// Test whole-scalar blank rejection is reported as a conversion failure.
#[test]
fn test_scalar_string_data_converters_rejects_blank_scalar() {
    let options = DataConversionOptions::default().with_string_options(
        StringConversionOptions::default()
            .with_trim(true)
            .with_blank_string_policy(BlankStringPolicy::Reject),
    );

    let error = ScalarStringDataConverters::from("   ")
        .to_vec_with::<u16>(&options)
        .expect_err("blank scalar should be rejected");
    assert_eq!(error.source_index(), 0);
    assert!(matches!(
        error.conversion_error(),
        conversion_error if matches!(conversion_error.reason(), Some(InvalidValueReason::BlankRejected)),
    ));
}
