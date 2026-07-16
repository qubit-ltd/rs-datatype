// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Private numeric conversion implementation modules.

mod conversion;
#[cfg(feature = "big-number")]
mod parsed_number;

pub(in crate::converter::data_converter) use conversion::{
    duration_to_u128,
    is_integer_syntax,
    source_to_integer,
};
