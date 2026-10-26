// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Demonstrates converting deployment port lists with explicit splitting,
//! diagnostics, and a shared operation budget.

use std::error::Error;

use qubit_datatype::CollectionConversionPolicy;
use qubit_datatype::ConversionLimits;
use qubit_datatype::ConversionOperationLimits;
use qubit_datatype::ConversionPolicy;
use qubit_datatype::ConversionResource;
use qubit_datatype::ConversionSession;
use qubit_datatype::DataConversionErrorKind;
use qubit_datatype::DataConverters;
use qubit_datatype::EmptyItemPolicy;
use qubit_datatype::ScalarStringDataConverters;

fn main() -> Result<(), Box<dyn Error>> {
    let collection = CollectionConversionPolicy::builder()
        .split_scalar_strings(true)
        .delimiters([',', ';'])
        .trim_items(true)
        .empty_item_policy(EmptyItemPolicy::Skip)
        .build();
    let policy = ConversionPolicy::builder().collection_policy(collection).build();
    let limits = ConversionLimits::default();
    let ports = ScalarStringDataConverters::from("8080,, 8443;9000").to_vec_with::<u16>(&policy, &limits)?;
    assert_eq!(ports, vec![8080, 8443, 9000]);

    let error = ScalarStringDataConverters::from("8080,,bad")
        .to_vec_with::<u16>(&policy, &limits)
        .unwrap_err();
    assert_eq!(error.source_index(), 2);
    assert_eq!(error.conversion_error().kind(), DataConversionErrorKind::InvalidValue);
    assert_eq!(
        ScalarStringDataConverters::from("8080,,bad").to_first_with::<u16>(&policy, &limits)?,
        8080
    );

    let values = vec!["8080", "8443"];
    assert_eq!(
        DataConverters::from(values).to_vec_with::<u16>(&policy, &limits)?,
        vec![8080, 8443]
    );

    let bounded = ConversionLimits::builder()
        .operation_limits(ConversionOperationLimits::builder().max_items(2).build())
        .build();
    let mut session = ConversionSession::new(&policy, &bounded);
    let error = ScalarStringDataConverters::from("8080,8443,9000")
        .to_vec_in::<u16>(&mut session)
        .unwrap_err();
    assert_eq!(error.source_index(), 2);
    assert_eq!(error.conversion_error().resource(), Some(ConversionResource::Items));
    Ok(())
}
