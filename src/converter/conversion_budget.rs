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
use qubit_budget::MeasuredBudgetError;
use qubit_budget::ResourceBudget;
use qubit_budget::ResourceQuantity;

use super::ConversionLimits;
use super::ConversionResource;

/// Mutable budgets owned by one [`super::ConversionSession`].
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct ConversionBudget {
    /// Cumulative converted-item budget.
    items: ResourceBudget<ConversionResource, u64>,

    /// Cumulative normalized input-byte budget.
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
        let structure_limits = structured
            .value()
            .structure_limits()
            .with_nodes_limit(*operation.structured_nodes_limit());
        let value_limits = (*structured.value())
            .with_structure_limits(structure_limits)
            .with_payload_bytes_limit(*operation.structured_payload_bytes_limit());
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
        self.items.try_consume(1)
    }

    /// Consumes normalized input bytes.
    #[inline]
    pub(crate) fn consume_input_bytes(
        &mut self,
        amount: u64,
    ) -> Result<(), BudgetError<ConversionResource, u64>> {
        self.input_bytes.try_consume(amount)
    }

    /// Consumes input bytes measured by Rust's native string length type.
    #[inline]
    pub(crate) fn consume_input_bytes_usize(
        &mut self,
        amount: usize,
    ) -> Result<(), MeasuredBudgetError<ConversionResource, u64>> {
        let amount = u64::try_from_usize(amount).map_err(|source| {
            MeasuredBudgetError::quantity(*self.input_bytes.resource(), source)
        })?;
        self.consume_input_bytes(amount)
            .map_err(MeasuredBudgetError::from)
    }

    /// Checks output capacity without changing accounting.
    #[inline]
    pub(crate) fn check_output_bytes(
        &self,
        amount: u64,
    ) -> Result<(), BudgetError<ConversionResource, u64>> {
        self.output_bytes.check_available(amount)
    }

    /// Checks output bytes measured by Rust's native string length type.
    #[inline]
    pub(crate) fn check_output_bytes_usize(
        &self,
        amount: usize,
    ) -> Result<(), MeasuredBudgetError<ConversionResource, u64>> {
        let amount = u64::try_from_usize(amount).map_err(|source| {
            MeasuredBudgetError::quantity(*self.output_bytes.resource(), source)
        })?;
        self.check_output_bytes(amount)
            .map_err(MeasuredBudgetError::from)
    }

    /// Consumes output payload bytes.
    #[inline]
    pub(crate) fn consume_output_bytes(
        &mut self,
        amount: u64,
    ) -> Result<(), BudgetError<ConversionResource, u64>> {
        self.output_bytes.try_consume(amount)
    }

    /// Consumes output bytes measured by Rust's native string length type.
    #[inline]
    pub(crate) fn consume_output_bytes_usize(
        &mut self,
        amount: usize,
    ) -> Result<(), MeasuredBudgetError<ConversionResource, u64>> {
        let amount = u64::try_from_usize(amount).map_err(|source| {
            MeasuredBudgetError::quantity(*self.output_bytes.resource(), source)
        })?;
        self.consume_output_bytes(amount)
            .map_err(MeasuredBudgetError::from)
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

    pub(crate) fn items_used(&self) -> u64 {
        self.items.used()
    }

    pub(crate) fn input_bytes_remaining(&self) -> u64 {
        self.input_bytes.remaining()
    }

    pub(crate) fn output_bytes_remaining(&self) -> u64 {
        self.output_bytes.remaining()
    }

    pub(crate) fn input_bytes_used(&self) -> u64 {
        self.input_bytes.used()
    }

    pub(crate) fn output_bytes_used(&self) -> u64 {
        self.output_bytes.used()
    }

    pub(crate) fn structured_nodes_used(&self) -> u64 {
        self.structured.structure_budget().used_nodes()
    }

    pub(crate) fn structured_payload_bytes_used(&self) -> u64 {
        match self.structured.payload_budget() {
            Some(payload) => payload.used(),
            None => 0,
        }
    }

    /// Returns shared structured accounting.
    #[inline(always)]
    pub(crate) const fn structured_mut(&mut self) -> &mut JsonValueBudget<ConversionResource, u64> {
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
