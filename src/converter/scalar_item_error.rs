// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
// =============================================================================
//! Scalar item iteration error.

use std::error::Error;
use std::fmt;

/// Target-independent error discovered while iterating scalar items.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScalarItemError {
    /// Zero-based index before empty-item filtering.
    pub source_index: usize,
}

impl fmt::Display for ScalarItemError {
    /// Formats the item rejection without assuming a conversion target.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "blank scalar item rejected at source index {}",
            self.source_index
        )
    }
}

impl Error for ScalarItemError {}
