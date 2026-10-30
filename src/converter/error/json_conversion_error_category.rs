// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Safe categories for JSON decoding failures.

/// A value-free category for a JSON decoding failure.
///
/// Obtain the category through
/// [`DataConversionError::json_diagnostic()`].
/// This type is part of the `converter` API; the built-in JSON decoding path
/// also requires the `json` feature. The diagnostic can be absent for
/// non-JSON failures and JSON budget failures.
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
/// let classification = match diagnostic.category() {
///     JsonConversionErrorCategory::InvalidJson => "invalid syntax",
///     _ => "other JSON decoding failure",
/// };
/// assert_eq!(classification, "invalid syntax");
/// # }
/// ```
///
/// Downstream matches must handle categories they do not recognize:
///
/// ```compile_fail
/// use qubit_datatype::JsonConversionErrorCategory;
///
/// let category = JsonConversionErrorCategory::Read;
/// match category {
///     JsonConversionErrorCategory::Read => {}
///     JsonConversionErrorCategory::EmptyInput => {}
///     JsonConversionErrorCategory::InvalidUtf8 => {}
///     JsonConversionErrorCategory::InvalidJson => {}
///     JsonConversionErrorCategory::UnexpectedTopLevel => {}
///     JsonConversionErrorCategory::Deserialize => {}
/// }
/// ```
///
/// [`DataConversionError::json_diagnostic()`]: crate::converter::DataConversionError::json_diagnostic
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum JsonConversionErrorCategory {
    /// Reading the JSON input failed.
    Read,
    /// The JSON input was empty.
    EmptyInput,
    /// The JSON input was not valid UTF-8.
    InvalidUtf8,
    /// The JSON syntax was invalid.
    InvalidJson,
    /// The JSON value had an unexpected top-level shape.
    UnexpectedTopLevel,
    /// The JSON value could not be materialized as the requested target.
    Deserialize,
}
