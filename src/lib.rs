/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Qubit Datatype
//!
//! Provides runtime data type descriptors and conversion utilities for supported
//! Rust data types.
//!
//! # Author
//!
//! Haixing Hu

/// Data type descriptors and compile-time type mappings.
pub mod datatype;

/// Runtime value conversion utilities.
pub mod converter;

pub use converter::{
    BlankStringPolicy, BooleanConversionOptions, CollectionConversionOptions, DataConversionError,
    DataConversionOptions, DataConversionResult, DataConvertTo, DataConverter, DataConverters,
    DataListConversionError, DataListConversionResult, EmptyItemPolicy, ScalarStringDataConverters,
    StringConversionOptions,
};
pub use datatype::{DataType, DataTypeOf, DataTypeParseError};
