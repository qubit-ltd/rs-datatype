// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_datatype::{
    DataConversionError, DataConversionOptions, DataConversionTarget, DataConverter, DataType,
    DataTypeOf,
};

/// A downstream-owned conversion target used to prove the target-side API is
/// extensible without implementing a foreign trait for `DataConverter`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Port(u16);

impl DataTypeOf for Port {
    const DATA_TYPE: DataType = DataType::UInt16;
}

impl DataConversionTarget for Port {
    fn convert_from(
        source: &DataConverter<'_>,
        options: &DataConversionOptions,
    ) -> Result<Self, DataConversionError> {
        u16::convert_from(source, options).map(Self)
    }
}

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
