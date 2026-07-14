// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
// =============================================================================
//! # String Normalization Error
//!
//! Defines policy outcomes produced before target-specific parsing.

/// Error returned while normalizing a string source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum StringNormalizationError {
    /// The blank source is treated as a missing value.
    #[error("missing string value")]
    Missing,
    /// The blank source is explicitly rejected.
    #[error("blank string rejected")]
    BlankRejected,
}
