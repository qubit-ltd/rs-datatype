// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Builder for duration text options.

use super::DurationTextOptions;
use crate::duration::DurationUnitParseMode;
use crate::duration::SuffixlessDurationPolicy;

/// Builder for [`DurationTextOptions`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DurationTextOptionsBuilder {
    /// Duration text options being configured.
    pub(super) options: DurationTextOptions,
}

impl DurationTextOptionsBuilder {
    /// Creates a builder initialized with the documented defaults.
    ///
    /// # Returns
    ///
    /// A builder containing the default duration text options.
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            options: DurationTextOptions::default(),
        }
    }

    /// Configures suffixless input handling.
    ///
    /// # Parameters
    ///
    /// * `policy` - Policy to apply when duration text omits a unit suffix.
    ///
    /// # Returns
    ///
    /// The builder with the suffixless-input policy updated.
    #[inline]
    #[must_use]
    pub const fn suffixless_policy(self, policy: SuffixlessDurationPolicy) -> Self {
        Self {
            options: self.options.with_suffixless_policy(policy),
        }
    }

    /// Configures explicit unit parsing.
    ///
    /// # Parameters
    ///
    /// * `mode` - Strictness to apply when parsing explicit duration units.
    ///
    /// # Returns
    ///
    /// The builder with the explicit-unit parse mode updated.
    #[inline]
    #[must_use]
    pub const fn unit_parse_mode(self, mode: DurationUnitParseMode) -> Self {
        Self {
            options: self.options.with_unit_parse_mode(mode),
        }
    }

    /// Configures the maximum text size.
    ///
    /// # Parameters
    ///
    /// * `maximum` - Maximum source text length in bytes.
    ///
    /// # Returns
    ///
    /// The builder with the source text byte limit updated.
    #[inline]
    #[must_use]
    pub const fn max_text_bytes(self, maximum: usize) -> Self {
        Self {
            options: self.options.with_max_text_bytes(maximum),
        }
    }

    /// Builds the configured text options.
    ///
    /// # Returns
    ///
    /// The configured duration text options.
    #[inline]
    pub const fn build(self) -> DurationTextOptions {
        self.options
    }
}
