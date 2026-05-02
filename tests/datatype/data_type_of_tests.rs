/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # DataTypeOf Unit Tests
//!
//! Tests for compile-time Rust type to `DataType` mapping.

use qubit_datatype::DataType;
use qubit_datatype::DataTypeOf;

/// Test DataTypeOf for v0.4.0 new types
#[test]
fn test_data_type_of_v040_types() {
    use std::collections::HashMap;
    use std::time::Duration;

    assert_eq!(isize::DATA_TYPE, DataType::IntSize);
    assert_eq!(usize::DATA_TYPE, DataType::UIntSize);
    assert_eq!(Duration::DATA_TYPE, DataType::Duration);
    assert_eq!(url::Url::DATA_TYPE, DataType::Url);
    assert_eq!(HashMap::<String, String>::DATA_TYPE, DataType::StringMap);
    assert_eq!(serde_json::Value::DATA_TYPE, DataType::Json);
}

// ============================================================================
// url::Url + DataTypeOf（crate 内绑定约定）
// ============================================================================

#[test]
fn test_data_type_of_url_generic() {
    fn data_type_of<T: DataTypeOf>() -> DataType {
        T::DATA_TYPE
    }

    assert_eq!(data_type_of::<url::Url>(), DataType::Url);
}

#[test]
fn test_data_type_of_url_distinct_from_string() {
    assert_ne!(url::Url::DATA_TYPE, String::DATA_TYPE);
    assert_eq!(String::DATA_TYPE, DataType::String);
}

#[test]
fn test_data_type_of_url_inferred_from_value() {
    fn mapping_for<T: DataTypeOf>(_sample: &T) -> DataType {
        T::DATA_TYPE
    }

    let https = url::Url::parse("https://example.com/path?x=1#frag").unwrap();
    assert_eq!(mapping_for(&https), DataType::Url);

    let file = url::Url::parse("file:///tmp/a.txt").unwrap();
    assert_eq!(mapping_for(&file), DataType::Url);
}
