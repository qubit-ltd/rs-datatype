// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Exact rational comparison tests.

#[cfg(feature = "big-number")]
use std::cmp::Ordering;
#[cfg(feature = "big-number")]
use std::str::FromStr;

#[cfg(feature = "big-number")]
use bigdecimal::BigDecimal;
#[cfg(feature = "big-number")]
use qubit_datatype::{
    NumericComparisonPolicy,
    NumericValueRef,
    compare_numeric,
};

/// Verifies an exactly binary-representable decimal equals its float value.
#[cfg(feature = "big-number")]
#[test]
fn test_exact_rational_matches_binary_fraction() {
    let decimal = BigDecimal::from_str("0.5").expect("test decimal");
    assert_eq!(
        compare_numeric(
            NumericValueRef::BigDecimal(&decimal),
            NumericValueRef::Float64(0.5),
            NumericComparisonPolicy::Exact,
        ),
        Some(Ordering::Equal),
    );
}
