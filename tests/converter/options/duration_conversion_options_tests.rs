// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0
// =============================================================================
use qubit_datatype::converter::{
    DurationConversionOptions,
    DurationUnit,
    SuffixlessDurationPolicy,
};

/// Test duration option defaults and builder methods.
#[test]
fn test_duration_conversion_options_builders_update_fields() {
    let defaults = DurationConversionOptions::default();
    assert_eq!(defaults.numeric_input_unit, DurationUnit::Milliseconds);
    assert_eq!(
        defaults.suffixless_string_policy,
        SuffixlessDurationPolicy::Assume(DurationUnit::Milliseconds),
    );
    assert_eq!(defaults.output_unit, DurationUnit::Milliseconds);
    assert!(defaults.append_unit_suffix);

    let options = defaults
        .with_numeric_input_unit(DurationUnit::Seconds)
        .with_suffixless_string_policy(SuffixlessDurationPolicy::Reject)
        .with_output_unit(DurationUnit::Minutes)
        .with_append_unit_suffix(false);

    assert_eq!(options.numeric_input_unit, DurationUnit::Seconds);
    assert_eq!(
        options.suffixless_string_policy,
        SuffixlessDurationPolicy::Reject,
    );
    assert_eq!(options.output_unit, DurationUnit::Minutes);
    assert!(!options.append_unit_suffix);
}

/// Test that the environment-variable duration profile owns its defaults.
#[test]
fn test_duration_conversion_options_env_friendly_profile() {
    assert_eq!(
        DurationConversionOptions::env_friendly(),
        DurationConversionOptions::default(),
    );
}
