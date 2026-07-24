// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Unvalidated Serde representation for Boolean conversion options.

use serde::Deserialize;

use super::super::{
    BooleanConversionOptions,
    BooleanNumericPolicy,
};

/// Holds deserialized Boolean fields before literal-set validation.
#[must_use]
#[derive(Deserialize)]
#[serde(default, deny_unknown_fields)]
pub(in crate::converter::options) struct UncheckedBooleanConversionOptions {
    /// String literals accepted as true.
    pub(in crate::converter::options) true_literals: Vec<String>,
    /// String literals accepted as false.
    pub(in crate::converter::options) false_literals: Vec<String>,
    /// Whether matching is case-sensitive.
    pub(in crate::converter::options) case_sensitive: bool,
    /// Numeric Boolean policy.
    pub(in crate::converter::options) numeric_policy: BooleanNumericPolicy,
}

impl Default for UncheckedBooleanConversionOptions {
    /// Creates the wire defaults used by [`BooleanConversionOptions`].
    ///
    /// # Returns
    ///
    /// An unchecked representation populated from validated defaults.
    #[inline]
    fn default() -> Self {
        let options = BooleanConversionOptions::default();
        Self {
            true_literals: options.true_literals().to_vec(),
            false_literals: options.false_literals().to_vec(),
            case_sensitive: options.case_sensitive(),
            numeric_policy: options.numeric_policy(),
        }
    }
}
