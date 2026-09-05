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

/// Test that admission preserves the original scalar item source index.
#[test]
fn test_admitted_scalar_item_preserves_source_index() {
    let policy = ConversionPolicy::env_friendly();
    let limits = ConversionLimits::default();
    let mut session = ConversionSession::new(&policy, &limits);
    let mut source = session.admit_scalar_string_source(",,,,,,,42").expect("source fits");
    let admitted = source.next_item().expect("retained item").expect("admission fits");

    assert_eq!(admitted.source_index(), 7);
    assert_eq!(admitted.convert::<u32>(), Ok(42));
    assert_eq!(session.items_used(), 1);
}

/// Verifies conversion can return the exact session retained by the token.
#[test]
fn test_admitted_scalar_item_can_return_its_session() {
    let policy = ConversionPolicy::env_friendly();
    let limits = ConversionLimits::default();
    let mut session = ConversionSession::new(&policy, &limits);
    let mut source = session.admit_scalar_string_source(",,,true").expect("source fits");
    let admitted = source.next_item().expect("retained item").expect("admission fits");

    let (value, returned_session) = admitted
        .convert_with_session::<bool>()
        .expect("admitted source should convert");
    assert!(value);
    assert_eq!(returned_session.items_used(), 1);
}
