#![cfg(feature = "json")]

use qubit_datatype::DataConversionError;
use qubit_datatype::DataConversionErrorKind;
use qubit_datatype::DataConverter;
use qubit_datatype::DataFormat;
use qubit_datatype::DataType;
use qubit_datatype::InvalidValueReason;
use qubit_datatype::JsonConversionErrorCategory;

#[test]
fn test_json_conversion_diagnostic_reports_syntax_location() {
    let error = DataConverter::from("{\n@}")
        .to::<serde_json::Value>()
        .expect_err("an object member cannot start with @");
    let diagnostic = error.json_diagnostic().expect("syntax location retained");
    assert_eq!(
        diagnostic.category(),
        JsonConversionErrorCategory::InvalidJson
    );
    assert_eq!(diagnostic.line(), Some(2));
    assert_eq!(diagnostic.column(), Some(1));
    assert_eq!(error.kind(), DataConversionErrorKind::InvalidValue);
    assert_eq!(error.from_type(), Some(DataType::String));
    assert_eq!(error.to_type(), DataType::Json);
    assert_eq!(
        error.reason(),
        Some(&InvalidValueReason::Deserialization {
            format: DataFormat::Json
        })
    );
    let legacy = DataConversionError::invalid(
        DataType::String,
        DataType::Json,
        InvalidValueReason::Deserialization {
            format: DataFormat::Json,
        },
    );
    assert_eq!(
        error, legacy,
        "diagnostics must not alter legacy error equality"
    );
    assert_eq!(error.clone().json_diagnostic(), error.json_diagnostic());
}

