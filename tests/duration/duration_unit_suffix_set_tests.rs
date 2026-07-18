// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests for Duration unit suffix sets.

use qubit_datatype::DurationUnitSuffixSet;

/// Tests the default and Serde names of supported suffix sets.
#[test]
fn test_duration_unit_suffix_set_default_and_serde() {
    assert_eq!(
        DurationUnitSuffixSet::default(),
        DurationUnitSuffixSet::Extended,
    );
    assert_eq!(
        serde_json::to_string(&DurationUnitSuffixSet::Ascii)
            .expect("ASCII suffix set should serialize"),
        "\"ascii\"",
    );
    assert_eq!(
        serde_json::from_str::<DurationUnitSuffixSet>("\"extended\"")
            .expect("extended suffix set should deserialize"),
        DurationUnitSuffixSet::Extended,
    );
}
