// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Session-admitted scalar collection item.

use super::scalar_item::ScalarItem;

/// A scalar item whose session item budget has already been admitted.
///
/// Construct this token through
/// [`crate::ConversionSession::admit_scalar_item`]. Downstream adapters can
/// consume it to continue conversion without charging the same item again.
#[must_use]
#[derive(Debug, PartialEq, Eq)]
pub struct AdmittedScalarItem {
    source_index: usize,
}

impl AdmittedScalarItem {
    /// Creates an admitted scalar item after the session budget succeeds.
    #[inline]
    pub(crate) const fn new(item: ScalarItem<'_>) -> Self {
        Self {
            source_index: item.source_index,
        }
    }

    /// Returns the original source index of the admitted item.
    #[inline(always)]
    #[must_use]
    pub const fn source_index(&self) -> usize {
        self.source_index
    }
}
