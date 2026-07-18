// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Exact decimal comparison tests.

#[cfg(feature = "big-number")]
use std::cmp::Ordering;
#[cfg(feature = "big-number")]
use std::str::FromStr;

#[cfg(feature = "big-number")]
use bigdecimal::BigDecimal;
#[cfg(feature = "big-number")]
use qubit_datatype::{NumericComparisonPolicy, NumericValueRef, compare_numeric};

/// Verifies decimal scale does not change exact mathematical equality.
#[cfg(feature = "big-number")]
#[test]
fn test_exact_decimal_ignores_redundant_scale() {
    let decimal = BigDecimal::from_str("1.00").expect("test decimal");
    assert_eq!(
        compare_numeric(
            NumericValueRef::BigDecimal(&decimal),
            NumericValueRef::UInt8(1),
            NumericComparisonPolicy::Exact,
        ),
        Some(Ordering::Equal),
    );
}
