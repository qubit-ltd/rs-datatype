// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # String Conversion Options
//!
//! Defines options that control string-source normalization.

use super::blank_string_policy::BlankStringPolicy;
use super::string_normalization_error::StringNormalizationError;
use serde::{
    Deserialize,
    Serialize,
};

/// Options that control string-source normalization.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct StringConversionOptions {
    /// Whether strings are trimmed before conversion.
    pub trim: bool,
    /// How blank strings are interpreted after optional trimming.
    pub blank_string_policy: BlankStringPolicy,
}

impl Default for StringConversionOptions {
    /// Creates default string conversion options.
    fn default() -> Self {
        Self {
            trim: false,
            blank_string_policy: BlankStringPolicy::Preserve,
        }
    }
}

impl StringConversionOptions {
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

    /// Returns a copy with string trimming enabled or disabled.
    ///
    /// # Parameters
    ///
    /// * `trim` - Whether strings should be trimmed.
    ///
    /// # Returns
    ///
    /// Updated options.
    #[must_use]
    pub fn with_trim(mut self, trim: bool) -> Self {
        self.trim = trim;
        self
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
    #[must_use]
    pub fn with_blank_string_policy(
        mut self,
        policy: BlankStringPolicy,
    ) -> Self {
        self.blank_string_policy = policy;
        self
    }

    /// Normalizes a string source according to these options.
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
}
