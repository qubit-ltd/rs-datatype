// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Builder for string conversion policy.

use super::StringConversionPolicy;
use crate::converter::options::BlankStringPolicy;

/// Builder for [`StringConversionPolicy`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StringConversionPolicyBuilder {
    /// String policy being configured.
    pub(super) policy: StringConversionPolicy,
}

impl StringConversionPolicyBuilder {
    /// Creates a builder initialized with the documented defaults.
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            policy: StringConversionPolicy::default(),
        }
    }

    /// Configures trimming.
    #[inline(always)]
    #[must_use]
    pub fn trim(self, trim: bool) -> Self {
        Self {
            policy: self.policy.with_trim(trim),
        }
    }

    /// Configures blank-string handling.
    #[inline(always)]
    #[must_use]
    pub fn blank_string_policy(self, policy: BlankStringPolicy) -> Self {
        Self {
            policy: self.policy.with_blank_string_policy(policy),
        }
    }

    /// Builds the configured string policy.
    #[inline]
    #[must_use]
    pub fn build(self) -> StringConversionPolicy {
        self.policy
    }
}
