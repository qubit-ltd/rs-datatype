// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Resource limits for structured text conversions.

use serde::{
    Deserialize,
    Serialize,
};

/// Bounds parsing work and allocations introduced by structured text
/// conversion.
///
/// The text limit applies to UTF-8 bytes after configured string normalization
/// and covers JSON values plus JSON objects converted to string maps.
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct StructuredConversionLimits {
    /// Maximum normalized structured source text length in bytes.
    max_text_bytes: usize,
}

impl StructuredConversionLimits {
    /// Default maximum normalized structured text length in bytes.
    pub const DEFAULT_MAX_TEXT_BYTES: usize = 1_048_576;

    /// Returns the maximum normalized structured text length in bytes.
    #[inline(always)]
    #[must_use]
    pub const fn max_text_bytes(&self) -> usize {
        self.max_text_bytes
    }

    /// Returns a copy with a different structured text byte limit.
    #[inline(always)]
    pub const fn with_max_text_bytes(mut self, maximum: usize) -> Self {
        self.max_text_bytes = maximum;
        self
    }
}

impl Default for StructuredConversionLimits {
    /// Creates the default structured conversion limits.
    #[inline(always)]
    fn default() -> Self {
        Self {
            max_text_bytes: Self::DEFAULT_MAX_TEXT_BYTES,
        }
    }
}
