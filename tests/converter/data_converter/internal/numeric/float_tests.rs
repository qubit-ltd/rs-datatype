// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Primitive floating-point conversion regression tests.

use qubit_datatype::DataConverter;

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
