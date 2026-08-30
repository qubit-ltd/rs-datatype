// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Budget-aware text output for custom conversion targets.

use std::fmt;

use qubit_budget::BudgetedStringWriter;

use super::conversion_resource::ConversionResource;
use super::error::DataConversionError;
use super::error::InvalidValueReason;
use super::internal::ConversionStringSink;
use crate::datatype::DataType;

/// Writes one conversion result into a session-owned, budgeted String.
///
/// Values of this type are supplied by [`ConversionSession::write_string`]
/// and cannot outlive that call. Bytes are committed to the session only when
/// the complete rendering operation succeeds.
///
/// [`ConversionSession::write_string`]: super::ConversionSession::write_string
pub struct ConversionStringWriter<'writer> {
    inner: &'writer mut dyn ConversionStringSink,
    from: DataType,
    to: DataType,
}

impl<'writer> ConversionStringWriter<'writer> {
    #[inline(always)]
    pub(crate) fn new<'budget>(
        inner: &'writer mut BudgetedStringWriter<'budget, ConversionResource>,
        from: DataType,
        to: DataType,
    ) -> Self {
        Self { inner, from, to }
    }

    /// Writes a UTF-8 string slice.
    ///
    /// # Errors
    ///
    /// Returns an invalid-value error if the underlying formatter rejects the
    /// write. Budget failures are reported by the enclosing
    /// [`ConversionSession::write_string`] call.
    ///
    /// [`ConversionSession::write_string`]: super::ConversionSession::write_string
    #[inline]
    pub fn write_str(&mut self, value: &str) -> Result<(), DataConversionError> {
        self.inner
            .write_str(value)
            .map_err(|_| DataConversionError::invalid(self.from, self.to, InvalidValueReason::OutOfRange))
    }

    /// Writes one Unicode scalar value.
    ///
    /// # Errors
    ///
    /// Returns the same errors as [`Self::write_str`].
    #[inline]
    pub fn write_char(&mut self, value: char) -> Result<(), DataConversionError> {
        self.inner
            .write_char(value)
            .map_err(|_| DataConversionError::invalid(self.from, self.to, InvalidValueReason::OutOfRange))
    }

    /// Writes pre-built formatting arguments without an intermediate String.
    ///
    /// # Errors
    ///
    /// Returns the same errors as [`Self::write_str`].
    #[inline]
    pub fn write_fmt(&mut self, arguments: fmt::Arguments<'_>) -> Result<(), DataConversionError> {
        self.inner
            .write_fmt(arguments)
            .map_err(|_| DataConversionError::invalid(self.from, self.to, InvalidValueReason::OutOfRange))
    }

    /// Formats a displayable value without an intermediate String.
    ///
    /// # Errors
    ///
    /// Returns the same errors as [`Self::write_str`].
    #[inline]
    pub fn write_display<T>(&mut self, value: &T) -> Result<(), DataConversionError>
    where
        T: fmt::Display + ?Sized,
    {
        self.write_fmt(format_args!("{value}"))
    }
}
