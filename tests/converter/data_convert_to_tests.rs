/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0
 *
 ******************************************************************************/
use qubit_datatype::converter::{
    DataConversionOptions,
    DataConvertTo,
    DataConverter,
};

/// Test direct trait dispatch for a supported target type.
#[test]
fn test_data_convert_to_trait_converts_supported_value() {
    let converter = DataConverter::from("42");
    let converted = <DataConverter<'_> as DataConvertTo<u16>>::convert(
        &converter,
        &DataConversionOptions::default(),
    )
    .expect("string value should convert through the trait");

    assert_eq!(converted, 42);
}
