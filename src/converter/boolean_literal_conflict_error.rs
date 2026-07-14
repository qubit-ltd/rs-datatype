// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
// =============================================================================
//! Boolean literal conflict error.

use std::error::Error;
use std::fmt;

/// Error returned when true and false literal sets overlap.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BooleanLiteralConflictError;

impl fmt::Display for BooleanLiteralConflictError {
    /// Formats a value-free conflict diagnostic.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("boolean true and false literals overlap")
    }
}

impl Error for BooleanLiteralConflictError {}
