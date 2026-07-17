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
    pub source_index: usize,
}
