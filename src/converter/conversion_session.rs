// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Reusable accounting session for one or more data conversions.
//!
//! `ConversionSession` owns cumulative limits and observability. Target
//! implementations receive [`super::ConversionContext`] instead, so every
//! budgeted operation is bound to the active source and target type pair.

use qubit_budget::BudgetError;
use qubit_budget::BudgetedStringError;
use qubit_budget::MeasuredBudgetError;
use qubit_budget::ResourceLimit;
use qubit_budget::ResourceQuantity;
#[cfg(feature = "json")]
use qubit_budget::json::JsonDecodeSession;
#[cfg(feature = "json")]
use qubit_budget::json::JsonEncodeSession;
use qubit_budget::json::JsonValueTransaction;
#[cfg(feature = "json")]
use qubit_json::decode::JsonDecodeError;
#[cfg(feature = "json")]
use qubit_json::decode::JsonDecoder;
#[cfg(feature = "json")]
use qubit_json::encode::JsonEncodeError;
#[cfg(feature = "json")]
use qubit_json::encode::JsonEncoder;
#[cfg(feature = "json")]
use qubit_json::value::traverse::JsonTreeReader;
#[cfg(feature = "json")]
use serde::Deserialize;
#[cfg(feature = "json")]
use serde::Serialize;
#[cfg(feature = "json")]
use serde::de::DeserializeSeed;
#[cfg(feature = "json")]
use serde_json::Value;

use super::admitted_scalar_source::AdmittedScalarSource;
use super::conversion_resource::ConversionResource;
use super::conversion_string_writer::ConversionStringWriter;
use super::error::DataConversionError;
use super::internal::ConversionBudget;
use super::options::ConversionLimits;
use super::options::ConversionPolicy;
use crate::datatype::DataType;

/// Mutable conversion accounting shared by nested and batch conversions.
///
/// # Examples
///
/// ```
/// use qubit_datatype::{
///     ConversionLimits, ConversionPolicy, ConversionSession, DataConverter,
/// };
///
/// let policy = ConversionPolicy::strict();
/// let limits = ConversionLimits::default();
/// let mut session = ConversionSession::new(&policy, &limits);
/// let value = DataConverter::from("42").to_in::<u16>(&mut session)?;
///
/// assert_eq!(value, 42);
/// # Ok::<(), qubit_datatype::DataConversionError>(())
/// ```
#[must_use]
#[derive(Debug, PartialEq, Eq)]
pub struct ConversionSession<'a> {
    /// Immutable conversion policy applied to every nested operation.
    policy: &'a ConversionPolicy,
    /// Immutable resource limits used to initialize the session budgets.
    limits: &'a ConversionLimits,
    /// Mutable cumulative accounting for this conversion session.
    budget: ConversionBudget,
}

impl<'a> ConversionSession<'a> {
    /// Creates a fresh session from immutable policy and limit configuration.
    #[inline]
    pub fn new(policy: &'a ConversionPolicy, limits: &'a ConversionLimits) -> Self {
        Self {
            policy,
            limits,
            budget: ConversionBudget::new(limits),
        }
    }

