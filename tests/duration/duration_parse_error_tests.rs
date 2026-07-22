// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests for duration parsing errors.

use qubit_datatype::{
    DurationParseError,
    DurationUnit,
};

/// Verifies unit errors contain only bounded, copyable diagnostics.
#[test]
fn test_duration_parse_error_does_not_own_source_text() {
    fn assert_copy<T: Copy>() {}

    assert_copy::<DurationParseError>();
    assert!(
        !std::mem::needs_drop::<DurationParseError>(),
        "DurationParseError must not retain owned source text",
    );
    assert_eq!(
        DurationUnit::parse_strict("m")
            .expect_err("strict parsing should reject the minute alias")
            .to_string(),
        "non-canonical duration unit; use `min`",
    );
    assert_eq!(
        DurationUnit::parse_lenient("fortnights")
            .expect_err("unknown unit should be rejected")
            .to_string(),
        "unsupported duration unit",
    );
}

/// Verifies that invalid syntax uses an accurate format-neutral diagnostic.
#[test]
fn test_duration_parse_error_invalid_syntax_display() {
    assert_eq!(
        DurationParseError::InvalidSyntax.to_string(),
        "invalid duration syntax",
    );
}

/// Verifies that resource-limit errors do not include the source value.
#[test]
fn test_duration_parse_error_limit_exceeded_display() {
    assert_eq!(
        DurationParseError::LimitExceeded { maximum: 12 }.to_string(),
        "duration text exceeds the 12-byte limit",
    );
}
