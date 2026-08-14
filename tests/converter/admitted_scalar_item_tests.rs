// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests for session-admitted scalar collection items.

use qubit_datatype::converter::ConversionLimits;
use qubit_datatype::converter::ConversionPolicy;
use qubit_datatype::converter::ConversionSession;
use qubit_datatype::converter::ScalarItem;

/// Test that admission preserves the original scalar item source index.
#[test]
fn test_admitted_scalar_item_preserves_source_index() {
    let policy = ConversionPolicy::default();
    let limits = ConversionLimits::default();
    let mut session = ConversionSession::new(&policy, &limits);
    let admitted = session
        .admit_scalar_item(ScalarItem {
            source_index: 7,
            value: "value",
        })
        .expect("admission should succeed within the default budget");

    assert_eq!(admitted.source_index(), 7);
}
