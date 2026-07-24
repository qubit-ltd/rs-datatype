// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests for the exact unit-suffixed duration string visitor.

use std::time::Duration;

use qubit_datatype::serde::duration_with_unit;
use serde::de::value::{
    Error as ValueError,
    StringDeserializer,
};

use super::BorrowedStrOnlyDeserializer;

/// Verifies the visitor accepts borrowed exact duration text.
#[test]
fn test_duration_with_unit_visitor_accepts_borrowed_text() {
    let deserializer = BorrowedStrOnlyDeserializer::new("42ns");
    let duration = duration_with_unit::deserialize(deserializer)
        .expect("borrowed duration text should deserialize");

    assert_eq!(duration, Duration::from_nanos(42));
}

/// Verifies the visitor accepts owned exact duration text.
#[test]
fn test_duration_with_unit_visitor_accepts_owned_text() {
    let deserializer =
        StringDeserializer::<ValueError>::new("42ns".to_string());
    let duration = duration_with_unit::deserialize(deserializer)
        .expect("owned duration text should deserialize");

    assert_eq!(duration, Duration::from_nanos(42));
}

/// Verifies the visitor maps unsupported units into the deserializer error.
#[test]
fn test_duration_with_unit_visitor_rejects_unsupported_unit() {
    let deserializer =
        StringDeserializer::<ValueError>::new("42fortnights".to_string());
    let error = duration_with_unit::deserialize(deserializer)
        .expect_err("an unsupported unit should be rejected");

    assert!(error.to_string().contains("unsupported duration unit"));
}
