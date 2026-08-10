// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Target-side data conversion extension point.

use super::ConversionSession;
use super::DataConversionError;
use super::DataConversionOptions;
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
///     ConversionSession, DataConversionError, DataConversionOptions,
///     DataConversionTarget, DataConverter, DataType, DataTypeOf,
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
///
///     fn convert_from_in(
///         source: &DataConverter<'_>,
///         session: &mut ConversionSession<'_>,
///     ) -> Result<Self, DataConversionError> {
///         u16::convert_from_in(source, session).map(Self)
///     }
///
///     fn convert_owned_in(
///         source: DataConverter<'_>,
///         session: &mut ConversionSession<'_>,
///     ) -> Result<Self, DataConversionError> {
///         u16::convert_owned_in(source, session).map(Self)
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
    /// The default implementation delegates to [`Self::convert_from`]. Targets
    /// that can reuse an owned source allocation may override this method.
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

    /// Converts a borrowed source with a reusable session.
    ///
    /// The default implementation delegates to the established options-based
    /// target hook and therefore does not pass `session` to another target.
    /// Custom targets that delegate to built-in targets and need their nested
    /// conversion to share item, byte, or structural budgets must override this
    /// hook and pass the same session onward.
    #[inline(always)]
    fn convert_from_in(
        source: &DataConverter<'_>,
        session: &mut ConversionSession<'_>,
    ) -> Result<Self, DataConversionError> {
        Self::convert_from(source, session.options())
    }

    /// Converts an owned source with a reusable session.
    ///
    /// The default implementation delegates to the options-based owned hook.
    /// Custom targets that delegate to another target and need its nested
    /// conversion to share session accounting must override this hook and pass
    /// the same session onward.
    #[inline(always)]
    fn convert_owned_in(
        source: DataConverter<'_>,
        session: &mut ConversionSession<'_>,
    ) -> Result<Self, DataConversionError> {
        Self::convert_owned(source, session.options())
    }
}
