/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # DataConversionError Unit Tests
//!
//! Tests for reusable data conversion errors.

use qubit_datatype::DataType;
use qubit_datatype::converter::DataConversionError;

/// Test the conversion error display strings.
#[test]
fn test_data_conversion_error_display() {
    assert_eq!(DataConversionError::NoValue.to_string(), "No value");
    let cloned_error = DataConversionError::NoValue.clone();
    assert_eq!(cloned_error, DataConversionError::NoValue);
    assert!(format!("{:?}", cloned_error).contains("NoValue"));

    assert_eq!(
        DataConversionError::ConversionFailed {
            from: DataType::String,
            to: DataType::Int32,
        }
        .to_string(),
        "Type conversion failed: from string to int32"
    );
    assert_eq!(
        DataConversionError::ConversionError("bad input".to_string()).to_string(),
        "Conversion error: bad input"
    );
    assert_eq!(
        DataConversionError::JsonSerializationError("bad json".to_string()).to_string(),
        "JSON serialization error: bad json"
    );
    assert_eq!(
        DataConversionError::JsonDeserializationError("bad json".to_string()).to_string(),
        "JSON deserialization error: bad json"
    );
}
