/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Data Type Tools
//!
//! Provides data type definitions, parse errors, and compile-time type mapping.
//!
//! # Author
//!
//! Haixing Hu

pub mod data_type;
pub mod data_type_of;
pub mod data_type_parse_error;

pub use data_type::DataType;
pub use data_type_of::DataTypeOf;
pub use data_type_parse_error::DataTypeParseError;
