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

/// Collects configuration for a [`BooleanConversionPolicy`].
///
/// Setters collect configuration; `build` validates the final literal sets
/// under the final case-sensitivity rule.
///
/// # Examples
///
/// ```
/// use qubit_datatype::{BooleanConversionPolicyBuilder, BooleanNumericPolicy};
///
/// let policy = BooleanConversionPolicyBuilder::new()
///     .true_literal("enabled")
///     .numeric_policy(BooleanNumericPolicy::Reject)
///     .build()
///     .expect("the true literal does not conflict with the defaults");
/// assert_eq!(policy.parse("enabled"), Some(true));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BooleanConversionPolicyBuilder {
    /// Collected true literals, which have not yet been validated.
    pub(super) true_literals: Vec<String>,
    /// Collected false literals, which have not yet been validated.
    pub(super) false_literals: Vec<String>,
    /// Collected case-sensitivity setting, which has not yet been validated.
    pub(super) case_sensitive: bool,
    /// Collected numeric policy, which has not yet been validated.
    pub(super) numeric_policy: BooleanNumericPolicy,
}

impl BooleanConversionPolicyBuilder {
    /// Creates a builder initialized with the strict boolean policy.
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        BooleanConversionPolicy::strict().into_builder()
    }

    /// Collects a literal recognized as `true`.
    #[inline]
    #[must_use]
    pub fn true_literal(mut self, literal: &str) -> Self {
        self.true_literals.push(literal.to_owned());
        self
    }

    /// Collects a literal recognized as `false`.
    #[inline]
    #[must_use]
    pub fn false_literal(mut self, literal: &str) -> Self {
        self.false_literals.push(literal.to_owned());
        self
    }

    /// Collects whether literal matching is case-sensitive.
    #[inline]
    #[must_use]
    pub fn case_sensitive(mut self, enabled: bool) -> Self {
        self.case_sensitive = enabled;
        self
    }

    /// Collects the integer-to-boolean conversion policy.
    #[inline]
    #[must_use]
    pub fn numeric_policy(mut self, policy: BooleanNumericPolicy) -> Self {
        self.numeric_policy = policy;
        self
    }

    /// Builds the configured policy and validates the final literal sets.
    ///
    /// Setters collect configuration; this method validates the final literal
    /// sets under the final case-sensitivity rule.
    ///
    /// # Returns
    ///
    /// The configured policy when all literals are disjoint under the
    /// selected case-sensitivity rule.
    ///
    /// # Errors
    ///
    /// Returns [`BooleanLiteralConflictError`] if the final true and false
    /// literal sets overlap under the final case-sensitivity rule.
    #[inline]
    pub fn build(self) -> Result<BooleanConversionPolicy, BooleanLiteralConflictError> {
        BooleanConversionPolicy::try_new(
            self.true_literals,
            self.false_literals,
            self.case_sensitive,
            self.numeric_policy,
        )
    }
}
