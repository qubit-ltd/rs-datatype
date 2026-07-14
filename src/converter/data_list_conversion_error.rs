// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Data List Conversion Error
//!
//! Defines errors returned by reusable batch data conversions.

use std::error::Error;
use std::fmt;

use super::data_conversion_error::DataConversionError;

/// Error type returned by reusable batch data conversions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataListConversionError {
    /// Zero-based index in the original source, before any items were skipped.
    pub source_index: usize,
    /// Original single-value conversion error.
    pub source: DataConversionError,
}

impl fmt::Display for DataListConversionError {
    /// Formats the list conversion error with the original source index.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Data conversion failed at source index {}: {}",
            self.source_index, self.source
        )
    }
}

impl Error for DataListConversionError {
    /// Returns the underlying single-value conversion error.
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}
