// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Target-side data conversion extension point.

use super::{
    DataConversionError,
    DataConversionOptions,
    DataConverter,
};
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
///     DataConversionError, DataConversionOptions, DataConversionTarget,
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
///         options: &DataConversionOptions,
///     ) -> Result<Self, DataConversionError> {
///         u16::convert_from(source, options).map(Self)
///     }
/// }
///
/// let port = DataConverter::from("8080").to::<Port>().unwrap();
/// assert_eq!(port.0, 8080);
/// ```
pub trait DataConversionTarget: DataTypeOf + Sized {
    /// Converts `source` into this target type using `options`.
    ///
    /// # Parameters
    ///
    /// * `source` - Borrowed runtime value to convert.
    /// * `options` - Policies controlling parsing and lossy conversion.
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
        options: &DataConversionOptions,
    ) -> Result<Self, DataConversionError>;

    /// Converts a consumed source into this target type using `options`.
    ///
    /// The default implementation preserves compatibility for downstream
    /// targets by delegating to [`Self::convert_from`]. Targets that can reuse
    /// an owned source allocation may override this method.
    ///
    /// # Parameters
    ///
    /// * `source` - Runtime value consumed by the conversion.
    /// * `options` - Policies controlling parsing and lossy conversion.
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
        options: &DataConversionOptions,
    ) -> Result<Self, DataConversionError> {
        Self::convert_from(&source, options)
    }
}
