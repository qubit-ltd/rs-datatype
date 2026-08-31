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

/// Verifies every stable variant maps to its semantic category.
#[test]
fn test_data_type_info_categories_cover_every_variant() {
    let cases = [
        (DataType::Bool, DataTypeCategory::Boolean),
        (DataType::Char, DataTypeCategory::Character),
        (DataType::Int8, DataTypeCategory::SignedInteger),
        (DataType::Int16, DataTypeCategory::SignedInteger),
        (DataType::Int32, DataTypeCategory::SignedInteger),
        (DataType::Int64, DataTypeCategory::SignedInteger),
        (DataType::Int128, DataTypeCategory::SignedInteger),
        (DataType::UInt8, DataTypeCategory::UnsignedInteger),
        (DataType::UInt16, DataTypeCategory::UnsignedInteger),
        (DataType::UInt32, DataTypeCategory::UnsignedInteger),
        (DataType::UInt64, DataTypeCategory::UnsignedInteger),
        (DataType::UInt128, DataTypeCategory::UnsignedInteger),
        (DataType::Float32, DataTypeCategory::FloatingPoint),
        (DataType::Float64, DataTypeCategory::FloatingPoint),
        (DataType::String, DataTypeCategory::Text),
        (DataType::Date, DataTypeCategory::Temporal),
        (DataType::Time, DataTypeCategory::Temporal),
        (DataType::DateTime, DataTypeCategory::Temporal),
        (DataType::Instant, DataTypeCategory::Temporal),
        (DataType::BigInteger, DataTypeCategory::BigNumber),
        (DataType::BigDecimal, DataTypeCategory::BigNumber),
        (DataType::Duration, DataTypeCategory::Duration),
        (DataType::Url, DataTypeCategory::Locator),
        (DataType::StringMap, DataTypeCategory::Structured),
        (DataType::Json, DataTypeCategory::Structured),
    ];

    assert_eq!(cases.len(), DataType::ALL.len());
    for (data_type, expected_category) in cases {
        assert_eq!(data_type.info().category(), expected_category);
    }
}
