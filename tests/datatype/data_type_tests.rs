// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # DataType Unit Tests
//!
//! Tests for data type enum functionality.

use qubit_datatype::DataType;
use std::collections::HashSet;
use std::str::FromStr;

/// Canonical protocol spelling for every data type in declaration order.
const DATA_TYPE_CASES: [(DataType, &str); 25] = [
    (DataType::Bool, "bool"),
    (DataType::Char, "char"),
    (DataType::Int8, "int8"),
    (DataType::Int16, "int16"),
    (DataType::Int32, "int32"),
    (DataType::Int64, "int64"),
    (DataType::Int128, "int128"),
    (DataType::UInt8, "uint8"),
    (DataType::UInt16, "uint16"),
    (DataType::UInt32, "uint32"),
    (DataType::UInt64, "uint64"),
    (DataType::UInt128, "uint128"),
    (DataType::Float32, "float32"),
    (DataType::Float64, "float64"),
    (DataType::String, "string"),
    (DataType::Date, "date"),
    (DataType::Time, "time"),
    (DataType::DateTime, "datetime"),
    (DataType::Instant, "instant"),
    (DataType::BigInteger, "biginteger"),
    (DataType::BigDecimal, "bigdecimal"),
    (DataType::Duration, "duration"),
    (DataType::Url, "url"),
    (DataType::StringMap, "stringmap"),
    (DataType::Json, "json"),
];

/// Verifies that the public catalog contains only platform-independent types.
#[test]
fn test_data_type_catalog_excludes_platform_sized_integers() {
    let names: Vec<&str> = DataType::ALL
        .iter()
        .copied()
        .map(DataType::as_str)
        .collect();

    assert_eq!(names.len(), 25);
    assert!(!names.contains(&"intsize"));
    assert!(!names.contains(&"uintsize"));
}

/// Test the exhaustive variant list and numeric classifications.
#[test]
fn test_data_type_all_and_numeric_classifications() {
    let cases = [
        (DataType::Bool, false, false, false, false, false, false),
        (DataType::Char, false, false, false, false, false, false),
        (DataType::Int8, true, true, true, false, false, false),
        (DataType::Int16, true, true, true, false, false, false),
        (DataType::Int32, true, true, true, false, false, false),
        (DataType::Int64, true, true, true, false, false, false),
        (DataType::Int128, true, true, true, false, false, false),
        (DataType::UInt8, true, true, false, true, false, false),
        (DataType::UInt16, true, true, false, true, false, false),
        (DataType::UInt32, true, true, false, true, false, false),
        (DataType::UInt64, true, true, false, true, false, false),
        (DataType::UInt128, true, true, false, true, false, false),
        (DataType::Float32, true, false, false, false, true, false),
        (DataType::Float64, true, false, false, false, true, false),
        (DataType::String, false, false, false, false, false, false),
        (DataType::Date, false, false, false, false, false, false),
        (DataType::Time, false, false, false, false, false, false),
        (DataType::DateTime, false, false, false, false, false, false),
        (DataType::Instant, false, false, false, false, false, false),
        (DataType::BigInteger, true, false, false, false, false, true),
        (DataType::BigDecimal, true, false, false, false, false, true),
        (DataType::Duration, false, false, false, false, false, false),
        (DataType::Url, false, false, false, false, false, false),
        (
            DataType::StringMap,
            false,
            false,
            false,
            false,
            false,
            false,
        ),
        (DataType::Json, false, false, false, false, false, false),
    ];
    for (data_type, numeric, integer, signed, unsigned, float, big_number) in
        cases
    {
        assert_eq!(data_type.is_numeric(), numeric, "{data_type}");
        assert_eq!(data_type.is_integer(), integer, "{data_type}");
        assert_eq!(data_type.is_signed_integer(), signed, "{data_type}");
        assert_eq!(data_type.is_unsigned_integer(), unsigned, "{data_type}");
        assert_eq!(data_type.is_float(), float, "{data_type}");
        assert_eq!(data_type.is_big_number(), big_number, "{data_type}");
    }
}

/// Verifies the stable catalog, text, parsing, and Serde protocol together.
#[test]
fn test_data_type_protocol_cases() {
    let expected: Vec<DataType> = DATA_TYPE_CASES
        .iter()
        .map(|(data_type, _)| *data_type)
        .collect();
    assert_eq!(DataType::ALL, expected.as_slice());

    for (data_type, canonical) in DATA_TYPE_CASES {
        assert_eq!(data_type.as_str(), canonical);
        assert_eq!(data_type.to_string(), canonical);
        assert_eq!(DataType::from_str(canonical), Ok(data_type));
        assert_eq!(
            DataType::from_str(&canonical.to_ascii_uppercase()),
            Ok(data_type),
        );

        let serialized = serde_json::to_string(&data_type)
            .expect("DataType serialization should succeed");
        assert_eq!(serialized, format!("\"{canonical}\""));
        assert_eq!(
            serde_json::from_str::<DataType>(&serialized)
                .expect("DataType deserialization should succeed"),
            data_type,
        );
    }
}

/// Ensures every canonical protocol spelling is unique.
#[test]
fn test_data_type_protocol_names_are_unique() {
    let mut names = HashSet::new();
    for (_, canonical) in DATA_TYPE_CASES {
        assert!(
            names.insert(canonical),
            "Duplicate as_str() value found: {}",
            canonical,
        );
    }
    assert_eq!(names.len(), DATA_TYPE_CASES.len());
}

/// Test DataType Clone and Copy traits
#[test]
fn test_data_type_clone_copy() {
    let dt1 = DataType::Int32;
    let dt2 = dt1; // Copy
    let dt3 = dt1; // Copy (DataType implements Copy, so direct assignment is sufficient)

    assert_eq!(dt1, dt2);
    assert_eq!(dt1, dt3);
    assert_eq!(dt2, dt3);
}

/// Test DataType PartialEq and Eq traits
#[test]
fn test_data_type_equality() {
    assert_eq!(DataType::Bool, DataType::Bool);
    assert_ne!(DataType::Bool, DataType::Int32);

    assert_eq!(DataType::Int32, DataType::Int32);
    assert_ne!(DataType::Int32, DataType::Int64);

    assert_eq!(DataType::String, DataType::String);
    assert_ne!(DataType::String, DataType::Date);
}

/// Test DataType Debug trait
#[test]
fn test_data_type_debug() {
    for (dt, _) in DATA_TYPE_CASES {
        let debug_str = format!("{:?}", dt);
        assert!(!debug_str.is_empty(), "Debug output should not be empty");
    }
}

#[test]
fn test_data_type_from_str_case_insensitive_for_all_variants() {
    for (data_type, canonical) in DATA_TYPE_CASES {
        let alternating_case: String = canonical
            .chars()
            .enumerate()
            .map(|(index, ch)| {
                if index % 2 == 0 {
                    ch.to_ascii_uppercase()
                } else {
                    ch.to_ascii_lowercase()
                }
            })
            .collect();
        assert_eq!(
            DataType::from_str(&alternating_case)
                .expect("alternating-case data type name should parse"),
            data_type
        );
    }
}

#[test]
fn test_data_type_deserialize_rejects_non_lowercase_values() {
    use serde_json;

    let cases = [
        "\"Bool\"",
        "\"Int32\"",
        "\"DateTime\"",
        "\"BigDecimal\"",
        "\"StringMap\"",
    ];

    for raw in cases {
        let deserialized: Result<DataType, _> = serde_json::from_str(raw);
        assert!(
            deserialized.is_err(),
            "Should reject non-lowercase JSON: {raw}"
        );
    }
}
