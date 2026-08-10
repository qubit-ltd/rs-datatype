// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Reusable accounting session for one or more data conversions.

use qubit_budget::{ResourceBudget, StructureBudget, StructureLimits};

use super::conversion_resource::ConversionResource;
use super::error::ConversionLimitExceeded;
use super::options::DataConversionOptions;

/// Mutable conversion accounting shared by nested and batch conversions.
#[must_use]
#[derive(Debug, PartialEq, Eq)]
pub struct ConversionSession<'a> {
    options: &'a DataConversionOptions,
    items: ResourceBudget<ConversionResource, usize>,
    input_bytes: ResourceBudget<ConversionResource, usize>,
    output_bytes: ResourceBudget<ConversionResource, usize>,
    structure: StructureBudget<ConversionResource, usize>,
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
            items: ResourceBudget::from_limit(*options.budget().max_items_limit()),
            input_bytes: ResourceBudget::from_limit(*options.budget().max_input_bytes_limit()),
            output_bytes: ResourceBudget::from_limit(*options.budget().max_output_bytes_limit()),
            structure: structure_limits.budget(),
        }
    }

    /// Returns the immutable conversion options associated with this session.
    #[inline(always)]
    pub const fn options(&self) -> &'a DataConversionOptions {
        self.options
    }

    /// Consumes one top-level conversion item.
    #[inline]
    pub fn consume_item(&mut self) -> Result<(), ConversionLimitExceeded> {
        self.items
            .try_consume(1)
            .map_err(ConversionLimitExceeded::from_budget_error)
    }

    /// Consumes cumulative normalized input bytes.
    #[inline]
    pub fn consume_input_bytes(&mut self, amount: usize) -> Result<(), ConversionLimitExceeded> {
        self.input_bytes
            .try_consume(amount)
            .map_err(ConversionLimitExceeded::from_budget_error)
    }

    /// Consumes cumulative newly allocated output bytes.
    #[inline]
    pub fn consume_output_bytes(&mut self, amount: usize) -> Result<(), ConversionLimitExceeded> {
        self.output_bytes
            .try_consume(amount)
            .map_err(ConversionLimitExceeded::from_budget_error)
    }

    /// Checks one numeric text measurement.
    #[inline]
    pub fn check_numeric_text_bytes(&self, actual: usize) -> Result<(), ConversionLimitExceeded> {
        self.options()
            .numeric()
            .limits()
            .max_text_bytes_limit()
            .check(actual)
            .map_err(ConversionLimitExceeded::from_budget_error)
    }

    /// Checks one BigInteger digit measurement.
    #[inline]
    pub fn check_big_integer_digits(&self, actual: usize) -> Result<(), ConversionLimitExceeded> {
        self.options()
            .numeric()
            .limits()
            .max_big_integer_digits_limit()
            .check(actual)
            .map_err(ConversionLimitExceeded::from_budget_error)
    }

    /// Checks one structured text measurement.
    #[inline]
    pub fn check_structured_text_bytes(
        &self,
        actual: usize,
    ) -> Result<(), ConversionLimitExceeded> {
        self.options()
            .structured()
            .max_text_bytes_limit()
            .check(actual)
            .map_err(ConversionLimitExceeded::from_budget_error)
    }

    /// Enters one structured scalar or container node.
    #[inline]
    pub fn enter_node(&mut self, depth: usize) -> Result<(), ConversionLimitExceeded> {
        self.structure
            .enter_node(depth)
            .map_err(ConversionLimitExceeded::from_budget_error)
    }

    /// Enters one structured sequence node after checking its item count.
    #[inline]
    pub fn enter_sequence(
        &mut self,
        depth: usize,
        items: usize,
    ) -> Result<(), ConversionLimitExceeded> {
        self.structure
            .enter_sequence(depth, items)
            .map_err(ConversionLimitExceeded::from_budget_error)
    }

    /// Enters one structured map node after checking its entry count.
    #[inline]
    pub fn enter_map(
        &mut self,
        depth: usize,
        entries: usize,
    ) -> Result<(), ConversionLimitExceeded> {
        self.structure
            .enter_map(depth, entries)
            .map_err(ConversionLimitExceeded::from_budget_error)
    }

    /// Checks a structured depth without changing session state.
    #[inline]
    pub fn check_depth(&self, depth: usize) -> Result<(), ConversionLimitExceeded> {
        self.structure
            .check_depth(depth)
            .map_err(ConversionLimitExceeded::from_budget_error)
    }

    /// Checks one sequence item count without changing session state.
    #[inline]
    pub fn check_sequence_items(&self, items: usize) -> Result<(), ConversionLimitExceeded> {
        self.structure
            .check_sequence_items(items)
            .map_err(ConversionLimitExceeded::from_budget_error)
    }

    /// Checks one map entry count without changing session state.
    #[inline]
    pub fn check_map_entries(&self, entries: usize) -> Result<(), ConversionLimitExceeded> {
        self.structure
            .check_map_entries(entries)
            .map_err(ConversionLimitExceeded::from_budget_error)
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
}
