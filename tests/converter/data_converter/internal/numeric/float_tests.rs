// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Floating-point conversion regression tests.

use qubit_datatype::{
    DataConversionOptions,
    DataConverter,
    InvalidValueReason,
};

/// Verifies that lossy text-to-f32 conversion rounds directly at target width.
#[test]
fn test_lossy_text_to_f32_avoids_double_rounding() {
    let options = DataConversionOptions::lossy();
    let converted = DataConverter::from("1.0000000596046448")
        .to_with::<f32>(&options)
        .expect("finite decimal text should convert lossily to f32");
    assert_eq!(converted.to_bits(), 0x3f80_0001);
}

/// Verifies that redundant decimal zeros do not defeat exact conversion.
#[test]
fn test_exact_float_text_normalizes_redundant_zeros() {
    let source =
        DataConverter::from("1.000000000000000000000000000000000000000");
    assert_eq!(
        source
            .to::<f32>()
            .expect("redundant zeros should remain exact for f32"),
        1.0,
    );
    assert_eq!(
        source
            .to::<f64>()
            .expect("redundant zeros should remain exact for f64"),
        1.0,
    );
}

/// Verifies f32 text conversion distinguishes range and precision failures.
#[test]
fn test_text_to_f32_classifies_range_and_precision_failures() {
    assert!(matches!(
        DataConverter::from("1e1000").to::<f32>(),
        Err(conversion_error) if matches!(conversion_error.reason(), Some(InvalidValueReason::OutOfRange)
    )));
    assert!(matches!(
        DataConverter::from("0.1").to::<f32>(),
        Err(conversion_error) if matches!(conversion_error.reason(), Some(InvalidValueReason::PrecisionLoss)
    )));
}

/// Verifies primitive non-finite f64 values retain their class in f32.
#[test]
fn test_non_finite_f64_to_f32_preserves_class() {
    assert!(
        DataConverter::from(f64::NAN)
            .to::<f32>()
            .expect("NaN should convert")
            .is_nan()
    );
    assert_eq!(
        DataConverter::from(f64::INFINITY).to::<f32>(),
        Ok(f32::INFINITY),
    );
    assert_eq!(
        DataConverter::from(f64::NEG_INFINITY).to::<f32>(),
        Ok(f32::NEG_INFINITY),
    );
}
