// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Canonical string-map serialization tests.

#[cfg(feature = "json")]
use std::collections::HashMap;

#[cfg(feature = "json")]
use qubit_datatype::DataConverter;

/// Verifies string-map JSON serialization uses lexicographic key order.
#[cfg(feature = "json")]
#[test]
fn test_canonical_string_map_serializes_keys_in_lexicographic_order() {
    let value = HashMap::from([
        ("zebra".to_owned(), "last".to_owned()),
        ("ant".to_owned(), "first".to_owned()),
    ]);

    assert_eq!(
        DataConverter::from(value).to::<String>(),
        Ok(r#"{"ant":"first","zebra":"last"}"#.to_owned()),
    );
}
