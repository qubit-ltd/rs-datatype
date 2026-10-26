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
///
/// Group setters replace the complete policy group, while leaf setters update
/// only the named setting within the current group. When setters address the
/// same setting, the last call takes effect. Choose a group first, then apply
/// any leaf overrides:
///
/// ```
/// use qubit_datatype::{
///     BlankStringPolicy, CollectionConversionPolicy, ConversionPolicy,
///     EmptyItemPolicy, StringConversionPolicy,
/// };
///
/// let policy = ConversionPolicy::builder()
///     .string_policy(StringConversionPolicy::default())
///     .blank_string_policy(BlankStringPolicy::Reject)
///     .collection_policy(CollectionConversionPolicy::default())
///     .empty_item_policy(EmptyItemPolicy::Reject)
///     .build();
///
/// assert_eq!(policy.string().blank_string_policy(), BlankStringPolicy::Reject);
/// assert_eq!(policy.collection().empty_item_policy(), EmptyItemPolicy::Reject);
/// ```
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
    #[inline]
    #[must_use]
    pub fn numeric_policy(self, policy: NumericConversionPolicy) -> Self {
        Self {
            policy: self.policy.with_numeric_policy(policy),
        }
    }

    /// Replaces the complete string conversion policy group.
    ///
    /// This also replaces its blank-string setting. A later
    /// [`blank_string_policy`](Self::blank_string_policy) call overrides only
    /// that setting.
    #[inline]
    #[must_use]
    pub fn string_policy(self, policy: StringConversionPolicy) -> Self {
        Self {
            policy: self.policy.with_string_policy(policy),
        }
    }

    /// Overrides blank-string handling in the current string policy group.
    ///
    /// A later [`string_policy`](Self::string_policy) call replaces the group
    /// and therefore its blank-string setting.
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

    /// Replaces the complete collection conversion policy group.
    ///
    /// This also replaces its empty-item setting. A later
    /// [`empty_item_policy`](Self::empty_item_policy) call overrides only that
    /// setting.
    #[inline]
    #[must_use]
    pub fn collection_policy(self, policy: CollectionConversionPolicy) -> Self {
        Self {
            policy: self.policy.with_collection_policy(policy),
        }
    }

    /// Overrides empty-item handling in the current collection policy group.
    ///
    /// A later [`collection_policy`](Self::collection_policy) call replaces
    /// the group and therefore its empty-item setting.
    #[inline]
    #[must_use]
    pub fn empty_item_policy(self, policy: EmptyItemPolicy) -> Self {
        Self {
            policy: self.policy.with_empty_item_policy(policy),
        }
    }

    /// Configures duration conversion policy.
    #[inline]
    #[must_use]
    pub fn duration_policy(self, policy: DurationConversionPolicy) -> Self {
        Self {
            policy: self.policy.with_duration_policy(policy),
        }
    }

    /// Builds the configured conversion policy.
    #[inline]
    pub fn build(self) -> ConversionPolicy {
        self.policy
    }
}
