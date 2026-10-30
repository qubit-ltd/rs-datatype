// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Reusable Data Conversion
//!
//! Defines the public source wrapper and delegates target families to focused
//! conversion modules.

mod boolean;
mod duration;
#[path = "data_converter/data_converter.rs"]
mod implementation;
mod internal;
mod numeric;
mod source;
mod string_source;
mod structured;
mod text;

pub use implementation::DataConverter;
