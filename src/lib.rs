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

pub mod lang;

/// Data type descriptors and compile-time type mappings.
pub mod datatype {
    pub use crate::lang::datatype::*;
}

/// Runtime value conversion utilities.
pub mod converter {
    pub use crate::lang::converter::*;
}

pub use lang::{
    converter::{
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
    },
    datatype::{
        DataType,
        DataTypeOf,
        DataTypeParseError,
    },
};
