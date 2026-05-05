/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0
 *
 ******************************************************************************/
use qubit_datatype::converter::BlankStringPolicy;

/// Test equality and clone semantics for blank string policies.
#[test]
fn test_blank_string_policy_variants_are_distinct() {
    assert_eq!(BlankStringPolicy::Preserve, BlankStringPolicy::Preserve);
    assert_eq!(
        BlankStringPolicy::TreatAsMissing,
        BlankStringPolicy::TreatAsMissing,
    );
    assert_eq!(BlankStringPolicy::Reject, BlankStringPolicy::Reject);
    assert_ne!(BlankStringPolicy::Preserve, BlankStringPolicy::Reject);
}
