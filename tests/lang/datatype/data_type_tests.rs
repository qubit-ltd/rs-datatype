/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # DataType Unit Tests
//!
//! Tests for data type enum functionality.
//!
//! # Author
//!
//! Haixing Hu

use qubit_datatype::DataType;
use std::collections::HashSet;
use std::str::FromStr;

/// Returns all DataType variants for exhaustive tests.
fn all_data_types() -> [DataType; 27] {
    [
        DataType::Bool,
        DataType::Char,
        DataType::Int8,
        DataType::Int16,
        DataType::Int32,
        DataType::Int64,
        DataType::Int128,
        DataType::UInt8,
        DataType::UInt16,
        DataType::UInt32,
        DataType::UInt64,
        DataType::UInt128,
        DataType::Float32,
        DataType::Float64,
        DataType::String,
        DataType::Date,
        DataType::Time,
        DataType::DateTime,
        DataType::Instant,
        DataType::BigInteger,
        DataType::BigDecimal,
        DataType::IntSize,
        DataType::UIntSize,
        DataType::Duration,
        DataType::Url,
        DataType::StringMap,
        DataType::Json,
    ]
}

/// Ensure `as_str()` mapping is exhaustive, unique, and consistent with Display.
#[test]
fn test_data_type_as_str_display_consistency_and_uniqueness() {
    let mut names = HashSet::new();
    for data_type in all_data_types() {
        let name = data_type.as_str();
        assert!(
            names.insert(name),
            "Duplicate as_str() value found: {}",
            name
        );
        assert_eq!(data_type.to_string(), name);
    }
    assert_eq!(names.len(), 27);
}

/// Test DataType::as_str method for all data types
#[test]
fn test_data_type_as_str_all_types() {
    // Boolean type
    assert_eq!(DataType::Bool.as_str(), "bool");

    // Character type
    assert_eq!(DataType::Char.as_str(), "char");

    // Signed integer types
    assert_eq!(DataType::Int8.as_str(), "int8");
    assert_eq!(DataType::Int16.as_str(), "int16");
    assert_eq!(DataType::Int32.as_str(), "int32");
    assert_eq!(DataType::Int64.as_str(), "int64");
    assert_eq!(DataType::Int128.as_str(), "int128");

    // Unsigned integer types
    assert_eq!(DataType::UInt8.as_str(), "uint8");
    assert_eq!(DataType::UInt16.as_str(), "uint16");
    assert_eq!(DataType::UInt32.as_str(), "uint32");
    assert_eq!(DataType::UInt64.as_str(), "uint64");
    assert_eq!(DataType::UInt128.as_str(), "uint128");

    // Floating point types
    assert_eq!(DataType::Float32.as_str(), "float32");
    assert_eq!(DataType::Float64.as_str(), "float64");

    // String type
    assert_eq!(DataType::String.as_str(), "string");

    // Date and time types
    assert_eq!(DataType::Date.as_str(), "date");
    assert_eq!(DataType::Time.as_str(), "time");
    assert_eq!(DataType::DateTime.as_str(), "datetime");
    assert_eq!(DataType::Instant.as_str(), "instant");

    // Big number types
    assert_eq!(DataType::BigInteger.as_str(), "biginteger");
    assert_eq!(DataType::BigDecimal.as_str(), "bigdecimal");
}

/// Test DataType Display trait implementation
#[test]
fn test_data_type_display_all_types() {
    // Boolean type
    assert_eq!(DataType::Bool.to_string(), "bool");

    // Character type
    assert_eq!(DataType::Char.to_string(), "char");

    // Signed integer types
    assert_eq!(DataType::Int8.to_string(), "int8");
    assert_eq!(DataType::Int16.to_string(), "int16");
    assert_eq!(DataType::Int32.to_string(), "int32");
    assert_eq!(DataType::Int64.to_string(), "int64");
    assert_eq!(DataType::Int128.to_string(), "int128");

    // Unsigned integer types
    assert_eq!(DataType::UInt8.to_string(), "uint8");
    assert_eq!(DataType::UInt16.to_string(), "uint16");
    assert_eq!(DataType::UInt32.to_string(), "uint32");
    assert_eq!(DataType::UInt64.to_string(), "uint64");
    assert_eq!(DataType::UInt128.to_string(), "uint128");

    // Floating point types
    assert_eq!(DataType::Float32.to_string(), "float32");
    assert_eq!(DataType::Float64.to_string(), "float64");

    // String type
    assert_eq!(DataType::String.to_string(), "string");

    // Date and time types
    assert_eq!(DataType::Date.to_string(), "date");
    assert_eq!(DataType::Time.to_string(), "time");
    assert_eq!(DataType::DateTime.to_string(), "datetime");
    assert_eq!(DataType::Instant.to_string(), "instant");

    // Big number types
    assert_eq!(DataType::BigInteger.to_string(), "biginteger");
    assert_eq!(DataType::BigDecimal.to_string(), "bigdecimal");
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
    // Ensure all types can be properly Debug output
    let types = all_data_types();

    for dt in types {
        let debug_str = format!("{:?}", dt);
        assert!(!debug_str.is_empty(), "Debug output should not be empty");
    }
}

