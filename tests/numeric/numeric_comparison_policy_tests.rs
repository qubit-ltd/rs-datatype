// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_datatype::NumericComparisonPolicy;

/// Verifies the stable default and serialized policy vocabulary.
#[test]
fn test_numeric_comparison_policy_default_and_serde() {
    assert_eq!(
        NumericComparisonPolicy::default(),
        NumericComparisonPolicy::Exact
    );
    assert_eq!(
        serde_json::to_string(&NumericComparisonPolicy::Approximate).unwrap(),
        "\"approximate\""
    );
}
