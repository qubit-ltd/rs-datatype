// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests for stable data type metadata.

use qubit_datatype::DataType;
use qubit_datatype::DataTypeCategory;

/// Verifies metadata names remain aligned with the stable protocol spelling.
#[test]
fn test_data_type_info_names_match_stable_names() {
    for data_type in DataType::ALL.iter().copied() {
        let info = data_type.info();
        assert_eq!(info.data_type(), data_type);
        assert_eq!(info.name(), data_type.as_str());
    }
}

/// Verifies representative variants map to their semantic categories.
#[test]
fn test_data_type_info_categories_cover_representative_families() {
    assert_eq!(DataType::Int32.info().category(), DataTypeCategory::SignedInteger,);
    assert_eq!(DataType::UInt64.info().category(), DataTypeCategory::UnsignedInteger,);
    assert_eq!(DataType::DateTime.info().category(), DataTypeCategory::Temporal,);
    assert_eq!(DataType::Json.info().category(), DataTypeCategory::Structured,);
}
