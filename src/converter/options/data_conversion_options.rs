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
use super::duration_rounding_policy::DurationRoundingPolicy;
use super::empty_item_policy::EmptyItemPolicy;
use super::numeric_conversion_options::NumericConversionOptions;
use super::string_conversion_options::StringConversionOptions;
use super::structured_conversion_limits::StructuredConversionLimits;

/// Aggregates all policies used by the conversion engine.
///
/// Pass this value to [`crate::DataConverter::to_with`] when conversion rules
/// need to differ from the strict defaults. The nested option groups keep
/// string normalization, boolean literals, collection splitting, duration
/// units, numeric precision, and structured text limits independently
/// configurable. The type is serializable with Serde and missing serialized
/// fields receive their group defaults.
///
/// # Examples
///
/// ```
/// use qubit_datatype::{DataConversionOptions, DataConverter};
///
/// let options = DataConversionOptions::env_friendly();
/// assert_eq!(DataConverter::from(" yes ").to_with::<bool>(&options), Ok(true));
/// ```
#[must_use]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct DataConversionOptions {
    /// Numeric precision and rounding behavior.
    numeric: NumericConversionOptions,
    /// String source conversion behavior.
    string: StringConversionOptions,
    /// Boolean string literal conversion behavior.
    boolean: BooleanConversionOptions,
    /// Scalar string collection conversion behavior.
    collection: CollectionConversionOptions,
    /// Duration conversion behavior.
    duration: DurationConversionOptions,
    /// Structured text conversion resource limits.
    structured: StructuredConversionLimits,
}

impl DataConversionOptions {
    /// Creates the strict conversion profile used by [`Default`].
    ///
    /// The profile requires exact numeric and duration conversions, preserves
    /// string whitespace and blank strings, accepts the default Boolean
    /// literals and numeric Boolean policy, does not split scalar strings into
    /// collections, uses the default millisecond Duration representation, and
    /// applies the default structured text limit. Use this profile for generic
    /// library conversions where changing or
    /// discarding source information would be surprising.
    ///
    /// # Returns
    ///
    /// Strict options equal to [`Self::default`].
    ///
    /// # Examples
    ///
    /// ```
    /// use qubit_datatype::{DataConversionOptions, DataConverter};
    ///
    /// let options = DataConversionOptions::strict();
    /// assert!(DataConverter::from("3.9").to_with::<i32>(&options).is_err());
    /// ```
    pub fn strict() -> Self {
        Self {
            numeric: NumericConversionOptions::strict(),
            string: StringConversionOptions::default(),
            boolean: BooleanConversionOptions::strict(),
            collection: CollectionConversionOptions::default(),
            duration: DurationConversionOptions::default(),
            structured: StructuredConversionLimits::default(),
        }
    }

    /// Creates a profile that permits precision loss and trims string input.
    ///
    /// Compared with [`Self::strict`], this profile permits fractional
    /// truncation, floating-point rounding, and Duration half-up rounding, and
    /// trims string input. Blank strings remain preserved while Boolean and
    /// collection rules remain strict and structured text keeps its default
    /// limit.
    ///
    /// # Returns
    ///
    /// Lossy, whitespace-trimming conversion options.
    ///
    /// # Examples
    ///
    /// ```
    /// use qubit_datatype::{DataConversionOptions, DataConverter};
    ///
    /// let options = DataConversionOptions::lossy();
    /// assert_eq!(DataConverter::from(" 3.9 ").to_with::<i32>(&options), Ok(3));
    /// ```
    pub fn lossy() -> Self {
        Self {
            numeric: NumericConversionOptions::lossy(),
            string: StringConversionOptions::default().with_trim(true),
            boolean: BooleanConversionOptions::strict(),
            collection: CollectionConversionOptions::default(),
            duration: DurationConversionOptions::default()
                .with_rounding_policy(DurationRoundingPolicy::HalfUp),
            structured: StructuredConversionLimits::default(),
        }
    }

