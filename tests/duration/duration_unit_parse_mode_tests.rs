// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests for Duration unit parse modes.

use qubit_datatype::DurationUnitParseMode;

/// Tests the default and JSON wire values of Duration unit parse modes.
#[test]
fn test_duration_unit_parse_mode_default_and_serde() {
    assert_eq!(
        DurationUnitParseMode::default(),
        DurationUnitParseMode::Strict,
    );
    assert_eq!(
        serde_json::to_string(&DurationUnitParseMode::Lenient)
            .expect("parse mode should serialize"),
        "\"lenient\"",
    );
    assert_eq!(
        serde_json::from_str::<DurationUnitParseMode>("\"strict\"")
            .expect("strict mode should deserialize"),
        DurationUnitParseMode::Strict,
    );
}
