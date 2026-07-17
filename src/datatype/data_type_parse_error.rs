// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Data Type Parse Error
//!
//! Provides the error returned when parsing `DataType` from text fails.

use std::fmt;

/// Error returned when parsing a `DataType` from text fails.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataTypeParseError {
    /// The rejected data type name.
    input: String,
}

impl DataTypeParseError {
    /// Creates a parse error that retains the rejected type name.
    ///
    /// The private constructor copies `input` so the error can outlive the
    /// parsed string.
    pub(crate) fn new(input: &str) -> Self {
        Self {
            input: input.to_string(),
        }
    }
}

impl fmt::Display for DataTypeParseError {
    #[inline(always)]
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "invalid data type: {}", self.input)
    }
}

impl std::error::Error for DataTypeParseError {}
