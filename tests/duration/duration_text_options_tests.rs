// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests for Duration text options.

use qubit_datatype::DurationTextOptions;
use qubit_datatype::DurationUnitParseMode;
use qubit_datatype::SuffixlessDurationPolicy;

/// Tests defaults, construction, and immutable option updates.
#[test]
fn test_duration_text_options_builders() {
    assert_eq!(DurationTextOptions::DEFAULT_MAX_TEXT_BYTES, 256);
    assert_eq!(
        DurationTextOptions::default(),
        DurationTextOptions::new(
            SuffixlessDurationPolicy::Reject,
            DurationUnitParseMode::Strict,
        ),
    );
    assert_eq!(
        DurationTextOptions::builder().build().max_text_bytes(),
        DurationTextOptions::DEFAULT_MAX_TEXT_BYTES,
    );

    let options = DurationTextOptions::builder()
        .suffixless_policy(SuffixlessDurationPolicy::Reject)
        .unit_parse_mode(DurationUnitParseMode::Lenient)
        .max_text_bytes(4_096)
        .build();
    assert_eq!(
        options.suffixless_policy(),
        SuffixlessDurationPolicy::Reject,
    );
    assert_eq!(options.unit_parse_mode(), DurationUnitParseMode::Lenient);
    assert_eq!(options.max_text_bytes(), 4_096);
}