    /// Returns the immutable conversion policy associated with this session.
    #[inline(always)]
    pub const fn policy(&self) -> &'a ConversionPolicy {
        self.policy
    }

    /// Returns the immutable conversion limits associated with this session.
    #[inline(always)]
    pub const fn limits(&self) -> &'a ConversionLimits {
        self.limits
    }

    /// Decodes JSON while charging the shared structured budget directly.
    ///
    /// The caller is responsible for charging any outer source/input bytes;
    /// this method accounts only the JSON value structure and payload.
    ///
    /// # Errors
    ///
    /// Returns the decoder error when the input is invalid or any structured
    /// node/payload budget is exhausted. Failed budget admissions do not
    /// increase the corresponding counters.
    #[cfg(feature = "json")]
    pub(crate) fn decode_json<'de, T>(
        &mut self,
        from: DataType,
        to: DataType,
        input: &'de [u8],
    ) -> Result<T, DataConversionError>
    where
        T: Deserialize<'de>,
    {
        let decode = JsonDecodeSession::borrowing_value(self.budget.structured_mut());
        JsonDecoder::new(decode)
            .decode_utf8(input)
            .map_err(|error| map_json_decode_error(from, to, error))
    }

    /// Decodes JSON through a seed while charging the shared budget directly.
    ///
    /// The caller is responsible for charging outer input bytes. This method
    /// accounts only the JSON structure and payload consumed by the decoder.
    #[cfg(feature = "json")]
    pub(crate) fn decode_json_seed<'de, S>(
        &mut self,
        from: DataType,
        to: DataType,
        seed: S,
        input: &'de [u8],
    ) -> Result<S::Value, DataConversionError>
    where
        S: DeserializeSeed<'de>,
    {
        let decode = JsonDecodeSession::borrowing_value(self.budget.structured_mut());
        JsonDecoder::new(decode)
            .decode_seed_utf8(seed, input)
            .map_err(|error| map_json_decode_error(from, to, error))
    }

    /// Accounts an already materialized JSON value through rs-budget's
    /// canonical iterative traversal.
    #[cfg(feature = "json")]
    pub(crate) fn account_json_value(
        &mut self,
        from: DataType,
        to: DataType,
        value: &Value,
    ) -> Result<(), DataConversionError> {
        let mut transaction = self.budget.structured_transaction();
        JsonTreeReader::new(&mut transaction)
            .account(value)
            .map_err(|error| DataConversionError::measured_limit(from, to, error))?;
        transaction
            .commit()
            .map_err(|error| DataConversionError::measured_limit(from, to, error))
    }

    /// Starts a transactional admission over the shared structured budget.
    pub(crate) fn structured_transaction(&mut self) -> JsonValueTransaction<'_, ConversionResource, u64> {
        self.budget.structured_transaction()
    }

    /// Encodes JSON through the shared structured and output budgets.
    ///
    /// # Errors
    ///
    /// Returns the encoder error when serialization fails or the structured or
    /// output budget cannot admit the next value. Output bytes are committed
    /// only for a successfully encoded payload.
    #[cfg(feature = "json")]
    pub(crate) fn encode_json<T>(
        &mut self,
        from: DataType,
        to: DataType,
        value: &T,
    ) -> Result<Vec<u8>, DataConversionError>
    where
        T: Serialize + ?Sized,
    {
        let (output, structured) = self.budget.split_json_mut();
        let encode = JsonEncodeSession::borrowing_output(output, structured);
        JsonEncoder::new(encode)
            .to_vec(value)
            .map_err(|error| map_json_encode_error(from, to, error))
    }

    /// Admits one complete scalar collection source before it is scanned.
    ///
    /// # Errors
    ///
    /// Returns a conversion-domain quantity or resource-limit error.
    #[inline]
    fn admit_scalar_source(&mut self, from: DataType, to: DataType, amount: usize) -> Result<(), DataConversionError> {
        self.admit_scalar_source_bytes_usize(amount)
            .map_err(|error| DataConversionError::measured_limit(from, to, error))
    }

    /// Admits one complete scalar String source before an external adapter
    /// splits it into individually admitted items.
    ///
    /// The complete source is charged once to the cumulative input budget.
    /// The returned source lends individually admitted items without charging
    /// their bytes again. Every item is a slice of this exact source; adapters
    /// cannot substitute an uncharged source. Splitting is lazy and retains
    /// original indices even when empty items are skipped.
    ///
    /// # Returns
    ///
    /// A source-bound capability borrowing this session and the source text.
    ///
    /// # Errors
    ///
    /// Returns a String-to-String conversion-domain quantity or
    /// resource-limit error.
    #[inline]
    pub fn admit_scalar_string_source<'session, 'source>(
        &'session mut self,
        source: &'source str,
    ) -> Result<AdmittedScalarSource<'session, 'a, 'source>, DataConversionError>
    where
        'a: 'source,
    {
        self.admit_scalar_source(DataType::String, DataType::String, source.len())?;
        Ok(AdmittedScalarSource::new(self, source))
    }

    /// Admits one completed output payload measured by a Rust String length.
    ///
    /// # Errors
    ///
    /// Returns a conversion-domain quantity or resource-limit error.
    #[inline]
    pub(crate) fn admit_output_bytes(
        &mut self,
        from: DataType,
        to: DataType,
        amount: usize,
    ) -> Result<(), DataConversionError> {
        self.budget
            .consume_output_bytes_usize(amount)
            .map_err(|error| DataConversionError::measured_limit(from, to, error))
    }

    /// Renders one String transactionally under the cumulative output budget.
    ///
    /// # Errors
    ///
    /// Returns the renderer's conversion error, or a conversion-domain error
    /// when output accounting or allocation fails. Failed renders do not
    /// consume output bytes.
    #[inline]
    pub(crate) fn write_string<F>(
        &mut self,
        from: DataType,
        to: DataType,
        render: F,
    ) -> Result<String, DataConversionError>
    where
        F: FnOnce(&mut ConversionStringWriter<'_>) -> Result<(), DataConversionError>,
    {
        self.budget
            .try_write_string(|writer| render(&mut ConversionStringWriter::new(writer, from, to)))
            .map_err(|error| map_string_write_error(from, to, error))
    }

    /// Consumes one top-level conversion item.
    ///
    /// # Errors
    ///
    /// Returns a budget error when the cumulative item limit is exhausted.
    #[inline]
    pub(crate) fn consume_item(&mut self) -> Result<(), BudgetError<ConversionResource, u64>> {
        self.budget.consume_item()
    }

    /// Checks and consumes one complete scalar collection source.
    ///
    /// This combines the source-size limit check with cumulative input-byte
    /// accounting so callers cannot accidentally scan an unchecked source or
    /// omit its input charge.
    ///
    /// # Errors
    ///
    /// Returns a measured budget error when the source is too large, its
    /// length cannot be represented as `u64`, or the cumulative input budget
    /// is exhausted.
    #[inline]
    pub(crate) fn admit_scalar_source_bytes_usize(
        &mut self,
        amount: usize,
    ) -> Result<(), MeasuredBudgetError<ConversionResource, u64>> {
        self.check_collection_source_bytes_usize(amount)?;
        self.consume_input_bytes_usize(amount)
    }

    /// Consumes input bytes measured by a native Rust string or slice length.
    ///
    /// # Errors
    ///
    /// Returns a quantity error if the `usize` value cannot be represented as
    /// `u64`, or a budget error when the cumulative limit is exhausted.
    #[inline]
    pub(crate) fn consume_input_bytes_usize(
        &mut self,
        amount: usize,
    ) -> Result<(), MeasuredBudgetError<ConversionResource, u64>> {
        self.budget.consume_input_bytes_usize(amount)
    }

    /// Checks a scalar collection source measured by a native slice length.
    #[inline]
    pub(crate) fn check_collection_source_bytes_usize(
        &self,
        actual: usize,
    ) -> Result<(), MeasuredBudgetError<ConversionResource, u64>> {
        let limit = ResourceLimit::new(
            ConversionResource::CollectionSourceBytes,
            self.limits.collection().max_source_bytes(),
        );
        let actual = u64::try_from_usize(actual)
            .map_err(|source| MeasuredBudgetError::quantity(ConversionResource::CollectionSourceBytes, source))?;
        limit.check(actual).map_err(MeasuredBudgetError::from)
    }

    /// Returns remaining top-level item capacity.
    #[inline(always)]
    #[must_use]
    pub const fn items_remaining(&self) -> u64 {
        self.budget.remaining_items()
    }

    /// Returns the configured cumulative item limit.
    #[inline(always)]
    #[must_use]
    pub const fn items_limit(&self) -> u64 {
        self.limits.operation().max_items()
    }

    /// Returns the configured cumulative input-byte limit.
    #[inline(always)]
    #[must_use]
    pub const fn input_bytes_limit(&self) -> u64 {
        self.limits.operation().max_input_bytes()
    }

    /// Returns the configured cumulative output-byte limit.
    #[inline(always)]
    #[must_use]
    pub const fn output_bytes_limit(&self) -> u64 {
        self.limits.operation().max_output_bytes()
    }

    /// Returns the configured cumulative structured-node limit.
    #[inline(always)]
    #[must_use]
    pub const fn structured_nodes_limit(&self) -> u64 {
        self.limits.operation().max_structured_nodes()
    }

    /// Returns the configured cumulative structured-payload limit.
    #[inline(always)]
    #[must_use]
    pub const fn structured_payload_bytes_limit(&self) -> u64 {
        self.limits.operation().max_structured_payload_bytes()
    }

    /// Returns remaining cumulative item capacity for internal adapters.
    #[inline(always)]
    pub(crate) fn remaining_items(&self) -> u64 {
        self.items_remaining()
    }

    /// Returns cumulative converted items already consumed.
    #[inline(always)]
    #[must_use]
    pub fn items_used(&self) -> u64 {
        self.budget.items_used()
    }

    /// Returns cumulative input bytes already consumed.
    #[inline(always)]
    #[must_use]
    pub fn input_bytes_used(&self) -> u64 {
        self.budget.input_bytes_used()
    }

    /// Returns remaining cumulative input-byte capacity.
    #[inline(always)]
    #[must_use]
    pub fn input_bytes_remaining(&self) -> u64 {
        self.budget.input_bytes_remaining()
    }

    /// Returns cumulative output bytes already consumed.
    #[inline(always)]
    #[must_use]
    pub fn output_bytes_used(&self) -> u64 {
        self.budget.output_bytes_used()
    }

    /// Returns remaining cumulative output-byte capacity.
    #[inline(always)]
    #[must_use]
    pub fn output_bytes_remaining(&self) -> u64 {
        self.budget.output_bytes_remaining()
    }

    /// Returns cumulative structured nodes already consumed.
    #[inline(always)]
    #[must_use]
    pub fn structured_nodes_used(&self) -> u64 {
        self.budget.structured_nodes_used()
    }

    /// Returns cumulative structured payload bytes already consumed.
    #[inline(always)]
    #[must_use]
    pub fn structured_payload_bytes_used(&self) -> u64 {
        self.budget.structured_payload_bytes_used()
    }
}

