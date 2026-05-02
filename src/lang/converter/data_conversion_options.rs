/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Data Conversion Options
//!
//! Defines grouped options for common data conversion behavior.
//!
//! # Author
//!
//! Haixing Hu

use super::blank_string_policy::BlankStringPolicy;
use super::boolean_conversion_options::BooleanConversionOptions;
use super::collection_conversion_options::CollectionConversionOptions;
use super::empty_item_policy::EmptyItemPolicy;
use super::string_conversion_options::StringConversionOptions;

/// Options that control common data conversion behavior.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct DataConversionOptions {
    /// String source conversion behavior.
    pub string: StringConversionOptions,
    /// Boolean string literal conversion behavior.
    pub boolean: BooleanConversionOptions,
    /// Scalar string collection conversion behavior.
    pub collection: CollectionConversionOptions,
}

impl DataConversionOptions {
    /// Creates options suitable for environment variable style values.
    ///
    /// # Returns
    ///
    /// Options that trim strings, treat blank scalar strings as missing, accept
    /// common boolean aliases, and split scalar strings on commas while
    /// skipping empty collection items.
    pub fn env_friendly() -> Self {
        Self {
            string: StringConversionOptions {
                trim: true,
                blank_string_policy: BlankStringPolicy::TreatAsMissing,
            },
            boolean: BooleanConversionOptions::env_friendly(),
            collection: CollectionConversionOptions {
                split_scalar_strings: true,
                delimiters: vec![','],
                trim_items: true,
                empty_item_policy: EmptyItemPolicy::Skip,
            },
        }
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
        self.string = self.string.with_blank_string_policy(policy);
        self
    }

    /// Returns a copy with a different empty collection item policy.
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
        self.collection = self.collection.with_empty_item_policy(policy);
        self
    }

    /// Returns a copy with different string conversion options.
    ///
    /// # Parameters
    ///
    /// * `string` - New string conversion options.
    ///
    /// # Returns
    ///
    /// Updated options.
    #[must_use]
    pub fn with_string_options(mut self, string: StringConversionOptions) -> Self {
        self.string = string;
        self
    }

    /// Returns a copy with different boolean conversion options.
    ///
    /// # Parameters
    ///
    /// * `boolean` - New boolean conversion options.
    ///
    /// # Returns
    ///
    /// Updated options.
    #[must_use]
    pub fn with_boolean_options(mut self, boolean: BooleanConversionOptions) -> Self {
        self.boolean = boolean;
        self
    }

    /// Returns a copy with different collection conversion options.
    ///
    /// # Parameters
    ///
    /// * `collection` - New collection conversion options.
    ///
    /// # Returns
    ///
    /// Updated options.
    #[must_use]
    pub fn with_collection_options(mut self, collection: CollectionConversionOptions) -> Self {
        self.collection = collection;
        self
    }
}
