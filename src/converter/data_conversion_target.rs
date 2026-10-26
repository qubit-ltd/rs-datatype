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
/// A target that delegates through [`AdmittedConversion::convert`] must use
/// the same [`DataType`](crate::DataType) as that active conversion; delegating
/// to a different data type is rejected before the delegated target runs.
///
/// # Examples
///
/// ```
/// use qubit_datatype::AdmittedConversion;
/// use qubit_datatype::DataConversionError;
/// use qubit_datatype::DataConversionTarget;
/// use qubit_datatype::DataConverter;
/// use qubit_datatype::DataType;
/// use qubit_datatype::DataTypeOf;
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
    /// Delegation to a different data type through
    /// [`AdmittedConversion::convert`] returns `Unsupported` before the
    /// delegated target runs, with the error bound to the active outer target.
    fn convert(input: AdmittedConversion<'_, '_, '_>) -> Result<Self, DataConversionError>;
}
