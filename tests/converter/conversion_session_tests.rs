use qubit_datatype::converter::{
    ConversionBudgetLimits, ConversionResource, DataConversionOptions,
};

#[test]
fn test_conversion_session_accumulates_budget_facts() {
    let options = DataConversionOptions::default().with_budget_limits(
        ConversionBudgetLimits::default()
            .with_max_items(1)
            .with_max_input_bytes(3)
            .with_max_output_bytes(2)
            .with_max_structured_nodes(1),
    );
    let mut session = options.session();

    session.consume_item().expect("first item should fit");
    let error = session.consume_item().expect_err("second item must fail");
    assert_eq!(error.resource(), ConversionResource::Items);
    assert_eq!(error.limit(), Some(1));
    assert_eq!(error.remaining(), Some(0));
    assert_eq!(error.requested(), Some(1));
    assert_eq!(error.used(), Some(1));
}

#[test]
fn test_conversion_session_checks_point_limits() {
    let options = DataConversionOptions::default();
    let session = options.session();
    let error = session
        .check_numeric_text_bytes(options.numeric().limits().max_text_bytes() + 1)
        .expect_err("oversized numeric text must fail");
    assert_eq!(error.resource(), ConversionResource::NumericTextBytes);
    assert_eq!(
        error.actual(),
        Some(options.numeric().limits().max_text_bytes() + 1)
    );
    assert_eq!(
        error.maximum(),
        Some(options.numeric().limits().max_text_bytes())
    );
}

#[test]
fn test_data_converter_to_in_consumes_shared_input_budget() {
    let options = DataConversionOptions::default()
        .with_budget_limits(ConversionBudgetLimits::default().with_max_input_bytes(3));
    let mut session = options.session();
    qubit_datatype::DataConverter::from("12")
        .to_in::<u16>(&mut session)
        .expect("first text should fit");
    let error = qubit_datatype::DataConverter::from("34")
        .to_in::<u16>(&mut session)
        .expect_err("second text should exceed cumulative bytes");
    assert_eq!(
        error.limit_facts().map(|facts| facts.resource()),
        Some(ConversionResource::InputBytes)
    );
}