    /// Creates options suitable for environment variable style values.
    ///
    /// # Returns
    ///
    /// Options that trim strings, treat blank scalar strings as missing, accept
    /// common boolean aliases, and split scalar strings on commas while
    /// skipping empty collection items. Text-to-float conversion permits IEEE
    /// nearest-even rounding, while fractional-to-integer and
    /// existing-numeric-to-float conversions remain exact. Structured text
    /// keeps its default limit.
    pub fn env_friendly() -> Self {
        Self {
            numeric: NumericConversionOptions::env_friendly(),
            string: StringConversionOptions::env_friendly(),
            boolean: BooleanConversionOptions::env_friendly(),
            collection: CollectionConversionOptions::env_friendly(),
            duration: DurationConversionOptions::env_friendly(),
            structured: StructuredConversionLimits::default(),
        }
    }

    /// Returns a shared reference to the default options.
    ///
    /// # Returns
    ///
    /// A process-wide lazily initialized default value.
    #[inline(always)]
    pub fn default_ref() -> &'static Self {
        static DEFAULT: LazyLock<DataConversionOptions> =
            LazyLock::new(DataConversionOptions::default);
        &DEFAULT
    }

    /// Returns the numeric conversion options.
    #[inline(always)]
    pub const fn numeric(&self) -> &NumericConversionOptions {
        &self.numeric
    }

    /// Returns a copy with different numeric conversion options.
    ///
    /// # Parameters
    ///
    /// * `numeric` - New numeric conversion options.
    ///
    /// # Returns
    ///
    /// Returns the updated options value.
    #[inline(always)]
    pub fn with_numeric_options(
        mut self,
        numeric: NumericConversionOptions,
    ) -> Self {
        self.numeric = numeric;
        self
    }

    /// Returns the string conversion options.
    #[inline(always)]
    pub const fn string(&self) -> &StringConversionOptions {
        &self.string
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
    #[inline(always)]
    pub fn with_string_options(
        mut self,
        string: StringConversionOptions,
    ) -> Self {
        self.string = string;
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
    #[inline(always)]
    pub fn with_blank_string_policy(
        mut self,
        policy: BlankStringPolicy,
    ) -> Self {
        self.string = self.string.with_blank_string_policy(policy);
        self
    }

    /// Returns the Boolean conversion options.
    #[inline(always)]
    pub const fn boolean(&self) -> &BooleanConversionOptions {
        &self.boolean
    }

    /// Returns a copy with different Boolean conversion options.
    ///
    /// # Parameters
    ///
    /// * `boolean` - New boolean conversion options.
    ///
    /// # Returns
    ///
    /// Updated options.
    #[inline(always)]
    pub fn with_boolean_options(
        mut self,
        boolean: BooleanConversionOptions,
    ) -> Self {
        self.boolean = boolean;
        self
    }

    /// Returns the collection conversion options.
    #[inline(always)]
    pub const fn collection(&self) -> &CollectionConversionOptions {
        &self.collection
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
    #[inline(always)]
    pub fn with_collection_options(
        mut self,
        collection: CollectionConversionOptions,
    ) -> Self {
        self.collection = collection;
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
    #[inline(always)]
    pub fn with_empty_item_policy(mut self, policy: EmptyItemPolicy) -> Self {
        self.collection = self.collection.with_empty_item_policy(policy);
        self
    }

    /// Returns the Duration conversion options.
    #[inline(always)]
    pub const fn duration(&self) -> &DurationConversionOptions {
        &self.duration
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
    #[inline(always)]
    pub fn with_duration_options(
        mut self,
        duration: DurationConversionOptions,
    ) -> Self {
        self.duration = duration;
        self
    }

    /// Returns the structured text conversion resource limits.
    #[inline(always)]
    pub const fn structured(&self) -> &StructuredConversionLimits {
        &self.structured
    }

    /// Returns a copy with different structured text conversion resource
    /// limits.
    ///
    /// # Parameters
    ///
    /// * `structured` - New structured text resource limits.
    ///
    /// # Returns
    ///
    /// Updated options.
    #[inline(always)]
    pub fn with_structured_limits(
        mut self,
        structured: StructuredConversionLimits,
    ) -> Self {
        self.structured = structured;
        self
    }
}

impl Default for DataConversionOptions {
    /// Creates the strict default conversion profile.
    #[inline(always)]
    fn default() -> Self {
        Self::strict()
    }
}
