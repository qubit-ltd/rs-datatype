// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests for internal numeric representation behavior through the public API.

use std::cmp::Ordering;

use qubit_datatype::{
    NumericComparisonPolicy,
    NumericValueRef,
    compare_numeric,
};

/// Verifies that distinct internal primitive representations compare exactly.
#[test]
fn test_numeric_value_representations_preserve_exact_values() {
    assert_eq!(
        compare_numeric(
            NumericValueRef::from(42_i8),
            NumericValueRef::from(42_u128),
            NumericComparisonPolicy::Exact,
        ),
        Some(Ordering::Equal),
    );
}
