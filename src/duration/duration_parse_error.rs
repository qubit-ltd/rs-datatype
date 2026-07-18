// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Error returned by Duration text parsing.

/// Error returned when parsing canonical Duration text.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum DurationParseError {
    /// The input does not match the configured non-negative integer grammar.
    #[error("invalid duration syntax")]
    InvalidSyntax,
    /// The input has a syntactically valid but unsupported unit suffix.
    #[error("unsupported duration unit `{unit}`")]
    UnsupportedUnit {
        /// Unsupported suffix without the numeric prefix.
        unit: String,
    },
    /// The numeric value cannot be represented as a Duration.
    #[error("duration value is out of range")]
    OutOfRange,
}
