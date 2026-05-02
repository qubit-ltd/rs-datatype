/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Datatype Language Layer
//!
//! Provides runtime data type definitions and conversion helpers.
//!
//! # Author
//!
//! Haixing Hu

pub mod converter;
pub mod datatype;

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
    EmptyItemPolicy,
    ScalarStringDataConverters,
    StringConversionOptions,
};
pub use datatype::{
    DataType,
    DataTypeOf,
    DataTypeParseError,
};
