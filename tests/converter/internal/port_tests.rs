// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Downstream-owned target used by conversion extension tests.

use qubit_datatype::{
    DataConversionError,
    DataConversionOptions,
    DataConversionTarget,
    DataConverter,
    DataType,
    DataTypeOf,
};

/// Port newtype proving downstream target extensibility.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::converter) struct Port(pub(in crate::converter) u16);

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
