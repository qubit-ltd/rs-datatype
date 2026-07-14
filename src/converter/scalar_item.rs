// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
// =============================================================================
//! Borrowed scalar collection item.

/// A borrowed scalar item together with its index in the original source.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScalarItem<'a> {
    /// Zero-based index before empty-item filtering.
    pub source_index: usize,
    /// Borrowed item text after optional per-item trimming.
    pub value: &'a str,
}
