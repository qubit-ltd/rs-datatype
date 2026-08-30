// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests for exact millisecond Duration text.

use std::time::Duration;

use qubit_datatype::DurationPrecisionLossError;
use qubit_datatype::serde::duration_millis_with_unit_exact;

use super::internal::DurationMillisWithUnitExactHolder;

#[test]
fn test_exact_millisecond_format_rejects_precision_loss() {
    assert_eq!(
        duration_millis_with_unit_exact::format(&Duration::from_millis(42)),
        Ok(String::from("42ms")),
    );
    assert_eq!(
        duration_millis_with_unit_exact::format(&Duration::from_micros(1)),
        Err(DurationPrecisionLossError),
    );
}

#[test]
fn test_exact_millisecond_serde_round_trip() {
    let holder = DurationMillisWithUnitExactHolder {
        duration: Duration::from_millis(42),
    };
    let text = serde_json::to_string(&holder).expect("exact duration should serialize");
    assert_eq!(text, r#"{"duration":"42ms"}"#);
    let decoded: DurationMillisWithUnitExactHolder =
        serde_json::from_str(&text).expect("exact duration should deserialize");
    assert_eq!(decoded, holder);
}

#[test]
fn test_exact_millisecond_serde_rejects_submillisecond_value() {
    let holder = DurationMillisWithUnitExactHolder {
        duration: Duration::from_micros(1),
    };
    assert!(serde_json::to_string(&holder).is_err());
}

#[test]
fn test_exact_millisecond_parse_uses_strict_millisecond_grammar() {
    assert_eq!(
        duration_millis_with_unit_exact::parse("42ms"),
        Ok(Duration::from_millis(42)),
    );
    assert!(duration_millis_with_unit_exact::parse("42s").is_err());
}
