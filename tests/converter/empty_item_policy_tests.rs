/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0
 *
 ******************************************************************************/
use qubit_datatype::converter::EmptyItemPolicy;

/// Test equality and clone semantics for empty item policies.
#[test]
fn test_empty_item_policy_variants_are_distinct() {
    assert_eq!(EmptyItemPolicy::Keep, EmptyItemPolicy::Keep);
    assert_eq!(EmptyItemPolicy::Skip, EmptyItemPolicy::Skip);
    assert_eq!(EmptyItemPolicy::Reject, EmptyItemPolicy::Reject);
    assert_ne!(EmptyItemPolicy::Keep, EmptyItemPolicy::Reject);
}
