// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Builder for aggregate conversion policy.

use super::ConversionPolicy;
use crate::converter::options::BlankStringPolicy;
use crate::converter::options::BooleanConversionPolicy;
use crate::converter::options::CollectionConversionPolicy;
use crate::converter::options::DurationConversionPolicy;
use crate::converter::options::EmptyItemPolicy;
use crate::converter::options::NumericConversionPolicy;
use crate::converter::options::StringConversionPolicy;

/// Builder for [`ConversionPolicy`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConversionPolicyBuilder {
    /// Conversion policy being configured.
    pub(super) policy: ConversionPolicy,
}

impl ConversionPolicyBuilder {
    /// Creates a builder initialized with the documented defaults.
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            policy: ConversionPolicy::default(),
        }
    }

    /// Configures numeric conversion policy.
    #[inline(always)]
    #[must_use]
    pub fn numeric_policy(self, policy: NumericConversionPolicy) -> Self {
        Self {
            policy: self.policy.with_numeric_policy(policy),
        }
    }

    /// Configures string conversion policy.
    #[inline(always)]
    #[must_use]
    pub fn string_policy(self, policy: StringConversionPolicy) -> Self {
        Self {
            policy: self.policy.with_string_policy(policy),
        }
    }

    /// Configures blank string handling.
    #[inline]
    #[must_use]
    pub fn blank_string_policy(self, policy: BlankStringPolicy) -> Self {
        Self {
            policy: self.policy.with_blank_string_policy(policy),
        }
    }

    /// Configures boolean conversion policy.
    #[inline]
    #[must_use]
    pub fn boolean_policy(self, policy: BooleanConversionPolicy) -> Self {
        Self {
            policy: self.policy.with_boolean_policy(policy),
        }
    }

    /// Configures collection conversion policy.
    #[inline(always)]
    #[must_use]
    pub fn collection_policy(self, policy: CollectionConversionPolicy) -> Self {
        Self {
            policy: self.policy.with_collection_policy(policy),
        }
    }

    /// Configures empty collection item handling.
    #[inline]
    #[must_use]
    pub fn empty_item_policy(self, policy: EmptyItemPolicy) -> Self {
        Self {
            policy: self.policy.with_empty_item_policy(policy),
        }
    }

    /// Configures duration conversion policy.
    #[inline(always)]
    #[must_use]
    pub fn duration_policy(self, policy: DurationConversionPolicy) -> Self {
        Self {
            policy: self.policy.with_duration_policy(policy),
        }
    }

    /// Builds the configured conversion policy.
    #[inline]
    #[must_use]
    pub fn build(self) -> ConversionPolicy {
        self.policy
    }
}
