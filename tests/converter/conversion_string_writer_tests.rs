// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests for domain-level bounded String construction.

use qubit_datatype::AdmittedConversion;
use qubit_datatype::ConversionLimits;
use qubit_datatype::ConversionOperationLimits;
use qubit_datatype::ConversionPolicy;
use qubit_datatype::ConversionResource;
use qubit_datatype::ConversionSession;
use qubit_datatype::DataConversionError;
use qubit_datatype::DataConversionErrorKind;
use qubit_datatype::DataConversionTarget;
use qubit_datatype::DataConverter;
use qubit_datatype::DataType;
use qubit_datatype::DataTypeOf;
use qubit_datatype::InvalidValueReason;

#[derive(Debug)]
struct Rendered<const MODE: u8>(String);

impl<const MODE: u8> DataTypeOf for Rendered<MODE> {
    const DATA_TYPE: DataType = DataType::String;
}

impl<const MODE: u8> DataConversionTarget for Rendered<MODE> {
    fn convert(mut input: AdmittedConversion<'_, '_, '_>) -> Result<Self, DataConversionError> {
        let from = input.from_type();
        let to = input.to_type();
        let output = input.write_string(|writer| match MODE {
            0 => {
                writer.write_str("value=")?;
                writer.write_display(&42)
            }
            1 => {
                writer.write_str("discarded")?;
                Err(DataConversionError::invalid(from, to, InvalidValueReason::OutOfRange))
            }
            2 => writer.write_str("too large"),
            3 => writer.write_char('💡'),
            4 => writer.write_char('x'),
            5 => {
                writer.write_char('x')?;
                writer.write_char('💡')
            }
            _ => writer.write_str("too large"),
        })?;
        Ok(Self(output))
    }
}

/// Verifies successful output is committed once through the public facade.
#[test]
fn test_write_string_commits_successful_output() {
    let policy = ConversionPolicy::default();
    let limits = ConversionLimits::default();
    let mut session = ConversionSession::new(&policy, &limits);

    let Rendered(output) = DataConverter::from(42_i32)
        .to_in::<Rendered<0>>(&mut session)
        .expect("bounded output should render");

    assert_eq!(output, "value=42");
    assert_eq!(session.output_bytes_used(), 8);
}

/// Verifies a rendering failure rolls back buffered output bytes.
#[test]
fn test_write_string_rolls_back_render_failure() {
    let policy = ConversionPolicy::default();
    let limits = ConversionLimits::default();
    let mut session = ConversionSession::new(&policy, &limits);

    let error = DataConverter::from("input")
        .to_in::<Rendered<1>>(&mut session)
        .expect_err("render failure should discard output");

    assert_eq!(error.kind(), DataConversionErrorKind::InvalidValue);
    assert_eq!(session.output_bytes_used(), 0);
}

/// Verifies the public writer maps output exhaustion to a domain error and
/// leaves the cumulative counter unchanged.
#[test]
fn test_write_string_maps_output_budget_failure() {
    let policy = ConversionPolicy::default();
    let limits = ConversionLimits::builder()
        .operation_limits(ConversionOperationLimits::builder().max_output_bytes(1).build())
        .build();
    let mut session = ConversionSession::new(&policy, &limits);

    let error = DataConverter::from("input")
        .to_in::<Rendered<2>>(&mut session)
        .expect_err("output above the cumulative limit should fail");

    assert_eq!(error.kind(), DataConversionErrorKind::LimitExceeded);
    assert_eq!(error.resource(), Some(ConversionResource::OutputBytes));
    assert_eq!(session.output_bytes_used(), 0);
}

#[test]
fn test_write_char_counts_utf8_bytes_and_reuses_failed_session() {
    let policy = ConversionPolicy::default();
    for (limit, succeeds) in [(3_u64, false), (4_u64, true)] {
        let limits = ConversionLimits::builder()
            .operation_limits(ConversionOperationLimits::builder().max_output_bytes(limit).build())
            .build();
        let mut session = ConversionSession::new(&policy, &limits);
        let result = DataConverter::from(0_u8).to_in::<Rendered<3>>(&mut session);
        if succeeds {
            assert_eq!(result.expect("four bytes fit exactly").0, "💡");
            assert_eq!(session.output_bytes_used(), 4);
            assert_eq!(session.output_bytes_remaining(), 0);
        } else {
            let error = result.expect_err("four bytes cannot fit three remaining bytes");
            assert_eq!(error.kind(), DataConversionErrorKind::LimitExceeded);
            assert_eq!(error.resource(), Some(ConversionResource::OutputBytes));
            assert_eq!(session.output_bytes_used(), 0);
            assert_eq!(session.output_bytes_remaining(), 3);
            let output = DataConverter::from(0_u8)
                .to_in::<Rendered<4>>(&mut session)
                .expect("a failed output must leave capacity reusable");
            assert_eq!(output.0, "x");
            assert_eq!(session.output_bytes_used(), 1);
            assert_eq!(session.output_bytes_remaining(), 2);
        }
    }
}

#[test]
fn test_write_char_counts_cumulative_output_bytes() {
    let policy = ConversionPolicy::default();
    let limits = ConversionLimits::builder()
        .operation_limits(ConversionOperationLimits::builder().max_output_bytes(4).build())
        .build();
    let mut session = ConversionSession::new(&policy, &limits);

    let output = DataConverter::from(0_u8)
        .to_in::<Rendered<4>>(&mut session)
        .expect("one byte fits");
    assert_eq!(output.0, "x");
    let error = DataConverter::from(0_u8)
        .to_in::<Rendered<3>>(&mut session)
        .expect_err("four cumulative bytes cannot fit after the first byte");

    assert_eq!(error.kind(), DataConversionErrorKind::LimitExceeded);
    assert_eq!(error.resource(), Some(ConversionResource::OutputBytes));
    assert_eq!(session.output_bytes_used(), 1);
    assert_eq!(session.output_bytes_remaining(), 3);
}

#[test]
fn test_write_char_rolls_back_partial_utf8_output() {
    let policy = ConversionPolicy::default();
    let limits = ConversionLimits::builder()
        .operation_limits(ConversionOperationLimits::builder().max_output_bytes(4).build())
        .build();
    let mut session = ConversionSession::new(&policy, &limits);

    let error = DataConverter::from(0_u8)
        .to_in::<Rendered<5>>(&mut session)
        .expect_err("the four byte character exceeds the three bytes remaining");

    assert_eq!(error.kind(), DataConversionErrorKind::LimitExceeded);
    assert_eq!(error.resource(), Some(ConversionResource::OutputBytes));
    assert_eq!(session.output_bytes_used(), 0);
    assert_eq!(session.output_bytes_remaining(), 4);
    let output = DataConverter::from(0_u8)
        .to_in::<Rendered<4>>(&mut session)
        .expect("failed partial writes must roll back and preserve capacity");
    assert_eq!(output.0, "x");
    assert_eq!(session.output_bytes_used(), 1);
    assert_eq!(session.output_bytes_remaining(), 3);
}
