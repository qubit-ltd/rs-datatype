// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests numeric limit deserialization through the public API.

use qubit_datatype::NumericConversionLimits;
use serde_json::from_str;

#[test]
fn test_numeric_wire_deserialization_uses_defaults() {
    let limits = from_str::<NumericConversionLimits>("{}").expect("omitted numeric limits should use their defaults");

    assert_eq!(limits, NumericConversionLimits::default());
}
