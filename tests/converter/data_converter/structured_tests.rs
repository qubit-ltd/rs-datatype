// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Structured conversion tests.

use std::collections::HashMap;
#[cfg(all(
    feature = "big-number",
    feature = "chrono",
    feature = "url",
    feature = "json"
))]
use std::time::Duration;

#[cfg(all(
    feature = "big-number",
    feature = "chrono",
    feature = "url",
    feature = "json"
))]
use bigdecimal::BigDecimal;
#[cfg(all(
    feature = "big-number",
    feature = "chrono",
    feature = "url",
    feature = "json"
))]
use chrono::DateTime;
#[cfg(all(
    feature = "big-number",
    feature = "chrono",
    feature = "url",
    feature = "json"
))]
use chrono::NaiveDate;
#[cfg(all(
    feature = "big-number",
    feature = "chrono",
    feature = "url",
    feature = "json"
))]
use chrono::NaiveDateTime;
#[cfg(all(
    feature = "big-number",
    feature = "chrono",
    feature = "url",
    feature = "json"
))]
use chrono::NaiveTime;
#[cfg(all(
    feature = "big-number",
    feature = "chrono",
    feature = "url",
    feature = "json"
))]
use chrono::Utc;
#[cfg(all(
    feature = "big-number",
    feature = "chrono",
    feature = "url",
    feature = "json"
))]
use num_bigint::BigInt;
#[cfg(any(feature = "json", feature = "url"))]
use qubit_budget::BudgetError;
#[cfg(any(feature = "json", feature = "url"))]
use qubit_budget::Observation;
#[cfg(any(feature = "json", feature = "url"))]
use qubit_datatype::ConversionLimits;
#[cfg(feature = "json")]
use qubit_datatype::ConversionOperationLimits;
#[cfg(any(feature = "json", feature = "url"))]
use qubit_datatype::ConversionPolicy;
#[cfg(any(feature = "json", feature = "url"))]
use qubit_datatype::ConversionResource;
#[cfg(feature = "json")]
use qubit_datatype::ConversionSession;
#[cfg(any(feature = "json", feature = "url"))]
use qubit_datatype::DataConversionError;
use qubit_datatype::DataConverter;
#[cfg(all(
    feature = "big-number",
    feature = "chrono",
    feature = "url",
    feature = "json"
))]
use qubit_datatype::DataFormat;
#[cfg(any(feature = "json", feature = "url"))]
use qubit_datatype::DataType;
#[cfg(all(
    feature = "big-number",
    feature = "chrono",
    feature = "url",
    feature = "json"
))]
use qubit_datatype::InvalidValueReason;
#[cfg(any(feature = "json", feature = "url"))]
use qubit_datatype::StructuredConversionLimits;
#[cfg(all(
    feature = "big-number",
    feature = "chrono",
    feature = "url",
    feature = "json"
))]
use qubit_datatype::converter::DataConversionErrorKind;
#[cfg(feature = "url")]
use url::Url;

/// Assert an invalid-syntax error with exact source and target types.
#[cfg(all(
    feature = "big-number",
    feature = "chrono",
    feature = "url",
    feature = "json"
))]
fn assert_invalid_syntax<T>(
    result: Result<T, DataConversionError>,
    to: DataType,
    expected: &'static str,
) {
    let matches_expected = matches!(
        &result,
        Err(error) if error.from_type() == Some(DataType::String)
            && error.to_type() == to
            && matches!(
                error.reason(),
                Some(InvalidValueReason::InvalidSyntax {
                expected: actual_expected }) if *actual_expected == expected
    )
    );
    assert!(matches_expected, "unexpected result: {:?}", result.err());
}

/// Assert an invalid error with exact source type, target type, and reason.
#[cfg(all(
    feature = "big-number",
    feature = "chrono",
    feature = "url",
    feature = "json"
))]
fn assert_invalid_reason<T>(
    result: Result<T, DataConversionError>,
    to: DataType,
    expected_reason: InvalidValueReason,
) {
    assert!(matches!(
        result,
        Err(error) if error.from_type() == Some(DataType::String)
            && error.to_type() == to && error.reason() == Some(&expected_reason),
    ));
}

