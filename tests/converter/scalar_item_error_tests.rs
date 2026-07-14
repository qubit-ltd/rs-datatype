// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
// =============================================================================
//! Tests for scalar item iteration errors.

use std::error::Error;

use qubit_datatype::converter::ScalarItemError;

/// Test that a rejected item reports its original unfiltered source index.
#[test]
fn test_scalar_item_error_reports_source_index() {
    let error = ScalarItemError { source_index: 3 };
    let copied = error;

    assert_eq!(copied.source_index, 3);
    assert_eq!(error, copied);
    assert_eq!(
        error.to_string(),
        "blank scalar item rejected at source index 3",
    );
    assert!(error.source().is_none());
}
