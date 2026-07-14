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

use super::data_conversion_error::DataConversionError;

/// Reports the first failed item in an ordered batch conversion.
///
/// [`Self::source_index`] refers to the original, zero-based source position,
/// including positions skipped by collection policies. [`Self::source`] keeps
/// the complete single-value conversion error and is exposed as the standard
/// error source.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("Data conversion failed at source index {source_index}: {source}")]
pub struct DataListConversionError {
    /// Zero-based index in the original source, before any items were skipped.
    pub source_index: usize,
    /// Original single-value conversion error.
    #[source]
    pub source: DataConversionError,
}
