/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # String Conversion Options
//!
//! Defines options that control string-source normalization.
//!
//! # Author
//!
//! Haixing Hu

use super::blank_string_policy::BlankStringPolicy;
use super::data_conversion_error::DataConversionError;
use super::data_conversion_result::DataConversionResult;

/// Options that control string-source normalization.
#[derive(Debug, Clone, PartialEq, Eq)]
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
    pub fn with_blank_string_policy(mut self, policy: BlankStringPolicy) -> Self {
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
    /// Returns [`DataConversionError::NoValue`] when blank strings are treated
    /// as missing, or [`DataConversionError::ConversionError`] when blank
    /// strings are rejected.
    pub fn normalize(&self, value: &str) -> DataConversionResult<String> {
        let value = if self.trim { value.trim() } else { value };
        if value.trim().is_empty() {
            match self.blank_string_policy {
                BlankStringPolicy::Preserve => Ok(value.to_string()),
                BlankStringPolicy::TreatAsMissing => Err(DataConversionError::NoValue),
                BlankStringPolicy::Reject => Err(DataConversionError::ConversionError(
                    "blank string is not allowed".to_string(),
                )),
            }
        } else {
            Ok(value.to_string())
        }
    }
}
