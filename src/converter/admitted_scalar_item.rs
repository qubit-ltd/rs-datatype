// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Session-admitted scalar collection item.

use super::conversion_context::ConversionContext;
use super::conversion_session::ConversionSession;
use super::data_conversion_target::DataConversionTarget;
use super::data_converter::DataConverter;
use super::error::DataConversionError;

/// A scalar item whose session item budget has already been admitted.
///
/// Construct this token through
/// [`crate::ConversionSession::admit_scalar_item`]. Downstream adapters can
/// consume it to continue conversion without charging the same item again.
///
/// An admission is a one-use capability and cannot be converted twice:
///
/// ```compile_fail
/// use qubit_datatype::{
///     ConversionLimits, ConversionPolicy, ConversionSession, DataConverter,
/// };
///
/// let policy = ConversionPolicy::default();
/// let limits = ConversionLimits::default();
/// let mut session = ConversionSession::new(&policy, &limits);
/// let admission = session
///     .admit_scalar_item(0, DataConverter::from("42"))
///     .unwrap();
/// let _: u32 = admission.convert().unwrap();
/// let _: u32 = admission.convert().unwrap();
/// ```
#[must_use]
#[derive(Debug)]
pub struct AdmittedScalarItem<'session, 'policy, 'source> {
    /// Session whose item budget admitted this scalar.
    session: &'session mut ConversionSession<'policy>,
    /// Zero-based position of the scalar in its original collection.
    source_index: usize,
    /// Source value whose conversion consumes this admission.
    source: DataConverter<'source>,
}

impl<'session, 'policy, 'source> AdmittedScalarItem<'session, 'policy, 'source> {
    /// Creates an admitted scalar item after the session budget succeeds.
    #[inline]
    pub(crate) const fn new(
        session: &'session mut ConversionSession<'policy>,
        source_index: usize,
        source: DataConverter<'source>,
    ) -> Self {
        Self {
            session,
            source_index,
            source,
        }
    }

    /// Returns the original source index of the admitted item.
    #[inline(always)]
    #[must_use]
    pub const fn source_index(&self) -> usize {
        self.source_index
    }

    /// Converts the admitted source without charging its item budget again.
    ///
    /// # Errors
    ///
    /// Returns the conversion-domain error produced by the requested target.
    #[inline(always)]
    pub fn convert<T>(self) -> Result<T, DataConversionError>
    where
        T: DataConversionTarget,
    {
        let from = self.source.data_type();
        let mut context = ConversionContext::new(self.session, from, T::DATA_TYPE);
        T::convert_owned(self.source, &mut context)
    }

    /// Converts the admitted source and returns its original session.
    ///
    /// This form supports nested protocols, such as Serde enum access, that
    /// must continue using the same session after the scalar value converts.
    ///
    /// # Errors
    ///
    /// Returns the conversion-domain error produced by the requested target.
    #[inline]
    pub fn convert_with_session<T>(self) -> Result<(T, &'session mut ConversionSession<'policy>), DataConversionError>
    where
        T: DataConversionTarget,
    {
        let from = self.source.data_type();
        let value = {
            let mut context = ConversionContext::new(self.session, from, T::DATA_TYPE);
            T::convert_owned(self.source, &mut context)?
        };
        Ok((value, self.session))
    }
}
