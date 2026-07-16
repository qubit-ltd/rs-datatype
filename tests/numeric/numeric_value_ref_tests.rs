// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_datatype::NumericValueRef;

/// Verifies that borrowed numeric values remain cheap to copy.
#[test]
fn test_numeric_value_ref_is_copy() {
    let value = NumericValueRef::UInt128(u128::MAX);
    let copied = value;
    assert!(matches!(copied, NumericValueRef::UInt128(u128::MAX)));
    assert!(matches!(value, NumericValueRef::UInt128(u128::MAX)));
}
