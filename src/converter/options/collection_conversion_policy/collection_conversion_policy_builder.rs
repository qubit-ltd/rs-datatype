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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CollectionConversionPolicyBuilder {
    /// Collection policy being configured.
    pub(super) policy: CollectionConversionPolicy,
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
