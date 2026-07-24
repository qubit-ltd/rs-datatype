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
///
/// The item limit counts only items retained after trimming and empty-item
/// policy handling. The first retained item beyond the limit produces one
/// [`ScalarItemError::ItemLimitExceeded`] error and exhausts the iterator.
///
/// # Type Parameters
///
/// * `'a` - Lifetime shared by the borrowed source text and collection options.
#[must_use]
#[derive(Debug, Clone)]
pub struct ScalarItems<'a> {
    /// Original scalar source.
    value: &'a str,
    /// Delimiters borrowed from the conversion options.
    delimiters: &'a [char],
    /// ASCII lookup table used for large delimiter sets.
    ascii_delimiters: Option<[bool; 128]>,
    /// Sorted non-ASCII lookup used for large delimiter sets.
    non_ascii_delimiters: Option<Vec<char>>,
    /// Whether delimiter-based splitting is enabled.
    split_scalar_strings: bool,
    /// Whether each raw item is trimmed.
    trim_items: bool,
    /// Policy applied to empty items after optional trimming.
    empty_item_policy: EmptyItemPolicy,
    /// Maximum number of retained items.
    max_items: usize,
    /// Number of retained items already returned successfully.
    retained_items: usize,
    /// Byte offset of the next raw item, or `None` after the final item.
    next_start: Option<usize>,
    /// Index of the next raw item before filtering.
    next_source_index: usize,
}

impl<'a> ScalarItems<'a> {
    /// Creates a lazy scalar-item iterator.
    ///
    /// # Parameters
    ///
    /// * `options` - Collection splitting and empty-item policies.
    /// * `value` - Scalar or delimited source text to iterate.
    ///
    /// # Returns
    ///
    /// An iterator borrowing both inputs and deferring all processing until
    /// iteration.
    pub(super) fn new(
        options: &'a CollectionConversionOptions,
        value: &'a str,
    ) -> Self {
        let delimiters = options.delimiters();
        let (ascii_delimiters, non_ascii_delimiters) = if delimiters.len() > 8 {
            let mut ascii = [false; 128];
            let mut non_ascii = Vec::new();
            for &delimiter in delimiters {
                if delimiter.is_ascii() {
                    ascii[delimiter as usize] = true;
                } else {
                    non_ascii.push(delimiter);
                }
            }
            non_ascii.sort_unstable();
            non_ascii.dedup();
            (Some(ascii), Some(non_ascii))
        } else {
            (None, None)
        };
        Self {
            value,
            delimiters,
            ascii_delimiters,
            non_ascii_delimiters,
            split_scalar_strings: options.split_scalar_strings(),
            trim_items: options.trim_items(),
            empty_item_policy: options.empty_item_policy(),
            max_items: options.max_items(),
            retained_items: 0,
            next_start: Some(0),
            next_source_index: 0,
        }
    }

    /// Returns the next unfiltered source slice and advances iterator state.
    ///
    /// # Returns
    ///
    /// `Some` with the next raw item and original index, or `None` after the
    /// final raw item.
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
        let ascii_delimiters = self.ascii_delimiters.as_ref();
        let non_ascii_delimiters = self.non_ascii_delimiters.as_deref();
        let delimiters = self.delimiters;
        match remaining.char_indices().find(|(_, character)| {
            let Some(ascii) = ascii_delimiters else {
                return delimiters.contains(character);
            };
            if character.is_ascii() {
                ascii[*character as usize]
            } else {
                non_ascii_delimiters.is_some_and(|sorted| {
                    sorted.binary_search(character).is_ok()
                })
            }
        }) {
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

    /// Retains one normalized item or reports the configured item limit.
    ///
    /// # Parameters
    ///
    /// * `item` - Candidate item after trimming and empty-item policy handling.
    ///
    /// # Returns
    ///
    /// The retained item when capacity remains.
    ///
    /// # Errors
    ///
    /// Returns [`ScalarItemError::ItemLimitExceeded`] for the first item beyond
    /// `max_items`. The iterator is exhausted before returning that error.
    #[inline]
    fn retain_item(
        &mut self,
        item: ScalarItem<'a>,
    ) -> Result<ScalarItem<'a>, ScalarItemError> {
        if self.retained_items >= self.max_items {
            self.next_start = None;
            return Err(ScalarItemError::item_limit_exceeded(
                item.source_index,
                self.max_items,
            ));
        }
        self.retained_items += 1;
        Ok(item)
    }
}

impl<'a> Iterator for ScalarItems<'a> {
    type Item = Result<ScalarItem<'a>, ScalarItemError>;

    /// Returns the next retained item or the next lazily discovered error.
    ///
    /// Empty-item policy is applied before retained items consume quota. A
    /// rejected empty item therefore takes precedence over the item limit and
    /// does not consume quota. The first retained item beyond the configured
    /// limit returns [`ScalarItemError::ItemLimitExceeded`] and exhausts this
    /// iterator.
    ///
    /// # Returns
    ///
    /// `Some(Ok(item))` for a retained item, `Some(Err(error))` for the first
    /// rejection or limit failure, and `None` after exhaustion.
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let mut item = self.next_raw()?;
            if self.trim_items {
                item.value = item.value.trim();
            }
            if !item.value.is_empty() {
                return Some(self.retain_item(item));
            }
            match self.empty_item_policy {
                EmptyItemPolicy::Keep => return Some(self.retain_item(item)),
                EmptyItemPolicy::Skip => {}
                EmptyItemPolicy::Reject => {
                    return Some(Err(ScalarItemError::new(item.source_index)));
                }
            }
        }
    }
}
