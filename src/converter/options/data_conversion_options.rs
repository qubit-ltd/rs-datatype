// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Data Conversion Options
//!
//! Defines grouped options for common data conversion behavior.

use std::sync::LazyLock;

use serde::{
    Deserialize,
    Serialize,
};

use super::blank_string_policy::BlankStringPolicy;
use super::boolean_conversion_options::BooleanConversionOptions;
use super::collection_conversion_options::CollectionConversionOptions;
use super::duration_conversion_options::DurationConversionOptions;
use super::empty_item_policy::EmptyItemPolicy;
use super::numeric_conversion_policy::NumericConversionPolicy;
use super::string_conversion_options::StringConversionOptions;

/// Aggregates all policies used by the conversion engine.
///
/// Pass this value to [`crate::DataConverter::to_with`] when conversion rules
/// need to differ from the strict defaults. The nested option groups keep
/// string normalization, boolean literals, collection splitting, duration
/// units, and numeric precision independently configurable. The type is
/// serializable with Serde and missing serialized fields receive their group
/// defaults.
///
/// # Examples
///
/// ```
/// use qubit_datatype::{DataConversionOptions, DataConverter};
///
/// let options = DataConversionOptions::env_friendly();
/// assert_eq!(DataConverter::from(" yes ").to_with::<bool>(&options), Ok(true));
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct DataConversionOptions {
    /// Numeric precision and rounding behavior.
    pub numeric_policy: NumericConversionPolicy,
    /// String source conversion behavior.
    pub string: StringConversionOptions,
    /// Boolean string literal conversion behavior.
    pub boolean: BooleanConversionOptions,
    /// Scalar string collection conversion behavior.
    pub collection: CollectionConversionOptions,
    /// Duration conversion behavior.
    pub duration: DurationConversionOptions,
}

impl DataConversionOptions {
    /// Returns a shared reference to the default options.
    ///
    /// # Returns
    ///
    /// A process-wide lazily initialized default value.
    pub fn default_ref() -> &'static Self {
        static DEFAULT: LazyLock<DataConversionOptions> =
            LazyLock::new(DataConversionOptions::default);
        &DEFAULT
    }

    /// Creates options suitable for environment variable style values.
    ///
    /// # Returns
    ///
    /// Options that trim strings, treat blank scalar strings as missing, accept
    /// common boolean aliases, and split scalar strings on commas while
    /// skipping empty collection items.
    pub fn env_friendly() -> Self {
        Self {
            numeric_policy: NumericConversionPolicy::env_friendly(),
            string: StringConversionOptions::env_friendly(),
            boolean: BooleanConversionOptions::env_friendly(),
            collection: CollectionConversionOptions::env_friendly(),
            duration: DurationConversionOptions::env_friendly(),
        }
    }

    /// Returns a copy with a different numeric conversion policy.
    ///
    /// # Parameters
    ///
    /// * `numeric_policy` - Precision policy used for every numeric target.
    ///
    /// # Returns
    ///
    /// Returns the updated options value.
    #[must_use]
    pub fn with_numeric_policy(
        mut self,
        numeric_policy: NumericConversionPolicy,
    ) -> Self {
        self.numeric_policy = numeric_policy;
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
    pub fn with_string_options(
        mut self,
        string: StringConversionOptions,
    ) -> Self {
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
    pub fn with_boolean_options(
        mut self,
        boolean: BooleanConversionOptions,
    ) -> Self {
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
    pub fn with_collection_options(
        mut self,
        collection: CollectionConversionOptions,
    ) -> Self {
        self.collection = collection;
        self
    }

    /// Returns a copy with different duration conversion options.
    ///
    /// # Parameters
    ///
    /// * `duration` - New duration conversion options.
    ///
    /// # Returns
    ///
    /// Updated options.
    #[must_use]
    pub fn with_duration_options(
        mut self,
        duration: DurationConversionOptions,
    ) -> Self {
        self.duration = duration;
        self
    }
}
