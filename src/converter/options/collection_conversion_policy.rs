// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Collection Conversion Policy
// qubit-style: allow multiple-public-types
//!
//! Defines policy that controls scalar-string-to-collection conversion.

use serde::Deserialize;
use serde::Serialize;

use super::super::scalar_items::ScalarItems;
use super::collection_conversion_limits::CollectionConversionLimits;
use super::empty_item_policy::EmptyItemPolicy;

/// Controls how one scalar string is exposed as collection items.
///
/// This policy is consumed by [`crate::ScalarStringDataConverters`], not by
/// [`crate::DataConverters`] over an already-materialized collection. Splitting
/// is lazy, preserves each raw item's original index, and applies trimming
/// before [`EmptyItemPolicy`]. Items removed by [`EmptyItemPolicy::Skip`] do
/// not affect other conversion policy.
///
/// # Examples
///
/// ```
/// use qubit_datatype::{
///     CollectionConversionLimits, CollectionConversionPolicy, EmptyItemPolicy,
/// };
///
/// let options = CollectionConversionPolicy::builder()
///     .split_scalar_strings(true)
///     .trim_items(true)
///     .empty_item_policy(EmptyItemPolicy::Skip)
///     .build();
/// let limits = CollectionConversionLimits::default();
/// let items: Vec<_> = options
///     .scalar_items(&limits, "1, ,3")
///     .map(|item| item.expect("empty items are skipped").value)
///     .collect();
/// assert_eq!(items, ["1", "3"]);
/// ```
#[must_use]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct CollectionConversionPolicy {
    /// Whether a scalar string can be split into collection items.
    split_scalar_strings: bool,
    /// Delimiters used to split scalar strings.
    delimiters: Vec<char>,
    /// Whether split items are trimmed before element conversion.
    trim_items: bool,
    /// How empty split items are interpreted.
    empty_item_policy: EmptyItemPolicy,
}

impl Default for CollectionConversionPolicy {
    /// Creates default collection conversion policy.
    ///
    /// # Returns
    ///
    /// Options using comma delimiters without splitting or item normalization.
    #[inline]
    fn default() -> Self {
        Self {
            split_scalar_strings: false,
            delimiters: vec![','],
            trim_items: false,
            empty_item_policy: EmptyItemPolicy::Keep,
        }
    }
}

impl CollectionConversionPolicy {
    /// Creates a builder initialized with the default collection policy.
    #[inline]
    #[must_use]
    pub fn builder() -> CollectionConversionPolicyBuilder {
        CollectionConversionPolicyBuilder::new()
    }

    /// Converts this policy into a builder for explicit customization.
    #[inline]
    #[must_use]
    pub fn into_builder(self) -> CollectionConversionPolicyBuilder {
        CollectionConversionPolicyBuilder { policy: self }
    }
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

    /// Returns whether scalar strings are split into collection items.
    ///
    /// # Returns
    ///
    /// `true` when scalar strings are split using configured delimiters.
    #[inline(always)]
    #[must_use]
    pub const fn split_scalar_strings(&self) -> bool {
        self.split_scalar_strings
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
    pub(crate) fn with_split_scalar_strings(mut self, split_scalar_strings: bool) -> Self {
        self.split_scalar_strings = split_scalar_strings;
        self
    }

    /// Returns the delimiters used to split scalar strings.
    ///
    /// # Returns
    ///
    /// The configured delimiter slice in matching order.
    ///
    /// ```compile_fail
    /// #![deny(unused_must_use)]
    /// use qubit_datatype::CollectionConversionPolicy;
    ///
    /// CollectionConversionPolicy::default().delimiters();
    /// ```
    #[must_use]
    #[inline(always)]
    pub fn delimiters(&self) -> &[char] {
        &self.delimiters
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
    #[inline]
    pub(crate) fn with_delimiters(mut self, delimiters: impl IntoIterator<Item = char>) -> Self {
        self.delimiters = delimiters.into_iter().collect();
        self
    }

    /// Returns whether collection items are trimmed before conversion.
    ///
    /// # Returns
    ///
    /// `true` when each split item is trimmed.
    #[inline(always)]
    #[must_use]
    pub const fn trim_items(&self) -> bool {
        self.trim_items
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
    pub(crate) fn with_trim_items(mut self, trim_items: bool) -> Self {
        self.trim_items = trim_items;
        self
    }

    /// Returns the empty collection item policy.
    ///
    /// # Returns
    ///
    /// The policy applied to empty items after optional trimming.
    #[must_use]
    #[inline(always)]
    pub const fn empty_item_policy(&self) -> EmptyItemPolicy {
        self.empty_item_policy
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
    pub(crate) fn with_empty_item_policy(mut self, policy: EmptyItemPolicy) -> Self {
        self.empty_item_policy = policy;
        self
    }

    /// Splits and normalizes a scalar string under explicit value limits.
    ///
    /// # Parameters
    ///
    /// * `limits` - Retained-item limits for this scalar collection.
    /// * `value` - Normalized scalar source text.
    ///
    /// # Returns
    ///
    /// A lazy iterator borrowing the policy and source text.
    #[must_use]
    #[inline(always)]
    pub fn scalar_items<'a>(&'a self, limits: &CollectionConversionLimits, value: &'a str) -> ScalarItems<'a> {
        ScalarItems::new(self, limits.max_items(), value)
    }
}

/// Builder for [`CollectionConversionPolicy`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CollectionConversionPolicyBuilder {
    /// Collection policy being configured.
    policy: CollectionConversionPolicy,
}

impl CollectionConversionPolicyBuilder {
    /// Creates a builder initialized with the documented defaults.
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            policy: CollectionConversionPolicy::default(),
        }
    }
    /// Configures scalar string splitting.
    #[inline(always)]
    #[must_use]
    pub fn split_scalar_strings(self, enabled: bool) -> Self {
        Self {
            policy: self.policy.with_split_scalar_strings(enabled),
        }
    }
    /// Configures scalar string delimiters.
    #[inline(always)]
    #[must_use]
    pub fn delimiters(self, delimiters: impl IntoIterator<Item = char>) -> Self {
        Self {
            policy: self.policy.with_delimiters(delimiters),
        }
    }
    /// Configures item trimming.
    #[inline(always)]
    #[must_use]
    pub fn trim_items(self, enabled: bool) -> Self {
        Self {
            policy: self.policy.with_trim_items(enabled),
        }
    }
    /// Configures empty item handling.
    #[inline(always)]
    #[must_use]
    pub fn empty_item_policy(self, policy: EmptyItemPolicy) -> Self {
        Self {
            policy: self.policy.with_empty_item_policy(policy),
        }
    }
    /// Builds the configured collection policy.
    #[inline]
    #[must_use]
    pub fn build(self) -> CollectionConversionPolicy {
        self.policy
    }
}
