// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Source constructors for DataConverter.

use super::DataConverter;
use crate::datatype::for_each_data_type_mapping;
use std::borrow::Cow;

macro_rules! impl_from_copy {
    ($source:ty, $variant:ident) => {
        impl<'a> From<$source> for DataConverter<'a> {
            /// Wraps a copied scalar source value.
            ///
            /// # Parameters
            ///
            /// * `value` - Scalar value to copy into the converter.
            ///
            /// # Returns
            ///
            /// A converter owning the copied scalar representation.
            #[inline(always)]
            fn from(value: $source) -> Self {
                Self::$variant(value)
            }
        }
        impl<'a> From<&'a $source> for DataConverter<'a> {
            /// Wraps a borrowed scalar by copying its value.
            ///
            /// # Parameters
            ///
            /// * `value` - Scalar value to copy into the converter.
            ///
            /// # Returns
            ///
            /// A converter owning the copied scalar representation.
            #[inline(always)]
            fn from(value: &'a $source) -> Self {
                Self::$variant(*value)
            }
        }
    };
}

macro_rules! impl_from_cow {
    ($source:ty, $variant:ident) => {
        impl<'a> From<$source> for DataConverter<'a> {
            /// Wraps an owned rich source value without cloning it.
            ///
            /// # Parameters
            ///
            /// * `value` - Source value to move into the converter.
            ///
            /// # Returns
            ///
            /// A converter owning the source value.
            #[inline(always)]
            fn from(value: $source) -> Self {
                Self::$variant(Cow::Owned(value))
            }
        }
        impl<'a> From<&'a $source> for DataConverter<'a> {
            /// Wraps a borrowed rich source value.
            ///
            /// # Parameters
            ///
            /// * `value` - Source value to borrow for the converter lifetime.
            ///
            /// # Returns
            ///
            /// A converter borrowing the source without cloning it.
            #[inline(always)]
            fn from(value: &'a $source) -> Self {
                Self::$variant(Cow::Borrowed(value))
            }
        }
    };
}

macro_rules! impl_from_string {
    ($source:ty, $variant:ident) => {
        impl<'a> From<$source> for DataConverter<'a> {
            /// Wraps an owned string source without cloning it.
            ///
            /// # Parameters
            ///
            /// * `value` - String to move into the converter.
            ///
            /// # Returns
            ///
            /// A converter owning the string storage.
            #[inline(always)]
            fn from(value: $source) -> Self {
                Self::$variant(Cow::Owned(value))
            }
        }
        impl<'a> From<&'a $source> for DataConverter<'a> {
            /// Wraps a borrowed string source.
            ///
            /// # Parameters
            ///
            /// * `value` - String to borrow for the converter lifetime.
            ///
            /// # Returns
            ///
            /// A converter borrowing the string storage.
            #[inline(always)]
            fn from(value: &'a $source) -> Self {
                Self::$variant(Cow::Borrowed(value))
            }
        }
    };
}

macro_rules! impl_from_strategy {
    (copy, $source:ty, $variant:ident) => {
        impl_from_copy!($source, $variant);
    };
    (cow, $source:ty, $variant:ident) => {
        impl_from_cow!($source, $variant);
    };
    (string, $source:ty, $variant:ident) => {
        impl_from_string!($source, $variant);
    };
}

macro_rules! impl_from_data_type_mappings {
    (; $( $(#[$meta:meta])* ($variant:ident, $source:ty, $strategy:ident) ),+ $(,)?) => {
        $(
            $(#[$meta])*
            impl_from_strategy!($strategy, $source, $variant);
        )+
    };
}

for_each_data_type_mapping!(impl_from_data_type_mappings);

impl<'a> From<&'a str> for DataConverter<'a> {
    /// Wraps a borrowed string slice.
    ///
    /// # Parameters
    ///
    /// * `value` - String slice to borrow for the converter lifetime.
    ///
    /// # Returns
    ///
    /// A converter borrowing the string slice.
    #[inline(always)]
    fn from(value: &'a str) -> Self {
        Self::String(Cow::Borrowed(value))
    }
}
