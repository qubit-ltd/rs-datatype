// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Reusable accounting session for one or more data conversions.

use qubit_budget::BudgetError;
use qubit_budget::BudgetedStringError;
use qubit_budget::BudgetedStringWriter;
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
use qubit_json::value::traverse::JsonTreeContext;
#[cfg(feature = "json")]
use qubit_json::value::traverse::JsonTreeProcessError;
#[cfg(feature = "json")]
use qubit_json::value::traverse::JsonTreeReader;
#[cfg(feature = "json")]
use qubit_json::value::traverse::JsonTreeVisitor;
#[cfg(feature = "json")]
use serde::Deserialize;
#[cfg(feature = "json")]
use serde::Serialize;
#[cfg(feature = "json")]
use serde::de::DeserializeSeed;
#[cfg(feature = "json")]
use serde_json::Value;

use super::admitted_scalar_item::AdmittedScalarItem;
use super::conversion_resource::ConversionResource;
use super::data_conversion_target::DataConversionTarget;
use super::data_converter::DataConverter;
use super::error::DataConversionError;
use super::internal::ConversionBudget;
use super::options::ConversionLimits;
use super::options::ConversionPolicy;
use super::scalar_item::ScalarItem;

/// Mutable conversion accounting shared by nested and batch conversions.
#[must_use]
#[derive(Debug, PartialEq, Eq)]
pub struct ConversionSession<'a> {
    policy: &'a ConversionPolicy,
    limits: &'a ConversionLimits,
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

    /// Delegates a nested conversion without charging a new top-level item or
    /// input payload.
    ///
    /// Custom [`DataConversionTarget`] implementations must use this method
    /// (or [`Self::delegate_owned`]) for nested work. The outer
    /// [`DataConverter::to_in`](crate::DataConverter::to_in) call has already
    /// admitted one item and its source input; direct calls to the consume
    /// methods are reserved for custom targets that own a distinct budgeted
    /// unit of work.
    ///
    /// The nested target must not call [`Self::consume_item`] or charge the
    /// source input again unless it intentionally represents another
    /// top-level unit of work.
    #[inline(always)]
    pub fn delegate<T>(&mut self, source: &DataConverter<'_>) -> Result<T, DataConversionError>
    where
        T: DataConversionTarget,
    {
        T::convert_from(source, self)
    }

    /// Delegates an owned nested conversion while preserving this session.
    ///
    /// This has the same accounting contract as [`Self::delegate`], while
    /// transferring ownership of the nested source to the target.
    #[inline(always)]
    pub fn delegate_owned<T>(&mut self, source: DataConverter<'_>) -> Result<T, DataConversionError>
    where
        T: DataConversionTarget,
    {
        T::convert_owned(source, self)
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
    pub fn decode_json<'de, T>(&mut self, input: &'de [u8]) -> Result<T, JsonDecodeError<ConversionResource, u64>>
    where
        T: Deserialize<'de>,
    {
        let decode = JsonDecodeSession::borrowing_value(self.budget.structured_mut());
        JsonDecoder::new(decode).decode_utf8(input)
    }

    /// Decodes JSON through a seed while charging the shared budget directly.
    ///
    /// The caller is responsible for charging outer input bytes. This method
    /// accounts only the JSON structure and payload consumed by the decoder.
    #[cfg(feature = "json")]
    pub fn decode_json_seed<'de, S>(
        &mut self,
        seed: S,
        input: &'de [u8],
    ) -> Result<S::Value, JsonDecodeError<ConversionResource, u64>>
    where
        S: DeserializeSeed<'de>,
    {
        let decode = JsonDecodeSession::borrowing_value(self.budget.structured_mut());
        JsonDecoder::new(decode).decode_seed_utf8(seed, input)
    }

    /// Accounts an already materialized JSON value through rs-budget's
    /// canonical iterative traversal.
    #[cfg(feature = "json")]
    pub(crate) fn account_json_value(
        &mut self,
        value: &Value,
    ) -> Result<(), JsonTreeProcessError<ConversionResource, u64, std::convert::Infallible>> {
        let mut transaction = self.budget.structured_transaction();
        let result = JsonTreeReader::new(&mut transaction).process(value, &mut JsonAccountingVisitor);
        if result.is_ok() {
            transaction.commit();
        }
        result
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
    pub fn encode_json<T>(&mut self, value: &T) -> Result<Vec<u8>, JsonEncodeError<ConversionResource, u64>>
    where
        T: Serialize + ?Sized,
    {
        let (output, structured) = self.budget.split_json_mut();
        let encode = JsonEncodeSession::borrowing_output(output, structured);
        JsonEncoder::new(encode).to_vec(value)
    }

    /// Consumes one top-level conversion item.
    ///
    /// # Errors
    ///
    /// Returns a budget error when the cumulative item limit is exhausted.
    #[inline]
    pub fn consume_item(&mut self) -> Result<(), BudgetError<ConversionResource, u64>> {
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
    pub fn admit_scalar_source_bytes_usize(
        &mut self,
        amount: usize,
    ) -> Result<(), MeasuredBudgetError<ConversionResource, u64>> {
        self.check_collection_source_bytes_usize(amount)?;
        self.consume_input_bytes_usize(amount)
    }

    /// Consumes one scalar item and returns a token for precharged conversion.
    ///
    /// Constructed tokens are intended for downstream deserializers that need
    /// to continue conversion without charging the same item a second time.
    ///
    /// # Errors
    ///
    /// Returns a budget error when the cumulative item limit is exhausted.
    #[inline]
    pub fn admit_scalar_item(
        &mut self,
        item: ScalarItem<'_>,
    ) -> Result<AdmittedScalarItem, BudgetError<ConversionResource, u64>> {
        self.consume_item()?;
        Ok(AdmittedScalarItem::new(item))
    }

    /// Consumes cumulative input bytes.
    ///
    /// # Errors
    ///
    /// Returns a budget error when the cumulative input-byte limit is
    /// exhausted. The counter is unchanged when admission fails.
    #[inline]
    pub fn consume_input_bytes(&mut self, amount: u64) -> Result<(), BudgetError<ConversionResource, u64>> {
        self.budget.consume_input_bytes(amount)
    }

    /// Consumes input bytes measured by a native Rust string or slice length.
    ///
    /// This is a convenience adapter for `usize` lengths. Use the `u64`
    /// variant when the measured quantity is already represented as `u64`.
    ///
    /// # Errors
    ///
    /// Returns a quantity error if the `usize` value cannot be represented as
    /// `u64`, or a budget error when the cumulative limit is exhausted.
    #[inline]
    pub fn consume_input_bytes_usize(
        &mut self,
        amount: usize,
    ) -> Result<(), MeasuredBudgetError<ConversionResource, u64>> {
        self.budget.consume_input_bytes_usize(amount)
    }

    /// Checks one complete scalar collection source before scanning it.
    #[inline]
    pub fn check_collection_source_bytes(&self, actual: u64) -> Result<(), BudgetError<ConversionResource, u64>> {
        ResourceLimit::new(
            ConversionResource::CollectionSourceBytes,
            self.limits.collection().max_source_bytes(),
        )
        .check(actual)
        .map_err(Into::into)
    }

    /// Checks a scalar collection source measured by a native slice length.
    #[inline]
    pub fn check_collection_source_bytes_usize(
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

    /// Checks cumulative built-in String output payload bytes.
    ///
    /// This is a non-mutating pre-check; call a consume method only after the
    /// payload has been materialized successfully.
    #[inline]
    pub fn check_output_bytes(&self, amount: u64) -> Result<(), BudgetError<ConversionResource, u64>> {
        self.budget.check_output_bytes(amount)
    }

    /// Checks output bytes measured by a native Rust string length.
    #[inline]
    pub fn check_output_bytes_usize(&self, amount: usize) -> Result<(), MeasuredBudgetError<ConversionResource, u64>> {
        self.budget.check_output_bytes_usize(amount)
    }

    /// Consumes cumulative output payload bytes after a successful pre-check.
    ///
    /// # Errors
    ///
    /// Returns a budget error when the cumulative output-byte limit is
    /// exhausted. The counter is unchanged when admission fails.
    #[inline]
    pub fn consume_output_bytes(&mut self, amount: u64) -> Result<(), BudgetError<ConversionResource, u64>> {
        self.budget.consume_output_bytes(amount)
    }

    /// Consumes output bytes measured by a native Rust string length.
    ///
    /// Use the `u64` variant when the measured quantity is already represented
    /// as `u64`; this adapter also reports native-length conversion overflow.
    #[inline]
    pub fn consume_output_bytes_usize(
        &mut self,
        amount: usize,
    ) -> Result<(), MeasuredBudgetError<ConversionResource, u64>> {
        self.budget.consume_output_bytes_usize(amount)
    }

    /// Renders a String payload in one pass and commits its UTF-8 bytes only
    /// after the renderer succeeds.
    #[inline]
    pub fn try_write_string<E, F>(&mut self, render: F) -> Result<String, BudgetedStringError<ConversionResource, E>>
    where
        E: std::fmt::Debug + std::fmt::Display,
        F: FnOnce(&mut BudgetedStringWriter<'_, ConversionResource>) -> Result<(), E>,
    {
        self.budget.try_write_string(render)
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

/// Performs full JSON budget admission without adding conversion-domain
/// behavior.
#[cfg(feature = "json")]
struct JsonAccountingVisitor;

#[cfg(feature = "json")]
impl JsonTreeVisitor for JsonAccountingVisitor {
    type Error = std::convert::Infallible;

    /// Accepts every budget-admitted JSON node.
    fn enter(&mut self, _value: &Value, _context: JsonTreeContext<'_>) -> Result<(), Self::Error> {
        Ok(())
    }
}
