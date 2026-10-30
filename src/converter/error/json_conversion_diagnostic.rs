// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Safe, optional location information for JSON conversion failures.

use super::json_conversion_error_category::JsonConversionErrorCategory;

/// A value-free diagnostic retained from JSON decoding.
///
/// A diagnostic records only the decoder's category and optional location.
/// It never retains the input or the decoder error text. Coordinates describe
/// the text passed to the decoder (which may already have been normalized);
/// column conventions are supplied by the decoder and can vary by category.
///
/// Obtain a diagnostic from a
/// [`DataConversionError::json_diagnostic()`]. The diagnostic type is part of
/// the `converter` API; the built-in JSON decoding path also requires the
/// `json` feature. The diagnostic may be absent for non-JSON failures and
/// JSON budget failures.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "json")]
/// # {
/// use qubit_datatype::DataConverter;
/// use qubit_datatype::JsonConversionErrorCategory;
///
/// let error = DataConverter::from("{\n@}")
///     .to::<serde_json::Value>()
///     .expect_err("the fixture contains invalid JSON");
/// let diagnostic = error.json_diagnostic().expect("JSON syntax diagnostic");
/// assert_eq!(diagnostic.category(), JsonConversionErrorCategory::InvalidJson);
/// if let (Some(line), Some(column)) = (diagnostic.line(), diagnostic.column()) {
///     assert!(line > 0);
///     assert!(column > 0);
/// }
/// # }
/// ```
///
/// The example uses a fixed, non-sensitive input and exposes only its
/// category and optional coordinates.
///
/// [`DataConversionError::json_diagnostic()`]: crate::converter::DataConversionError::json_diagnostic
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct JsonConversionDiagnostic {
    category: JsonConversionErrorCategory,
    line: Option<usize>,
    column: Option<usize>,
}

impl JsonConversionDiagnostic {
    /// Returns the safe category of the decoding failure.
    #[inline]
    pub const fn category(&self) -> JsonConversionErrorCategory {
        self.category
    }

    /// Returns the one-based line, when the decoder provides one.
    #[inline]
    pub const fn line(&self) -> Option<usize> {
        self.line
    }

    /// Returns the decoder-provided column, when available.
    ///
    /// Invalid JSON syntax reports a character column, while target
    /// deserialization reports a byte column.
    #[inline]
    pub const fn column(&self) -> Option<usize> {
        self.column
    }

    /// Creates a diagnostic from safe decoder metadata.
    #[cfg(feature = "json")]
    #[inline]
    pub(crate) const fn new(category: JsonConversionErrorCategory, line: Option<usize>, column: Option<usize>) -> Self {
        Self { category, line, column }
    }
}
