// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests for duration parsing errors.

use qubit_datatype::DurationParseError;

/// Verifies that invalid syntax uses an accurate format-neutral diagnostic.
#[test]
fn test_duration_parse_error_invalid_syntax_display() {
    assert_eq!(
        DurationParseError::InvalidSyntax.to_string(),
        "invalid duration syntax",
    );
}
