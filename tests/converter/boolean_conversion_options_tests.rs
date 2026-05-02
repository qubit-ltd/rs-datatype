/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! # BooleanConversionOptions Unit Tests
//!
//! Tests for string-to-boolean conversion options.
//!

use qubit_datatype::converter::BooleanConversionOptions;

/// Test boolean option literals and case-sensitive parsing.
#[test]
fn test_boolean_conversion_options_cover_literal_branches() {
    let env_options = BooleanConversionOptions::env_friendly();
    assert_eq!(env_options.parse(" YES "), Some(true));
    assert_eq!(env_options.parse(" off "), Some(false));
    assert_eq!(env_options.parse("maybe"), None);

    let case_sensitive = BooleanConversionOptions::strict()
        .with_case_sensitive(true)
        .with_true_literal("Enabled")
        .with_false_literal("Disabled");

    assert_eq!(case_sensitive.parse("Enabled"), Some(true));
    assert_eq!(case_sensitive.parse("enabled"), None);
    assert_eq!(case_sensitive.parse("Disabled"), Some(false));
    assert_eq!(case_sensitive.parse("disabled"), None);
}
