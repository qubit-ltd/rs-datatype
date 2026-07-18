// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests for fractional-to-integer conversion policy wire behavior.

use qubit_datatype::converter::FractionalToIntegerPolicy;

/// Tests the default and stable snake-case Serde names.
#[test]
fn test_fractional_to_integer_policy_serde_and_default() {
    assert_eq!(
        FractionalToIntegerPolicy::default(),
        FractionalToIntegerPolicy::Reject,
    );
    assert_eq!(
        serde_json::to_string(&FractionalToIntegerPolicy::Reject)
            .expect("reject policy should serialize"),
        r#""reject""#,
    );
    assert_eq!(
        serde_json::from_str::<FractionalToIntegerPolicy>(r#""truncate""#)
            .expect("truncate policy should deserialize"),
        FractionalToIntegerPolicy::Truncate,
    );
}
