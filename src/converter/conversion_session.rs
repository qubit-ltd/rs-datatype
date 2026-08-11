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
use qubit_budget::ResourceLimit;

use super::conversion_budget::ConversionBudget;
use super::conversion_resource::ConversionResource;
use super::data_conversion_target::DataConversionTarget;
use super::data_converter::DataConverter;
use super::error::DataConversionError;
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
    pub fn new(
        policy: &'a ConversionPolicy,
        limits: &'a ConversionLimits,
    ) -> Self {
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
    #[inline(always)]
    pub fn delegate<T>(
        &mut self,
        source: &DataConverter<'_>,
    ) -> Result<T, DataConversionError>
    where
        T: DataConversionTarget,
    {
        T::convert_from(source, self)
    }

    /// Delegates an owned nested conversion while preserving this session.
    #[inline(always)]
    pub fn delegate_owned<T>(
        &mut self,
        source: DataConverter<'_>,
    ) -> Result<T, DataConversionError>
    where
        T: DataConversionTarget,
    {
        T::convert_owned(source, self)
    }

    /// Consumes one top-level conversion item.
    #[inline]
    pub(crate) fn consume_item(
        &mut self,
    ) -> Result<(), BudgetError<ConversionResource, usize>> {
        self.budget.consume_item()
    }

    /// Consumes cumulative normalized input bytes.
    #[inline]
    pub(crate) fn consume_input_bytes(
        &mut self,
        amount: usize,
    ) -> Result<(), BudgetError<ConversionResource, usize>> {
        self.budget.consume_input_bytes(amount)
    }

    /// Checks one complete scalar collection source before scanning it.
    #[inline]
    pub(crate) fn check_collection_source_bytes(
        &self,
        actual: usize,
    ) -> Result<(), BudgetError<ConversionResource, usize>> {
        ResourceLimit::new(
            ConversionResource::CollectionSourceBytes,
            self.limits.collection().max_source_bytes(),
        )
        .check(actual)
    }

    /// Checks cumulative built-in String output payload bytes.
    #[inline]
    pub(crate) fn check_output_bytes(
        &self,
        amount: usize,
    ) -> Result<(), BudgetError<ConversionResource, usize>> {
        self.budget.check_output_bytes(amount)
    }

    /// Consumes cumulative output payload bytes after a successful pre-check.
    #[inline]
    pub(crate) fn consume_output_bytes(
        &mut self,
        amount: usize,
    ) -> Result<(), BudgetError<ConversionResource, usize>> {
        self.budget.consume_output_bytes(amount)
    }

    /// Renders a String payload in one pass and commits its UTF-8 bytes only
    /// after the renderer succeeds.
    #[inline]
    pub(crate) fn try_write_string<E, F>(
        &mut self,
        render: F,
    ) -> Result<String, BudgetedStringError<ConversionResource, E>>
    where
        E: std::fmt::Debug + std::fmt::Display,
        F: FnOnce(
            &mut BudgetedStringWriter<'_, ConversionResource>,
        ) -> Result<(), E>,
    {
        self.budget.try_write_string(render)
    }

    /// Enters one structured scalar or container node.
    #[inline]
    pub(crate) fn enter_node(
        &mut self,
        depth: usize,
    ) -> Result<(), BudgetError<ConversionResource, usize>> {
        self.budget.structured_mut().enter_node(depth)
    }

    /// Enters one structured sequence node after checking its item count.
    #[cfg(feature = "json")]
    #[inline]
    pub(crate) fn enter_sequence(
        &mut self,
        depth: usize,
        items: usize,
    ) -> Result<(), BudgetError<ConversionResource, usize>> {
        self.budget.structured_mut().enter_array(depth, items)
    }

    /// Enters one structured map node after checking its entry count.
    #[inline]
    pub(crate) fn enter_map(
        &mut self,
        depth: usize,
        entries: usize,
    ) -> Result<(), BudgetError<ConversionResource, usize>> {
        self.budget.structured_mut().enter_object(depth, entries)
    }

    /// Checks and consumes one structured object key payload.
    #[inline]
    pub(crate) fn consume_structured_key_bytes(
        &mut self,
        amount: usize,
    ) -> Result<(), BudgetError<ConversionResource, usize>> {
        self.budget.structured_mut().consume_key_bytes(amount)
    }

    /// Checks and consumes one structured string value payload.
    #[inline]
    pub(crate) fn consume_structured_string_bytes(
        &mut self,
        amount: usize,
    ) -> Result<(), BudgetError<ConversionResource, usize>> {
        self.budget.structured_mut().consume_string_bytes(amount)
    }

    /// Checks and consumes one structured number representation payload.
    #[cfg(feature = "json")]
    #[inline]
    pub(crate) fn consume_structured_number_bytes(
        &mut self,
        amount: usize,
    ) -> Result<(), BudgetError<ConversionResource, usize>> {
        self.budget.structured_mut().consume_number_bytes(amount)
    }

    /// Returns remaining top-level item capacity.
    #[inline(always)]
    pub(crate) const fn remaining_items(&self) -> usize {
        self.budget.remaining_items()
    }
}
