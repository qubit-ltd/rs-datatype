/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Collection Conversion Options
//!
//! Defines options that control scalar-string-to-collection conversion.
//!
//! # Author
//!
//! Haixing Hu

use super::data_conversion_error::DataConversionError;
use super::data_list_conversion_error::DataListConversionError;
use super::data_list_conversion_result::DataListConversionResult;
use super::empty_item_policy::EmptyItemPolicy;

/// Options that control scalar-string-to-collection conversion.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CollectionConversionOptions {
    /// Whether a scalar string can be split into collection items.
    pub split_scalar_strings: bool,
    /// Delimiters used to split scalar strings.
    pub delimiters: Vec<char>,
    /// Whether split items are trimmed before element conversion.
    pub trim_items: bool,
    /// How empty split items are interpreted.
    pub empty_item_policy: EmptyItemPolicy,
}

impl Default for CollectionConversionOptions {
    /// Creates default collection conversion options.
    fn default() -> Self {
        Self {
            split_scalar_strings: false,
            delimiters: vec![','],
            trim_items: false,
            empty_item_policy: EmptyItemPolicy::Keep,
        }
    }
}

impl CollectionConversionOptions {
    /// Returns a copy with scalar string splitting enabled or disabled.
    ///
    /// # Parameters
    ///
    /// * `split_scalar_strings` - Whether scalar strings should be split.
    ///
    /// # Returns
    ///
    /// Updated options.
    #[must_use]
    pub fn with_split_scalar_strings(mut self, split_scalar_strings: bool) -> Self {
        self.split_scalar_strings = split_scalar_strings;
        self
    }

    /// Returns a copy with different scalar string delimiters.
    ///
    /// # Parameters
    ///
    /// * `delimiters` - Delimiters used when splitting is enabled.
    ///
    /// # Returns
    ///
    /// Updated options.
    #[must_use]
    pub fn with_delimiters(mut self, delimiters: impl IntoIterator<Item = char>) -> Self {
        self.delimiters = delimiters.into_iter().collect();
        self
    }

    /// Returns a copy with per-item trimming enabled or disabled.
    ///
    /// # Parameters
    ///
    /// * `trim_items` - Whether split items should be trimmed.
    ///
    /// # Returns
    ///
    /// Updated options.
    #[must_use]
    pub fn with_trim_items(mut self, trim_items: bool) -> Self {
        self.trim_items = trim_items;
        self
    }

    /// Returns a copy with a different empty item policy.
    ///
    /// # Parameters
    ///
    /// * `policy` - New empty item policy.
    ///
    /// # Returns
    ///
    /// Updated options.
    #[must_use]
    pub fn with_empty_item_policy(mut self, policy: EmptyItemPolicy) -> Self {
        self.empty_item_policy = policy;
        self
    }

    /// Splits and normalizes a scalar string into collection items.
    ///
    /// # Parameters
    ///
    /// * `value` - Normalized scalar string.
    ///
    /// # Returns
    ///
    /// Returns collection items after optional splitting, per-item trimming,
    /// and empty item handling.
    ///
    /// # Errors
    ///
    /// Returns [`DataListConversionError`] when an empty item is rejected.
    pub fn scalar_items(&self, value: &str) -> DataListConversionResult<Vec<String>> {
        let raw_items: Vec<&str> = if self.split_scalar_strings {
            value.split(|c| self.delimiters.contains(&c)).collect()
        } else {
            vec![value]
        };
        let mut items = Vec::with_capacity(raw_items.len());
        for (index, item) in raw_items.into_iter().enumerate() {
            let item = if self.trim_items { item.trim() } else { item };
            if item.is_empty() {
                match self.empty_item_policy {
                    EmptyItemPolicy::Keep => {}
                    EmptyItemPolicy::Skip => continue,
                    EmptyItemPolicy::Reject => {
                        return Err(DataListConversionError {
                            index,
                            source: DataConversionError::ConversionError(
                                "empty collection item is not allowed".to_string(),
                            ),
                        });
                    }
                }
            }
            items.push(item.to_string());
        }
        Ok(items)
    }
}
