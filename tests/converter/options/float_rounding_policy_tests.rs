// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests for floating-point rounding policy wire behavior.

use qubit_datatype::converter::FloatRoundingPolicy;

/// Tests the default and stable snake-case Serde names.
#[test]
fn test_float_rounding_policy_serde_and_default() {
    assert_eq!(FloatRoundingPolicy::default(), FloatRoundingPolicy::Exact,);
    assert_eq!(
        serde_json::to_string(&FloatRoundingPolicy::Exact)
            .expect("exact policy should serialize"),
        r#""exact""#,
    );
    assert_eq!(
        serde_json::from_str::<FloatRoundingPolicy>(r#""nearest_even""#)
            .expect("nearest-even policy should deserialize"),
        FloatRoundingPolicy::NearestEven,
    );
}
