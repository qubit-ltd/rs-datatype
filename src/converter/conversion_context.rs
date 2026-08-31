// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Execution capability bound to one admitted data conversion.

#[cfg(feature = "json")]
use serde::Deserialize;
#[cfg(feature = "json")]
use serde::Serialize;
#[cfg(feature = "json")]
use serde::de::DeserializeSeed;
#[cfg(feature = "json")]
use serde_json::Value;

use super::conversion_session::ConversionSession;
use super::conversion_string_writer::ConversionStringWriter;
use super::data_conversion_target::DataConversionTarget;
use super::data_converter::DataConverter;
use super::error::DataConversionError;
use super::options::ConversionLimits;
use super::options::ConversionPolicy;
use crate::datatype::DataType;

/// Executes one target conversion after its top-level source was admitted.
///
/// A context binds the runtime source and target types to every budgeted
/// operation it exposes. Custom [`DataConversionTarget`] implementations use
/// it for nested conversion, JSON processing, and transactional String output
/// without receiving raw accounting primitives.
#[must_use]
pub struct ConversionContext<'session, 'policy> {
    /// Shared session whose budget backs the current target conversion.
    session: &'session mut ConversionSession<'policy>,
    /// Runtime type of the admitted source value.
    from: DataType,
    /// Runtime type requested by the active target conversion.
    to: DataType,
}

impl<'session, 'policy> ConversionContext<'session, 'policy> {
    /// Creates a context for a source that has completed top-level admission.
    #[inline(always)]
    pub(crate) fn new(session: &'session mut ConversionSession<'policy>, from: DataType, to: DataType) -> Self {
        Self { session, from, to }
    }

    /// Returns the policy shared by the active conversion.
    #[inline(always)]
    pub const fn policy(&self) -> &'policy ConversionPolicy {
        self.session.policy()
    }

    /// Returns the resource limits shared by the active conversion.
    #[inline(always)]
    pub const fn limits(&self) -> &'policy ConversionLimits {
        self.session.limits()
    }

    /// Returns the runtime type of the admitted source.
    #[inline(always)]
    pub const fn from_type(&self) -> DataType {
        self.from
    }

    /// Returns the runtime type requested by the active target conversion.
    #[inline(always)]
    pub const fn to_type(&self) -> DataType {
        self.to
    }

    /// Delegates nested conversion without admitting another top-level item.
    ///
    /// # Errors
    ///
    /// Returns the nested target's conversion error while retaining this
    /// context's shared policy and cumulative budget.
    #[inline(always)]
    pub fn delegate<T>(&mut self, source: &DataConverter<'_>) -> Result<T, DataConversionError>
    where
        T: DataConversionTarget,
    {
        T::convert_from(source, self)
    }

    /// Delegates an owned nested conversion without admitting another item.
    ///
    /// # Errors
    ///
    /// Returns the nested target's conversion error while retaining this
    /// context's shared policy and cumulative budget.
    #[inline(always)]
    pub fn delegate_owned<T>(&mut self, source: DataConverter<'_>) -> Result<T, DataConversionError>
    where
        T: DataConversionTarget,
    {
        T::convert_owned(source, self)
    }

    /// Decodes JSON while charging this conversion's structured budget.
    ///
    /// # Errors
    ///
    /// Returns a JSON syntax, deserialization, or resource-limit error whose
    /// type context is bound to this conversion.
    #[cfg(feature = "json")]
    #[inline]
    pub fn decode_json<'de, T>(&mut self, input: &'de [u8]) -> Result<T, DataConversionError>
    where
        T: Deserialize<'de>,
    {
        self.session.decode_json(self.from, self.to, input)
    }

    /// Decodes JSON through a seed while charging this conversion's budget.
    ///
    /// # Errors
    ///
    /// Returns a JSON syntax, deserialization, or resource-limit error whose
    /// type context is bound to this conversion.
    #[cfg(feature = "json")]
    #[inline]
    pub fn decode_json_seed<'de, S>(&mut self, seed: S, input: &'de [u8]) -> Result<S::Value, DataConversionError>
    where
        S: DeserializeSeed<'de>,
    {
        self.session.decode_json_seed(self.from, self.to, seed, input)
    }

    /// Accounts an existing JSON value against this conversion's structure
    /// budget.
    ///
    /// # Errors
    ///
    /// Returns a structured resource-limit error whose type context is bound
    /// to this conversion.
    #[cfg(feature = "json")]
    #[inline]
    pub fn account_json_value(&mut self, value: &Value) -> Result<(), DataConversionError> {
        self.session.account_json_value(self.from, self.to, value)
    }

    /// Encodes JSON through this conversion's structured and output budgets.
    ///
    /// # Errors
    ///
    /// Returns a serialization or resource-limit error whose type context is
    /// bound to this conversion.
    #[cfg(feature = "json")]
    #[inline]
    pub fn encode_json<T>(&mut self, value: &T) -> Result<Vec<u8>, DataConversionError>
    where
        T: Serialize + ?Sized,
    {
        self.session.encode_json(self.from, self.to, value)
    }

    /// Renders one String transactionally under this conversion's output
    /// budget.
    ///
    /// # Errors
    ///
    /// Returns the renderer's conversion error or an output resource error;
    /// failed rendering does not consume output bytes.
    #[inline]
    pub fn write_string<F>(&mut self, render: F) -> Result<String, DataConversionError>
    where
        F: FnOnce(&mut ConversionStringWriter<'_>) -> Result<(), DataConversionError>,
    {
        self.session.write_string(self.from, self.to, render)
    }

    /// Admits built-in owned String output without allocating another buffer.
    ///
    /// # Errors
    ///
    /// Returns an output resource error whose type context is bound to this
    /// conversion.
    #[inline]
    pub(crate) fn admit_output_bytes(&mut self, amount: usize) -> Result<(), DataConversionError> {
        self.session.admit_output_bytes(self.from, self.to, amount)
    }
}
