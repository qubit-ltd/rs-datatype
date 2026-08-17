// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # String Conversion Policy
// qubit-style: allow multiple-public-types
//!
//! Defines policy that controls string-source normalization.

use serde::Deserialize;
use serde::Serialize;

use super::super::error::StringNormalizationError;
use super::blank_string_policy::BlankStringPolicy;

/// Controls normalization applied once before parsing a string source.
///
/// Trimming happens before blank detection. A value is considered blank when
/// it consists only of Unicode whitespace, even when `trim` is disabled;
/// [`BlankStringPolicy::Preserve`] returns the original untrimmed slice in that
/// case.
///
/// # Examples
///
/// ```
/// use qubit_datatype::StringConversionPolicy;
///
/// let options = StringConversionPolicy::env_friendly();
/// assert_eq!(options.normalize("  value  "), Ok("value"));
/// assert!(options.normalize("   ").is_err());
/// ```
#[must_use]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct StringConversionPolicy {
    /// Whether strings are trimmed before conversion.
    trim: bool,
    /// How blank strings are interpreted after optional trimming.
    blank_string_policy: BlankStringPolicy,
}

impl Default for StringConversionPolicy {
    /// Creates default string conversion policy.
    ///
    /// # Returns
    ///
    /// Options preserving source whitespace and blank strings.
    #[inline]
    fn default() -> Self {
        Self {
            trim: false,
            blank_string_policy: BlankStringPolicy::Preserve,
        }
    }
}

impl StringConversionPolicy {
    /// Creates a builder initialized with the default string policy.
    #[inline]
    #[must_use]
    pub fn builder() -> StringConversionPolicyBuilder {
        StringConversionPolicyBuilder::new()
    }

    /// Converts this policy into a builder for explicit customization.
    #[inline]
    #[must_use]
    pub fn into_builder(self) -> StringConversionPolicyBuilder {
        StringConversionPolicyBuilder { policy: self }
    }
    /// Creates options suitable for environment-variable input.
    ///
    /// The profile trims surrounding whitespace and treats a blank value as
    /// missing, matching common environment configuration conventions.
    ///
    /// # Returns
    ///
    /// Environment-friendly string normalization options.
    #[inline]
    pub const fn env_friendly() -> Self {
        Self {
            trim: true,
            blank_string_policy: BlankStringPolicy::TreatAsMissing,
        }
    }

    /// Returns whether strings are trimmed before conversion.
    ///
    /// # Returns
    ///
    /// `true` when surrounding whitespace is removed.
    #[inline(always)]
    #[must_use]
    pub const fn trim(&self) -> bool {
        self.trim
    }

    /// Returns a copy with string trimming enabled or disabled.
    ///
    /// # Parameters
    ///
    /// * `trim` - Whether strings should be trimmed.
    ///
    /// # Returns
    ///
    /// Updated options.
    #[inline(always)]
    pub(crate) fn with_trim(mut self, trim: bool) -> Self {
        self.trim = trim;
        self
    }

    /// Returns the blank string policy.
    ///
    /// # Returns
    ///
    /// The policy applied after optional trimming.
    #[inline(always)]
    pub const fn blank_string_policy(&self) -> BlankStringPolicy {
        self.blank_string_policy
    }

    /// Returns a copy with a different blank string policy.
    ///
    /// # Parameters
    ///
    /// * `policy` - New blank string policy.
    ///
    /// # Returns
    ///
    /// Updated options.
    #[inline(always)]
    pub(crate) fn with_blank_string_policy(
        mut self,
        policy: BlankStringPolicy,
    ) -> Self {
        self.blank_string_policy = policy;
        self
    }

    /// Normalizes a string source according to these options.
    ///
    /// # Type Parameters
    ///
    /// * `'a` - Lifetime of the returned slice borrowed from `value`.
    ///
    /// # Parameters
    ///
    /// * `value` - Source string.
    ///
    /// # Returns
    ///
    /// Returns the normalized string.
    ///
    /// # Errors
    ///
    /// Returns [`StringNormalizationError::Missing`] when blank strings are
    /// treated as missing, or [`StringNormalizationError::BlankRejected`] when
    /// blank strings are rejected.
    pub fn normalize<'a>(
        &self,
        value: &'a str,
    ) -> Result<&'a str, StringNormalizationError> {
        let value = if self.trim { value.trim() } else { value };
        if value.trim().is_empty() {
            match self.blank_string_policy {
                BlankStringPolicy::Preserve => Ok(value),
                BlankStringPolicy::TreatAsMissing => {
                    Err(StringNormalizationError::Missing)
                }
                BlankStringPolicy::Reject => {
                    Err(StringNormalizationError::BlankRejected)
                }
            }
        } else {
            Ok(value)
        }
    }

    /// Normalizes a string and represents a missing blank value as `None`.
    ///
    /// # Type Parameters
    ///
    /// * `'a` - Lifetime of the optional returned slice borrowed from `value`.
    ///
    /// # Parameters
    ///
    /// * `value` - Source string.
    ///
    /// # Returns
    ///
    /// Returns `Some` with the borrowed normalized string, or `None` when the
    /// configured blank policy treats the value as missing.
    ///
    /// # Errors
    ///
    /// Returns [`StringNormalizationError::BlankRejected`] when the configured
    /// blank policy rejects the value.
    #[inline]
    pub fn normalize_optional<'a>(
        &self,
        value: &'a str,
    ) -> Result<Option<&'a str>, StringNormalizationError> {
        match self.normalize(value) {
            Ok(value) => Ok(Some(value)),
            Err(StringNormalizationError::Missing) => Ok(None),
            Err(error) => Err(error),
        }
    }
}

/// Builder for [`StringConversionPolicy`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StringConversionPolicyBuilder {
    policy: StringConversionPolicy,
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
