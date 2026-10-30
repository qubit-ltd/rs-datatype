// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Safe categories for JSON decoding failures.

/// A value-free category for a JSON decoding failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum JsonConversionErrorCategory {
    /// Reading the JSON input failed.
    Read,
    /// The JSON input was empty.
    EmptyInput,
    /// The JSON input was not valid UTF-8.
    InvalidUtf8,
    /// The JSON syntax was invalid.
    InvalidJson,
    /// The JSON value had an unexpected top-level shape.
    UnexpectedTopLevel,
    /// The JSON value could not be materialized as the requested target.
    Deserialize,
}
