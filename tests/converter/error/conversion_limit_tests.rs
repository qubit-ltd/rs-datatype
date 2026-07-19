// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests for numeric conversion resource limit descriptions.

use qubit_datatype::ConversionLimit;

/// Tests stable, source-value-free resource limit diagnostics.
#[test]
fn test_conversion_limit_display() {
    assert_eq!(
        ConversionLimit::NumericTextBytes { maximum: 12 }.to_string(),
        "numeric text exceeds the 12-byte limit",
    );
    assert_eq!(
        ConversionLimit::BigIntegerDigits { maximum: 34 }.to_string(),
        "BigInteger result exceeds the 34-decimal-digit limit",
    );
    assert_eq!(
        ConversionLimit::DurationTextBytes { maximum: 56 }.to_string(),
        "duration text exceeds the 56-byte limit",
    );
}
