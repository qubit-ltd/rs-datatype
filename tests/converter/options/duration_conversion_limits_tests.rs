// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests for duration conversion limits.

use qubit_datatype::DurationTextOptions;
use qubit_datatype::converter::DurationConversionLimits;

#[test]
fn test_duration_conversion_limits_defaults_builder_and_wire() {
    let defaults = DurationConversionLimits::default();
    assert_eq!(defaults.max_text_bytes(), DurationTextOptions::DEFAULT_MAX_TEXT_BYTES);

    let limits = DurationConversionLimits::builder().max_text_bytes(4_096).build();
    assert_eq!(
        serde_json::to_string(&limits).expect("duration limits should serialize"),
        r#"{"max_text_bytes":4096}"#
    );
    assert_eq!(
        serde_json::from_str::<DurationConversionLimits>(r#"{"max_text_bytes":4096}"#)
            .expect("duration limits should deserialize"),
        limits
    );
}
