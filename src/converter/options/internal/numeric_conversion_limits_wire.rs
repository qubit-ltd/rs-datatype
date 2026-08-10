// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Private Serde wire representation for numeric conversion limits.

use serde::Deserialize;
use serde::Serialize;

use super::super::NumericConversionLimits;

#[derive(Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub(crate) struct NumericConversionLimitsWire {
    pub(crate) max_text_bytes: usize,
    pub(crate) max_big_integer_digits: usize,
}

impl Default for NumericConversionLimitsWire {
    fn default() -> Self {
        let limits = NumericConversionLimits::default();
        Self {
            max_text_bytes: limits.max_text_bytes(),
            max_big_integer_digits: limits.max_big_integer_digits(),
        }
    }
}
