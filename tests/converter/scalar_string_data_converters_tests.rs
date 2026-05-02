/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! # ScalarStringDataConverters Unit Tests
//!
//! Tests for scalar string collection conversion behavior.
//!

use qubit_datatype::converter::{
    BlankStringPolicy,
    CollectionConversionOptions,
    DataConversionError,
    DataConversionOptions,
    EmptyItemPolicy,
    ScalarStringDataConverters,
    StringConversionOptions,
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

    assert_eq!(error.index, 0);
    assert!(matches!(error.source, DataConversionError::NoValue));
}

/// Test scalar string first-value normalization errors.
#[test]
fn test_scalar_string_data_converters_to_first_with_reports_missing_scalar() {
    let options = DataConversionOptions::default().with_string_options(
        StringConversionOptions::default()
            .with_trim(true)
            .with_blank_string_policy(BlankStringPolicy::TreatAsMissing),
    );

    assert!(matches!(
        ScalarStringDataConverters::from("   ").to_first_with::<u16>(&options),
        Err(DataConversionError::NoValue),
    ));
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

    assert_eq!(error.index, 1);
    assert!(matches!(
        error.source,
        DataConversionError::ConversionError(_),
    ));
}

/// Test scalar string first-value empty item rejection.
#[test]
fn test_scalar_string_data_converters_to_first_with_rejects_empty_item() {
    let options = DataConversionOptions::default().with_collection_options(
        CollectionConversionOptions::default()
            .with_split_scalar_strings(true)
            .with_empty_item_policy(EmptyItemPolicy::Reject),
    );

    assert!(matches!(
        ScalarStringDataConverters::from("1,,2").to_first_with::<u16>(&options),
        Err(DataConversionError::ConversionError(_)),
    ));
}

/// Test scalar string first-value behavior when all items are skipped.
#[test]
fn test_scalar_string_data_converters_to_first_with_reports_no_value_after_skip() {
    let options = DataConversionOptions::default().with_collection_options(
        CollectionConversionOptions::default()
            .with_split_scalar_strings(true)
            .with_empty_item_policy(EmptyItemPolicy::Skip),
    );

    assert!(matches!(
        ScalarStringDataConverters::from(",,").to_first_with::<u16>(&options),
        Err(DataConversionError::NoValue),
    ));
}
