// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Conversion Policy
//!
//! Defines grouped semantic policy for common data conversion behavior.

use std::sync::LazyLock;

use serde::Deserialize;
use serde::Serialize;

use super::blank_string_policy::BlankStringPolicy;
use super::boolean_conversion_policy::BooleanConversionPolicy;
use super::collection_conversion_policy::CollectionConversionPolicy;
use super::duration_conversion_policy::DurationConversionPolicy;
use super::duration_rounding_policy::DurationRoundingPolicy;
use super::empty_item_policy::EmptyItemPolicy;
use super::numeric_conversion_policy::NumericConversionPolicy;
use super::string_conversion_policy::StringConversionPolicy;

/// Aggregates all policies used by the conversion engine.
///
/// Pair this value with [`super::ConversionLimits`] when conversion rules need
/// to differ from the strict defaults. The nested policy groups keep
/// string normalization, boolean literals, collection splitting, duration
/// units, and numeric precision independently configurable without embedding
/// resource limits. Missing serialized fields receive their group defaults.
///
/// # Examples
///
/// ```
/// use qubit_datatype::ConversionPolicy;
///
/// let policy = ConversionPolicy::env_friendly();
/// assert_eq!(policy.boolean().parse("yes"), Some(true));
/// ```
#[must_use]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct ConversionPolicy {
    /// String source conversion behavior.
    string: StringConversionPolicy,

    /// Boolean string literal conversion behavior.
    boolean: BooleanConversionPolicy,

    /// Numeric precision and rounding behavior.
    numeric: NumericConversionPolicy,

    /// Scalar string collection conversion behavior.
    collection: CollectionConversionPolicy,

    /// Duration conversion behavior.
    duration: DurationConversionPolicy,
}

impl ConversionPolicy {
    /// Creates the strict conversion profile used by [`Default`].
    ///
    /// The profile requires exact numeric and duration conversions, preserves
    /// string whitespace and blank strings, accepts the default Boolean
    /// literals and numeric Boolean policy, does not split scalar strings into
    /// collections, and uses the default millisecond Duration representation.
    /// Use this profile where changing or discarding source information would
    /// be surprising.
    ///
    /// # Returns
    ///
    /// Strict policy equal to [`Self::default`].
    ///
    /// # Examples
    ///
    /// ```
    /// use qubit_datatype::ConversionPolicy;
    ///
    /// assert_eq!(ConversionPolicy::strict(), ConversionPolicy::default());
    /// ```
    #[inline]
    pub fn strict() -> Self {
        Self {
            string: StringConversionPolicy::default(),
            boolean: BooleanConversionPolicy::strict(),
            numeric: NumericConversionPolicy::strict(),
            collection: CollectionConversionPolicy::default(),
            duration: DurationConversionPolicy::default(),
        }
    }

