// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Lazy scalar collection item iterator.

use super::error::ScalarItemError;
use super::options::{
    CollectionConversionOptions,
    EmptyItemPolicy,
};
use super::scalar_item::ScalarItem;

/// A lazy iterator over scalar collection items.
///
/// Delimiters are scanned only as items are requested. Consequently, callers
/// that need only the first retained item do not validate or allocate the
/// unconsumed tail. Each item borrows the original source string; iteration
/// allocates no item strings.
#[must_use]
#[derive(Debug, Clone)]
pub struct ScalarItems<'a> {
    /// Original scalar source.
    value: &'a str,
    /// Delimiters borrowed from the conversion options.
    delimiters: &'a [char],
    /// Whether delimiter-based splitting is enabled.
    split_scalar_strings: bool,
    /// Whether each raw item is trimmed.
    trim_items: bool,
    /// Policy applied to empty items after optional trimming.
    empty_item_policy: EmptyItemPolicy,
    /// Byte offset of the next raw item, or `None` after the final item.
    next_start: Option<usize>,
    /// Index of the next raw item before filtering.
    next_source_index: usize,
}

impl<'a> ScalarItems<'a> {
    /// Creates a lazy scalar-item iterator.
    ///
    /// The iterator borrows both `options` and `value` for its lifetime and
    /// defers splitting, trimming, and empty-item handling until iteration.
    pub(super) fn new(
        options: &'a CollectionConversionOptions,
        value: &'a str,
    ) -> Self {
        Self {
            value,
            delimiters: &options.delimiters,
            split_scalar_strings: options.split_scalar_strings,
            trim_items: options.trim_items,
            empty_item_policy: options.empty_item_policy,
            next_start: Some(0),
            next_source_index: 0,
        }
    }

    /// Returns the next unfiltered source slice and advances iterator state.
    ///
    /// Returns `Some` with the next raw item and its original index. `None`
    /// means the final raw item has already been returned.
    fn next_raw(&mut self) -> Option<ScalarItem<'a>> {
        let start = self.next_start?;
        let source_index = self.next_source_index;
        self.next_source_index += 1;

        if !self.split_scalar_strings {
            self.next_start = None;
            return Some(ScalarItem {
                source_index,
                value: self.value,
            });
        }

        let remaining = &self.value[start..];
        match remaining
            .char_indices()
            .find(|(_, character)| self.delimiters.contains(character))
        {
            Some((relative_end, delimiter)) => {
                let end = start + relative_end;
                self.next_start = Some(end + delimiter.len_utf8());
                Some(ScalarItem {
                    source_index,
                    value: &self.value[start..end],
                })
            }
            None => {
                self.next_start = None;
                Some(ScalarItem {
                    source_index,
                    value: remaining,
                })
            }
        }
    }
}

impl<'a> Iterator for ScalarItems<'a> {
    type Item = Result<ScalarItem<'a>, ScalarItemError>;

    /// Returns the next retained item or the next lazily discovered rejection.
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let mut item = self.next_raw()?;
            if self.trim_items {
                item.value = item.value.trim();
            }
            if !item.value.is_empty() {
                return Some(Ok(item));
            }
            match self.empty_item_policy {
                EmptyItemPolicy::Keep => return Some(Ok(item)),
                EmptyItemPolicy::Skip => {}
                EmptyItemPolicy::Reject => {
                    return Some(Err(ScalarItemError::new(item.source_index)));
                }
            }
        }
    }
}
