// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests for stable data type metadata.

#![allow(deprecated)]

use qubit_datatype::DataType;

/// Verifies metadata names remain aligned with the stable protocol spelling.
#[test]
fn test_data_type_info_names_match_stable_names() {
    for data_type in DataType::ALL.iter().copied() {
        let info = data_type.info();
        assert_eq!(info.data_type(), data_type);
        assert_eq!(info.name(), data_type.as_str());
    }
}

/// Verifies the legacy metadata wrapper delegates semantic categories.
#[test]
fn test_data_type_info_category_matches_direct_api() {
    for data_type in DataType::ALL.iter().copied() {
        assert_eq!(data_type.info().category(), data_type.category());
    }
}