/// Converts a budgeted string rendering failure into a conversion error.
///
/// # Parameters
///
/// * `from` - Runtime source type retained in the error.
/// * `to` - Requested target type retained in the error.
/// * `error` - Rendering or budget failure to convert.
///
/// # Returns
///
/// A conversion error preserving budget, quantity, render, or encoding
/// failure details where the public error model supports them.
fn map_string_write_error(
    from: DataType,
    to: DataType,
    error: BudgetedStringError<ConversionResource, DataConversionError>,
) -> DataConversionError {
    match error {
        BudgetedStringError::Budget(error) => DataConversionError::limit_exceeded(from, to, error.into()),
        BudgetedStringError::Quantity { resource, source } => DataConversionError::quantity(from, to, resource, source),
        BudgetedStringError::Render(error) => error,
        BudgetedStringError::InvalidUtf8(_)
        | BudgetedStringError::LengthOverflow
        | BudgetedStringError::Allocation(_) => {
            DataConversionError::invalid(from, to, super::error::InvalidValueReason::OutOfRange)
        }
    }
}

/// Converts a JSON decoder failure into a conversion error.
///
/// # Parameters
///
/// * `from` - Runtime source type retained in the error.
/// * `to` - Requested target type retained in the error.
/// * `error` - JSON decoding failure to convert.
///
/// # Returns
///
/// A measured-limit error when the decoder exhausted a budget, otherwise a
/// JSON deserialization invalid-value error.
#[cfg(feature = "json")]
fn map_json_decode_error(
    from: DataType,
    to: DataType,
    error: JsonDecodeError<ConversionResource, u64>,
) -> DataConversionError {
    match error.budget_error().cloned() {
        Some(error) => DataConversionError::measured_limit(from, to, error),
        None => DataConversionError::invalid(
            from,
            to,
            super::error::InvalidValueReason::Deserialization {
                format: super::error::DataFormat::Json,
            },
        ),
    }
}

/// Converts a JSON encoder failure into a conversion error.
///
/// # Parameters
///
/// * `from` - Runtime source type retained in the error.
/// * `to` - Requested target type retained in the error.
/// * `error` - JSON encoding failure to convert.
///
/// # Returns
///
/// A measured-limit error for budget failures, or a JSON serialization
/// invalid-value error for encoding failures.
#[cfg(feature = "json")]
fn map_json_encode_error(
    from: DataType,
    to: DataType,
    error: JsonEncodeError<ConversionResource, u64>,
) -> DataConversionError {
    if let Some(error) = error.into_budget_error() {
        DataConversionError::measured_limit(from, to, error)
    } else {
        DataConversionError::invalid(
            from,
            to,
            super::error::InvalidValueReason::Serialization {
                format: super::error::DataFormat::Json,
            },
        )
    }
}
