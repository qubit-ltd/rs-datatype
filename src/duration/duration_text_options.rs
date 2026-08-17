// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Options for Duration text parsing.
// qubit-style: allow multiple-public-types

use serde::Deserialize;
use serde::Serialize;

use super::DurationUnitParseMode;
use super::SuffixlessDurationPolicy;

/// Controls suffixless input and Duration unit parsing.
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct DurationTextOptions {
    /// Policy applied when text omits a unit suffix.
    suffixless_policy: SuffixlessDurationPolicy,
    /// Strictness applied to explicit unit symbols.
    unit_parse_mode: DurationUnitParseMode,
    /// Maximum accepted source text length in bytes.
    max_text_bytes: usize,
}

impl DurationTextOptions {
    /// Creates a builder initialized with the default text options.
    #[inline]
    #[must_use]
    pub fn builder() -> DurationTextOptionsBuilder {
        DurationTextOptionsBuilder::new()
    }

    /// Converts these options into a builder for explicit customization.
    #[inline]
    #[must_use]
    pub fn into_builder(self) -> DurationTextOptionsBuilder {
        DurationTextOptionsBuilder { options: self }
    }
    /// Default maximum accepted Duration source text length in bytes.
    pub const DEFAULT_MAX_TEXT_BYTES: usize = 256;

    /// Creates Duration text options from independent parsing policies.
    ///
    /// # Parameters
    ///
    /// * `suffixless_policy` - Policy for text without a unit suffix.
    /// * `unit_parse_mode` - Strictness applied to explicit unit symbols.
    ///
    /// # Returns
    ///
    /// The configured options.
    #[inline(always)]
    pub const fn new(
        suffixless_policy: SuffixlessDurationPolicy,
        unit_parse_mode: DurationUnitParseMode,
    ) -> Self {
        Self {
            suffixless_policy,
            unit_parse_mode,
            max_text_bytes: Self::DEFAULT_MAX_TEXT_BYTES,
        }
    }

    /// Returns the policy applied when text omits a unit suffix.
    ///
    /// # Returns
    ///
    /// The configured suffixless-input policy.
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
    pub(crate) const fn with_suffixless_policy(
        mut self,
        suffixless_policy: SuffixlessDurationPolicy,
    ) -> Self {
        self.suffixless_policy = suffixless_policy;
        self
    }

    /// Returns the strictness applied to explicit unit symbols.
    ///
    /// # Returns
    ///
    /// The configured strict or lenient unit parse mode.
    #[inline(always)]
    pub const fn unit_parse_mode(&self) -> DurationUnitParseMode {
        self.unit_parse_mode
    }

    /// Returns a copy with a different unit parse mode.
    ///
    /// # Parameters
    ///
    /// * `unit_parse_mode` - Replacement unit parse mode.
    ///
    /// # Returns
    ///
    /// Updated options.
    #[inline(always)]
    pub(crate) const fn with_unit_parse_mode(
        mut self,
        unit_parse_mode: DurationUnitParseMode,
    ) -> Self {
        self.unit_parse_mode = unit_parse_mode;
        self
    }

    /// Returns the maximum accepted source text length in bytes.
    ///
    /// # Returns
    ///
    /// The configured source text byte limit.
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
    pub(crate) const fn with_max_text_bytes(mut self, maximum: usize) -> Self {
        self.max_text_bytes = maximum;
        self
    }
}

/// Builder for [`DurationTextOptions`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DurationTextOptionsBuilder {
    options: DurationTextOptions,
}

impl DurationTextOptionsBuilder {
    /// Creates a builder initialized with the documented defaults.
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            options: DurationTextOptions::default(),
        }
    }
    /// Configures suffixless input handling.
    #[inline(always)]
    #[must_use]
    pub const fn suffixless_policy(
        self,
        policy: SuffixlessDurationPolicy,
    ) -> Self {
        Self {
            options: self.options.with_suffixless_policy(policy),
        }
    }
    /// Configures explicit unit parsing.
    #[inline(always)]
    #[must_use]
    pub const fn unit_parse_mode(self, mode: DurationUnitParseMode) -> Self {
        Self {
            options: self.options.with_unit_parse_mode(mode),
        }
    }
    /// Configures the maximum text size.
    #[inline(always)]
    #[must_use]
    pub const fn max_text_bytes(self, maximum: usize) -> Self {
        Self {
            options: self.options.with_max_text_bytes(maximum),
        }
    }
    /// Builds the configured text options.
    #[inline]
    #[must_use]
    pub const fn build(self) -> DurationTextOptions {
        self.options
    }
}

impl Default for DurationTextOptions {
    /// Creates the default profile that rejects suffixless text and uses strict
    /// unit parsing.
    ///
    /// # Returns
    ///
    /// Strict options using the default source text byte limit.
    #[inline]
    fn default() -> Self {
        Self::new(
            SuffixlessDurationPolicy::Reject,
            DurationUnitParseMode::Strict,
        )
    }
}
