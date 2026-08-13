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
#[cfg(feature = "json")]
use qubit_json::text::JsonDecodeError;
#[cfg(feature = "json")]
use qubit_json::text::JsonEncodeError;
#[cfg(feature = "json")]
use qubit_json::text::decode_slice;
#[cfg(feature = "json")]
use qubit_json::text::decode_slice_seed;
#[cfg(feature = "json")]
use qubit_json::text::encode_to_vec;
#[cfg(feature = "json")]
use qubit_json::tree::JsonTreeContext;
#[cfg(feature = "json")]
use qubit_json::tree::JsonTreeProcessError;
#[cfg(feature = "json")]
use qubit_json::tree::JsonTreeProcessor;
#[cfg(feature = "json")]
use qubit_json::tree::JsonTreeVisitor;
#[cfg(feature = "json")]
use serde::Deserialize;
#[cfg(feature = "json")]
use serde::Serialize;
#[cfg(feature = "json")]
use serde::de::DeserializeSeed;
#[cfg(feature = "json")]
use serde_json::Value;

use super::conversion_resource::ConversionResource;
use super::data_conversion_target::DataConversionTarget;
use super::data_converter::DataConverter;
use super::error::DataConversionError;
use super::internal::ConversionBudget;
use super::options::ConversionLimits;
use super::options::ConversionPolicy;

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
    pub fn decode_json<'de, T>(
        &mut self,
        input: &'de [u8],
    ) -> Result<T, JsonDecodeError<ConversionResource, u64>>
    where
        T: Deserialize<'de>,
    {
        let mut decode = JsonDecodeSession::borrowing(None, None, self.budget.structured_mut());
        decode_slice(input, &mut decode)
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
        let mut decode = JsonDecodeSession::borrowing(None, None, self.budget.structured_mut());
        decode_slice_seed(seed, input, &mut decode)
    }

    /// Accounts an already materialized JSON value through rs-budget's
    /// canonical iterative traversal.
    #[cfg(feature = "json")]
    pub(crate) fn account_json_value(
        &mut self,
        value: &Value,
    ) -> Result<(), JsonTreeProcessError<ConversionResource, u64, std::convert::Infallible>> {
        JsonTreeProcessor::new(self.budget.structured_mut())
            .process(value, &mut JsonAccountingVisitor)
    }

    /// Encodes JSON through the shared structured and output budgets.
    ///
    /// # Errors
    ///
    /// Returns the encoder error when serialization fails or the structured or
    /// output budget cannot admit the next value. Output bytes are committed
    /// only for a successfully encoded payload.
    #[cfg(feature = "json")]
    pub fn encode_json<T>(
        &mut self,
        value: &T,
    ) -> Result<Vec<u8>, JsonEncodeError<ConversionResource, u64>>
    where
        T: Serialize + ?Sized,
    {
        let (output, structured) = self.budget.split_json_mut();
        let mut encode = JsonEncodeSession::borrowing(Some(output), structured);
        encode_to_vec(value, &mut encode)
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

    /// Consumes cumulative normalized input bytes.
    ///
    /// # Errors
    ///
    /// Returns a budget error when the cumulative input-byte limit is
    /// exhausted. The counter is unchanged when admission fails.
    #[inline]
    pub fn consume_input_bytes(
        &mut self,
        amount: u64,
    ) -> Result<(), BudgetError<ConversionResource, u64>> {
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
    pub fn check_collection_source_bytes(
        &self,
        actual: u64,
    ) -> Result<(), BudgetError<ConversionResource, u64>> {
        ResourceLimit::new(
            ConversionResource::CollectionSourceBytes,
            self.limits.collection().max_source_bytes(),
        )
        .check(actual)
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
        let actual = u64::try_from_usize(actual).map_err(|source| {
            MeasuredBudgetError::quantity(ConversionResource::CollectionSourceBytes, source)
        })?;
        limit.check(actual).map_err(MeasuredBudgetError::from)
    }

    /// Checks cumulative built-in String output payload bytes.
    ///
    /// This is a non-mutating pre-check; call a consume method only after the
    /// payload has been materialized successfully.
    #[inline]
    pub fn check_output_bytes(
        &self,
        amount: u64,
    ) -> Result<(), BudgetError<ConversionResource, u64>> {
        self.budget.check_output_bytes(amount)
    }

    /// Checks output bytes measured by a native Rust string length.
    #[inline]
    pub fn check_output_bytes_usize(
        &self,
        amount: usize,
    ) -> Result<(), MeasuredBudgetError<ConversionResource, u64>> {
        self.budget.check_output_bytes_usize(amount)
    }

    /// Consumes cumulative output payload bytes after a successful pre-check.
    ///
    /// # Errors
    ///
    /// Returns a budget error when the cumulative output-byte limit is
    /// exhausted. The counter is unchanged when admission fails.
    #[inline]
    pub fn consume_output_bytes(
        &mut self,
        amount: u64,
    ) -> Result<(), BudgetError<ConversionResource, u64>> {
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
    pub fn try_write_string<E, F>(
        &mut self,
        render: F,
    ) -> Result<String, BudgetedStringError<ConversionResource, E>>
    where
        E: std::fmt::Debug + std::fmt::Display,
        F: FnOnce(&mut BudgetedStringWriter<'_, ConversionResource>) -> Result<(), E>,
    {
        self.budget.try_write_string(render)
    }

    /// Enters one structured scalar or container node.
    #[inline]
    pub fn enter_structured_node(
        &mut self,
        depth: u64,
    ) -> Result<(), BudgetError<ConversionResource, u64>> {
        self.budget.structured_mut().enter_node(depth)
    }

    /// Enters a structured node measured from a native traversal depth.
    #[inline]
    pub fn enter_structured_node_usize(
        &mut self,
        depth: usize,
    ) -> Result<(), MeasuredBudgetError<ConversionResource, u64>> {
        self.budget.structured_mut().enter_node_usize(depth)
    }

    /// Enters one structured sequence node after checking a native item count.
    #[cfg(feature = "json")]
    #[inline]
    pub fn enter_structured_sequence_usize(
        &mut self,
        depth: usize,
        items: usize,
    ) -> Result<(), MeasuredBudgetError<ConversionResource, u64>> {
        self.budget.structured_mut().enter_array_usize(depth, items)
    }

    /// Enters one structured sequence node after checking its item count.
    #[cfg(feature = "json")]
    #[inline]
    pub fn enter_structured_sequence(
        &mut self,
        depth: u64,
        items: u64,
    ) -> Result<(), BudgetError<ConversionResource, u64>> {
        self.budget.structured_mut().enter_array(depth, items)
    }

    /// Enters one structured map node after checking its entry count.
    #[inline]
    pub fn enter_structured_map(
        &mut self,
        depth: u64,
        entries: u64,
    ) -> Result<(), BudgetError<ConversionResource, u64>> {
        self.budget.structured_mut().enter_object(depth, entries)
    }

    /// Enters a structured map measured from native depth and entry counts.
    #[inline]
    pub fn enter_structured_map_usize(
        &mut self,
        depth: usize,
        entries: usize,
    ) -> Result<(), MeasuredBudgetError<ConversionResource, u64>> {
        self.budget
            .structured_mut()
            .enter_object_usize(depth, entries)
    }

    /// Checks and consumes one structured object key payload.
    #[inline]
    pub fn consume_structured_key_bytes(
        &mut self,
        amount: u64,
    ) -> Result<(), BudgetError<ConversionResource, u64>> {
        self.budget.structured_mut().consume_key_bytes(amount)
    }

    /// Checks and consumes a structured key measured by a native string length.
    #[inline]
    pub fn consume_structured_key_bytes_usize(
        &mut self,
        amount: usize,
    ) -> Result<(), MeasuredBudgetError<ConversionResource, u64>> {
        self.budget.structured_mut().consume_key_bytes_usize(amount)
    }

    /// Checks and consumes one structured string value payload.
    #[inline]
    pub fn consume_structured_string_bytes(
        &mut self,
        amount: u64,
    ) -> Result<(), BudgetError<ConversionResource, u64>> {
        self.budget.structured_mut().consume_string_bytes(amount)
    }

    /// Checks and consumes a structured string measured by a native length.
    #[inline]
    pub fn consume_structured_string_bytes_usize(
        &mut self,
        amount: usize,
    ) -> Result<(), MeasuredBudgetError<ConversionResource, u64>> {
        self.budget
            .structured_mut()
            .consume_string_bytes_usize(amount)
    }

    /// Checks and consumes one structured number representation payload.
    #[cfg(feature = "json")]
    #[inline]
    pub fn consume_structured_number_bytes(
        &mut self,
        amount: u64,
    ) -> Result<(), BudgetError<ConversionResource, u64>> {
        self.budget.structured_mut().consume_number_bytes(amount)
    }

    /// Checks and consumes a structured number measured by native text length.
    #[cfg(feature = "json")]
    #[inline]
    pub fn consume_structured_number_bytes_usize(
        &mut self,
        amount: usize,
    ) -> Result<(), MeasuredBudgetError<ConversionResource, u64>> {
        self.budget
            .structured_mut()
            .consume_number_bytes_usize(amount)
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
