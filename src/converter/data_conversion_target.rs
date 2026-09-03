// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Target-side data conversion extension point.

use super::ConversionContext;
use super::DataConversionError;
use super::DataConverter;
use crate::DataTypeOf;

/// Defines how a target type is constructed from a [`DataConverter`].
///
/// Implementing the trait on the target makes the API extensible for
/// downstream-owned newtypes without lifetime-wide bounds on the source.
///
/// # Examples
///
/// ```
/// use qubit_datatype::{
///     ConversionContext, DataConversionError, DataConversionTarget,
///     DataConverter, DataType, DataTypeOf,
/// };
///
/// struct Port(u16);
///
/// impl DataTypeOf for Port {
///     const DATA_TYPE: DataType = DataType::UInt16;
/// }
///
/// impl DataConversionTarget for Port {
///     fn convert_from(
///         source: &DataConverter<'_>,
///         context: &mut ConversionContext<'_, '_>,
///     ) -> Result<Self, DataConversionError> {
///         // SAFETY: `source` is the value admitted by the outer conversion.
///         unsafe { context.delegate::<u16>(source) }.map(Self)
///     }
/// }
///
/// let port = DataConverter::from("8080").to::<Port>().unwrap();
/// assert_eq!(port.0, 8080);
/// ```
pub trait DataConversionTarget: DataTypeOf + Sized {
    /// Converts `source` into this target type using a bound context.
    ///
    /// # Parameters
    ///
    /// * `source` - Borrowed runtime value to convert.
    /// * `context` - Bound policies, budgets, and execution operations.
    ///
    /// # Returns
    ///
    /// The converted target value.
    ///
    /// # Errors
    ///
    /// Returns [`DataConversionError`] when the source is missing, the source
    /// and target are unsupported, or the value violates a conversion policy.
    fn convert_from(
        source: &DataConverter<'_>,
        context: &mut ConversionContext<'_, '_>,
    ) -> Result<Self, DataConversionError>;

    /// Converts a consumed source into this target type using a bound context.
    ///
    /// The default implementation borrows the consumed source. Targets that
    /// can reuse an owned source allocation may override this method.
    ///
    /// # Parameters
    ///
    /// * `source` - Runtime value consumed by the conversion.
    /// * `context` - Bound policies, budgets, and execution operations.
    ///
    /// # Returns
    ///
    /// The converted target value.
    ///
    /// # Errors
    ///
    /// Returns [`DataConversionError`] under the same conditions as
    /// [`Self::convert_from`].
    #[inline(always)]
    fn convert_owned(
        source: DataConverter<'_>,
        context: &mut ConversionContext<'_, '_>,
    ) -> Result<Self, DataConversionError> {
        Self::convert_from(&source, context)
    }
}
