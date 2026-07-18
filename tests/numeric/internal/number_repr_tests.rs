// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests for the private number representation through its public owner.

use std::cmp::Ordering;

use qubit_datatype::{
    NumberRef,
    NumericComparisonPolicy,
};

/// Verifies that distinct internal primitive representations compare exactly.
#[test]
fn test_number_repr_preserves_exact_values() {
    assert_eq!(
        NumberRef::from(42_i8).compare_to(
            NumberRef::from(42_u128),
            NumericComparisonPolicy::Exact,
        ),
        Some(Ordering::Equal),
    );
}
