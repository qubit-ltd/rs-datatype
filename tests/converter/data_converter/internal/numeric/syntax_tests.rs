// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Numeric syntax validation tests.

use qubit_datatype::{
    DataConverter,
    InvalidValueReason,
};

/// Verifies malformed signs are classified as invalid numeric syntax.
#[test]
fn test_numeric_text_rejects_duplicate_signs() {
    assert!(matches!(
        DataConverter::from("--1").to::<f64>(),
        Err(conversion_error) if matches!(conversion_error.reason(), Some(InvalidValueReason::InvalidSyntax { .. })
    )));
}
