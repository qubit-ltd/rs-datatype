// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests for structured conversion resource limits.

use qubit_datatype::ConversionResource;
use qubit_datatype::StructuredConversionLimits;

/// Tests the default and customized structured text byte limits.
#[test]
fn test_structured_conversion_limits_defaults_and_customization() {
    let defaults = StructuredConversionLimits::default();
    assert_eq!(StructuredConversionLimits::DEFAULT_MAX_TEXT_BYTES, 1_048_576);
    assert_eq!(
        defaults.max_text_bytes(),
        StructuredConversionLimits::DEFAULT_MAX_TEXT_BYTES,
    );
    assert_eq!(
        StructuredConversionLimits::builder()
            .max_text_bytes(16)
            .build()
            .max_text_bytes(),
        16
    );
    assert_eq!(defaults.max_depth(), StructuredConversionLimits::DEFAULT_MAX_DEPTH);
    assert_eq!(
        defaults.max_sequence_items(),
        StructuredConversionLimits::DEFAULT_MAX_SEQUENCE_ITEMS
    );
    assert_eq!(
        defaults.max_map_entries(),
        StructuredConversionLimits::DEFAULT_MAX_MAP_ENTRIES
    );
    assert_eq!(
        defaults.text().utf8_bytes_limit().expect("text limit").maximum(),
        defaults.max_text_bytes()
    );
    assert_eq!(defaults.max_text_bytes_limit().maximum(), defaults.max_text_bytes());
    assert_eq!(defaults.value().max_depth().expect("depth limit"), defaults.max_depth());
    assert_eq!(defaults.max_depth_limit().maximum(), defaults.max_depth());
    assert_eq!(
        defaults.max_sequence_items_limit().maximum(),
        defaults.max_sequence_items()
    );
    assert_eq!(defaults.max_map_entries_limit().maximum(), defaults.max_map_entries());
    let customized = StructuredConversionLimits::builder()
        .max_depth(4)
        .max_sequence_items(5)
        .max_map_entries(6)
        .build();
    assert_eq!(customized.max_depth(), 4);
    assert_eq!(customized.max_sequence_items(), 5);
    assert_eq!(customized.max_map_entries(), 6);
}

/// Tests that the reusable value template retains its per-string limit.
#[test]
fn test_structured_limit_contract_keeps_reusable_string_limit() {
    let limits = StructuredConversionLimits::builder().max_text_bytes(2).build();
    assert_eq!(limits.max_text_bytes(), 2);
    assert_eq!(limits.value().max_string_bytes(), Some(2));
    assert_eq!(
        limits.into_builder().max_depth(4).build().value().max_string_bytes(),
        Some(2)
    );
    let wire = serde_json::to_string(&limits).expect("serialize limits");
    let restored: StructuredConversionLimits = serde_json::from_str(&wire).expect("restore limits");
    assert_eq!(restored, limits);
}

/// Tests Serde defaults and unknown-field rejection for structured limits.
#[test]
fn test_structured_conversion_limits_serde_contract() {
    assert_eq!(
        serde_json::from_str::<StructuredConversionLimits>("{}").expect("empty structured limits should use defaults"),
        StructuredConversionLimits::default(),
    );
    assert!(
        serde_json::from_str::<StructuredConversionLimits>(r#"{"max_text_bytes":32,"unexpected":true}"#,)
            .expect_err("unknown structured limit fields must be rejected")
            .to_string()
            .contains("unknown field `unexpected`")
    );
}

#[test]
fn test_structured_conversion_limits_single_override_preserves_other_limits() {
    let base = StructuredConversionLimits::builder()
        .max_text_bytes(17)
        .max_depth(19)
        .max_sequence_items(23)
        .max_map_entries(29)
        .build();
    for maximum in [0_u64, 41] {
        let cases = [
            (
                base.into_builder().max_text_bytes(maximum).build(),
                [maximum, 19, 23, 29],
            ),
            (base.into_builder().max_depth(maximum).build(), [17, maximum, 23, 29]),
            (
                base.into_builder().max_sequence_items(maximum).build(),
                [17, 19, maximum, 29],
            ),
            (
                base.into_builder().max_map_entries(maximum).build(),
                [17, 19, 23, maximum],
            ),
        ];
        for (updated, [text, depth, sequence, map]) in cases {
            let expected = StructuredConversionLimits::builder()
                .max_text_bytes(text)
                .max_depth(depth)
                .max_sequence_items(sequence)
                .max_map_entries(map)
                .build();
            assert_eq!(updated, expected);
            assert_eq!(updated.max_text_bytes(), text);
            assert_eq!(updated.max_depth(), depth);
            assert_eq!(updated.max_sequence_items(), sequence);
            assert_eq!(updated.max_map_entries(), map);
            assert_eq!(updated.value().max_string_bytes(), Some(text));
            assert_eq!(updated.value().max_depth(), Some(depth));
            assert_eq!(
                updated.max_text_bytes_limit().resource(),
                &ConversionResource::StructuredTextBytes
            );
            assert_eq!(
                updated.max_depth_limit().resource(),
                &ConversionResource::StructuredDepth
            );
            assert_eq!(
                updated.max_sequence_items_limit().resource(),
                &ConversionResource::SequenceItems
            );
            assert_eq!(
                updated.max_map_entries_limit().resource(),
                &ConversionResource::MapEntries
            );
        }
    }
}
