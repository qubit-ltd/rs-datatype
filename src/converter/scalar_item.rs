// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Borrowed scalar collection item.

/// A borrowed scalar item together with its index in the original source.
///
/// The value may be trimmed according to
/// [`crate::CollectionConversionPolicy`], while the index always identifies
/// the raw item before empty-item filtering.
///
/// # Type Parameters
///
/// * `'a` - Lifetime of the borrowed item text.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "converter")]
/// # {
/// use qubit_datatype::ScalarItem;
///
/// let item = ScalarItem {
///     source_index: 2,
///     value: " 42 ",
/// };
/// assert_eq!(item.source_index, 2);
/// assert_eq!(item.value, " 42 ");
/// # }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[must_use]
pub struct ScalarItem<'a> {
    /// Zero-based index before empty-item filtering.
    pub source_index: usize,
    /// Borrowed item text after optional per-item trimming.
    pub value: &'a str,
}
