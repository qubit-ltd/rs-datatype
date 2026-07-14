// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
// =============================================================================
//! # String Normalization Error
//!
//! Defines policy outcomes produced before target-specific parsing.

use std::error::Error;
use std::fmt;

/// Error returned while normalizing a string source.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringNormalizationError {
    /// The blank source is treated as a missing value.
    Missing,
    /// The blank source is explicitly rejected.
    BlankRejected,
}

impl fmt::Display for StringNormalizationError {
    /// Formats the normalization outcome without including source text.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Missing => formatter.write_str("missing string value"),
            Self::BlankRejected => formatter.write_str("blank string rejected"),
        }
    }
}

impl Error for StringNormalizationError {}
