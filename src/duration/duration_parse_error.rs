// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Error returned by Duration text parsing.

/// Error returned when parsing Duration text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum DurationParseError {
    /// The source text exceeded the configured byte limit.
    #[error("duration text exceeds the {maximum}-byte limit")]
    LimitExceeded {
        /// Configured maximum source text length in bytes.
        maximum: usize,
    },
    /// The input does not match the configured non-negative integer grammar.
    #[error("invalid duration syntax")]
    InvalidSyntax,
    /// The input is a Lenient-only alias rather than a strict symbol.
    #[error("non-canonical duration unit; use `{canonical}`")]
    NonCanonicalUnit {
        /// Preferred strict unit symbol.
        canonical: &'static str,
    },
    /// The input has a syntactically valid but unsupported unit suffix.
    #[error("unsupported duration unit")]
    UnsupportedUnit,
    /// The numeric value cannot be represented as a Duration.
    #[error("duration value is out of range")]
    OutOfRange,
}
