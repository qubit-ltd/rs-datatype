// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests for borrowed scalar collection items.

use qubit_datatype::converter::ScalarItem;

/// Test that a scalar item preserves its source index and borrowed slice.
#[test]
fn test_scalar_item_preserves_borrowed_source_identity() {
    let source = String::from("alpha,beta");
    let item = ScalarItem {
        source_index: 1,
        value: &source[6..],
    };
    let copied = item;

    assert_eq!(copied.source_index, 1);
    assert_eq!(copied.value, "beta");
    assert_eq!(item, copied);
    assert_eq!(item.value.as_ptr(), source[6..].as_ptr());
}
