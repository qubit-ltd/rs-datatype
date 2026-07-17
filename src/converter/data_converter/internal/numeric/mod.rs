// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Private numeric conversion implementation modules.

#[cfg(feature = "big-number")]
mod big_number;
mod float;
mod integer;
#[cfg(feature = "big-number")]
mod parsed_number;
mod syntax;

pub(in crate::converter::data_converter) use integer::{
    duration_to_u128,
    source_to_integer,
};
pub(in crate::converter::data_converter) use syntax::is_integer_syntax;
