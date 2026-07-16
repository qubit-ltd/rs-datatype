// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # DataConversionOptions Unit Tests
//!
//! Tests for grouped data conversion options.

use qubit_datatype::DataType;
use qubit_datatype::converter::{
    BlankStringPolicy,
    BooleanConversionOptions,
    BooleanNumericPolicy,
    DataConversionError,
    DataConversionOptions,
    DataConverter,
    DurationConversionOptions,
    DurationUnit,
    EmptyItemPolicy,
    InvalidValueReason,
    NumericConversionPolicy,
    StringConversionOptions,
    SuffixlessDurationPolicy,
};

/// Test the complete strict and lossy profile contracts.
#[test]
fn test_data_conversion_options_profiles() {
    let defaults = DataConversionOptions::default();
    let strict = DataConversionOptions::strict();
    assert_eq!(strict, defaults);

    let lossy = DataConversionOptions::lossy();
    assert_eq!(lossy.numeric_policy, NumericConversionPolicy::Lossy);
    assert!(lossy.string.trim);
    assert_eq!(
        lossy.string.blank_string_policy,
        defaults.string.blank_string_policy,
    );
    assert_eq!(lossy.boolean, defaults.boolean);
    assert_eq!(lossy.collection, defaults.collection);
    assert_eq!(lossy.duration, defaults.duration);

    assert_eq!(DataConverter::from(" 3.9 ").to_with::<i32>(&lossy), Ok(3),);
}

/// Test that misspelled top-level option fields are rejected.
#[test]
fn test_data_conversion_options_reject_unknown_fields() {
    let error = serde_json::from_str::<DataConversionOptions>(
        r#"{"numeric_policy":"exact","unexpected":true}"#,
    )
    .expect_err("unknown top-level fields must be rejected");

    assert!(error.to_string().contains("unknown field `unexpected`"));
}

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
                .expect("enabled should not conflict")
                .with_false_literal("disabled")
                .expect("disabled should not conflict"),
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
    assert!(matches!(
        missing,
        Err(DataConversionError::Missing {
            from: DataType::String,
            to: DataType::String,
        }),
    ));
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
        Err(DataConversionError::InvalidValue {
            from: DataType::String,
            to: DataType::String,
            reason: InvalidValueReason::BlankRejected,
        }),
    ));
}