/// Asserts that a conversion fails because the expected budget resource was
/// exceeded.
#[cfg(feature = "json")]
fn assert_budget_resource<T>(result: Result<T, DataConversionError>, expected: ConversionResource) {
    let error = match result {
        Ok(_) => panic!("conversion should exceed the configured budget"),
        Err(error) => error,
    };
    assert_eq!(
        error.budget_error().map(|error| *error.resource()),
        Some(expected),
    );
}

/// Tests that structured text conversions enforce the configured byte limit.
#[cfg(feature = "json")]
#[test]
fn test_data_converter_rejects_oversize_structured_text() {
    let policy = ConversionPolicy::default();
    let limits = ConversionLimits::default()
        .with_structured_limits(StructuredConversionLimits::default().with_max_text_bytes(2));

    assert!(
        DataConverter::from("[]")
            .to_with::<serde_json::Value>(&policy, &limits)
            .is_ok()
    );
    assert_eq!(
        DataConverter::from("[0]").to_with::<serde_json::Value>(&policy, &limits),
        Err(DataConversionError::limit_exceeded(
            DataType::String,
            DataType::Json,
            BudgetError::LimitExceeded {
                resource: ConversionResource::StructuredTextBytes,
                observed: Observation::Exact(3),
                maximum: 2,
            },
        )),
    );

    assert!(
        DataConverter::from("{}")
            .to_with::<HashMap<String, String>>(&policy, &limits)
            .is_ok()
    );
    assert_eq!(
        DataConverter::from(r#"{"a":"b"}"#).to_with::<HashMap<String, String>>(&policy, &limits),
        Err(DataConversionError::limit_exceeded(
            DataType::String,
            DataType::StringMap,
            BudgetError::LimitExceeded {
                resource: ConversionResource::StructuredTextBytes,
                observed: Observation::Exact(9),
                maximum: 2,
            },
        )),
    );
}

/// Tests that JSON text decoding enforces every configured structural point
/// limit before constructing a target value.
#[cfg(feature = "json")]
#[test]
fn test_data_converter_rejects_json_text_exceeding_structure_limits() {
    let policy = ConversionPolicy::default();
    let depth_limits = ConversionLimits::default()
        .with_structured_limits(StructuredConversionLimits::default().with_max_depth(2));
    assert_budget_resource(
        DataConverter::from("[[0]]").to_with::<serde_json::Value>(&policy, &depth_limits),
        ConversionResource::StructuredDepth,
    );

    let sequence_limits = ConversionLimits::default()
        .with_structured_limits(StructuredConversionLimits::default().with_max_sequence_items(1));
    assert_budget_resource(
        DataConverter::from("[0,1]").to_with::<serde_json::Value>(&policy, &sequence_limits),
        ConversionResource::SequenceItems,
    );

    let map_limits = ConversionLimits::default()
        .with_structured_limits(StructuredConversionLimits::default().with_max_map_entries(1));
    assert_budget_resource(
        DataConverter::from(r#"{"first":"1","second":"2"}"#)
            .to_with::<HashMap<String, String>>(&policy, &map_limits),
        ConversionResource::MapEntries,
    );
}

/// Tests that JSON text decoding consumes the shared structural node budget.
#[cfg(feature = "json")]
#[test]
fn test_data_converter_json_text_accumulates_structured_nodes_in_session() {
    let policy = ConversionPolicy::default();
    let limits = ConversionLimits::default()
        .with_operation_limits(ConversionOperationLimits::default().with_max_structured_nodes(2));
    let mut session = ConversionSession::new(&policy, &limits);

    DataConverter::from("[0]")
        .to_in::<serde_json::Value>(&mut session)
        .expect("the first two JSON nodes should fit");
    assert_budget_resource(
        DataConverter::from("0").to_in::<serde_json::Value>(&mut session),
        ConversionResource::StructuredNodes,
    );
}

/// Tests that structured-source budget enforcement does not depend on source
/// ownership or whether the target type changes.
#[cfg(feature = "json")]
#[test]
fn test_data_converter_structured_budget_is_content_invariant() {
    let policy = ConversionPolicy::default();
    let json_limits = ConversionLimits::default()
        .with_operation_limits(ConversionOperationLimits::default().with_max_structured_nodes(1));
    let json = serde_json::json!(["value"]);
    assert_budget_resource(
        DataConverter::from(&json).to_with::<serde_json::Value>(&policy, &json_limits),
        ConversionResource::StructuredNodes,
    );
    assert_budget_resource(
        DataConverter::from(json).into_target_with::<serde_json::Value>(&policy, &json_limits),
        ConversionResource::StructuredNodes,
    );

    let map_limits = ConversionLimits::default()
        .with_operation_limits(ConversionOperationLimits::default().with_max_structured_nodes(1));
    let map = HashMap::from([("key".to_owned(), "value".to_owned())]);
    assert_budget_resource(
        DataConverter::from(&map).to_with::<HashMap<String, String>>(&policy, &map_limits),
        ConversionResource::StructuredNodes,
    );
    assert_budget_resource(
        DataConverter::from(map).into_target_with::<HashMap<String, String>>(&policy, &map_limits),
        ConversionResource::StructuredNodes,
    );

    let string_limits = ConversionLimits::default()
        .with_operation_limits(ConversionOperationLimits::default().with_max_structured_nodes(1));
    let json = serde_json::json!(["value"]);
    assert_budget_resource(
        DataConverter::from(json).into_target_with::<String>(&policy, &string_limits),
        ConversionResource::StructuredNodes,
    );
}

/// Tests that URL parsing enforces the configured structured text byte limit.
#[cfg(feature = "url")]
#[test]
fn test_data_converter_rejects_oversize_url_text() {
    let policy = ConversionPolicy::default();
    let limits = ConversionLimits::default()
        .with_structured_limits(StructuredConversionLimits::default().with_max_text_bytes(14));

    assert_eq!(
        DataConverter::from("https://a.test").to_with::<Url>(&policy, &limits),
        Ok(Url::parse("https://a.test").expect("test URL should parse")),
    );
    assert_eq!(
        DataConverter::from("https://a.test/x").to_with::<Url>(&policy, &limits),
        Err(DataConversionError::limit_exceeded(
            DataType::String,
            DataType::Url,
            BudgetError::LimitExceeded {
                resource: ConversionResource::StructuredTextBytes,
                observed: Observation::Exact(16),
                maximum: 14,
            },
        )),
    );
}

/// Test rejection boundaries for every canonical rich textual format.
#[test]
#[cfg(all(
    feature = "big-number",
    feature = "chrono",
    feature = "url",
    feature = "json"
))]
fn test_data_converter_rich_targets_reject_noncanonical_text() {
    for value in ["", "ab"] {
        assert_invalid_syntax(
            DataConverter::from(value).to::<char>(),
            DataType::Char,
            "one Unicode scalar value",
        );
    }
    assert_invalid_syntax(
        DataConverter::from("2026/07/12").to::<NaiveDate>(),
        DataType::Date,
        "YYYY-MM-DD",
    );
    assert_invalid_syntax(
        DataConverter::from("10:11:12.1234567890").to::<NaiveTime>(),
        DataType::Time,
        "HH:MM:SS[.fraction]",
    );
    assert_invalid_syntax(
        DataConverter::from("2026-07-12 10:11:12").to::<NaiveDateTime>(),
        DataType::DateTime,
        "YYYY-MM-DDTHH:MM:SS[.fraction]",
    );
    assert_invalid_syntax(
        DataConverter::from("2026-07-12T10:11:12").to::<DateTime<Utc>>(),
        DataType::Instant,
        "RFC 3339 timestamp with offset",
    );
    assert_invalid_reason(
        DataConverter::from("1.5").to::<BigInt>(),
        DataType::BigInteger,
        InvalidValueReason::PrecisionLoss,
    );
    assert_invalid_syntax(
        DataConverter::from("1.2.3").to::<BigDecimal>(),
        DataType::BigDecimal,
        "decimal number with optional exponent",
    );
    assert_invalid_syntax(
        DataConverter::from("relative/path").to::<Url>(),
        DataType::Url,
        "absolute URL",
    );
    assert_invalid_reason(
        DataConverter::from("{").to::<serde_json::Value>(),
        DataType::Json,
        InvalidValueReason::Deserialization {
            format: DataFormat::Json,
        },
    );
    for value in [r#"{"key":1}"#, r#"{"key":"one","key":"two"}"#] {
        assert_invalid_reason(
            DataConverter::from(value).to::<HashMap<String, String>>(),
            DataType::StringMap,
            InvalidValueReason::Deserialization {
                format: DataFormat::Json,
            },
        );
    }
    assert_invalid_reason(
        DataConverter::from("[]").to::<HashMap<String, String>>(),
        DataType::StringMap,
        InvalidValueReason::Deserialization {
            format: DataFormat::Json,
        },
    );
    assert_invalid_reason(
        DataConverter::from(r#"{"key":"value"} []"#).to::<HashMap<String, String>>(),
        DataType::StringMap,
        InvalidValueReason::Deserialization {
            format: DataFormat::Json,
        },
    );
    for value in [" 1s", "+1s", "1.5s"] {
        assert_invalid_syntax(
            DataConverter::from(value).to::<Duration>(),
            DataType::Duration,
            "[0-9]+(ns|us|µs|μs|ms|s|min|h|d)?",
        );
    }
    assert_invalid_reason(
        DataConverter::from("1fortnight").to::<Duration>(),
        DataType::Duration,
        InvalidValueReason::UnsupportedDurationUnit,
    );
}

/// Test that string-map identity conversion only needs `converter`.
#[test]
fn test_data_converter_string_map_identity_without_json() {
    let source = HashMap::from([("key".to_owned(), "value".to_owned())]);
    let converted: HashMap<String, String> = DataConverter::from(&source)
        .to()
        .expect("a string map should convert to itself");

    assert_eq!(converted, source);
}

/// Verifies consuming string-map conversion reuses owned value storage while
/// a borrowed source remains independently owned.
#[test]
fn test_data_converter_consuming_string_map_identity_reuses_owned_storage() {
    let source = HashMap::from([("key".to_owned(), "owned payload".to_owned())]);
    let source_pointer = source
        .get("key")
        .expect("test map should contain key")
        .as_ptr();
    let converted = DataConverter::from(source)
        .into_target::<HashMap<String, String>>()
        .expect("owned string map identity conversion should succeed");
    assert_eq!(
        converted
            .get("key")
            .expect("converted map should contain key")
            .as_ptr(),
        source_pointer,
    );

    let borrowed = HashMap::from([("key".to_owned(), "borrowed payload".to_owned())]);
    let borrowed_pointer = borrowed
        .get("key")
        .expect("test map should contain key")
        .as_ptr();
    let converted = DataConverter::from(&borrowed)
        .into_target::<HashMap<String, String>>()
        .expect("borrowed string map identity conversion should succeed");
    assert_ne!(
        converted
            .get("key")
            .expect("converted map should contain key")
            .as_ptr(),
        borrowed_pointer,
    );
}

/// Verifies consuming JSON conversion reuses owned string storage.
#[test]
#[cfg(feature = "json")]
fn test_data_converter_consuming_json_identity_reuses_owned_storage() {
    let source = serde_json::Value::String("owned payload".to_owned());
    let source_pointer = source
        .as_str()
        .expect("test JSON should contain a string")
        .as_ptr();
    let converted = DataConverter::from(source)
        .into_target::<serde_json::Value>()
        .expect("owned JSON identity conversion should succeed");

    assert_eq!(
        converted
            .as_str()
            .expect("converted JSON should contain a string")
            .as_ptr(),
        source_pointer,
    );
}

/// Verifies consuming StringMap-to-JSON conversion emits object members in
/// canonical key order.
#[test]
#[cfg(feature = "json")]
fn test_data_converter_consuming_string_map_to_json_orders_keys() {
    let source = (0..64)
        .find_map(|_| {
            let source = HashMap::from([
                ("zeta".to_owned(), "last".to_owned()),
                ("alpha".to_owned(), "first".to_owned()),
                ("omega".to_owned(), "middle".to_owned()),
                ("beta".to_owned(), "second".to_owned()),
            ]);
            let keys = source.keys().map(String::as_str).collect::<Vec<_>>();
            (!keys.is_sorted()).then_some(source)
        })
        .expect("randomized HashMap iteration should produce a noncanonical order");

    let converted = DataConverter::from(source)
        .into_target::<serde_json::Value>()
        .expect("owned string map should convert to JSON");

    assert_eq!(
        converted.to_string(),
        r#"{"alpha":"first","beta":"second","omega":"middle","zeta":"last"}"#,
    );
}

/// Verifies consuming StringMap-to-JSON conversion preserves value storage.
#[test]
#[cfg(feature = "json")]
fn test_data_converter_consuming_string_map_to_json_reuses_storage() {
    let source = HashMap::from([("alpha".to_owned(), "first".to_owned())]);
    let source_pointer = source
        .get("alpha")
        .expect("test map should contain alpha")
        .as_ptr();

    let converted = DataConverter::from(source)
        .into_target::<serde_json::Value>()
        .expect("owned string map should convert to JSON");

    assert_eq!(
        converted["alpha"]
            .as_str()
            .expect("converted JSON should contain alpha")
            .as_ptr(),
        source_pointer,
    );
}

/// Verifies consuming structured conversions delegate non-identity sources to
/// the shared borrowed conversion path.
#[test]
#[cfg(feature = "json")]
fn test_data_converter_consuming_structured_non_identity_sources() {
    let json = DataConverter::from(String::from(r#"{"key":"value"}"#))
        .into_target::<serde_json::Value>()
        .expect("owned JSON text should convert through the borrowed path");
    assert_eq!(json["key"], "value");

    let map = DataConverter::from(String::from(r#"{"key":"value"}"#))
        .into_target::<HashMap<String, String>>()
        .expect("owned map text should convert through the borrowed path");
    assert_eq!(map.get("key").map(String::as_str), Some("value"));
}

/// Test URL and JSON conversion behavior.
#[test]
#[cfg(all(
    feature = "big-number",
    feature = "chrono",
    feature = "url",
    feature = "json"
))]
fn test_data_converter_url_and_json_conversions() {
    let url: Url = DataConverter::from("https://example.com/path")
        .to()
        .expect("URL string should parse");
    assert_eq!(url.as_str(), "https://example.com/path");

    let direct_url: Url = DataConverter::from(&url)
        .to()
        .expect("URL should convert to URL");
    assert_eq!(direct_url, url);

    assert!(matches!(
        DataConverter::from("not a url").to::<Url>(),
        Err(conversion_error) if conversion_error.kind() == DataConversionErrorKind::InvalidValue
    ));
    assert!(matches!(
        DataConverter::Unset(DataType::Url).to::<Url>(),
        Err(conversion_error) if conversion_error.kind() == DataConversionErrorKind::Missing
    ));
    assert!(matches!(
        DataConverter::from(1i32).to::<Url>(),
        Err(conversion_error) if conversion_error.kind() == DataConversionErrorKind::Unsupported
    ));

    let json: serde_json::Value = DataConverter::from(r#"{"answer":42}"#)
        .to()
        .expect("JSON string should parse");
    assert_eq!(json["answer"], 42);

    let direct_json: serde_json::Value = DataConverter::from(&json)
        .to()
        .expect("JSON should convert to JSON");
    assert_eq!(direct_json["answer"], 42);

    let mut map = HashMap::new();
    map.insert("answer".to_string(), "42".to_string());
    let json: serde_json::Value = DataConverter::from(&map)
        .to()
        .expect("string map should convert to JSON object");
    assert_eq!(json["answer"], "42");

    let text: String = DataConverter::from(json)
        .to()
        .expect("JSON value should serialize to string");
    assert_eq!(text, r#"{"answer":"42"}"#);

    assert!(matches!(
        DataConverter::from("{").to::<serde_json::Value>(),
        Err(conversion_error) if matches!(conversion_error.reason(), Some(InvalidValueReason::Deserialization {
                format: DataFormat::Json,
            })
    )));
    assert!(matches!(
        DataConverter::Unset(DataType::Json).to::<serde_json::Value>(),
        Err(conversion_error) if conversion_error.kind() == DataConversionErrorKind::Missing
    ));
    assert!(matches!(
        DataConverter::from(1i32).to::<serde_json::Value>(),
        Err(conversion_error) if conversion_error.kind() == DataConversionErrorKind::Unsupported
    ));
}
