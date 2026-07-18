// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests for Duration output rounding policy wire behavior.

use qubit_datatype::converter::DurationRoundingPolicy;

/// Tests the default and stable snake-case Serde names.
#[test]
fn test_duration_rounding_policy_serde_and_default() {
    assert_eq!(
        DurationRoundingPolicy::default(),
        DurationRoundingPolicy::Reject,
    );
    assert_eq!(
        serde_json::to_string(&DurationRoundingPolicy::Reject)
            .expect("reject policy should serialize"),
        r#""reject""#,
    );
    assert_eq!(
        serde_json::from_str::<DurationRoundingPolicy>(r#""half_up""#)
            .expect("half-up policy should deserialize"),
        DurationRoundingPolicy::HalfUp,
    );
}
