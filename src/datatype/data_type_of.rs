// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Compile-time Data Type Mapping
//!
//! Provides the `DataTypeOf` trait and implementations to map Rust types to
//! `DataType`.

use super::{
    DataType,
    for_each_data_type_mapping,
};

/// Maps a concrete Rust type to its runtime [`DataType`] descriptor.
///
/// This trait carries type metadata only; it does not convert values. Generic
/// APIs use it when they need a stable target descriptor without receiving a
/// value of that type. Implementations for third-party types are enabled by
/// their corresponding crate features. The standard-library mapping for
/// `HashMap<String, String>` is always available because it does not depend on
/// JSON support.
///
/// # Examples
///
/// ```
/// use std::collections::HashMap;
///
/// use qubit_datatype::{DataType, DataTypeOf};
///
/// fn declared_type<T: DataTypeOf>() -> DataType {
///     T::DATA_TYPE
/// }
///
/// assert_eq!(declared_type::<u64>(), DataType::UInt64);
/// assert_eq!(
///     declared_type::<HashMap<String, String>>(),
///     DataType::StringMap,
/// );
/// ```
///
/// Platform-sized integers deliberately have no runtime descriptor:
///
/// ```compile_fail
/// use qubit_datatype::DataTypeOf;
///
/// let _ = usize::DATA_TYPE;
/// ```
pub trait DataTypeOf {
    /// The stable [`DataType`] corresponding to `Self`.
    const DATA_TYPE: DataType;
}

macro_rules! impl_data_type_of {
    (; $( $(#[$meta:meta])* ($variant:ident, $source:ty, $strategy:ident) ),+ $(,)?) => {
        $(
            $(#[$meta])*
            impl DataTypeOf for $source {
                const DATA_TYPE: DataType = DataType::$variant;
            }
        )+
    };
}

for_each_data_type_mapping!(impl_data_type_of);
