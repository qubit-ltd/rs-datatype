// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
// =============================================================================
//! Tests for structured conversion error reasons.

use qubit_datatype::converter::{
    DataConversionErrorKind,
    DataFormat,
};

/// Test stable, value-free error-kind formatting.
#[test]
fn test_data_conversion_error_kind_display_is_value_free() {
    let secret = "secret-marker-9271";
    let cases = [
        (
            DataConversionErrorKind::BlankRejected,
            "blank string rejected",
        ),
        (
            DataConversionErrorKind::InvalidSyntax {
                expected: "integer",
            },
            "invalid syntax; expected integer",
        ),
        (DataConversionErrorKind::OutOfRange, "value out of range"),
        (DataConversionErrorKind::PrecisionLoss, "precision loss"),
        (DataConversionErrorKind::NonFinite, "non-finite value"),
        (DataConversionErrorKind::InvalidBoolean, "invalid boolean"),
        (
            DataConversionErrorKind::NegativeDuration,
            "negative duration",
        ),
        (
            DataConversionErrorKind::UnsupportedDurationUnit,
            "unsupported duration unit",
        ),
        (
            DataConversionErrorKind::Serialization {
                format: DataFormat::Json,
            },
            "json serialization failed",
        ),
        (
            DataConversionErrorKind::Deserialization {
                format: DataFormat::Json,
            },
            "json deserialization failed",
        ),
    ];
    for (kind, expected) in cases {
        let display = kind.to_string();
        assert_eq!(display, expected);
        assert!(!display.contains(secret));
    }
}
