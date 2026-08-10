// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Reusable accounting session for one or more data conversions.

use qubit_budget::BudgetError;
use qubit_budget::JsonBudget;
use qubit_budget::JsonLimits;
use qubit_budget::ResourceBudget;
use qubit_budget::StructureLimits;

use super::conversion_resource::ConversionResource;
use super::data_conversion_target::DataConversionTarget;
use super::data_converter::DataConverter;
use super::error::DataConversionError;
use super::options::DataConversionOptions;

/// Mutable conversion accounting shared by nested and batch conversions.
#[must_use]
#[derive(Debug, PartialEq, Eq)]
pub struct ConversionSession<'a> {
    options: &'a DataConversionOptions,
    items: ResourceBudget<ConversionResource, usize>,
    input_bytes: ResourceBudget<ConversionResource, usize>,
    output_bytes: ResourceBudget<ConversionResource, usize>,
    json: JsonBudget<ConversionResource, usize>,
}

impl<'a> ConversionSession<'a> {
    /// Creates a fresh session with all budgets copied from `options`.
    #[inline]
    pub fn new(options: &'a DataConversionOptions) -> Self {
        let structured = options.structured();
        let structure_limits = StructureLimits::empty()
            .with_depth_limit(*structured.max_depth_limit())
            .with_nodes_limit(*options.budget().max_structured_nodes_limit())
            .with_sequence_items_limit(*structured.max_sequence_items_limit())
            .with_map_entries_limit(*structured.max_map_entries_limit());
        Self {
            options,
            items: ResourceBudget::from_limit(
                *options.budget().max_items_limit(),
            ),
            input_bytes: ResourceBudget::from_limit(
                *options.budget().max_input_bytes_limit(),
            ),
            output_bytes: ResourceBudget::from_limit(
                *options.budget().max_output_bytes_limit(),
            ),
            json: JsonLimits::empty()
                .with_structure_limits(structure_limits)
                .budget(),
        }
    }

    /// Returns the immutable conversion options associated with this session.
    #[inline(always)]
    pub const fn options(&self) -> &'a DataConversionOptions {
        self.options
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
    pub fn consume_item(
        &mut self,
    ) -> Result<(), BudgetError<ConversionResource, usize>> {
        self.items.try_consume(1)
    }

    /// Consumes cumulative normalized input bytes.
    #[inline]
    pub fn consume_input_bytes(
        &mut self,
        amount: usize,
    ) -> Result<(), BudgetError<ConversionResource, usize>> {
        self.input_bytes.try_consume(amount)
    }

    /// Checks cumulative built-in String output payload bytes.
    #[inline]
    pub fn check_output_bytes(
        &self,
        amount: usize,
    ) -> Result<(), BudgetError<ConversionResource, usize>> {
        self.output_bytes.check_available(amount)
    }

    /// Consumes cumulative output payload bytes after a successful pre-check.
    #[inline]
    pub fn consume_output_bytes(
        &mut self,
        amount: usize,
    ) -> Result<(), BudgetError<ConversionResource, usize>> {
        self.output_bytes.try_consume(amount)
    }

    /// Checks one numeric text measurement.
    #[inline]
    pub fn check_numeric_text_bytes(
        &self,
        actual: usize,
    ) -> Result<(), BudgetError<ConversionResource, usize>> {
        self.options()
            .numeric()
            .limits()
            .max_text_bytes_limit()
            .check(actual)
    }

    /// Checks one BigInteger digit measurement.
    #[inline]
    pub fn check_big_integer_digits(
        &self,
        actual: usize,
    ) -> Result<(), BudgetError<ConversionResource, usize>> {
        self.options()
            .numeric()
            .limits()
            .max_big_integer_digits_limit()
            .check(actual)
    }

    /// Checks one structured text measurement.
    #[inline]
    pub fn check_structured_text_bytes(
        &self,
        actual: usize,
    ) -> Result<(), BudgetError<ConversionResource, usize>> {
        self.options()
            .structured()
            .max_text_bytes_limit()
            .check(actual)
    }

    /// Enters one structured scalar or container node.
    #[inline]
    pub fn enter_node(
        &mut self,
        depth: usize,
    ) -> Result<(), BudgetError<ConversionResource, usize>> {
        self.json.enter_node(depth)
    }

    /// Enters one structured sequence node after checking its item count.
    #[inline]
    pub fn enter_sequence(
        &mut self,
        depth: usize,
        items: usize,
    ) -> Result<(), BudgetError<ConversionResource, usize>> {
        self.json.enter_array(depth, items)
    }

    /// Enters one structured map node after checking its entry count.
    #[inline]
    pub fn enter_map(
        &mut self,
        depth: usize,
        entries: usize,
    ) -> Result<(), BudgetError<ConversionResource, usize>> {
        self.json.enter_object(depth, entries)
    }

    /// Checks a structured depth without changing session state.
    #[inline]
    pub fn check_depth(
        &self,
        depth: usize,
    ) -> Result<(), BudgetError<ConversionResource, usize>> {
        self.json.check_depth(depth)
    }

    /// Checks one sequence item count without changing session state.
    #[inline]
    pub fn check_sequence_items(
        &self,
        items: usize,
    ) -> Result<(), BudgetError<ConversionResource, usize>> {
        self.json.check_sequence_items(items)
    }

    /// Checks one map entry count without changing session state.
    #[inline]
    pub fn check_map_entries(
        &self,
        entries: usize,
    ) -> Result<(), BudgetError<ConversionResource, usize>> {
        self.json.check_map_entries(entries)
    }

    /// Returns cumulative item usage.
    #[inline(always)]
    pub fn used_items(&self) -> usize {
        self.items.used()
    }

    /// Returns cumulative input byte usage.
    #[inline(always)]
    pub fn used_input_bytes(&self) -> usize {
        self.input_bytes.used()
    }

    /// Returns cumulative output byte usage.
    #[inline(always)]
    pub fn used_output_bytes(&self) -> usize {
        self.output_bytes.used()
    }

    /// Returns remaining top-level item capacity.
    #[inline(always)]
    pub const fn remaining_items(&self) -> usize {
        self.items.remaining()
    }

    /// Returns the shared JSON budget for budget-aware structured decoding.
    ///
    /// This crate-private hook must be used only by converters that have
    /// already selected a supported JSON target and normalized any textual
    /// input.
    ///
    /// # Returns
    ///
    /// Returns the mutable JSON budget shared by this conversion session.
    #[cfg(feature = "json")]
    #[inline(always)]
    pub(crate) fn json_budget_mut(
        &mut self,
    ) -> &mut JsonBudget<ConversionResource, usize> {
        &mut self.json
    }
}
