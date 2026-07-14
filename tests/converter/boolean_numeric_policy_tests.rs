// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
// =============================================================================
//! Tests for boolean numeric policy wire behavior.

use qubit_datatype::converter::BooleanNumericPolicy;

/// Test defaults and stable snake-case Serde names.
#[test]
fn test_boolean_numeric_policy_serde_and_default() {
    assert_eq!(
        BooleanNumericPolicy::default(),
        BooleanNumericPolicy::ZeroOrOne
    );
    let cases = [
        (BooleanNumericPolicy::ZeroOrOne, r#""zero_or_one""#),
        (BooleanNumericPolicy::NonZero, r#""non_zero""#),
        (BooleanNumericPolicy::Reject, r#""reject""#),
    ];
    for (policy, wire) in cases {
        assert_eq!(
            serde_json::to_string(&policy).expect("policy should serialize"),
            wire,
        );
        assert_eq!(
            serde_json::from_str::<BooleanNumericPolicy>(wire)
                .expect("policy should deserialize"),
            policy,
        );
    }
}
