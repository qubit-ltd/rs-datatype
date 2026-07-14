// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
// =============================================================================
//! Tests for boolean literal conflict diagnostics.

use std::error::Error;

use qubit_datatype::converter::BooleanLiteralConflictError;

/// Test the conflict error's stable, value-free diagnostic contract.
#[test]
fn test_boolean_literal_conflict_error_reports_overlap() {
    let error = BooleanLiteralConflictError;
    let copied = error;
    let cloned = error;

    assert_eq!(copied, cloned);
    assert_eq!(error.to_string(), "boolean true and false literals overlap");
    assert!(error.source().is_none());
    assert_eq!(format!("{error:?}"), "BooleanLiteralConflictError");
}
