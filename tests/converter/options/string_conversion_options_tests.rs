// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # StringConversionOptions Unit Tests
//!
//! Tests for string-source normalization options.

use qubit_datatype::converter::{
    BlankStringPolicy, StringConversionOptions, StringNormalizationError,
};

/// Test the environment-variable string profile.
#[test]
fn test_string_conversion_options_env_friendly_profile() {
    let options = StringConversionOptions::env_friendly();
    assert!(options.trim);
    assert_eq!(
        options.blank_string_policy,
        BlankStringPolicy::TreatAsMissing,
    );
}

/// Test that misspelled string option fields are rejected.
#[test]
fn test_string_conversion_options_reject_unknown_fields() {
    let error =
        serde_json::from_str::<StringConversionOptions>(r#"{"trim":true,"unexpected":false}"#)
            .expect_err("unknown string option fields must be rejected");

    assert!(error.to_string().contains("unknown field `unexpected`"));
}

/// Test string option policy branches.
#[test]
fn test_string_conversion_options_cover_policy_branches() {
    let preserved = StringConversionOptions::default()
        .with_trim(false)
        .with_blank_string_policy(BlankStringPolicy::Preserve)
        .normalize("   ")
        .expect("blank string should be preserved");
    assert_eq!(preserved, "   ");

    let rejected = StringConversionOptions::default()
        .with_blank_string_policy(BlankStringPolicy::Reject)
        .normalize("   ");
    assert_eq!(rejected, Err(StringNormalizationError::BlankRejected));

    let missing = StringConversionOptions::default()
        .with_trim(true)
        .with_blank_string_policy(BlankStringPolicy::TreatAsMissing)
        .normalize("   ");
    assert_eq!(missing, Err(StringNormalizationError::Missing));
}

/// Test that normalization borrows the original input without allocation.
#[test]
fn test_string_conversion_options_normalize_returns_borrowed_slice() {
    let input = String::from("  value  ");
    let normalized = StringConversionOptions::default()
        .with_trim(true)
        .normalize(&input)
        .expect("non-blank input should normalize");

    assert_eq!(normalized, "value");
    assert_eq!(normalized.as_ptr(), input[2..].as_ptr());
}

/// Test the structured blank rejection category.
#[test]
fn test_string_conversion_options_rejects_blank_structurally() {
    let error = StringConversionOptions::default()
        .with_blank_string_policy(BlankStringPolicy::Reject)
        .normalize("   ")
        .expect_err("blank text should be rejected");

    assert_eq!(error, StringNormalizationError::BlankRejected);
}

/// Test the normalization result used for blank-as-missing policy.
#[test]
fn test_string_conversion_options_reports_missing_normalization() {
    let error = StringConversionOptions::default()
        .with_trim(true)
        .with_blank_string_policy(BlankStringPolicy::TreatAsMissing)
        .normalize("   ")
        .expect_err("blank text should be treated as missing");

    assert_eq!(error, StringNormalizationError::Missing);
}
