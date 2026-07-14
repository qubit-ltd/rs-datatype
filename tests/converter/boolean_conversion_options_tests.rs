// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # BooleanConversionOptions Unit Tests
//!
//! Tests for string-to-boolean conversion options.

use qubit_datatype::converter::{
    BooleanConversionOptions,
    BooleanNumericPolicy,
};

/// Test boolean option literals and case-sensitive parsing.
#[test]
fn test_boolean_conversion_options_cover_literal_branches() {
    let env_options = BooleanConversionOptions::env_friendly();
    assert_eq!(env_options.parse("YES"), Some(true));
    assert_eq!(env_options.parse("off"), Some(false));
    assert_eq!(env_options.parse(" YES "), None);
    assert_eq!(env_options.parse("maybe"), None);

    let case_sensitive = BooleanConversionOptions::strict()
        .with_case_sensitive(true)
        .expect("case-sensitive literals should remain disjoint")
        .with_true_literal("Enabled")
        .expect("Enabled should not conflict")
        .with_false_literal("Disabled")
        .expect("Disabled should not conflict");

    assert_eq!(case_sensitive.parse("Enabled"), Some(true));
    assert_eq!(case_sensitive.parse("enabled"), None);
    assert_eq!(case_sensitive.parse("Disabled"), Some(false));
    assert_eq!(case_sensitive.parse("disabled"), None);
}

/// Test every public mutation preserves disjoint literal sets.
#[test]
fn test_boolean_conversion_options_builders_preserve_literal_invariant() {
    assert!(
        BooleanConversionOptions::strict()
            .with_false_literal("TRUE")
            .is_err(),
    );
    assert!(
        BooleanConversionOptions::try_new(
            vec!["enabled".to_string()],
            vec!["ENABLED".to_string()],
            true,
            BooleanNumericPolicy::ZeroOrOne,
        )
        .expect("case-sensitive literals are distinct")
        .with_case_sensitive(false)
        .is_err(),
    );
}

/// Test validated construction and Serde rejection of literal conflicts.
#[test]
fn test_boolean_conversion_options_reject_literal_conflicts() {
    assert!(
        BooleanConversionOptions::try_new(
            vec!["enabled".to_string()],
            vec!["ENABLED".to_string()],
            false,
            BooleanNumericPolicy::ZeroOrOne,
        )
        .is_err(),
    );
    assert!(
        serde_json::from_str::<BooleanConversionOptions>(
            r#"{"true_literals":["yes"],"false_literals":["YES"]}"#,
        )
        .is_err(),
    );
}

/// Test default literal identity and options Serde round-trip.
#[test]
fn test_boolean_conversion_options_serde_and_defaults() {
    let defaults = BooleanConversionOptions::default();
    assert_eq!(
        BooleanConversionOptions::DEFAULT_TRUE_LITERALS,
        &["true"],
    );
    assert_eq!(
        BooleanConversionOptions::DEFAULT_FALSE_LITERALS,
        &["false"],
    );
    assert_eq!(defaults.true_literals(), &["true".to_string()]);
    assert_eq!(defaults.false_literals(), &["false".to_string()]);
    let wire = serde_json::to_string(&defaults)
        .expect("boolean options should serialize");
    assert_eq!(
        serde_json::from_str::<BooleanConversionOptions>(&wire)
            .expect("boolean options should deserialize"),
        defaults,
    );
}
