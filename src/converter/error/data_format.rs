// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Data Format
//!
//! Identifies structured formats involved in conversion errors.

use serde::{
    Deserialize,
    Serialize,
};

/// Structured data format used by a conversion operation.
///
/// This enum may gain additional formats in future releases. External
/// matches must include a wildcard arm.
///
/// ```compile_fail
/// use qubit_datatype::DataFormat;
///
/// fn format_name(format: DataFormat) -> &'static str {
///     match format {
///         DataFormat::Json => "json",
///     }
/// }
/// ```
#[must_use]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DataFormat {
    /// JavaScript Object Notation.
    Json,
}

impl DataFormat {
    /// Returns the stable lowercase name of this format.
    ///
    /// # Returns
    ///
    /// The stable lowercase serialization name.
    #[must_use]
    #[inline(always)]
    pub const fn as_str(self) -> &'static str {
        match self {
            DataFormat::Json => "json",
        }
    }
}
