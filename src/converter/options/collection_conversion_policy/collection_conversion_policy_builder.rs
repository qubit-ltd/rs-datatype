// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Builder for scalar-string collection conversion policy.

use super::CollectionConversionPolicy;
use crate::converter::options::EmptyItemPolicy;

/// Builder for [`CollectionConversionPolicy`].
///
/// Enable scalar string splitting explicitly before configuring delimiters.
/// The example trims each item and skips empty entries.
///
/// # Examples
///
/// ```rust
/// use qubit_datatype::CollectionConversionPolicy;
/// use qubit_datatype::ConversionLimits;
/// use qubit_datatype::ConversionPolicy;
/// use qubit_datatype::EmptyItemPolicy;
/// use qubit_datatype::ScalarStringDataConverters;
///
/// let collection = CollectionConversionPolicy::builder()
///     .split_scalar_strings(true)
///     .delimiters([',', ';'])
///     .trim_items(true)
///     .empty_item_policy(EmptyItemPolicy::Skip)
///     .build();
/// let policy = ConversionPolicy::builder()
///     .collection_policy(collection)
///     .build();
/// let ports = ScalarStringDataConverters::from("8080,, 8443;9000")
///     .to_vec_with::<u16>(&policy, &ConversionLimits::default())
///     .expect("all configured ports should be valid");
/// assert_eq!(ports, vec![8080, 8443, 9000]);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CollectionConversionPolicyBuilder {
    /// Collection policy being configured.
    pub(super) policy: CollectionConversionPolicy,
}

impl CollectionConversionPolicyBuilder {
    /// Creates a builder initialized with the documented defaults.
    ///
    /// # Returns
    ///
    /// A builder configured with the default collection policy.
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            policy: CollectionConversionPolicy::default(),
        }
    }

    /// Configures scalar string splitting.
    ///
    /// # Parameters
    ///
    /// * `enabled` - Whether scalar strings are split into collection items.
    ///
    /// # Returns
    ///
    /// The builder with scalar string splitting updated.
    #[inline]
    #[must_use]
    pub fn split_scalar_strings(self, enabled: bool) -> Self {
        Self {
            policy: self.policy.with_split_scalar_strings(enabled),
        }
    }

    /// Configures scalar string delimiters.
    ///
    /// # Parameters
    ///
    /// * `delimiters` - Characters used to split scalar strings when splitting
    ///   is enabled.
    ///
    /// # Returns
    ///
    /// The builder with the delimiter sequence replaced.
    #[inline]
    #[must_use]
    pub fn delimiters(self, delimiters: impl IntoIterator<Item = char>) -> Self {
        Self {
            policy: self.policy.with_delimiters(delimiters),
        }
    }

    /// Configures item trimming.
    ///
    /// # Parameters
    ///
    /// * `enabled` - Whether split items are trimmed before conversion.
    ///
    /// # Returns
    ///
    /// The builder with item trimming updated.
    #[inline]
    #[must_use]
    pub fn trim_items(self, enabled: bool) -> Self {
        Self {
            policy: self.policy.with_trim_items(enabled),
        }
    }

    /// Configures empty item handling.
    ///
    /// # Parameters
    ///
    /// * `policy` - How empty items are handled after optional trimming.
    ///
    /// # Returns
    ///
    /// The builder with empty item handling updated.
    #[inline]
    #[must_use]
    pub fn empty_item_policy(self, policy: EmptyItemPolicy) -> Self {
        Self {
            policy: self.policy.with_empty_item_policy(policy),
        }
    }

    /// Builds the configured collection policy.
    ///
    /// # Returns
    ///
    /// The collection policy assembled from this builder's settings.
    #[inline]
    pub fn build(self) -> CollectionConversionPolicy {
        self.policy
    }
}
