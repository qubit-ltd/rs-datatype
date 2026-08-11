// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Focused test target for conversion policy and limit types.

#![cfg(feature = "converter")]

#[path = "converter/options/mod.rs"]
mod options;

/// Verifies this focused target includes the conversion option test modules.
#[test]
fn test_conversion_option_modules_are_enabled() {
    const { assert!(cfg!(feature = "converter")) };
}