/// Test duration conversion options default values and builders.
#[test]
fn test_data_conversion_options_duration_builders() {
    let defaults = DataConversionOptions::default();
    assert_eq!(
        defaults.duration.numeric_input_unit,
        DurationUnit::Milliseconds,
    );
    assert_eq!(
        defaults.duration.suffixless_string_policy,
        SuffixlessDurationPolicy::Assume(DurationUnit::Milliseconds),
    );
    assert_eq!(defaults.duration.output_unit, DurationUnit::Milliseconds);
    assert!(defaults.duration.append_unit_suffix);
    assert_eq!(defaults.duration.output_unit.suffix(), "ms");

    let options = DataConversionOptions::default().with_duration_options(
        DurationConversionOptions::default()
            .with_numeric_input_unit(DurationUnit::Seconds)
            .with_suffixless_string_policy(SuffixlessDurationPolicy::Reject)
            .with_output_unit(DurationUnit::Minutes)
            .with_append_unit_suffix(false),
    );

    assert_eq!(options.duration.numeric_input_unit, DurationUnit::Seconds,);
    assert_eq!(
        options.duration.suffixless_string_policy,
        SuffixlessDurationPolicy::Reject,
    );
    assert_eq!(options.duration.output_unit, DurationUnit::Minutes);
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

/// Test all duration unit suffixes and rounding conversions.
#[test]
fn test_duration_unit_suffixes_and_rounding_cover_all_units() {
    let cases = [
        (DurationUnit::Nanoseconds, "ns", 1),
        (DurationUnit::Microseconds, "us", 1_000),
        (DurationUnit::Milliseconds, "ms", 1_000_000),
        (DurationUnit::Seconds, "s", 1_000_000_000),
        (DurationUnit::Minutes, "m", 60_000_000_000),
        (DurationUnit::Hours, "h", 3_600_000_000_000),
        (DurationUnit::Days, "d", 86_400_000_000_000),
    ];

    for (unit, suffix, nanos) in cases {
        assert_eq!(unit.suffix(), suffix);
        assert_eq!(
            unit.rounded_units(std::time::Duration::from_nanos(nanos)),
            1
        );
    }
}

/// Test exact and lossy numeric policies for typed and textual sources.
#[test]
fn test_data_conversion_options_numeric_policy_is_source_independent() {
    let exact = DataConversionOptions::default()
        .with_numeric_policy(NumericConversionPolicy::Exact);
    for (converter, from) in [
        (DataConverter::from(3.9f64), DataType::Float64),
        (DataConverter::from("3.9"), DataType::String),
    ] {
        assert!(matches!(
            converter.to_with::<i32>(&exact),
            Err(DataConversionError::InvalidValue {
                from: actual_from,
                to: DataType::Int32,
                reason: InvalidValueReason::PrecisionLoss,
            }) if actual_from == from,
        ));
    }

    let lossy = DataConversionOptions::lossy();
    for converter in [DataConverter::from(3.9f64), DataConverter::from("3.9")] {
        assert_eq!(
            converter
                .to_with::<i32>(&lossy)
                .expect("lossy conversion should truncate toward zero"),
            3,
        );
    }
    for converter in [DataConverter::from(-3.9f64), DataConverter::from("-3.9")]
    {
        assert_eq!(
            converter.to_with::<i32>(&lossy).expect(
                "lossy negative conversion should truncate toward zero"
            ),
            -3,
        );
    }
}

/// Test every numeric-to-boolean policy for typed and textual integers.
#[test]
fn test_data_conversion_options_boolean_numeric_policy_is_source_independent() {
    let zero_or_one = DataConversionOptions::default().with_boolean_options(
        BooleanConversionOptions::default()
            .with_numeric_policy(BooleanNumericPolicy::ZeroOrOne),
    );
    for (converter, from) in [
        (DataConverter::from(2i32), DataType::Int32),
        (DataConverter::from("2"), DataType::String),
    ] {
        assert!(matches!(
            converter.to_with::<bool>(&zero_or_one),
            Err(DataConversionError::InvalidValue {
                from: actual_from,
                to: DataType::Bool,
                reason: InvalidValueReason::InvalidBoolean,
            }) if actual_from == from,
        ));
    }
    for converter in [DataConverter::from(0i32), DataConverter::from("0")] {
        assert!(
            !converter
                .to_with::<bool>(&zero_or_one)
                .expect("zero should convert to false"),
        );
    }
    for converter in [DataConverter::from(1i32), DataConverter::from("1")] {
        assert!(
            converter
                .to_with::<bool>(&zero_or_one)
                .expect("one should convert to true"),
        );
    }

    let non_zero = DataConversionOptions::default().with_boolean_options(
        BooleanConversionOptions::default()
            .with_numeric_policy(BooleanNumericPolicy::NonZero),
    );
    for converter in [DataConverter::from(2i32), DataConverter::from("2")] {
        assert!(
            converter
                .to_with::<bool>(&non_zero)
                .expect("non-zero integer should convert to true"),
        );
    }
    for converter in [DataConverter::from(0i32), DataConverter::from("0")] {
        assert!(
            !converter
                .to_with::<bool>(&non_zero)
                .expect("zero should convert to false"),
        );
    }
    for converter in [DataConverter::from(-2i32), DataConverter::from("-2")] {
        assert!(
            converter
                .to_with::<bool>(&non_zero)
                .expect("negative non-zero integer should convert to true"),
        );
    }

    let reject = DataConversionOptions::default().with_boolean_options(
        BooleanConversionOptions::default()
            .with_numeric_policy(BooleanNumericPolicy::Reject),
    );
    for (converter, from) in [
        (DataConverter::from(1i32), DataType::Int32),
        (DataConverter::from("1"), DataType::String),
    ] {
        assert!(matches!(
            converter.to_with::<bool>(&reject),
            Err(DataConversionError::InvalidValue {
                from: actual_from,
                to: DataType::Bool,
                reason: InvalidValueReason::InvalidBoolean,
            }) if actual_from == from,
        ));
    }
}

/// Test defaults, shared default identity, and nested Serde defaults.
#[test]
fn test_data_conversion_options_serde_and_default_ref() {
    let defaults = DataConversionOptions::default();
    assert_eq!(defaults.numeric_policy, NumericConversionPolicy::Exact);
    assert_eq!(
        defaults.boolean.numeric_policy(),
        BooleanNumericPolicy::ZeroOrOne,
    );
    assert!(std::ptr::eq(
        DataConversionOptions::default_ref(),
        DataConversionOptions::default_ref(),
    ));
    assert_eq!(
        serde_json::from_str::<DataConversionOptions>("{}")
            .expect("empty options object should use defaults"),
        defaults,
    );
    let wire = serde_json::to_string(&defaults)
        .expect("conversion options should serialize");
    assert_eq!(
        serde_json::from_str::<DataConversionOptions>(&wire)
            .expect("conversion options should deserialize"),
        defaults,
    );
}
