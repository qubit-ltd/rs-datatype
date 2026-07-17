// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Scalar item iteration error.

/// Target-independent error discovered while iterating scalar items.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error("blank scalar item rejected at source index {source_index}")]
pub struct ScalarItemError {
    /// Zero-based index before empty-item filtering.
    source_index: usize,
}

impl ScalarItemError {
    /// Creates an item error for the specified original source position.
    ///
    /// # Parameters
    ///
    /// * `source_index` - Zero-based position before empty-item filtering.
    ///
    /// # Returns
    ///
    /// A scalar item error containing the rejected source position.
    #[inline(always)]
    pub const fn new(source_index: usize) -> Self {
        Self { source_index }
    }

    /// Returns the zero-based source position before empty-item filtering.
    ///
    /// # Returns
    ///
    /// The original source position of the rejected scalar item.
    #[inline(always)]
    pub const fn source_index(&self) -> usize {
        self.source_index
    }
}
