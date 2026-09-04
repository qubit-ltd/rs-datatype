// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Crate-private adapter for built-in conversion targets.

use crate::DataTypeOf;
use crate::converter::AdmittedConversion;
use crate::converter::ConversionContext;
use crate::converter::DataConversionError;
use crate::converter::DataConversionTarget;
use crate::converter::DataConverter;
use crate::converter::internal::supports_conversion;

/// Preserves specialized conversion paths for crate-owned target types.
///
/// This adapter is crate-private so downstream extension targets can use only
/// the safe [`DataConversionTarget::convert`] boundary.
pub(crate) trait BuiltinDataConversionTarget: DataTypeOf + Sized {
    /// Converts a consumed source into this target type using a bound context.
    ///
    /// The default implementation borrows the consumed source. Targets that
    /// can reuse an owned source allocation may override this method.
    #[inline(always)]
    fn convert_owned(
        source: DataConverter<'_>,
        context: &mut ConversionContext<'_, '_>,
    ) -> Result<Self, DataConversionError> {
        Self::convert_from(&source, context)
    }

    /// Converts a borrowed framework-owned source.
    fn convert_from(
        source: &DataConverter<'_>,
        context: &mut ConversionContext<'_, '_>,
    ) -> Result<Self, DataConversionError> {
        Err(DataConversionError::unsupported(source.data_type(), context.to_type()))
    }
}

impl<T> DataConversionTarget for T
where
    T: BuiltinDataConversionTarget,
{
    #[inline(always)]
    fn convert(input: AdmittedConversion<'_, '_, '_>) -> Result<Self, DataConversionError> {
        let (source, mut context) = input.into_parts();
        if !supports_conversion(source.data_type(), context.to_type()) {
            return Err(DataConversionError::unsupported(source.data_type(), context.to_type()));
        }
        T::convert_owned(source, &mut context)
    }
}
