/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # DataConversionOptions Unit Tests
//!
//! Tests for grouped data conversion options.
//!
//! # Author
//!
//! Haixing Hu

use qubit_datatype::converter::{
    BlankStringPolicy,
    BooleanConversionOptions,
    DataConversionError,
    DataConversionOptions,
    DataConverter,
    DurationConversionOptions,
    DurationUnit,
    EmptyItemPolicy,
    StringConversionOptions,
};

/// Test configurable string normalization and boolean literal parsing.
#[test]
fn test_data_conversion_options_apply_to_converter() {
    let options = DataConversionOptions::default()
        .with_string_options(
            StringConversionOptions::default()
                .with_trim(true)
                .with_blank_string_policy(BlankStringPolicy::TreatAsMissing),
        )
        .with_boolean_options(
            BooleanConversionOptions::strict()
                .with_true_literal("enabled")
                .with_false_literal("disabled"),
        );

    let enabled: bool = DataConverter::from(" enabled ")
        .to_with(&options)
        .expect("custom true literal should parse");
    assert!(enabled);

    let disabled: bool = DataConverter::from(" DISABLED ")
        .to_with(&options)
        .expect("custom false literal should parse case-insensitively");
    assert!(!disabled);

    let port: u16 = DataConverter::from(" 8080 ")
        .to_with(&options)
        .expect("trimmed numeric text should parse");
    assert_eq!(port, 8080);

    let missing = DataConverter::from("   ").to_with::<String>(&options);
    assert!(matches!(missing, Err(DataConversionError::NoValue)));
}

/// Test convenience constructors and branch-specific normalization.
#[test]
fn test_data_conversion_options_convenience_builders() {
    let options = DataConversionOptions::env_friendly()
        .with_blank_string_policy(BlankStringPolicy::Reject)
        .with_empty_item_policy(EmptyItemPolicy::Reject);

    assert!(options.string.trim);
    assert!(options.boolean.true_literals().contains(&"yes".to_string()));
    assert!(
        options
            .boolean
            .false_literals()
            .contains(&"off".to_string())
    );
    assert!(options.collection.split_scalar_strings);
    assert_eq!(
        options.string.blank_string_policy,
        BlankStringPolicy::Reject
    );
    assert_eq!(
        options.collection.empty_item_policy,
        EmptyItemPolicy::Reject,
    );

    let blank = DataConverter::from("   ").to_with::<String>(&options);
    assert!(matches!(
        blank,
        Err(DataConversionError::ConversionError(_))
    ));
}

/// Test duration conversion options default values and builders.
#[test]
fn test_data_conversion_options_duration_builders() {
    let defaults = DataConversionOptions::default();
    assert_eq!(defaults.duration.unit, DurationUnit::Milliseconds);
    assert!(defaults.duration.append_unit_suffix);
    assert_eq!(defaults.duration.unit.suffix(), "ms");

    let options = DataConversionOptions::default().with_duration_options(
        DurationConversionOptions::default()
            .with_unit(DurationUnit::Seconds)
            .with_append_unit_suffix(false),
    );

    assert_eq!(options.duration.unit, DurationUnit::Seconds);
    assert!(!options.duration.append_unit_suffix);
    assert_eq!(DurationUnit::from_suffix("s"), Some(DurationUnit::Seconds));
    assert_eq!(
        DurationUnit::from_suffix("µs"),
        Some(DurationUnit::Microseconds)
    );
    assert_eq!(
        DurationUnit::from_suffix("μs"),
        Some(DurationUnit::Microseconds)
    );
    assert_eq!(DurationUnit::from_suffix("fortnights"), None);
}
