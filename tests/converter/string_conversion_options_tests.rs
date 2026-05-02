/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # StringConversionOptions Unit Tests
//!
//! Tests for string-source normalization options.
//!
//! # Author
//!
//! Haixing Hu

use qubit_datatype::converter::{
    BlankStringPolicy,
    DataConversionError,
    StringConversionOptions,
};

/// Test string option policy branches.
#[test]
fn test_string_conversion_options_cover_policy_branches() {
    let preserved = StringConversionOptions::default()
        .with_trim(false)
        .with_blank_string_policy(BlankStringPolicy::Preserve)
        .normalize("   ")
        .expect("blank string should be preserved");
    assert_eq!(preserved, "   ");

    let rejected = StringConversionOptions::default()
        .with_blank_string_policy(BlankStringPolicy::Reject)
        .normalize("   ");
    assert!(matches!(
        rejected,
        Err(DataConversionError::ConversionError(_)),
    ));

    let missing = StringConversionOptions::default()
        .with_trim(true)
        .with_blank_string_policy(BlankStringPolicy::TreatAsMissing)
        .normalize("   ");
    assert!(matches!(missing, Err(DataConversionError::NoValue)));
}
