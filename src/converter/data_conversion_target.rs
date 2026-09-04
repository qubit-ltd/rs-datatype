// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Target-side data conversion extension point.

use super::AdmittedConversion;
use super::DataConversionError;
use crate::DataTypeOf;

/// Defines how a target type is constructed from a
/// [`DataConverter`](crate::DataConverter).
///
/// Implementing the trait on the target makes the API extensible for
/// downstream-owned newtypes without lifetime-wide bounds on the source.
///
/// # Examples
///
/// ```
/// use qubit_datatype::{AdmittedConversion, DataConversionError,
///     DataConversionTarget, DataConverter, DataType, DataTypeOf};
///
/// struct Port(u16);
///
/// impl DataTypeOf for Port {
///     const DATA_TYPE: DataType = DataType::UInt16;
/// }
///
/// impl DataConversionTarget for Port {
///     fn convert(input: AdmittedConversion<'_, '_, '_>)
///         -> Result<Self, DataConversionError> {
///         input.convert::<u16>().map(Self)
///     }
/// }
///
/// let port = DataConverter::from("8080").to::<Port>().unwrap();
/// assert_eq!(port.0, 8080);
/// ```
pub trait DataConversionTarget: DataTypeOf + Sized {
    /// Converts one framework-admitted source into this target type.
    ///
    /// # Parameters
    ///
    /// * `input` - The sole source admitted for this conversion together with
    ///   its bound policies, budgets, and execution operations.
    ///
    /// # Returns
    ///
    /// The converted target value.
    ///
    /// # Errors
    ///
    /// Returns [`DataConversionError`] when the source is missing, the source
    /// and target are unsupported, or the value violates a conversion policy.
    fn convert(input: AdmittedConversion<'_, '_, '_>) -> Result<Self, DataConversionError>;
}
