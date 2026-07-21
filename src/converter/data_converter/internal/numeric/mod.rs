// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Private numeric conversion implementation modules.

#[cfg(any(feature = "big-integer", feature = "big-decimal"))]
mod big_number;
mod float;
#[cfg(any(feature = "big-integer", feature = "big-decimal"))]
mod float_big_number;
mod float_text;
mod integer;
#[cfg(feature = "big-decimal")]
mod parsed_number;
mod syntax;

pub(in crate::converter::data_converter) use integer::{
    duration_to_u128,
    source_to_integer,
};
pub(in crate::converter::data_converter) use syntax::{
    check_numeric_text_limit,
    is_integer_syntax,
};
