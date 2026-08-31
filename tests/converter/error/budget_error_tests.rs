// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Regression tests for stable resource-error facts.

use qubit_datatype::ConversionLimits;
use qubit_datatype::ConversionOperationLimits;
use qubit_datatype::ConversionPolicy;
use qubit_datatype::ConversionResource;
use qubit_datatype::ConversionSession;
use qubit_datatype::DataConverter;

#[test]
fn test_resource_accessors_preserve_complete_budget_facts() {
    let policy = ConversionPolicy::default();
    let limits = ConversionLimits::builder()
        .operation_limits(ConversionOperationLimits::builder().max_output_bytes(4).build())
        .build();
    let mut session = ConversionSession::new(&policy, &limits);
    DataConverter::from(123_i32)
        .to_in::<String>(&mut session)
        .expect("initial output should fit");
    let error = DataConverter::from(456_i32)
        .to_in::<String>(&mut session)
        .expect_err("next output should exceed remaining capacity");

    assert_eq!(error.resource(), Some(ConversionResource::OutputBytes));
    assert_eq!(error.configured_limit(), Some(4));
    assert_eq!(error.remaining(), Some(1));
    assert_eq!(error.requested(), Some(3));
}
