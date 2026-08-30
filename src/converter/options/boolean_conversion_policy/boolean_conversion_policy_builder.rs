// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Builder for Boolean conversion policy.

use super::BooleanConversionPolicy;
use crate::converter::BooleanLiteralConflictError;
use crate::converter::options::BooleanNumericPolicy;

/// Builder for [`BooleanConversionPolicy`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BooleanConversionPolicyBuilder {
    /// Validated policy or the first literal-conflict error encountered.
    pub(super) policy: Result<BooleanConversionPolicy, BooleanLiteralConflictError>,
}

impl BooleanConversionPolicyBuilder {
    /// Creates a builder initialized with the strict boolean policy.
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            policy: Ok(BooleanConversionPolicy::strict()),
        }
    }

    /// Adds a literal recognized as `true`.
    #[inline]
    #[must_use]
    pub fn true_literal(mut self, literal: &str) -> Self {
        self.policy = self.policy.and_then(|policy| policy.with_true_literal(literal));
        self
    }

    /// Adds a literal recognized as `false`.
    #[inline]
    #[must_use]
    pub fn false_literal(mut self, literal: &str) -> Self {
        self.policy = self.policy.and_then(|policy| policy.with_false_literal(literal));
        self
    }

    /// Configures case sensitivity.
    #[inline]
    #[must_use]
    pub fn case_sensitive(mut self, enabled: bool) -> Self {
        self.policy = self.policy.and_then(|policy| policy.with_case_sensitive(enabled));
        self
    }

    /// Configures integer-to-boolean conversion.
    #[inline(always)]
    #[must_use]
    pub fn numeric_policy(mut self, policy: BooleanNumericPolicy) -> Self {
        self.policy = self.policy.map(|options| options.with_numeric_policy(policy));
        self
    }

    /// Builds the configured policy and validates literal conflicts.
    #[inline]
    #[must_use]
    pub fn build(self) -> Result<BooleanConversionPolicy, BooleanLiteralConflictError> {
        self.policy
    }
}
