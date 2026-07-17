// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Parsed-number dispatch tests.

#[cfg(feature = "big-number")]
use bigdecimal::BigDecimal;
#[cfg(feature = "big-number")]
use qubit_datatype::DataConverter;

/// Verifies scientific decimal text is preserved by big-decimal conversion.
#[cfg(feature = "big-number")]
#[test]
fn test_scientific_text_converts_to_big_decimal() {
    let converted = DataConverter::from("1.25e2")
        .to::<BigDecimal>()
        .expect("scientific decimal text should convert");
    assert_eq!(converted, BigDecimal::from(125));
}
