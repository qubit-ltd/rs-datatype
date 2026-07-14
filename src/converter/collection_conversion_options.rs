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

use super::empty_item_policy::EmptyItemPolicy;
use super::scalar_items::ScalarItems;
use serde::{
    Deserialize,
    Serialize,
};

/// Options that control scalar-string-to-collection conversion.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
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
    /// Returns a copy with scalar string splitting enabled or disabled.
    ///
    /// # Parameters
    ///
    /// * `split_scalar_strings` - Whether scalar strings should be split.
    ///
    /// # Returns
    ///
    /// Updated options.
    #[must_use]
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
    /// * `delimiters` - Delimiters used when splitting is enabled.
    ///
    /// # Returns
    ///
    /// Updated options.
    #[must_use]
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
    #[must_use]
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
    #[must_use]
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
    pub fn scalar_items<'a>(&'a self, value: &'a str) -> ScalarItems<'a> {
        ScalarItems::new(self, value)
    }
}
