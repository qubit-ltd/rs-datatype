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
use qubit_budget::MeasuredBudgetError;
use qubit_budget::ResourceBudget;
use qubit_budget::StructureLimits;
use qubit_budget::json::JsonValueBudget;
use qubit_budget::json::JsonValueLimits;
use qubit_budget::json::JsonValueTransaction;

use super::super::conversion_resource::ConversionResource;
use super::super::options::ConversionLimits;

/// Mutable budgets owned by one [`crate::converter::ConversionSession`].
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct ConversionBudget {
    /// Cumulative converted-item budget.
    items: ResourceBudget<ConversionResource, u64>,

    /// Cumulative input-byte budget.
    input_bytes: ResourceBudget<ConversionResource, u64>,

    /// Cumulative output payload-byte budget.
    output_bytes: ResourceBudget<ConversionResource, u64>,

    /// Shared structured value accounting.
    structured: JsonValueBudget<ConversionResource, u64>,
}

impl ConversionBudget {
    /// Creates fresh accounting from immutable conversion limits.
    #[inline]
    pub(crate) fn new(limits: &ConversionLimits) -> Self {
        let structured = limits.structured();
        let operation = limits.operation();
        let source_structure = structured.value().structure_limits();
        let mut structure_builder = StructureLimits::builder();
        if let Some(limit) = source_structure.depth_limit() {
            structure_builder = structure_builder.depth_limit(*limit);
        }
        if let Some(limit) = source_structure.sequence_items_limit() {
            structure_builder = structure_builder.sequence_items_limit(*limit);
        }
        if let Some(limit) = source_structure.map_entries_limit() {
            structure_builder = structure_builder.map_entries_limit(*limit);
        }
        if let Some(limit) = source_structure.key_bytes_limit() {
            structure_builder = structure_builder.key_bytes_limit(*limit);
        }
        let structure_limits = structure_builder
            .nodes_limit(*operation.structured_nodes_limit())
            .build();
        let value_limits = JsonValueLimits::builder()
            .structure_limits(structure_limits)
            .payload_bytes_limit(*operation.structured_payload_bytes_limit())
            .build();
        Self {
            items: ResourceBudget::from_limit(*operation.items_limit()),
            input_bytes: ResourceBudget::from_limit(*operation.input_bytes_limit()),
            output_bytes: ResourceBudget::from_limit(*operation.output_bytes_limit()),
            structured: JsonValueBudget::new(value_limits),
        }
    }

    /// Consumes one converted item.
    #[inline]
    pub(crate) fn consume_item(&mut self) -> Result<(), BudgetError<ConversionResource, u64>> {
        self.items.try_consume(1).map_err(Into::into)
    }

    /// Consumes input bytes measured by Rust's native string length type.
    #[inline]
    pub(crate) fn consume_input_bytes_usize(
        &mut self,
        amount: usize,
    ) -> Result<(), MeasuredBudgetError<ConversionResource, u64>> {
        self.input_bytes.try_consume_usize(amount)
    }

    /// Consumes output bytes measured by Rust's native string length type.
    #[inline]
    pub(crate) fn consume_output_bytes_usize(
        &mut self,
        amount: usize,
    ) -> Result<(), MeasuredBudgetError<ConversionResource, u64>> {
        self.output_bytes.try_consume_usize(amount)
    }

    /// Renders and commits one String payload transactionally.
    #[inline]
    pub(crate) fn try_write_string<E, F>(
        &mut self,
        render: F,
    ) -> Result<String, BudgetedStringError<ConversionResource, E>>
    where
        E: std::fmt::Debug + std::fmt::Display,
        F: FnOnce(&mut BudgetedStringWriter<'_, ConversionResource>) -> Result<(), E>,
    {
        self.output_bytes.try_write_string(render)
    }

    /// Returns the remaining item capacity for internal allocation planning.
    #[inline(always)]
    pub(crate) const fn remaining_items(&self) -> u64 {
        self.items.remaining()
    }

    /// Returns cumulative converted items already consumed.
    pub(crate) fn items_used(&self) -> u64 {
        self.items.used()
    }

    /// Returns remaining cumulative input-byte capacity.
    pub(crate) fn input_bytes_remaining(&self) -> u64 {
        self.input_bytes.remaining()
    }

    /// Returns remaining cumulative output-byte capacity.
    pub(crate) fn output_bytes_remaining(&self) -> u64 {
        self.output_bytes.remaining()
    }

    /// Returns cumulative input bytes already consumed.
    pub(crate) fn input_bytes_used(&self) -> u64 {
        self.input_bytes.used()
    }

    /// Returns cumulative output bytes already consumed.
    pub(crate) fn output_bytes_used(&self) -> u64 {
        self.output_bytes.used()
    }

    /// Returns cumulative structured nodes already consumed.
    pub(crate) fn structured_nodes_used(&self) -> u64 {
        self.structured.used_nodes().unwrap_or(0)
    }

    /// Returns cumulative structured payload bytes already consumed.
    pub(crate) fn structured_payload_bytes_used(&self) -> u64 {
        self.structured.used_payload_bytes().unwrap_or(0)
    }

    /// Starts an all-or-nothing transaction over the shared structured budget.
    #[inline(always)]
    pub(crate) fn structured_transaction(&mut self) -> JsonValueTransaction<'_, ConversionResource, u64> {
        self.structured.transaction()
    }

    /// Returns shared structured accounting for JSON sessions.
    #[cfg(feature = "json")]
    #[inline(always)]
    pub(crate) fn structured_mut(&mut self) -> &mut JsonValueBudget<ConversionResource, u64> {
        &mut self.structured
    }

    /// Returns the shared output and structured budgets for online encoding.
    #[cfg(feature = "json")]
    #[inline(always)]
    pub(crate) fn split_json_mut(
        &mut self,
    ) -> (
        &mut ResourceBudget<ConversionResource, u64>,
        &mut JsonValueBudget<ConversionResource, u64>,
    ) {
        (&mut self.output_bytes, &mut self.structured)
    }
}
