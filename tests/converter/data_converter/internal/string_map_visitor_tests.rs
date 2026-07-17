// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Duplicate-aware string-map deserialization tests.

#[cfg(feature = "json")]
use std::collections::HashMap;

#[cfg(feature = "json")]
use qubit_datatype::DataConverter;

/// Verifies duplicate JSON object keys are rejected for string maps.
#[cfg(feature = "json")]
#[test]
fn test_string_map_rejects_duplicate_json_keys() {
    assert!(
        DataConverter::from(r#"{"key":"first","key":"second"}"#)
            .to::<HashMap<String, String>>()
            .is_err()
    );
}
