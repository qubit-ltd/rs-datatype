// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests for structured conversion error reasons.

use qubit_datatype::converter::{DataFormat, InvalidValueReason};

/// Test stable, value-free invalid-value reason formatting.
#[test]
fn test_invalid_value_reason_display_is_value_free() {
    let secret = "secret-marker-9271";
    let cases = [
        (InvalidValueReason::BlankRejected, "blank string rejected"),
        (
            InvalidValueReason::InvalidSyntax {
                expected: "integer",
            },
            "invalid syntax; expected integer",
        ),
        (InvalidValueReason::OutOfRange, "value out of range"),
        (InvalidValueReason::PrecisionLoss, "precision loss"),
        (InvalidValueReason::NonFinite, "non-finite value"),
        (InvalidValueReason::InvalidBoolean, "invalid boolean"),
        (InvalidValueReason::NegativeDuration, "negative duration"),
        (
            InvalidValueReason::UnsupportedDurationUnit,
            "unsupported duration unit",
        ),
        (
            InvalidValueReason::Serialization {
                format: DataFormat::Json,
            },
            "json serialization failed",
        ),
        (
            InvalidValueReason::Deserialization {
                format: DataFormat::Json,
            },
            "json deserialization failed",
        ),
    ];
    for (reason, expected) in cases {
        let display = reason.to_string();
        assert_eq!(display, expected);
        assert!(!display.contains(secret));
    }
}
