// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Type-erased bridge for conversion String accounting.

use std::fmt;
use std::fmt::Write;

use qubit_budget::BudgetedStringWriter;

use super::super::conversion_resource::ConversionResource;

/// Keeps the accounting writer out of the public API while preserving
/// streaming output.
pub(crate) trait ConversionStringSink {
    /// Writes one string slice through the underlying formatter.
    fn write_str(&mut self, value: &str) -> fmt::Result;

    /// Writes one Unicode scalar through the underlying formatter.
    fn write_char(&mut self, value: char) -> fmt::Result;

    /// Writes formatting arguments through the underlying formatter.
    fn write_fmt(&mut self, arguments: fmt::Arguments<'_>) -> fmt::Result;
}

impl ConversionStringSink for BudgetedStringWriter<'_, ConversionResource> {
    /// Forwards string slices to the budgeted formatter.
    #[inline]
    fn write_str(&mut self, value: &str) -> fmt::Result {
        self.as_fmt().write_str(value)
    }

    /// Forwards one Unicode scalar to the budgeted formatter.
    #[inline]
    fn write_char(&mut self, value: char) -> fmt::Result {
        self.as_fmt().write_char(value)
    }

    /// Forwards formatting arguments to the budgeted formatter.
    #[inline]
    fn write_fmt(&mut self, arguments: fmt::Arguments<'_>) -> fmt::Result {
        self.as_fmt().write_fmt(arguments)
    }
}
