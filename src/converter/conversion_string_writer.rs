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

/// Writes one conversion result into a context-owned, budgeted String.
///
/// Values of this type are supplied by [`AdmittedConversion::write_string`]
/// and cannot outlive that call. Bytes are committed to the session only when
/// the complete rendering operation succeeds.
///
/// [`AdmittedConversion::write_string`]: super::AdmittedConversion::write_string
pub struct ConversionStringWriter<'writer> {
    /// Type-erased budgeted output sink receiving rendered bytes.
    inner: &'writer mut dyn ConversionStringSink,
    /// Runtime source type used to contextualize rendering errors.
    from: DataType,
    /// Runtime target type used to contextualize rendering errors.
    to: DataType,
}

impl<'writer> ConversionStringWriter<'writer> {
    /// Creates a writer connected to one conversion context's output budget.
    ///
    /// # Parameters
    ///
    /// * `inner` - Budgeted sink that receives rendered output.
    /// * `from` - Runtime source type for conversion errors.
    /// * `to` - Runtime target type for conversion errors.
    ///
    /// # Returns
    ///
    /// A writer borrowing the supplied sink for the duration of the render.
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
    /// # Parameters
    ///
    /// * `value` - UTF-8 text appended to the current output.
    ///
    /// # Errors
    ///
    /// Returns an invalid-value error if the underlying formatter rejects the
    /// write. Budget failures are reported by the enclosing
    /// [`AdmittedConversion::write_string`] call.
    ///
    /// [`AdmittedConversion::write_string`]: super::AdmittedConversion::write_string
    #[inline]
    pub fn write_str(&mut self, value: &str) -> Result<(), DataConversionError> {
        self.inner
            .write_str(value)
            .map_err(|_| DataConversionError::invalid(self.from, self.to, InvalidValueReason::OutOfRange))
    }

    /// Writes one Unicode scalar value.
    ///
    /// # Parameters
    ///
    /// * `value` - Character appended to the current output.
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
    /// # Parameters
    ///
    /// * `arguments` - Formatting arguments rendered into the current output.
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
    /// # Type Parameters
    ///
    /// * `T` - Displayable value type.
    ///
    /// # Parameters
    ///
    /// * `value` - Value rendered into the current output.
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
