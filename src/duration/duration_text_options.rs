// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Options for Duration text parsing.

use serde::{
    Deserialize,
    Serialize,
};

use super::{
    DurationUnitSuffixSet,
    SuffixlessDurationPolicy,
};

/// Controls suffixless input and supported unit suffixes.
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct DurationTextOptions {
    /// Policy applied when text omits a unit suffix.
    suffixless_policy: SuffixlessDurationPolicy,
    /// Set of explicit unit suffixes accepted by the parser.
    unit_suffix_set: DurationUnitSuffixSet,
    /// Maximum accepted source text length in bytes.
    max_text_bytes: usize,
}

impl DurationTextOptions {
    /// Default maximum accepted Duration source text length in bytes.
    pub const DEFAULT_MAX_TEXT_BYTES: usize = 1_048_576;

    /// Creates Duration text options from independent parsing policies.
    ///
    /// # Parameters
    ///
    /// * `suffixless_policy` - Policy for text without a unit suffix.
    /// * `unit_suffix_set` - Set of explicit suffixes accepted by the parser.
    ///
    /// # Returns
    ///
    /// The configured options.
    #[inline(always)]
    pub const fn new(
        suffixless_policy: SuffixlessDurationPolicy,
        unit_suffix_set: DurationUnitSuffixSet,
    ) -> Self {
        Self {
            suffixless_policy,
            unit_suffix_set,
            max_text_bytes: Self::DEFAULT_MAX_TEXT_BYTES,
        }
    }

    /// Returns the policy applied when text omits a unit suffix.
    #[inline(always)]
    pub const fn suffixless_policy(&self) -> SuffixlessDurationPolicy {
        self.suffixless_policy
    }

    /// Returns a copy with a different suffixless-input policy.
    ///
    /// # Parameters
    ///
    /// * `suffixless_policy` - Replacement suffixless-input policy.
    ///
    /// # Returns
    ///
    /// Updated options.
    #[inline(always)]
    pub const fn with_suffixless_policy(
        mut self,
        suffixless_policy: SuffixlessDurationPolicy,
    ) -> Self {
        self.suffixless_policy = suffixless_policy;
        self
    }

    /// Returns the set of explicit unit suffixes accepted by the parser.
    #[inline(always)]
    pub const fn unit_suffix_set(&self) -> DurationUnitSuffixSet {
        self.unit_suffix_set
    }

    /// Returns a copy with a different accepted suffix set.
    ///
    /// # Parameters
    ///
    /// * `unit_suffix_set` - Replacement explicit suffix set.
    ///
    /// # Returns
    ///
    /// Updated options.
    #[inline(always)]
    pub const fn with_unit_suffix_set(
        mut self,
        unit_suffix_set: DurationUnitSuffixSet,
    ) -> Self {
        self.unit_suffix_set = unit_suffix_set;
        self
    }

    /// Returns the maximum accepted source text length in bytes.
    ///
    /// ```compile_fail
    /// #![deny(unused_must_use)]
    /// use qubit_datatype::DurationTextOptions;
    ///
    /// DurationTextOptions::default().max_text_bytes();
    /// ```
    #[must_use]
    #[inline(always)]
    pub const fn max_text_bytes(&self) -> usize {
        self.max_text_bytes
    }

    /// Returns a copy with a different source text byte limit.
    ///
    /// # Parameters
    ///
    /// * `maximum` - Maximum accepted source text length in bytes.
    ///
    /// # Returns
    ///
    /// Updated options.
    #[inline(always)]
    pub const fn with_max_text_bytes(mut self, maximum: usize) -> Self {
        self.max_text_bytes = maximum;
        self
    }
}

impl Default for DurationTextOptions {
    /// Creates the extended, suffixless-millisecond compatibility profile.
    fn default() -> Self {
        Self::new(
            SuffixlessDurationPolicy::default(),
            DurationUnitSuffixSet::default(),
        )
    }
}
