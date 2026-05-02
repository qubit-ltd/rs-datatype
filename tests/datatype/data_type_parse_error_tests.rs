/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # DataTypeParseError Unit Tests
//!
//! Tests for data type parse errors.

use qubit_datatype::{DataType, DataTypeParseError};
use std::error::Error;
use std::str::FromStr;

#[test]
fn test_data_type_from_str_invalid_returns_parse_error() {
    let error = DataType::from_str("not-a-type").unwrap_err();

    assert_eq!(error.to_string(), "invalid data type: not-a-type");
}

#[test]
fn test_data_type_parse_error_traits() {
    fn assert_error<T: Error>() {}

    assert_error::<DataTypeParseError>();

    let error = DataType::from_str("not-a-type").unwrap_err();
    assert_eq!(error.clone(), error);
    assert!(!format!("{:?}", error).is_empty());
}
