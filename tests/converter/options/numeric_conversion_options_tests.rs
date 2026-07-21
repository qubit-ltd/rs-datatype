// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests for independently configurable numeric conversion options.

use qubit_datatype::converter::{
    FloatRoundingPolicy,
    FractionalToIntegerPolicy,
    NumericConversionLimits,
    NumericConversionOptions,
};

/// Tests strict, lossy, and environment-friendly profile contracts.
#[test]
fn test_numeric_conversion_options_profiles() {
    let strict = NumericConversionOptions::strict();
    assert_eq!(strict, NumericConversionOptions::default());
    assert_eq!(
        strict.fractional_to_integer(),
        FractionalToIntegerPolicy::Reject,
    );
    assert_eq!(strict.numeric_to_float(), FloatRoundingPolicy::Exact);
    assert_eq!(strict.text_to_float(), FloatRoundingPolicy::Exact);
    assert_eq!(strict.limits(), &NumericConversionLimits::default());

    let lossy = NumericConversionOptions::lossy();
    assert_eq!(
        lossy.fractional_to_integer(),
        FractionalToIntegerPolicy::Truncate,
    );
    assert_eq!(lossy.numeric_to_float(), FloatRoundingPolicy::NearestEven,);
    assert_eq!(lossy.text_to_float(), FloatRoundingPolicy::NearestEven);
    assert_eq!(lossy.limits(), &NumericConversionLimits::default());

    let env = NumericConversionOptions::env_friendly();
    assert_eq!(
        env.fractional_to_integer(),
        FractionalToIntegerPolicy::Reject,
    );
    assert_eq!(env.numeric_to_float(), FloatRoundingPolicy::Exact);
    assert_eq!(env.text_to_float(), FloatRoundingPolicy::NearestEven);
    assert_eq!(env.limits(), &NumericConversionLimits::default());
}

/// Tests each immutable builder and the exact nested Serde contract.
#[test]
fn test_numeric_conversion_options_builders_and_serde() {
    let limits = NumericConversionLimits::default()
        .with_max_text_bytes(32)
        .with_max_big_integer_digits(8);
    let options = NumericConversionOptions::strict()
        .with_fractional_to_integer(FractionalToIntegerPolicy::Truncate)
        .with_numeric_to_float(FloatRoundingPolicy::NearestEven)
        .with_text_to_float(FloatRoundingPolicy::NearestEven)
        .with_limits(limits);

    assert_eq!(
        options.fractional_to_integer(),
        FractionalToIntegerPolicy::Truncate,
    );
    assert_eq!(options.numeric_to_float(), FloatRoundingPolicy::NearestEven,);
    assert_eq!(options.text_to_float(), FloatRoundingPolicy::NearestEven,);
    assert_eq!(options.limits(), &limits);

    let wire = serde_json::to_string(&options)
        .expect("numeric options should serialize");
    assert_eq!(
        wire,
        r#"{"fractional_to_integer":"truncate","numeric_to_float":"nearest_even","text_to_float":"nearest_even","limits":{"max_text_bytes":32,"max_big_integer_digits":8}}"#,
    );
    assert_eq!(
        serde_json::from_str::<NumericConversionOptions>(&wire)
            .expect("numeric options should deserialize"),
        options,
    );
}

/// Tests that misspelled numeric option fields are rejected.
#[test]
fn test_numeric_conversion_options_reject_unknown_fields() {
    let error = serde_json::from_str::<NumericConversionOptions>(
        r#"{"numeric_to_float":"exact","unexpected":true}"#,
    )
    .expect_err("unknown numeric option fields must be rejected");

    assert!(error.to_string().contains("unknown field `unexpected`"));
}
