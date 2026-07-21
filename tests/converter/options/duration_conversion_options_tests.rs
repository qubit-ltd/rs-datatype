// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
use qubit_datatype::converter::{
    DurationConversionOptions,
    DurationRoundingPolicy,
    DurationUnit,
    SuffixlessDurationPolicy,
};
use qubit_datatype::{
    DurationTextOptions,
    DurationUnitSuffixSet,
};
use serde_json::json;

const DURATION_UNITS: [(DurationUnit, &str); 7] = [
    (DurationUnit::Nanoseconds, "nanoseconds"),
    (DurationUnit::Microseconds, "microseconds"),
    (DurationUnit::Milliseconds, "milliseconds"),
    (DurationUnit::Seconds, "seconds"),
    (DurationUnit::Minutes, "minutes"),
    (DurationUnit::Hours, "hours"),
    (DurationUnit::Days, "days"),
];

/// Test duration option defaults and builder methods.
#[test]
fn test_duration_conversion_options_builders_update_fields() {
    let defaults = DurationConversionOptions::default();
    assert_eq!(defaults.numeric_input_unit(), DurationUnit::Milliseconds,);
    assert_eq!(
        defaults.suffixless_string_policy(),
        SuffixlessDurationPolicy::Assume(DurationUnit::Milliseconds),
    );
    assert_eq!(defaults.unit_suffix_set(), DurationUnitSuffixSet::Extended,);
    assert_eq!(
        defaults.max_text_bytes(),
        DurationTextOptions::DEFAULT_MAX_TEXT_BYTES,
    );
    assert_eq!(defaults.output_unit(), DurationUnit::Milliseconds);
    assert!(defaults.append_unit_suffix());
    assert_eq!(defaults.rounding_policy(), DurationRoundingPolicy::Reject,);

    let options = defaults
        .with_numeric_input_unit(DurationUnit::Seconds)
        .with_suffixless_string_policy(SuffixlessDurationPolicy::Reject)
        .with_unit_suffix_set(DurationUnitSuffixSet::Ascii)
        .with_max_text_bytes(4_096)
        .with_output_unit(DurationUnit::Minutes)
        .with_append_unit_suffix(false)
        .with_rounding_policy(DurationRoundingPolicy::HalfUp);

    assert_eq!(options.numeric_input_unit(), DurationUnit::Seconds);
    assert_eq!(
        options.suffixless_string_policy(),
        SuffixlessDurationPolicy::Reject,
    );
    assert_eq!(options.unit_suffix_set(), DurationUnitSuffixSet::Ascii);
    assert_eq!(options.max_text_bytes(), 4_096);
    assert_eq!(options.output_unit(), DurationUnit::Minutes);
    assert!(!options.append_unit_suffix());
    assert_eq!(options.rounding_policy(), DurationRoundingPolicy::HalfUp,);
}

/// Test that the environment-variable duration profile owns its defaults.
#[test]
fn test_duration_conversion_options_env_friendly_profile() {
    assert_eq!(
        DurationConversionOptions::env_friendly(),
        DurationConversionOptions::default(),
    );
}

/// Test the exact seven-field JSON wire format and its round trip.
#[test]
fn test_duration_conversion_options_exact_json_wire_and_round_trip() {
    let options = DurationConversionOptions::default()
        .with_numeric_input_unit(DurationUnit::Seconds)
        .with_suffixless_string_policy(SuffixlessDurationPolicy::Assume(
            DurationUnit::Minutes,
        ))
        .with_unit_suffix_set(DurationUnitSuffixSet::Ascii)
        .with_max_text_bytes(4_096)
        .with_output_unit(DurationUnit::Hours)
        .with_append_unit_suffix(false)
        .with_rounding_policy(DurationRoundingPolicy::HalfUp);

    let wire = serde_json::to_string(&options)
        .expect("duration options should serialize");
    assert_eq!(
        wire,
        r#"{"numeric_input_unit":"seconds","suffixless_string_policy":{"assume":"minutes"},"unit_suffix_set":"ascii","max_text_bytes":4096,"output_unit":"hours","append_unit_suffix":false,"rounding_policy":"half_up"}"#,
    );
    assert_eq!(
        serde_json::from_str::<DurationConversionOptions>(&wire)
            .expect("duration options should deserialize"),
        options,
    );
}

