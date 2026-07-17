// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Primitive and text to arbitrary-precision number tests.

#[cfg(feature = "big-number")]
use std::str::FromStr;

#[cfg(feature = "big-number")]
use bigdecimal::BigDecimal;
#[cfg(feature = "big-number")]
use num_bigint::BigInt;
#[cfg(feature = "big-number")]
use qubit_datatype::DataConverter;

/// Verifies primitive integer conversion preserves the complete value.
#[cfg(feature = "big-number")]
#[test]
fn test_integer_to_bigint_preserves_value() {
    assert_eq!(
        DataConverter::from(i128::MIN).to::<BigInt>(),
        Ok(BigInt::from(i128::MIN)),
    );
}

/// Verifies decimal text conversion preserves the represented value.
#[cfg(feature = "big-number")]
#[test]
fn test_text_to_big_decimal_preserves_value() {
    let expected = BigDecimal::from_str("123.50").expect("valid test decimal");
    assert_eq!(
        DataConverter::from("123.50").to::<BigDecimal>(),
        Ok(expected),
    );
}
