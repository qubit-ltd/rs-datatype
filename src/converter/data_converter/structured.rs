// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
// =============================================================================
//! Structured JSON and string-map conversion implementations.

use std::collections::HashMap;
use std::fmt;

use serde::Deserializer;
use serde::de::{
    MapAccess,
    Visitor,
};

use super::DataConverter;
use super::string_source::normalize;
use crate::converter::{
    DataConversionError,
    DataConversionOptions,
    DataConvertTo,
    DataFormat,
    InvalidValueReason,
};
use crate::datatype::DataType;

impl DataConvertTo<serde_json::Value> for DataConverter<'_> {
    fn convert(
        &self,
        options: &DataConversionOptions,
    ) -> Result<serde_json::Value, DataConversionError> {
        match self {
            Self::Json(value) => Ok(value.as_ref().clone()),
            Self::String(value) => {
                let value = normalize(value, options, DataType::Json)?;
                match serde_json::from_str(value) {
                    Ok(value) => Ok(value),
                    Err(_) => Err(self.invalid(
                        DataType::Json,
                        InvalidValueReason::Deserialization {
                            format: DataFormat::Json,
                        },
                    )),
                }
            }
            Self::StringMap(value) => Ok(serde_json::Value::Object(
                value
                    .iter()
                    .map(|(key, value)| {
                        (key.clone(), serde_json::Value::String(value.clone()))
                    })
                    .collect(),
            )),
            Self::Empty(_) => Err(self.missing(DataType::Json)),
            _ => Err(self.unsupported(DataType::Json)),
        }
    }
}

/// Visitor that accepts string-valued JSON objects and rejects duplicate keys.
struct StringMapVisitor;

impl<'de> Visitor<'de> for StringMapVisitor {
    type Value = HashMap<String, String>;

    /// Describes the accepted JSON shape.
    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a JSON object with unique keys and string values")
    }

    /// Reads the map while checking duplicate keys.
    fn visit_map<A>(self, mut access: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut result =
            HashMap::with_capacity(access.size_hint().unwrap_or(0));
        while let Some((key, value)) = access.next_entry::<String, String>()? {
            if result.insert(key, value).is_some() {
                return Err(serde::de::Error::custom("duplicate object key"));
            }
        }
        Ok(result)
    }
}

/// Deserializes a string map through the duplicate-aware visitor.
///
/// `value` must contain exactly one JSON object with unique keys and string
/// values. The returned map owns all keys and values. Syntax errors, trailing
/// data, duplicate keys, and non-string values return `serde_json::Error`.
fn deserialize_string_map(
    value: &str,
) -> Result<HashMap<String, String>, serde_json::Error> {
    let mut deserializer = serde_json::Deserializer::from_str(value);
    let result = deserializer.deserialize_map(StringMapVisitor)?;
    deserializer.end()?;
    Ok(result)
}

impl DataConvertTo<HashMap<String, String>> for DataConverter<'_> {
    fn convert(
        &self,
        options: &DataConversionOptions,
    ) -> Result<HashMap<String, String>, DataConversionError> {
        match self {
            Self::StringMap(value) => Ok(value.as_ref().clone()),
            Self::String(value) => {
                let value = normalize(value, options, DataType::StringMap)?;
                match deserialize_string_map(value) {
                    Ok(value) => Ok(value),
                    Err(_) => Err(self.invalid(
                        DataType::StringMap,
                        InvalidValueReason::Deserialization {
                            format: DataFormat::Json,
                        },
                    )),
                }
            }
            Self::Empty(_) => Err(self.missing(DataType::StringMap)),
            _ => Err(self.unsupported(DataType::StringMap)),
        }
    }
}
