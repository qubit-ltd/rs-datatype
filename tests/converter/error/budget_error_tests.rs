// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Regression tests for direct rs-budget error exposure.

use qubit_budget::BudgetError;
use qubit_datatype::ConversionResource;
use qubit_datatype::DataConversionError;
use qubit_datatype::DataType;

#[test]
fn test_budget_error_accessor_preserves_complete_budget_facts() {
    let budget_error = BudgetError::Insufficient {
        resource: ConversionResource::OutputBytes,
        limit: 4,
        remaining: 1,
        requested: 3,
    };
    let error = DataConversionError::limit_exceeded(DataType::Int32, DataType::String, budget_error.clone());

    assert_eq!(error.budget_error(), Some(&budget_error));
    assert_eq!(error.budget_error(), Some(&budget_error));
}
