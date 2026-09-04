// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! One source value that has completed top-level conversion admission.

use super::conversion_context::ConversionContext;
use super::conversion_string_writer::ConversionStringWriter;
use super::data_conversion_target::DataConversionTarget;
use super::data_converter::DataConverter;
use super::error::DataConversionError;
use crate::datatype::DataType;

/// A source and execution capability admitted together for one conversion.
///
/// Instances are constructed only by the conversion framework after it has
/// charged the top-level item and input resources. Consuming this value with
/// [`Self::convert`] safely delegates the same admitted source to another
/// target without charging it a second time.
#[must_use]
pub struct AdmittedConversion<'session, 'policy, 'source> {
    /// The sole source admitted for this conversion.
    source: DataConverter<'source>,
    /// Policy, error binding, and budget capability for this conversion.
    context: ConversionContext<'session, 'policy>,
}

impl<'session, 'policy, 'source> AdmittedConversion<'session, 'policy, 'source> {
    /// Creates an admitted conversion after framework-owned resource charging.
    #[inline(always)]
    pub(crate) fn new(source: DataConverter<'source>, context: ConversionContext<'session, 'policy>) -> Self {
        Self { source, context }
    }

    /// Returns the source bound to this conversion.
    #[must_use = "the admitted source reference is returned"]
    #[inline(always)]
    pub const fn source(&self) -> &DataConverter<'source> {
        &self.source
    }

    /// Returns the runtime type of the bound source.
    #[must_use = "the bound source data type is returned"]
    #[inline(always)]
    pub const fn from_type(&self) -> DataType {
        self.context.from_type()
    }

    /// Returns the runtime type requested by the active target.
    #[must_use = "the bound target data type is returned"]
    #[inline(always)]
    pub const fn to_type(&self) -> DataType {
        self.context.to_type()
    }

    /// Produces an unsupported-pair error for the bound source and target.
    #[must_use = "the bound unsupported-pair error is returned"]
    #[inline(always)]
    pub const fn unsupported(&self) -> DataConversionError {
        DataConversionError::unsupported(self.from_type(), self.to_type())
    }

    /// Safely delegates the exact admitted source to another target.
    #[inline(always)]
    pub fn convert<T>(self) -> Result<T, DataConversionError>
    where
        T: DataConversionTarget,
    {
        T::convert(self)
    }

    /// Decodes JSON under this conversion's bound resource context.
    #[cfg(feature = "json")]
    #[inline]
    pub fn decode_json<'de, T>(&mut self, input: &'de [u8]) -> Result<T, DataConversionError>
    where
        T: serde::Deserialize<'de>,
    {
        self.context.decode_json(input)
    }

    /// Accounts a JSON value under this conversion's bound resource context.
    #[cfg(feature = "json")]
    #[inline]
    pub fn account_json_value(&mut self, value: &serde_json::Value) -> Result<(), DataConversionError> {
        self.context.account_json_value(value)
    }

    /// Encodes JSON under this conversion's bound resource context.
    #[cfg(feature = "json")]
    #[inline]
    pub fn encode_json<T>(&mut self, value: &T) -> Result<Vec<u8>, DataConversionError>
    where
        T: serde::Serialize + ?Sized,
    {
        self.context.encode_json(value)
    }

    /// Renders String output transactionally under this conversion's budget.
    #[inline]
    pub fn write_string<F>(&mut self, render: F) -> Result<String, DataConversionError>
    where
        F: FnOnce(&mut ConversionStringWriter<'_>) -> Result<(), DataConversionError>,
    {
        self.context.write_string(render)
    }

    /// Splits the admitted source from its context for compatibility adapters.
    #[inline(always)]
    pub(crate) fn into_parts(self) -> (DataConverter<'source>, ConversionContext<'session, 'policy>) {
        (self.source, self.context)
    }
}
