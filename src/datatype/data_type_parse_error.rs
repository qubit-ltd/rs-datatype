/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Data Type Parse Error
//!
//! Provides the error returned when parsing `DataType` from text fails.
//!
//! # Author
//!
//! Haixing Hu

use std::fmt;

/// Error returned when parsing a `DataType` from text fails.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataTypeParseError {
    input: String,
}

impl DataTypeParseError {
    pub(crate) fn new(input: &str) -> Self {
        Self {
            input: input.to_string(),
        }
    }
}

impl fmt::Display for DataTypeParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "invalid data type: {}", self.input)
    }
}

impl std::error::Error for DataTypeParseError {}
