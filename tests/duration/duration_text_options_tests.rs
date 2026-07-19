// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests for Duration text options.

use qubit_datatype::{
    DurationTextOptions,
    DurationUnit,
    DurationUnitSuffixSet,
    SuffixlessDurationPolicy,
};

/// Tests defaults, construction, and immutable option updates.
#[test]
fn test_duration_text_options_builders() {
    assert_eq!(
        DurationTextOptions::default(),
        DurationTextOptions::new(
            SuffixlessDurationPolicy::Assume(DurationUnit::Milliseconds),
            DurationUnitSuffixSet::Extended,
        ),
    );
    assert_eq!(
        DurationTextOptions::default().max_text_bytes(),
        DurationTextOptions::DEFAULT_MAX_TEXT_BYTES,
    );

    let options = DurationTextOptions::default()
        .with_suffixless_policy(SuffixlessDurationPolicy::Reject)
        .with_unit_suffix_set(DurationUnitSuffixSet::Ascii)
        .with_max_text_bytes(4_096);
    assert_eq!(
        options.suffixless_policy(),
        SuffixlessDurationPolicy::Reject,
    );
    assert_eq!(options.unit_suffix_set(), DurationUnitSuffixSet::Ascii);
    assert_eq!(options.max_text_bytes(), 4_096);
}
