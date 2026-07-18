// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Fixed-width exact comparison tests.

use std::cmp::Ordering;

use qubit_datatype::{NumericComparisonPolicy, NumericValueRef, compare_numeric};

/// Verifies fixed signed and unsigned values retain exact ordering.
#[test]
fn test_fixed_numeric_compares_signed_and_unsigned_values() {
    assert_eq!(
        compare_numeric(
            NumericValueRef::Int64(-1),
            NumericValueRef::UInt64(0),
            NumericComparisonPolicy::Exact,
        ),
        Some(Ordering::Less),
    );
}