#[test]
fn test_json_conversion_diagnostic_counts_syntax_columns_as_characters() {
    let error = DataConverter::from(r#"{"字":0,@}"#)
        .to::<serde_json::Value>()
        .expect_err("syntax must fail after the multibyte key");
    let diagnostic = error.json_diagnostic().expect("diagnostic retained");
    assert_eq!(
        diagnostic.category(),
        JsonConversionErrorCategory::InvalidJson
    );
    assert_eq!(diagnostic.line(), Some(1));
    assert_eq!(diagnostic.column(), Some(8));
}

#[test]
fn test_json_conversion_diagnostic_follows_normalized_text() {
    use qubit_datatype::ConversionPolicy;
    use qubit_datatype::StringConversionPolicy;

    let policy = ConversionPolicy::builder()
        .string_policy(StringConversionPolicy::builder().trim(true).build())
        .build();
    let error = DataConverter::from("\n  {\n@}\n")
        .to_with::<serde_json::Value>(&policy, &qubit_datatype::ConversionLimits::default())
        .expect_err("normalized JSON remains invalid");
    let diagnostic = error.json_diagnostic().expect("diagnostic retained");
    assert_eq!(diagnostic.line(), Some(2));
    assert_eq!(diagnostic.column(), Some(1));
}

#[test]
fn test_json_conversion_diagnostic_reports_deserialize_byte_columns() {
    use std::collections::HashMap;

    let error = DataConverter::from(r#"{"字":1}"#)
        .to::<HashMap<String, String>>()
        .expect_err("number cannot be materialized as String");
    let diagnostic = error.json_diagnostic().expect("diagnostic retained");
    assert_eq!(
        diagnostic.category(),
        JsonConversionErrorCategory::Deserialize
    );
    assert_eq!(diagnostic.line(), Some(1));
    assert_eq!(diagnostic.column(), Some(8));
    assert_eq!(error.to_type(), DataType::StringMap);
}

#[derive(Debug)]
struct DecodedJson {
    _value: serde_json::Value,
}

impl qubit_datatype::DataTypeOf for DecodedJson {
    const DATA_TYPE: DataType = DataType::Json;
}

impl qubit_datatype::DataConversionTarget for DecodedJson {
    fn convert(
        mut input: qubit_datatype::AdmittedConversion<'_, '_, '_>,
    ) -> Result<Self, DataConversionError> {
        let source = input.source().clone();
        match source {
            DataConverter::String(value) => input
                .decode_json(value.as_bytes())
                .map(|_value| Self { _value }),
            _ => Err(input.unsupported()),
        }
    }
}

#[test]
fn test_json_conversion_diagnostic_preserves_custom_target_input_location() {
    let error = DataConverter::from("{\n@}")
        .to::<DecodedJson>()
        .expect_err("custom target JSON decoder rejects malformed input");
    let diagnostic = error.json_diagnostic().expect("diagnostic retained");
    assert_eq!(
        diagnostic.category(),
        JsonConversionErrorCategory::InvalidJson
    );
    assert_eq!((diagnostic.line(), diagnostic.column()), (Some(2), Some(1)));
}

#[test]
fn test_json_conversion_diagnostic_preserves_list_error() {
    let error = DataConverter::from("{\n@}")
        .to::<serde_json::Value>()
        .expect_err("invalid JSON");
    let list_error = qubit_datatype::DataListConversionError::new(2, error);
    assert_eq!(list_error.source_index(), 2);
    assert_eq!(
        list_error
            .conversion_error()
            .json_diagnostic()
            .unwrap()
            .category(),
        JsonConversionErrorCategory::InvalidJson
    );
}

#[test]
fn test_json_conversion_diagnostic_retains_no_secret() {
    use std::error::Error;

    let error = DataConverter::from(r#"{"TOP_SECRET_KEY":"TOP_SECRET_VALUE",@}"#)
        .to::<serde_json::Value>()
        .expect_err("malformed JSON");
    let markers = ["TOP_SECRET_KEY", "TOP_SECRET_VALUE"];
    for marker in markers {
        assert!(!error.to_string().contains(marker));
        assert!(!format!("{error:?}").contains(marker));
    }
    let mut source = Error::source(&error);
    while let Some(current) = source {
        for marker in markers {
            assert!(!current.to_string().contains(marker));
            assert!(!format!("{current:?}").contains(marker));
        }
        source = current.source();
    }
}

#[test]
fn test_json_conversion_diagnostic_keeps_budget_error_classification() {
    use qubit_datatype::ConversionLimits;
    use qubit_datatype::StructuredConversionLimits;

    let limits = ConversionLimits::builder()
        .structured_limits(StructuredConversionLimits::builder().max_depth(0).build())
        .build();
    let error = DataConverter::from("0")
        .to_with::<serde_json::Value>(&qubit_datatype::ConversionPolicy::default(), &limits)
        .expect_err("depth zero rejects structured JSON decode");
    assert_eq!(error.kind(), DataConversionErrorKind::LimitExceeded);
    assert_eq!(
        error.resource(),
        Some(qubit_datatype::ConversionResource::StructuredDepth)
    );
    assert_eq!(error.json_diagnostic(), None);
}

#[test]
fn test_json_conversion_diagnostic_preserves_unknown_location() {
    let error = DataConverter::from("0").to::<Refuses>().unwrap_err();
    let diagnostic = error.json_diagnostic().expect("category retained");
    assert_eq!(
        diagnostic.category(),
        JsonConversionErrorCategory::Deserialize
    );
    assert_eq!(diagnostic.line(), None);
    assert_eq!(diagnostic.column(), None);
}

#[derive(Debug)]
struct Refuses;

impl<'de> serde::Deserialize<'de> for Refuses {
    fn deserialize<D>(_: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Err(serde::de::Error::custom("safe failure"))
    }
}

impl qubit_datatype::DataTypeOf for Refuses {
    const DATA_TYPE: DataType = DataType::Json;
}

impl qubit_datatype::DataConversionTarget for Refuses {
    fn convert(
        mut input: qubit_datatype::AdmittedConversion<'_, '_, '_>,
    ) -> Result<Self, DataConversionError> {
        input.decode_json(b"0")
    }
}

#[test]
fn test_json_conversion_diagnostic_does_not_change_legacy_display_or_equality() {
    let first = DataConverter::from("{\n@}")
        .to::<serde_json::Value>()
        .unwrap_err();
    let second = DataConverter::from("{")
        .to::<serde_json::Value>()
        .unwrap_err();
    assert_eq!(first, second);
    assert_ne!(first.json_diagnostic(), second.json_diagnostic());
    assert_eq!(
        first.to_string(),
        "Invalid conversion from string to json: json deserialization failed"
    );
}