// ============================================================================
// v0.4.0 新增类型测试
// ============================================================================

/// Test as_str for v0.4.0 new types
#[test]
fn test_data_type_as_str_v040_types() {
    assert_eq!(DataType::IntSize.as_str(), "intsize");
    assert_eq!(DataType::UIntSize.as_str(), "uintsize");
    assert_eq!(DataType::Duration.as_str(), "duration");
    assert_eq!(DataType::Url.as_str(), "url");
    assert_eq!(DataType::StringMap.as_str(), "stringmap");
    assert_eq!(DataType::Json.as_str(), "json");
}

/// Test Display for v0.4.0 new types
#[test]
fn test_data_type_display_v040_types() {
    assert_eq!(DataType::IntSize.to_string(), "intsize");
    assert_eq!(DataType::UIntSize.to_string(), "uintsize");
    assert_eq!(DataType::Duration.to_string(), "duration");
    assert_eq!(DataType::Url.to_string(), "url");
    assert_eq!(DataType::StringMap.to_string(), "stringmap");
    assert_eq!(DataType::Json.to_string(), "json");
}

#[test]
fn test_data_type_from_str_all_types() {
    assert_eq!(DataType::from_str("bool").unwrap(), DataType::Bool);
    assert_eq!(DataType::from_str("char").unwrap(), DataType::Char);
    assert_eq!(DataType::from_str("Int32").unwrap(), DataType::Int32);
    assert_eq!(
        DataType::from_str("stringmap").unwrap(),
        DataType::StringMap
    );
    assert_eq!(DataType::from_str("json").unwrap(), DataType::Json);
}

#[test]
fn test_data_type_from_str_with_case_insensitive_match() {
    assert_eq!(DataType::from_str("UINT64").unwrap(), DataType::UInt64);
    assert_eq!(
        DataType::from_str("BigDecimal").unwrap(),
        DataType::BigDecimal
    );
}

#[test]
fn test_data_type_from_str_case_insensitive_for_all_variants() {
    for data_type in all_data_types() {
        let canonical = data_type.as_str();
        let uppercase = canonical.to_ascii_uppercase();
        assert_eq!(DataType::from_str(&uppercase).unwrap(), data_type);

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
        assert_eq!(DataType::from_str(&alternating_case).unwrap(), data_type);
    }
}

#[test]
fn test_data_type_equality_v040_types() {
    assert_eq!(DataType::IntSize, DataType::IntSize);
    assert_ne!(DataType::IntSize, DataType::UIntSize);
    assert_ne!(DataType::IntSize, DataType::Int64);

    assert_eq!(DataType::UIntSize, DataType::UIntSize);
    assert_ne!(DataType::UIntSize, DataType::UInt64);

    assert_eq!(DataType::Duration, DataType::Duration);
    assert_ne!(DataType::Duration, DataType::Int64);

    assert_eq!(DataType::Url, DataType::Url);
    assert_ne!(DataType::Url, DataType::String);

    assert_eq!(DataType::StringMap, DataType::StringMap);
    assert_ne!(DataType::StringMap, DataType::String);

    assert_eq!(DataType::Json, DataType::Json);
    assert_ne!(DataType::Json, DataType::StringMap);
}

/// Test Copy for v0.4.0 new types
#[test]
fn test_data_type_copy_v040_types() {
    let dt = DataType::Duration;
    let dt2 = dt;
    assert_eq!(dt, dt2);

    let dt = DataType::Url;
    let dt2 = dt;
    assert_eq!(dt, dt2);
}

/// Test serde serialization for v0.4.0 new types
///
/// serde uses canonical lowercase protocol strings, aligned with `as_str()`.
#[test]
fn test_data_type_serde_v040_types() {
    use serde_json;

    let cases = [
        (DataType::IntSize, "\"intsize\""),
        (DataType::UIntSize, "\"uintsize\""),
        (DataType::Duration, "\"duration\""),
        (DataType::Url, "\"url\""),
        (DataType::StringMap, "\"stringmap\""),
        (DataType::Json, "\"json\""),
    ];

    for (dt, expected_json) in cases {
        let serialized = serde_json::to_string(&dt).unwrap();
        assert_eq!(
            serialized, expected_json,
            "Serialization mismatch for {:?}",
            dt
        );

        let deserialized: DataType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, dt, "Deserialization mismatch for {:?}", dt);
    }
}

#[test]
fn test_data_type_serde_all_types_use_as_str_protocol() {
    use serde_json;

    for data_type in all_data_types() {
        let serialized = serde_json::to_string(&data_type).unwrap();
        assert_eq!(serialized, format!("\"{}\"", data_type.as_str()));

        let deserialized: DataType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, data_type);
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
        "\"IntSize\"",
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
