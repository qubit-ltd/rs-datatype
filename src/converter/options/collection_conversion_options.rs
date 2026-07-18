// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Collection Conversion Options
//!
//! Defines options that control scalar-string-to-collection conversion.

use super::super::scalar_items::ScalarItems;
use super::empty_item_policy::EmptyItemPolicy;
use serde::{
    Deserialize,
    Serialize,
};

/// Controls how one scalar string is exposed as collection items.
///
/// These options are consumed by [`crate::ScalarStringDataConverters`], not by
/// [`crate::DataConverters`] over an already-materialized collection. Splitting
/// is lazy, preserves each raw item's original index, and applies trimming
/// before [`EmptyItemPolicy`].
///
/// # Examples
///
/// ```
/// use qubit_datatype::{CollectionConversionOptions, EmptyItemPolicy};
///
/// let options = CollectionConversionOptions::default()
///     .with_split_scalar_strings(true)
///     .with_trim_items(true)
///     .with_empty_item_policy(EmptyItemPolicy::Skip);
/// let items: Vec<_> = options
///     .scalar_items("1, ,3")
///     .map(|item| item.expect("empty items are skipped").value)
///     .collect();
/// assert_eq!(items, ["1", "3"]);
/// ```
#[must_use]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct CollectionConversionOptions {
    /// Whether a scalar string can be split into collection items.
    pub split_scalar_strings: bool,
    /// Delimiters used to split scalar strings.
    pub delimiters: Vec<char>,
    /// Whether split items are trimmed before element conversion.
    pub trim_items: bool,
    /// How empty split items are interpreted.
    pub empty_item_policy: EmptyItemPolicy,
}

impl Default for CollectionConversionOptions {
    /// Creates default collection conversion options.
    fn default() -> Self {
        Self {
            split_scalar_strings: false,
            delimiters: vec![','],
            trim_items: false,
            empty_item_policy: EmptyItemPolicy::Keep,
        }
    }
}

impl CollectionConversionOptions {
    /// Creates options suitable for environment-variable lists.
    ///
    /// The profile splits comma-separated scalar strings, trims each item,
    /// and skips empty items while preserving the original source indices of
    /// retained items.
    ///
    /// # Returns
    ///
    /// Environment-friendly scalar-to-collection options.
    #[inline]
    pub fn env_friendly() -> Self {
        Self {
            split_scalar_strings: true,
            delimiters: vec![','],
            trim_items: true,
            empty_item_policy: EmptyItemPolicy::Skip,
        }
    }

    /// Returns a copy with scalar string splitting enabled or disabled.
    ///
    /// # Parameters
    ///
    /// * `split_scalar_strings` - Whether scalar strings should be split.
    ///
    /// # Returns
    ///
    /// Updated options.
    #[inline(always)]
    pub fn with_split_scalar_strings(
        mut self,
        split_scalar_strings: bool,
    ) -> Self {
        self.split_scalar_strings = split_scalar_strings;
        self
    }

    /// Returns a copy with different scalar string delimiters.
    ///
    /// # Parameters
    ///
    /// * `delimiters` - Delimiters used when splitting is enabled. An empty
    ///   iterator disables delimiter matches even if splitting is enabled, so
    ///   the source is yielded as one item.
    ///
    /// # Returns
    ///
    /// Updated options.
    #[inline(always)]
    pub fn with_delimiters(
        mut self,
        delimiters: impl IntoIterator<Item = char>,
    ) -> Self {
        self.delimiters = delimiters.into_iter().collect();
        self
    }

    /// Returns a copy with per-item trimming enabled or disabled.
    ///
    /// # Parameters
    ///
    /// * `trim_items` - Whether split items should be trimmed.
    ///
    /// # Returns
    ///
    /// Updated options.
    #[inline(always)]
    pub fn with_trim_items(mut self, trim_items: bool) -> Self {
        self.trim_items = trim_items;
        self
    }

    /// Returns a copy with a different empty item policy.
    ///
    /// # Parameters
    ///
    /// * `policy` - New empty item policy.
    ///
    /// # Returns
    ///
    /// Updated options.
    #[inline(always)]
    pub fn with_empty_item_policy(mut self, policy: EmptyItemPolicy) -> Self {
        self.empty_item_policy = policy;
        self
    }

    /// Splits and normalizes a scalar string into collection items.
    ///
    /// # Parameters
    ///
    /// * `value` - Normalized scalar string.
    ///
    /// # Returns
    ///
    /// Returns a lazy iterator that borrows `value` and these options. Each
    /// yielded item retains its index in the unsuppressed split sequence.
    /// Rejected empty items are reported only when iteration reaches them.
    #[inline(always)]
    pub fn scalar_items<'a>(&'a self, value: &'a str) -> ScalarItems<'a> {
        ScalarItems::new(self, value)
    }
}
