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
use qubit_datatype::converter::DataConverter;

/// Test that admission preserves the original scalar item source index.
#[test]
fn test_admitted_scalar_item_preserves_source_index() {
    let policy = ConversionPolicy::default();
    let limits = ConversionLimits::default();
    let mut session = ConversionSession::new(&policy, &limits);
    let admitted = session
        .admit_scalar_item(7, DataConverter::from(String::from("42")))
        .expect("admission should succeed within the default budget");

    assert_eq!(admitted.source_index(), 7);
    assert_eq!(admitted.convert::<u32>(), Ok(42));
    assert_eq!(session.items_used(), 1);
}

/// Verifies conversion can return the exact session retained by the token.
#[test]
fn test_admitted_scalar_item_can_return_its_session() {
    let policy = ConversionPolicy::default();
    let limits = ConversionLimits::default();
    let mut session = ConversionSession::new(&policy, &limits);
    let admitted = session
        .admit_scalar_item(3, DataConverter::from("true"))
        .expect("admission should succeed");

    let (value, returned_session) = admitted
        .convert_with_session::<bool>()
        .expect("admitted source should convert");
    assert!(value);
    assert_eq!(returned_session.items_used(), 1);
}
