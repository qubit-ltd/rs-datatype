// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests for the fixed-millisecond duration string visitor.

use std::time::Duration;

use qubit_datatype::serde::duration_millis_with_unit;
use serde::de::value::{
    Error as ValueError,
    StringDeserializer,
};

use super::BorrowedStrOnlyDeserializer;

/// Verifies the visitor accepts borrowed fixed-millisecond text.
#[test]
fn test_duration_millis_with_unit_visitor_accepts_borrowed_text() {
    let deserializer = BorrowedStrOnlyDeserializer::new("42ms");
    let duration = duration_millis_with_unit::deserialize(deserializer)
        .expect("borrowed duration text should deserialize");

    assert_eq!(duration, Duration::from_millis(42));
}

/// Verifies the visitor accepts owned fixed-millisecond text.
#[test]
fn test_duration_millis_with_unit_visitor_accepts_owned_text() {
    let deserializer =
        StringDeserializer::<ValueError>::new("42ms".to_string());
    let duration = duration_millis_with_unit::deserialize(deserializer)
        .expect("owned duration text should deserialize");

    assert_eq!(duration, Duration::from_millis(42));
}

/// Verifies the visitor maps invalid text into the deserializer error.
#[test]
fn test_duration_millis_with_unit_visitor_rejects_invalid_text() {
    let deserializer = StringDeserializer::<ValueError>::new("42s".to_string());
    let error = duration_millis_with_unit::deserialize(deserializer)
        .expect_err("a non-millisecond unit should be rejected");

    assert!(error.to_string().contains("invalid duration syntax"));
}
