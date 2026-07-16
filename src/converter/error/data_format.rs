// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Data Format
//!
//! Identifies structured formats involved in conversion errors.

use serde::{
    Deserialize,
    Serialize,
};

/// Structured data format used by a conversion operation.
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DataFormat {
    /// JavaScript Object Notation.
    Json,
}

impl DataFormat {
    /// Returns the stable lowercase name of this format.
    #[inline]
    pub const fn as_str(self) -> &'static str {
        match self {
            DataFormat::Json => "json",
        }
    }
}
