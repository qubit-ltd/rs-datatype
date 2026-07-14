// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
// =============================================================================
//! Tests for Duration overflow errors.

use qubit_datatype::DurationOverflowError;

/// Test the stable, value-free Duration overflow diagnostic.
#[test]
fn test_duration_overflow_error_display() {
    let error = DurationOverflowError;
    assert_eq!(
        error.to_string(),
        "duration exceeds the range supported by std::time::Duration",
    );
    assert_eq!(error.clone(), error);
}
