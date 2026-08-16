// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # StringConversionPolicy Unit Tests
//!
//! Tests for string-source normalization options.

use qubit_datatype::converter::BlankStringPolicy;
use qubit_datatype::converter::StringConversionPolicy;
use qubit_datatype::converter::StringNormalizationError;

/// Test the environment-variable string profile.
#[test]
fn test_string_conversion_policy_env_friendly_profile() {
    let options = StringConversionPolicy::env_friendly();
    assert!(options.trim());
    assert_eq!(
        options.blank_string_policy(),
        BlankStringPolicy::TreatAsMissing,
    );
}

/// Test that misspelled string policy fields are rejected.
#[test]
fn test_string_conversion_policy_reject_unknown_fields() {
    let error = serde_json::from_str::<StringConversionPolicy>(
        r#"{"trim":true,"unexpected":false}"#,
    )
    .expect_err("unknown string policy fields must be rejected");

    assert!(error.to_string().contains("unknown field `unexpected`"));
}

/// Test string policy policy branches.
#[test]
fn test_string_conversion_policy_cover_policy_branches() {
    let preserved = StringConversionPolicy::builder()
        .trim(false)
        .blank_string_policy(BlankStringPolicy::Preserve)
        .build()
        .normalize("   ")
        .expect("blank string should be preserved");
    assert_eq!(preserved, "   ");

    let rejected = StringConversionPolicy::builder()
        .blank_string_policy(BlankStringPolicy::Reject)
        .build()
        .normalize("   ");
    assert_eq!(rejected, Err(StringNormalizationError::BlankRejected));

    let missing = StringConversionPolicy::builder()
        .trim(true)
        .blank_string_policy(BlankStringPolicy::TreatAsMissing)
        .build()
        .normalize("   ");
    assert_eq!(missing, Err(StringNormalizationError::Missing));
}

/// Test that normalization borrows the original input without allocation.
#[test]
fn test_string_conversion_policy_normalize_returns_borrowed_slice() {
    let input = String::from("  value  ");
    let normalized = StringConversionPolicy::builder()
        .trim(true)
        .build()
        .normalize(&input)
        .expect("non-blank input should normalize");

    assert_eq!(normalized, "value");
    assert_eq!(normalized.as_ptr(), input[2..].as_ptr());
}

/// Test the structured blank rejection category.
#[test]
fn test_string_conversion_policy_rejects_blank_structurally() {
    let error = StringConversionPolicy::builder()
        .blank_string_policy(BlankStringPolicy::Reject)
        .build()
        .normalize("   ")
        .expect_err("blank text should be rejected");

    assert_eq!(error, StringNormalizationError::BlankRejected);
}

/// Test the normalization result used for blank-as-missing policy.
#[test]
fn test_string_conversion_policy_reports_missing_normalization() {
    let error = StringConversionPolicy::builder()
        .trim(true)
        .blank_string_policy(BlankStringPolicy::TreatAsMissing)
        .build()
        .normalize("   ")
        .expect_err("blank text should be treated as missing");

    assert_eq!(error, StringNormalizationError::Missing);
}

/// Test the optional normalization adapter for value-or-missing flows.
#[test]
fn test_string_conversion_policy_normalize_optional() {
    let input = String::from("  value  ");
    let normalized = StringConversionPolicy::builder()
        .trim(true)
        .build()
        .normalize_optional(&input)
        .expect("non-blank input should normalize");
    assert_eq!(normalized, Some("value"));
    assert_eq!(
        normalized
            .expect("normalized value should be present")
            .as_ptr(),
        input[2..].as_ptr(),
    );

    let missing = StringConversionPolicy::builder()
        .blank_string_policy(BlankStringPolicy::TreatAsMissing)
        .build()
        .normalize_optional("   ");
    assert_eq!(missing, Ok(None));

    let rejected = StringConversionPolicy::builder()
        .blank_string_policy(BlankStringPolicy::Reject)
        .build()
        .normalize_optional("   ");
    assert_eq!(rejected, Err(StringNormalizationError::BlankRejected));
}
