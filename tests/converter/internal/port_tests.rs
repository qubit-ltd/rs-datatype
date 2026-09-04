// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Downstream-owned target used by conversion extension tests.

use qubit_datatype::AdmittedConversion;
use qubit_datatype::DataConversionError;
use qubit_datatype::DataConversionTarget;
use qubit_datatype::DataType;
use qubit_datatype::DataTypeOf;
use qubit_datatype::InvalidValueReason;

/// Port newtype proving downstream target extensibility.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::converter) struct Port(pub(in crate::converter) u16);

impl DataTypeOf for Port {
    const DATA_TYPE: DataType = DataType::UInt16;
}

impl DataConversionTarget for Port {
    fn convert(input: AdmittedConversion<'_, '_, '_>) -> Result<Self, DataConversionError> {
        let from = input.from_type();
        let to = input.to_type();
        let value = input.convert::<u16>()?;
        if value == 0 {
            return Err(DataConversionError::invalid(from, to, InvalidValueReason::OutOfRange));
        }
        Ok(Self(value))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(in crate::converter) struct Text(pub(in crate::converter) String);

impl DataTypeOf for Text {
    const DATA_TYPE: DataType = DataType::String;
}

impl DataConversionTarget for Text {
    fn convert(input: AdmittedConversion<'_, '_, '_>) -> Result<Self, DataConversionError> {
        input.convert::<String>().map(Self)
    }
}
