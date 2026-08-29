// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests for duration conversion policy.

use qubit_datatype::DurationUnitParseMode;
use qubit_datatype::converter::DurationConversionPolicy;
use qubit_datatype::converter::DurationRoundingPolicy;
use qubit_datatype::converter::DurationUnit;
use qubit_datatype::converter::SuffixlessDurationPolicy;

#[test]
fn test_duration_conversion_policy_defaults_preserve_semantics() {
    let policy = DurationConversionPolicy::default();
    assert_eq!(policy.numeric_input_unit(), DurationUnit::Milliseconds);
    assert_eq!(policy.suffixless_string_policy(), SuffixlessDurationPolicy::Reject);
    assert_eq!(policy.unit_parse_mode(), DurationUnitParseMode::Strict);
    assert_eq!(policy.output_unit(), DurationUnit::Milliseconds);
    assert!(policy.append_unit_suffix());
    assert_eq!(policy.rounding_policy(), DurationRoundingPolicy::Reject);
}

#[test]
fn test_duration_conversion_policy_exact_wire_has_no_limits() {
    let policy = DurationConversionPolicy::builder()
        .numeric_input_unit(DurationUnit::Seconds)
        .suffixless_string_policy(SuffixlessDurationPolicy::Assume(DurationUnit::Minutes))
        .unit_parse_mode(DurationUnitParseMode::Lenient)
        .output_unit(DurationUnit::Hours)
        .append_unit_suffix(false)
        .rounding_policy(DurationRoundingPolicy::HalfUp)
        .build();
    let wire = serde_json::to_string(&policy).expect("duration policy should serialize");
    assert_eq!(
        wire,
        r#"{"numeric_input_unit":"seconds","suffixless_string_policy":{"assume":"minutes"},"unit_parse_mode":"lenient","output_unit":"hours","append_unit_suffix":false,"rounding_policy":"half_up"}"#
    );
    assert_eq!(
        serde_json::from_str::<DurationConversionPolicy>(&wire).expect("duration policy should deserialize"),
        policy
    );
}

#[test]
fn test_duration_conversion_policy_rejects_limit_fields() {
    let error = serde_json::from_str::<DurationConversionPolicy>(r#"{"max_text_bytes":8}"#)
        .expect_err("duration limits must not deserialize as policy");
    assert!(error.to_string().contains("unknown field `max_text_bytes`"));
}
