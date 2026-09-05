// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Contracts for source-bound scalar collection admission.

use qubit_datatype::CollectionConversionPolicy;
use qubit_datatype::ConversionLimits;
use qubit_datatype::ConversionOperationLimits;
use qubit_datatype::ConversionPolicy;
use qubit_datatype::ConversionResource;
use qubit_datatype::ConversionSession;
use qubit_datatype::EmptyItemPolicy;

/// A source is charged once and filtered elements keep their raw positions.
#[test]
fn test_source_admission_preserves_indices_and_charges_once() {
    let policy = ConversionPolicy::env_friendly();
    let limits = ConversionLimits::default();
    let mut session = ConversionSession::new(&policy, &limits);
    {
        let mut source = session.admit_scalar_string_source("1,,3").expect("source fits");
        let first = source.next_item().expect("first item").expect("first admission");
        assert_eq!(first.source_index(), 0);
        assert_eq!(first.convert::<u8>(), Ok(1));
        let third = source.next_item().expect("third raw item").expect("second admission");
        assert_eq!(third.source_index(), 2);
        assert_eq!(third.convert::<u8>(), Ok(3));
        assert!(source.next_item().is_none());
    }
    assert_eq!(session.input_bytes_used(), 4);
    assert_eq!(session.items_used(), 2);
}

/// Rejected source admission does not scan or charge its elements.
#[test]
fn test_source_admission_rejects_input_before_items() {
    let policy = ConversionPolicy::env_friendly();
    let limits = ConversionLimits::builder()
        .operation_limits(ConversionOperationLimits::builder().max_input_bytes(0).build())
        .build();
    let mut session = ConversionSession::new(&policy, &limits);
    let error = session.admit_scalar_string_source("1").expect_err("input is forbidden");
    assert_eq!(error.resource(), Some(ConversionResource::InputBytes));
    assert_eq!(session.input_bytes_used(), 0);
    assert_eq!(session.items_used(), 0);
}

/// A failed item admission retains the raw index and exhausts the source.
#[test]
fn test_source_admission_stops_at_session_item_limit() {
    let policy = ConversionPolicy::env_friendly();
    let limits = ConversionLimits::builder()
        .operation_limits(ConversionOperationLimits::builder().max_items(1).build())
        .build();
    let mut session = ConversionSession::new(&policy, &limits);
    let mut source = session.admit_scalar_string_source("1,,2,3").expect("source fits");
    assert_eq!(
        source.next_item().expect("first").expect("admitted").convert::<u8>(),
        Ok(1)
    );
    let error = source
        .next_item()
        .expect("second retained item")
        .expect_err("item limit");
    assert_eq!(error.source_index(), 2);
    assert_eq!(error.conversion_error().resource(), Some(ConversionResource::Items));
    assert!(source.next_item().is_none());
}

/// A source remains exhausted after its first splitting error.
#[test]
fn test_source_admission_exhausts_after_rejected_empty_item() {
    let policy = ConversionPolicy::builder()
        .collection_policy(
            CollectionConversionPolicy::env_friendly()
                .into_builder()
                .empty_item_policy(EmptyItemPolicy::Reject)
                .build(),
        )
        .build();
    let limits = ConversionLimits::default();
    let mut session = ConversionSession::new(&policy, &limits);
    {
        let mut source = session.admit_scalar_string_source(",1").expect("source fits");
        let error = source
            .next_item()
            .expect("empty item")
            .expect_err("empty item rejected");
        assert_eq!(error.source_index(), 0);
        assert!(source.next_item().is_none());
    }
    assert_eq!(session.items_used(), 0);
    assert_eq!(session.input_bytes_used(), 2);
}

/// Dropping a partially consumed source keeps full input but only consumed
/// items.
#[test]
fn test_source_admission_early_drop_keeps_only_admitted_items() {
    let policy = ConversionPolicy::env_friendly();
    let limits = ConversionLimits::default();
    let mut session = ConversionSession::new(&policy, &limits);
    {
        let mut source = session.admit_scalar_string_source("1,2,3").expect("source fits");
        assert_eq!(
            source.next_item().expect("first").expect("admitted").convert::<u8>(),
            Ok(1)
        );
    }
    assert_eq!(session.items_used(), 1);
    assert_eq!(session.input_bytes_used(), 5);
}