/// Test that omitted JSON fields receive their default values.
#[test]
fn test_duration_conversion_options_partial_json_uses_defaults() {
    let options: DurationConversionOptions =
        serde_json::from_str(r#"{"output_unit":"seconds"}"#)
            .expect("partial duration options should deserialize");

    assert_eq!(options.numeric_input_unit(), DurationUnit::Milliseconds,);
    assert_eq!(
        options.suffixless_string_policy(),
        SuffixlessDurationPolicy::Assume(DurationUnit::Milliseconds),
    );
    assert_eq!(options.unit_suffix_set(), DurationUnitSuffixSet::Extended,);
    assert_eq!(
        options.max_text_bytes(),
        DurationTextOptions::DEFAULT_MAX_TEXT_BYTES,
    );
    assert_eq!(options.output_unit(), DurationUnit::Seconds);
    assert!(options.append_unit_suffix());
    assert_eq!(options.rounding_policy(), DurationRoundingPolicy::Reject,);
}

/// Test all duration units, suffixless policies, and suffix flag values.
#[test]
fn test_duration_conversion_options_all_serde_values() {
    for (unit, wire_name) in DURATION_UNITS {
        let options = DurationConversionOptions::default()
            .with_numeric_input_unit(unit)
            .with_suffixless_string_policy(SuffixlessDurationPolicy::Assume(
                unit,
            ))
            .with_output_unit(unit);
        let wire = serde_json::to_value(&options)
            .expect("duration options should serialize");

        assert_eq!(
            wire,
            json!({
                "numeric_input_unit": wire_name,
                "suffixless_string_policy": { "assume": wire_name },
                "unit_suffix_set": "extended",
                "max_text_bytes": DurationTextOptions::DEFAULT_MAX_TEXT_BYTES,
                "output_unit": wire_name,
                "append_unit_suffix": true,
                "rounding_policy": "reject",
            }),
        );
        assert_eq!(
            serde_json::from_value::<DurationConversionOptions>(wire)
                .expect("duration options should deserialize"),
            options,
        );
    }

    for append_unit_suffix in [true, false] {
        let options = DurationConversionOptions::default()
            .with_suffixless_string_policy(SuffixlessDurationPolicy::Reject)
            .with_append_unit_suffix(append_unit_suffix);
        let wire = serde_json::to_value(&options)
            .expect("duration options should serialize");

        assert_eq!(wire["suffixless_string_policy"], json!("reject"));
        assert_eq!(wire["append_unit_suffix"], json!(append_unit_suffix),);
        assert_eq!(
            serde_json::from_value::<DurationConversionOptions>(wire)
                .expect("duration options should deserialize"),
            options,
        );
    }
}

/// Test that the removed `unit` field is rejected with a precise error.
#[test]
fn test_duration_conversion_options_rejects_legacy_unit_field() {
    let error = serde_json::from_str::<DurationConversionOptions>(
        r#"{"unit":"seconds"}"#,
    )
    .expect_err("the legacy unit field should be rejected");

    assert!(
        error.to_string().contains("unknown field `unit`"),
        "error should identify the rejected field: {error}",
    );
}

/// Test that an arbitrary unknown field is rejected with a precise error.
#[test]
fn test_duration_conversion_options_rejects_unknown_field() {
    let error = serde_json::from_str::<DurationConversionOptions>(
        r#"{"future_option":true}"#,
    )
    .expect_err("unknown duration option fields should be rejected");

    assert!(
        error.to_string().contains("future_option"),
        "error should identify the rejected field: {error}",
    );
}
