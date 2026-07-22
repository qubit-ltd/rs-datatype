// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_datatype::DataConverter;

use super::internal::Port;

/// Verifies that a downstream newtype can own its target conversion.
#[test]
fn test_data_conversion_target_supports_downstream_newtype() {
    let port = DataConverter::from("8080")
        .to::<Port>()
        .expect("string should convert through local target implementation");

    assert_eq!(port, Port(8080));
}

/// Verifies the consuming API falls back to a downstream target's borrowed
/// conversion implementation.
#[test]
fn test_data_conversion_target_consuming_api_supports_downstream_newtype() {
    let port = DataConverter::from(String::from("8080"))
        .into_target::<Port>()
        .expect("owned string should use the downstream conversion fallback");

    assert_eq!(port, Port(8080));
}
