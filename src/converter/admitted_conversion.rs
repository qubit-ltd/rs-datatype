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
/// target with the same [`DataType`], without charging it a second time.
///
/// # Examples
///
/// ```
/// use qubit_datatype::{
///     AdmittedConversion,
///     DataConversionError,
///     DataConversionTarget,
///     DataConverter,
///     DataType,
///     DataTypeOf,
/// };
///
/// struct Port(u16);
///
/// impl DataTypeOf for Port {
///     const DATA_TYPE: DataType = DataType::UInt16;
/// }
///
/// impl DataConversionTarget for Port {
///     fn convert(input: AdmittedConversion<'_, '_, '_>)
///         -> Result<Self, DataConversionError> {
///         input.convert::<u16>().map(Self)
///     }
/// }
///
/// let port = DataConverter::from("8080").to::<Port>().unwrap();
/// assert_eq!(port.0, 8080);
/// ```
#[must_use]
pub struct AdmittedConversion<'session, 'policy, 'source> {
    /// The sole source admitted for this conversion.
    source: DataConverter<'source>,
    /// Policy, error binding, and budget capability for this conversion.
    context: ConversionContext<'session, 'policy>,
}

impl<'session, 'policy, 'source> AdmittedConversion<'session, 'policy, 'source> {
    /// Creates an admitted conversion after framework-owned resource charging.
    ///
    /// # Returns
    ///
    /// An admitted conversion bound to the charged source and context.
    #[inline]
    pub(crate) fn new(source: DataConverter<'source>, context: ConversionContext<'session, 'policy>) -> Self {
        Self { source, context }
    }

    /// Returns the source bound to this conversion.
    ///
    /// # Returns
    ///
    /// The source value admitted for this conversion.
    #[must_use = "the admitted source reference is returned"]
    #[inline]
    pub const fn source(&self) -> &DataConverter<'source> {
        &self.source
    }

    /// Returns the runtime type of the bound source.
    ///
    /// # Returns
    ///
    /// The data type of the admitted source.
    #[inline]
    pub const fn from_type(&self) -> DataType {
        self.context.source_type()
    }

    /// Returns the runtime type requested by the active target.
    ///
    /// # Returns
    ///
    /// The data type requested by the active target.
    #[inline]
    pub const fn to_type(&self) -> DataType {
        self.context.to_type()
    }

    /// Produces an unsupported-pair error for the bound source and target.
    ///
    /// # Returns
    ///
    /// An error describing the bound source and target types as unsupported.
    #[inline]
    pub const fn unsupported(&self) -> DataConversionError {
        DataConversionError::unsupported(self.from_type(), self.to_type())
    }

    /// Safely delegates the exact admitted source to a target with the same
    /// [`DataType`] as the active target.
    ///
    /// A different target data type is rejected before the delegated target is
    /// called. The returned [`DataConversionError`] retains the original source
    /// and active outer target types. Top-level item and input charges remain
    /// consumed; no second admission or structured-resource charge is made.
    ///
    /// # Type Parameters
    ///
    /// * `T` - Target type that implements the crate's conversion contract.
    ///
    /// # Errors
    ///
    /// Returns [`DataConversionError`] when the delegated target has a
    /// different data type, the source is missing, the source-target pair is
    /// unsupported, the value is invalid, or a conversion resource limit is
    /// exceeded. A descriptor mismatch is reported against the active outer
    /// target before the delegated target runs.
    ///
    /// # Returns
    ///
    /// The converted target value when conversion succeeds.
    #[inline]
    pub fn convert<T>(self) -> Result<T, DataConversionError>
    where
        T: DataConversionTarget,
    {
        if T::DATA_TYPE != self.to_type() {
            return Err(self.unsupported());
        }
        T::convert(self)
    }

    /// Decodes JSON under this conversion's bound resource context.
    ///
    /// The byte limit applies to the exact input slice, including whitespace.
    /// This check does not normalize the input or admit and charge it again;
    /// the top-level item and source bytes have already been charged.
    ///
    /// # Type Parameters
    ///
    /// * `'de` - Lifetime of borrowed data returned by the deserializer.
    /// * `T` - Deserialized output type.
    ///
    /// # Parameters
    ///
    /// * `input` - UTF-8 JSON bytes to decode.
    ///
    /// # Returns
    ///
    /// The value deserialized from the JSON input.
    ///
    /// # Errors
    ///
    /// Returns a JSON syntax or deserialization error, or a structured-resource
    /// error when the input bytes or decoded structure exceed their limits.
    /// Text-limit errors retain the top-level item and source-byte charges
    /// and add no structured or output charges.
    #[cfg(feature = "json")]
    #[inline]
    pub fn decode_json<'de, T>(&mut self, input: &'de [u8]) -> Result<T, DataConversionError>
    where
        T: serde::Deserialize<'de>,
    {
        let from = self.from_type();
        let to = self.to_type();
        self.context
            .limits()
            .structured()
            .max_text_bytes_limit()
            .check_usize(input.len())
            .map_err(|error| DataConversionError::measured_limit(from, to, error))?;
        self.context.decode_json(input)
    }

    /// Accounts a JSON value under this conversion's bound resource context.
    ///
    /// # Parameters
    ///
    /// * `value` - Materialized JSON value whose structure is charged.
    ///
    /// # Returns
    ///
    /// `()` when the JSON structure is accounted successfully.
    ///
    /// # Errors
    ///
    /// Returns a structured-resource error when accounting exceeds the bound
    /// conversion limits.
    #[cfg(feature = "json")]
    #[inline]
    pub fn account_json_value(&mut self, value: &serde_json::Value) -> Result<(), DataConversionError> {
        self.context.account_json_value(value)
    }

    /// Encodes JSON under this conversion's bound resource context.
    ///
    /// # Type Parameters
    ///
    /// * `T` - Serializable value to encode.
    ///
    /// # Parameters
    ///
    /// * `value` - Value to serialize as JSON.
    ///
    /// # Returns
    ///
    /// The JSON bytes produced by serializing `value`.
    ///
    /// # Errors
    ///
    /// Returns a serialization or structured/output-resource error.
    #[cfg(feature = "json")]
    #[inline]
    pub fn encode_json<T>(&mut self, value: &T) -> Result<Vec<u8>, DataConversionError>
    where
        T: serde::Serialize + ?Sized,
    {
        self.context.encode_json(value)
    }

    /// Renders String output transactionally under this conversion's budget.
    ///
    /// # Type Parameters
    ///
    /// * `F` - Rendering closure that writes to the supplied budgeted writer.
    ///
    /// # Parameters
    ///
    /// * `render` - Closure that produces the complete String output.
    ///
    /// # Returns
    ///
    /// The complete String produced by the renderer.
    ///
    /// # Errors
    ///
    /// Returns the renderer's conversion error or an output-resource error.
    #[inline]
    pub fn write_string<F>(&mut self, render: F) -> Result<String, DataConversionError>
    where
        F: FnOnce(&mut ConversionStringWriter<'_>) -> Result<(), DataConversionError>,
    {
        self.context.write_string(render)
    }

    /// Unpacks the admitted source and its conversion context for built-in
    /// targets.
    ///
    /// # Returns
    ///
    /// The admitted source and its bound conversion context.
    #[inline]
    pub(crate) fn into_parts(self) -> (DataConverter<'source>, ConversionContext<'session, 'policy>) {
        (self.source, self.context)
    }
}
