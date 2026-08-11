// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Private mutable accounting for one logical conversion operation.

use qubit_budget::BudgetError;
use qubit_budget::BudgetedStringError;
use qubit_budget::BudgetedStringWriter;
use qubit_budget::JsonValueBudget;
use qubit_budget::JsonValueLimits;
use qubit_budget::ResourceBudget;
use qubit_budget::StructureLimits;

use super::ConversionLimits;
use super::ConversionResource;

/// Mutable budgets owned by one [`super::ConversionSession`].
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct ConversionBudget {
    /// Cumulative converted-item budget.
    items: ResourceBudget<ConversionResource, usize>,

    /// Cumulative normalized input-byte budget.
    input_bytes: ResourceBudget<ConversionResource, usize>,

    /// Cumulative output payload-byte budget.
    output_bytes: ResourceBudget<ConversionResource, usize>,

    /// Shared structured value accounting.
    structured: JsonValueBudget<ConversionResource, usize>,
}

impl ConversionBudget {
    /// Creates fresh accounting from immutable conversion limits.
    #[inline]
    pub(crate) fn new(limits: &ConversionLimits) -> Self {
        let structured = limits.structured();
        let operation = limits.operation();
        let structure_limits = StructureLimits::empty()
            .with_depth_limit(*structured.max_depth_limit())
            .with_nodes_limit(*operation.structured_nodes_limit())
            .with_sequence_items_limit(*structured.max_sequence_items_limit())
            .with_map_entries_limit(*structured.max_map_entries_limit());
        let value_limits = JsonValueLimits::default()
            .with_structure_limits(structure_limits)
            .with_payload_bytes_limit(
                *operation.structured_payload_bytes_limit(),
            );
        Self {
            items: ResourceBudget::from_limit(*operation.items_limit()),
            input_bytes: ResourceBudget::from_limit(
                *operation.input_bytes_limit(),
            ),
            output_bytes: ResourceBudget::from_limit(
                *operation.output_bytes_limit(),
            ),
            structured: JsonValueBudget::new(value_limits),
        }
    }

    /// Consumes one converted item.
    #[inline]
    pub(crate) fn consume_item(
        &mut self,
    ) -> Result<(), BudgetError<ConversionResource, usize>> {
        self.items.try_consume(1)
    }

    /// Consumes normalized input bytes.
    #[inline]
    pub(crate) fn consume_input_bytes(
        &mut self,
        amount: usize,
    ) -> Result<(), BudgetError<ConversionResource, usize>> {
        self.input_bytes.try_consume(amount)
    }

    /// Checks output capacity without changing accounting.
    #[inline]
    pub(crate) fn check_output_bytes(
        &self,
        amount: usize,
    ) -> Result<(), BudgetError<ConversionResource, usize>> {
        self.output_bytes.check_available(amount)
    }

    /// Consumes output payload bytes.
    #[inline]
    pub(crate) fn consume_output_bytes(
        &mut self,
        amount: usize,
    ) -> Result<(), BudgetError<ConversionResource, usize>> {
        self.output_bytes.try_consume(amount)
    }

    /// Renders and commits one String payload transactionally.
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
        self.output_bytes.try_write_string(render)
    }

    /// Returns the remaining item capacity for internal allocation planning.
    #[inline(always)]
    pub(crate) const fn remaining_items(&self) -> usize {
        self.items.remaining()
    }

    /// Returns shared structured accounting.
    #[inline(always)]
    pub(crate) const fn structured_mut(
        &mut self,
    ) -> &mut JsonValueBudget<ConversionResource, usize> {
        &mut self.structured
    }
}
