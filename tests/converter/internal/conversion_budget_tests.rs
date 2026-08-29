// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Conversion-budget integration tests.

use qubit_datatype::ConversionLimits;
use qubit_datatype::ConversionOperationLimits;
use qubit_datatype::ConversionPolicy;
use qubit_datatype::ConversionSession;

/// Verifies output accounting rejects a request exceeding the remaining budget.
#[test]
fn test_output_budget_precheck_preserves_remaining_capacity() {
    let policy = ConversionPolicy::default();
    let limits = ConversionLimits::builder()
        .operation_limits(ConversionOperationLimits::builder().max_output_bytes(2).build())
        .build();
    let mut session = ConversionSession::new(&policy, &limits);

    session
        .consume_output_bytes_usize(1)
        .expect("first output byte should fit");
    let _error = session
        .check_output_bytes_usize(2)
        .expect_err("precheck should reject bytes beyond remaining capacity");
    session
        .consume_output_bytes_usize(1)
        .expect("failed precheck must not consume capacity");
}
