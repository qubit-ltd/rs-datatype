// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Downstream-owned target used by conversion extension tests.

use qubit_datatype::ConversionContext;
use qubit_datatype::DataConversionError;
use qubit_datatype::DataConversionTarget;
use qubit_datatype::DataConverter;
use qubit_datatype::DataType;
use qubit_datatype::DataTypeOf;

/// Port newtype proving downstream target extensibility.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::converter) struct Port(pub(in crate::converter) u16);

impl DataTypeOf for Port {
    const DATA_TYPE: DataType = DataType::UInt16;
}

impl DataConversionTarget for Port {
    fn convert_from(
        source: &DataConverter<'_>,
        context: &mut ConversionContext<'_, '_>,
    ) -> Result<Self, DataConversionError> {
        // SAFETY: `source` is the value admitted by the outer conversion.
        unsafe { context.delegate::<u16>(source) }.map(Self)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(in crate::converter) struct Text(pub(in crate::converter) String);

impl DataTypeOf for Text {
    const DATA_TYPE: DataType = DataType::String;
}

impl DataConversionTarget for Text {
    fn convert_from(
        source: &DataConverter<'_>,
        context: &mut ConversionContext<'_, '_>,
    ) -> Result<Self, DataConversionError> {
        // SAFETY: `source` is the value admitted by the outer conversion.
        unsafe { context.delegate::<String>(source) }.map(Self)
    }

    fn convert_owned(
        source: DataConverter<'_>,
        context: &mut ConversionContext<'_, '_>,
    ) -> Result<Self, DataConversionError> {
        // SAFETY: `source` is the value admitted by the outer conversion.
        unsafe { context.delegate_owned::<String>(source) }.map(Self)
    }
}
