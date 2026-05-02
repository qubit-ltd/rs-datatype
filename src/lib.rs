/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! # Qubit Datatype
//!
//! Provides runtime data type descriptors and conversion utilities for supported
//! Rust data types.
//!

/// Data type descriptors and compile-time type mappings.
pub mod datatype;

/// Runtime value conversion utilities.
pub mod converter;

pub use converter::{
    BlankStringPolicy,
    BooleanConversionOptions,
    CollectionConversionOptions,
    DataConversionError,
    DataConversionOptions,
    DataConversionResult,
    DataConvertTo,
    DataConverter,
    DataConverters,
    DataListConversionError,
    DataListConversionResult,
    DurationConversionOptions,
    DurationUnit,
    EmptyItemPolicy,
    ScalarStringDataConverters,
    StringConversionOptions,
};
pub use datatype::{
    DataType,
    DataTypeOf,
    DataTypeParseError,
};
