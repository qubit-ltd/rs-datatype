// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # DataConversionErrorKind Unit Tests
//!
//! Tests for stable data conversion error classifications.

use qubit_datatype::DataConversionErrorKind;

/// Test that every conversion error kind is copyable and comparable.
#[test]
fn test_data_conversion_error_kind_is_copyable_and_comparable() {
    let kinds = [
        DataConversionErrorKind::Missing,
        DataConversionErrorKind::EmptyCollection,
        DataConversionErrorKind::Unsupported,
        DataConversionErrorKind::InvalidValue,
        DataConversionErrorKind::LimitExceeded,
    ];
    for kind in kinds {
        let copied = kind;
        assert_eq!(copied, kind);
    }
}
