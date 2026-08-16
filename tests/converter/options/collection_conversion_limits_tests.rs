// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests for scalar collection conversion limits.

use qubit_datatype::converter::CollectionConversionLimits;

#[test]
fn test_collection_conversion_limits_defaults_builders_and_wire() {
    let defaults = CollectionConversionLimits::default();
    assert_eq!(
        defaults.max_source_bytes(),
        CollectionConversionLimits::DEFAULT_MAX_SOURCE_BYTES
    );
    assert_eq!(
        defaults.max_items(),
        CollectionConversionLimits::DEFAULT_MAX_ITEMS
    );

    let limits = CollectionConversionLimits::builder()
        .max_source_bytes(128)
        .max_items(3)
        .build();
    assert_eq!(
        serde_json::to_string(&limits)
            .expect("collection limits should serialize"),
        r#"{"max_source_bytes":128,"max_items":3}"#
    );
    assert_eq!(
        serde_json::from_str::<CollectionConversionLimits>(
            r#"{"max_source_bytes":128,"max_items":3}"#
        )
        .expect("collection limits should deserialize"),
        limits
    );
}
