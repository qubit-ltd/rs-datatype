// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests for suffixless Duration string policies.

use qubit_datatype::{
    DurationUnit,
    SuffixlessDurationPolicy,
};

/// Tests the default and Serde representation of suffixless policies.
#[test]
fn test_suffixless_duration_policy_default_and_serde() {
    let default = SuffixlessDurationPolicy::default();
    assert_eq!(
        default,
        SuffixlessDurationPolicy::Assume(DurationUnit::Milliseconds),
    );

    let reject = serde_json::to_string(&SuffixlessDurationPolicy::Reject)
        .expect("reject policy should serialize");
    assert_eq!(reject, "\"reject\"");

    let assume = serde_json::to_string(&SuffixlessDurationPolicy::Assume(
        DurationUnit::Seconds,
    ))
    .expect("assume policy should serialize");
    assert_eq!(assume, r#"{"assume":"seconds"}"#);
    assert_eq!(
        serde_json::from_str::<SuffixlessDurationPolicy>(&assume)
            .expect("assume policy should deserialize"),
        SuffixlessDurationPolicy::Assume(DurationUnit::Seconds),
    );
}
