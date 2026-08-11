// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests for independently configurable numeric conversion policy.

use qubit_datatype::converter::FloatRoundingPolicy;
use qubit_datatype::converter::FractionalToIntegerPolicy;
use qubit_datatype::converter::NumericConversionPolicy;

#[test]
fn test_numeric_conversion_policy_profiles_preserve_semantics() {
    let strict = NumericConversionPolicy::strict();
    assert_eq!(strict, NumericConversionPolicy::default());
    assert_eq!(
        strict.fractional_to_integer(),
        FractionalToIntegerPolicy::Reject
    );
    assert_eq!(strict.numeric_to_float(), FloatRoundingPolicy::Exact);
    assert_eq!(strict.text_to_float(), FloatRoundingPolicy::Exact);

    let lossy = NumericConversionPolicy::lossy();
    assert_eq!(
        lossy.fractional_to_integer(),
        FractionalToIntegerPolicy::Truncate
    );
    assert_eq!(lossy.numeric_to_float(), FloatRoundingPolicy::NearestEven);
    assert_eq!(lossy.text_to_float(), FloatRoundingPolicy::NearestEven);

    let env = NumericConversionPolicy::env_friendly();
    assert_eq!(
        env.fractional_to_integer(),
        FractionalToIntegerPolicy::Reject
    );
    assert_eq!(env.numeric_to_float(), FloatRoundingPolicy::Exact);
    assert_eq!(env.text_to_float(), FloatRoundingPolicy::NearestEven);
}

#[test]
fn test_numeric_conversion_policy_exact_wire_has_no_limits() {
    let policy = NumericConversionPolicy::strict()
        .with_fractional_to_integer(FractionalToIntegerPolicy::Truncate)
        .with_numeric_to_float(FloatRoundingPolicy::NearestEven)
        .with_text_to_float(FloatRoundingPolicy::NearestEven);
    let wire = serde_json::to_string(&policy).expect("numeric policy should serialize");
    assert_eq!(
        wire,
        r#"{"fractional_to_integer":"truncate","numeric_to_float":"nearest_even","text_to_float":"nearest_even"}"#
    );
    assert_eq!(
        serde_json::from_str::<NumericConversionPolicy>(&wire)
            .expect("numeric policy should deserialize"),
        policy
    );
}

#[test]
fn test_numeric_conversion_policy_rejects_limit_fields() {
    let error = serde_json::from_str::<NumericConversionPolicy>(r#"{"max_text_bytes":32}"#)
        .expect_err("numeric limits must not deserialize as policy");
    assert!(error.to_string().contains("unknown field `max_text_bytes`"));
}
