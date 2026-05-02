/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! # Data Type Parse Error
//!
//! Provides the error returned when parsing `DataType` from text fails.
//!

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
