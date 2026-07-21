// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Arbitrary-precision source to primitive float tests.

#[cfg(feature = "big-decimal")]
use std::str::FromStr;

#[cfg(feature = "big-decimal")]
use bigdecimal::BigDecimal;
#[cfg(feature = "big-integer")]
use num_bigint::BigInt;
#[cfg(feature = "big-integer")]
use qubit_datatype::DataConversionOptions;
#[cfg(any(feature = "big-integer", feature = "big-decimal"))]
use qubit_datatype::DataConverter;

/// Verifies that BigInt-to-f32 conversion applies the target precision policy.
#[cfg(feature = "big-integer")]
#[test]
fn test_bigint_to_f32_applies_target_width_precision() {
    let value = BigInt::from(16_777_217_u32);
    assert!(DataConverter::from(&value).to::<f32>().is_err());
    assert_eq!(
        DataConverter::from(&value)
            .to_with::<f32>(&DataConversionOptions::lossy()),
        Ok(16_777_216.0),
    );
}

/// Verifies exact BigDecimal-to-f32 conversion rejects rounded values.
#[cfg(feature = "big-decimal")]
#[test]
fn test_big_decimal_to_f32_rejects_precision_loss() {
    let value = BigDecimal::from_str("0.1").expect("test decimal");
    assert!(DataConverter::from(&value).to::<f32>().is_err());
}
