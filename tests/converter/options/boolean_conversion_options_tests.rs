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

use proptest::arbitrary::any;
use proptest::collection;
use proptest::strategy::Just;
use proptest::{
    prop_assert_eq,
    prop_oneof,
    proptest,
};
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

/// Test every field in the environment-friendly boolean profile.
#[test]
fn test_boolean_conversion_options_env_friendly_profile() {
    let options = BooleanConversionOptions::env_friendly();
    assert_eq!(
        options
            .true_literals()
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        ["true", "yes", "on"],
    );
    assert_eq!(
        options
            .false_literals()
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        ["false", "no", "off"],
    );
    assert!(!options.case_sensitive());
    assert_eq!(options.numeric_policy(), BooleanNumericPolicy::ZeroOrOne);
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

/// Verifies duplicates on one side remain valid while cross-side conflicts
/// fail.
#[test]
fn test_boolean_conversion_options_only_reject_cross_set_conflicts() {
    let options = BooleanConversionOptions::try_new(
        vec!["yes".to_owned(), "yes".to_owned()],
        vec!["no".to_owned(), "no".to_owned()],
        false,
        BooleanNumericPolicy::ZeroOrOne,
    )
    .expect("same-side duplicates must remain valid");

    assert_eq!(options.parse("YES"), Some(true));
    assert_eq!(options.parse("NO"), Some(false));
    assert!(
        BooleanConversionOptions::try_new(
            vec!["Enabled".to_owned()],
            vec!["enabled".to_owned()],
            false,
            BooleanNumericPolicy::ZeroOrOne,
        )
        .is_err(),
    );
    assert!(
        BooleanConversionOptions::try_new(
            vec!["Ä".to_owned()],
            vec!["ä".to_owned()],
            false,
            BooleanNumericPolicy::ZeroOrOne,
        )
        .is_ok(),
    );
}

/// Characterizes validation for large disjoint literal collections.
#[test]
fn test_boolean_conversion_options_validate_large_disjoint_sets() {
    let true_literals =
        (0..4096).map(|index| format!("true-{index}")).collect();
    let false_literals =
        (0..4096).map(|index| format!("false-{index}")).collect();

    assert!(
        BooleanConversionOptions::try_new(
            true_literals,
            false_literals,
            false,
            BooleanNumericPolicy::ZeroOrOne,
        )
        .is_ok(),
    );
}

/// Test default literal identity and options Serde round-trip.
#[test]
fn test_boolean_conversion_options_serde_and_defaults() {
    let defaults = BooleanConversionOptions::default();
    assert_eq!(BooleanConversionOptions::DEFAULT_TRUE_LITERALS, &["true"],);
    assert_eq!(BooleanConversionOptions::DEFAULT_FALSE_LITERALS, &["false"],);
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

/// Test that misspelled Boolean option fields are rejected.
#[test]
fn test_boolean_conversion_options_reject_unknown_fields() {
    let error = serde_json::from_str::<BooleanConversionOptions>(
        r#"{"case_sensitive":true,"unexpected":false}"#,
    )
    .expect_err("unknown Boolean option fields must be rejected");

    assert!(error.to_string().contains("unknown field `unexpected`"));
}

proptest! {
    /// Test that every successfully constructed literal configuration remains
    /// valid after a Serde round trip.
    #[test]
    fn test_boolean_conversion_options_validated_round_trip_property(
        true_literals in collection::vec("[A-Za-z0-9]{0,8}", 0..8),
        false_literals in collection::vec("[A-Za-z0-9]{0,8}", 0..8),
        case_sensitive in any::<bool>(),
        numeric_policy in prop_oneof![
            Just(BooleanNumericPolicy::ZeroOrOne),
            Just(BooleanNumericPolicy::NonZero),
            Just(BooleanNumericPolicy::Reject),
        ],
    ) {
        let Ok(options) = BooleanConversionOptions::try_new(
            true_literals,
            false_literals,
            case_sensitive,
            numeric_policy,
        ) else {
            return Ok(());
        };
        let wire = serde_json::to_string(&options)
            .expect("validated Boolean options should serialize");
        let restored: BooleanConversionOptions = serde_json::from_str(&wire)
            .expect("validated Boolean options should deserialize");

        prop_assert_eq!(&restored, &options);
        prop_assert_eq!(restored.validate(), Ok(()));
    }
}
