// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Fixed-width integer conversion tests.

use qubit_datatype::{
    DataConversionOptions,
    DataConverter,
};

/// Verifies exact and lossy integer-to-f32 conversion at the mantissa boundary.
#[test]
fn test_integer_to_f32_checks_target_mantissa() {
    let source = DataConverter::from(16_777_217_u32);
    assert!(source.to::<f32>().is_err());
    assert_eq!(
        source.to_with::<f32>(&DataConversionOptions::lossy()),
        Ok(16_777_216.0),
    );
}
