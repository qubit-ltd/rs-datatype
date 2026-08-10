// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests for structured data format identifiers.

use qubit_datatype::converter::DataFormat;

/// Test the stable JSON format name and wire representation.
#[test]
fn test_data_format_json_name_and_serde() {
    assert_eq!(DataFormat::Json.as_str(), "json");
    assert_eq!(
        serde_json::to_string(&DataFormat::Json).expect("format should serialize"),
        r#""json""#,
    );
}