    /// Creates a profile that permits precision loss and trims string input.
    ///
    /// Compared with [`Self::strict`], this profile permits fractional
    /// truncation, floating-point rounding, and Duration half-up rounding, and
    /// trims string input. Blank strings remain preserved while Boolean and
    /// collection rules remain strict.
    ///
    /// # Returns
    ///
    /// Lossy, whitespace-trimming conversion policy.
    ///
    /// # Examples
    ///
    /// ```
    /// use qubit_datatype::ConversionPolicy;
    ///
    /// let policy = ConversionPolicy::lossy();
    /// assert!(policy.string().trim());
    /// ```
    #[inline]
    pub fn lossy() -> Self {
        Self {
            string: StringConversionPolicy::default().with_trim(true),
            boolean: BooleanConversionPolicy::strict(),
            numeric: NumericConversionPolicy::lossy(),
            collection: CollectionConversionPolicy::default(),
            duration: DurationConversionPolicy::default()
                .with_rounding_policy(DurationRoundingPolicy::HalfUp),
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
    /// existing-numeric-to-float conversions remain exact.
    #[inline]
    pub fn env_friendly() -> Self {
        Self {
            string: StringConversionPolicy::env_friendly(),
            boolean: BooleanConversionPolicy::env_friendly(),
            numeric: NumericConversionPolicy::env_friendly(),
            collection: CollectionConversionPolicy::env_friendly(),
            duration: DurationConversionPolicy::env_friendly(),
        }
    }

    /// Returns a shared reference to the default policy.
    ///
    /// # Returns
    ///
    /// A process-wide lazily initialized default value.
    ///
    /// ```compile_fail
    /// #![deny(unused_must_use)]
    /// use qubit_datatype::ConversionPolicy;
    ///
    /// let policy = ConversionPolicy::default();
    /// ConversionPolicy::default_ref();
    /// policy.numeric();
    /// policy.string();
    /// policy.boolean();
    /// policy.collection();
    /// policy.duration();
    /// ```
    #[must_use = "the default conversion policy should be inspected"]
    #[inline(always)]
    pub fn default_ref() -> &'static Self {
        static DEFAULT: LazyLock<ConversionPolicy> =
            LazyLock::new(ConversionPolicy::default);
        &DEFAULT
    }

    /// Returns the numeric conversion policy.
    ///
    /// # Returns
    ///
    /// A shared reference to the numeric policy group.
    #[must_use = "numeric conversion policy should be inspected"]
    #[inline(always)]
    pub const fn numeric(&self) -> &NumericConversionPolicy {
        &self.numeric
    }

    /// Returns a copy with different numeric conversion policy.
    ///
    /// # Parameters
    ///
    /// * `numeric` - New numeric conversion policy.
    ///
    /// # Returns
    ///
    /// Returns the updated options value.
    #[inline(always)]
    pub fn with_numeric_policy(
        mut self,
        numeric: NumericConversionPolicy,
    ) -> Self {
        self.numeric = numeric;
        self
    }

    /// Returns the string conversion policy.
    ///
    /// # Returns
    ///
    /// A shared reference to the string normalization policies.
    #[must_use = "string conversion policy should be inspected"]
    #[inline(always)]
    pub const fn string(&self) -> &StringConversionPolicy {
        &self.string
    }

    /// Returns a copy with different string conversion policy.
    ///
    /// # Parameters
    ///
    /// * `string` - New string conversion policy.
    ///
    /// # Returns
    ///
    /// Updated options.
    #[inline(always)]
    pub fn with_string_policy(
        mut self,
        string: StringConversionPolicy,
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

    /// Returns the Boolean conversion policy.
    ///
    /// # Returns
    ///
    /// A shared reference to the Boolean conversion policies.
    #[must_use = "Boolean conversion policy should be inspected"]
    #[inline(always)]
    pub const fn boolean(&self) -> &BooleanConversionPolicy {
        &self.boolean
    }

    /// Returns a copy with different Boolean conversion policy.
    ///
    /// # Parameters
    ///
    /// * `boolean` - New boolean conversion policy.
    ///
    /// # Returns
    ///
    /// Updated options.
    #[inline(always)]
    pub fn with_boolean_policy(
        mut self,
        boolean: BooleanConversionPolicy,
    ) -> Self {
        self.boolean = boolean;
        self
    }

    /// Returns the collection conversion policy.
    ///
    /// # Returns
    ///
    /// A shared reference to the scalar collection policies.
    #[must_use = "collection conversion policy should be inspected"]
    #[inline(always)]
    pub const fn collection(&self) -> &CollectionConversionPolicy {
        &self.collection
    }

    /// Returns a copy with different collection conversion policy.
    ///
    /// # Parameters
    ///
    /// * `collection` - New collection conversion policy.
    ///
    /// # Returns
    ///
    /// Updated options.
    #[inline(always)]
    pub fn with_collection_policy(
        mut self,
        collection: CollectionConversionPolicy,
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

    /// Returns the Duration conversion policy.
    ///
    /// # Returns
    ///
    /// A shared reference to the Duration conversion policies.
    #[must_use = "Duration conversion policy should be inspected"]
    #[inline(always)]
    pub const fn duration(&self) -> &DurationConversionPolicy {
        &self.duration
    }

    /// Returns a copy with different duration conversion policy.
    ///
    /// # Parameters
    ///
    /// * `duration` - New duration conversion policy.
    ///
    /// # Returns
    ///
    /// Updated options.
    #[inline(always)]
    pub fn with_duration_policy(
        mut self,
        duration: DurationConversionPolicy,
    ) -> Self {
        self.duration = duration;
        self
    }
}

impl Default for ConversionPolicy {
    /// Creates the strict default conversion profile.
    ///
    /// # Returns
    ///
    /// The strict conversion profile.
    #[inline(always)]
    fn default() -> Self {
        Self::strict()
    }
}
