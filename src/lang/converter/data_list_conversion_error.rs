/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Data List Conversion Error
//!
//! Defines errors returned by reusable batch data conversions.
//!
//! # Author
//!
//! Haixing Hu

use std::error::Error;
use std::fmt;

use super::data_conversion_error::DataConversionError;

/// Error type returned by reusable batch data conversions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataListConversionError {
    /// Zero-based index of the source element that failed conversion.
    pub index: usize,
    /// Original single-value conversion error.
    pub source: DataConversionError,
}

impl fmt::Display for DataListConversionError {
    /// Formats the list conversion error with the failing element index.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Data conversion failed at index {}: {}",
            self.index, self.source
        )
    }
}

impl Error for DataListConversionError {
    /// Returns the underlying single-value conversion error.
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}
