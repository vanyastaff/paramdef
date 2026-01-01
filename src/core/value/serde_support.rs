//! Serde serialization support for Value.

use std::fmt;
use std::str::FromStr;
use std::sync::Arc;

use serde::{Deserialize, Serialize};

use super::Value;

impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Value::Null => serializer.serialize_none(),
            Value::Bool(b) => serializer.serialize_bool(*b),
            Value::Int(i) => serializer.serialize_i64(*i),
            Value::Float(f) => serializer.serialize_f64(*f),
            Value::Text(s) => serializer.serialize_str(s),
            Value::Array(arr) => arr.serialize(serializer),
            Value::Object(obj) => obj.serialize(serializer),
            Value::Binary(bytes) => serializer.serialize_bytes(bytes),
        }
    }
}

impl<'de> Deserialize<'de> for Value {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Deserialize as serde_json::Value first, then convert
        let json: serde_json::Value = Deserialize::deserialize(deserializer)?;
        Ok(Value::from(json))
    }
}

impl From<Value> for serde_json::Value {
    fn from(value: Value) -> Self {
        match value {
            Value::Null => serde_json::Value::Null,
            Value::Bool(b) => serde_json::Value::Bool(b),
            Value::Int(i) => serde_json::Value::Number(i.into()),
            Value::Float(f) => {
                // Handle non-finite floats by converting to string representation
                // to preserve information (NaN, Infinity, -Infinity)
                if let Some(n) = serde_json::Number::from_f64(f) {
                    serde_json::Value::Number(n)
                } else {
                    // Non-finite float: use string representation to preserve value
                    serde_json::Value::String(f.to_string())
                }
            }
            Value::Text(s) => serde_json::Value::String(s.to_string()),
            Value::Array(arr) => {
                // Pre-allocate with known size
                let mut vec = Vec::with_capacity(arr.len());
                vec.extend(arr.iter().cloned().map(Into::into));
                serde_json::Value::Array(vec)
            }
            Value::Object(obj) => {
                // Pre-allocate with known size to avoid rehashing
                let mut map = serde_json::Map::with_capacity(obj.len());
                map.extend(obj.iter().map(|(k, v)| (k.to_string(), v.clone().into())));
                serde_json::Value::Object(map)
            }
            Value::Binary(bytes) => {
                use base64::Engine;
                let encoded = base64::engine::general_purpose::STANDARD.encode(&*bytes);
                serde_json::Value::String(encoded)
            }
        }
    }
}

impl From<serde_json::Value> for Value {
    fn from(json: serde_json::Value) -> Self {
        match json {
            serde_json::Value::Null => Value::Null,
            serde_json::Value::Bool(b) => Value::Bool(b),
            serde_json::Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    Value::Int(i)
                } else if let Some(f) = n.as_f64() {
                    Value::Float(f)
                } else {
                    // Large u64 values that don't fit in i64 or f64
                    // Store as text to preserve the value
                    Value::text(n.to_string())
                }
            }
            serde_json::Value::String(s) => {
                // Check if this might be a non-finite float that was serialized as string
                match s.as_str() {
                    "NaN" => Value::Float(f64::NAN),
                    "inf" | "Infinity" => Value::Float(f64::INFINITY),
                    "-inf" | "-Infinity" => Value::Float(f64::NEG_INFINITY),
                    _ => Value::text(s),
                }
            }
            serde_json::Value::Array(arr) => {
                // Pre-allocate with known size
                let mut vec = Vec::with_capacity(arr.len());
                vec.extend(arr.into_iter().map(Value::from));
                Value::Array(Arc::from(vec.into_boxed_slice()))
            }
            serde_json::Value::Object(obj) => {
                Value::object(obj.into_iter().map(|(k, v)| (k, Value::from(v))))
            }
        }
    }
}

impl FromStr for Value {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let json: serde_json::Value = serde_json::from_str(s)?;
        Ok(Value::from(json))
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json: serde_json::Value = self.clone().into();
        let result = if f.alternate() {
            serde_json::to_string_pretty(&json)
        } else {
            serde_json::to_string(&json)
        };
        match result {
            Ok(s) => write!(f, "{s}"),
            Err(_) => write!(f, "<serialization error>"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_to_json() {
        let value = Value::object([("name", Value::text("Alice")), ("age", Value::Int(30))]);

        let json: serde_json::Value = value.into();
        assert!(json.is_object());
        assert_eq!(json["name"], "Alice");
        assert_eq!(json["age"], 30);
    }

    #[test]
    fn test_json_to_value() {
        let json = serde_json::json!({
            "name": "Bob",
            "active": true,
            "score": 95.5
        });

        let value: Value = json.into();
        assert!(value.is_object());

        let obj = value.as_object().unwrap();
        assert_eq!(obj.get("name").and_then(|v| v.as_text()), Some("Bob"));
        assert_eq!(obj.get("active").and_then(|v| v.as_bool()), Some(true));
        assert_eq!(obj.get("score").and_then(|v| v.as_float()), Some(95.5));
    }

    #[test]
    fn test_value_from_str() {
        let value: Value = r#"{"key": "value"}"#.parse().unwrap();
        assert!(value.is_object());
    }

    #[test]
    fn test_value_display() {
        let value = Value::object([("a", Value::Int(1))]);
        let display = format!("{}", value);
        assert!(display.contains("\"a\""));
        assert!(display.contains("1"));
    }

    #[test]
    fn test_value_display_pretty() {
        let value = Value::object([("a", Value::Int(1))]);
        let display = format!("{:#}", value);
        assert!(display.contains('\n')); // Pretty print has newlines
    }

    #[test]
    fn test_value_serialize_deserialize() {
        let original = Value::array([Value::Int(1), Value::text("two"), Value::Bool(true)]);

        let json_str = serde_json::to_string(&original).unwrap();
        let restored: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(original, restored);
    }

    #[test]
    fn test_float_nan_to_json() {
        let value = Value::Float(f64::NAN);
        let json: serde_json::Value = value.into();
        // NaN is converted to string "NaN"
        assert_eq!(json, serde_json::Value::String("NaN".to_string()));
    }

    #[test]
    fn test_float_infinity_to_json() {
        let pos_inf = Value::Float(f64::INFINITY);
        let json: serde_json::Value = pos_inf.into();
        assert_eq!(json, serde_json::Value::String("inf".to_string()));

        let neg_inf = Value::Float(f64::NEG_INFINITY);
        let json: serde_json::Value = neg_inf.into();
        assert_eq!(json, serde_json::Value::String("-inf".to_string()));
    }

    #[test]
    fn test_json_nan_string_to_value() {
        let json = serde_json::Value::String("NaN".to_string());
        let value: Value = json.into();
        assert!(value.is_float());
        assert!(value.as_float().unwrap().is_nan());
    }

    #[test]
    fn test_json_infinity_string_to_value() {
        // "Infinity" format
        let json = serde_json::Value::String("Infinity".to_string());
        let value: Value = json.into();
        assert!(value.is_float());
        assert_eq!(value.as_float(), Some(f64::INFINITY));

        // "inf" format
        let json = serde_json::Value::String("inf".to_string());
        let value: Value = json.into();
        assert!(value.is_float());
        assert_eq!(value.as_float(), Some(f64::INFINITY));

        // "-Infinity" format
        let json = serde_json::Value::String("-Infinity".to_string());
        let value: Value = json.into();
        assert!(value.is_float());
        assert_eq!(value.as_float(), Some(f64::NEG_INFINITY));

        // "-inf" format
        let json = serde_json::Value::String("-inf".to_string());
        let value: Value = json.into();
        assert!(value.is_float());
        assert_eq!(value.as_float(), Some(f64::NEG_INFINITY));
    }

    #[test]
    fn test_non_finite_float_roundtrip() {
        // NaN roundtrip
        let original = Value::Float(f64::NAN);
        let json: serde_json::Value = original.into();
        let restored: Value = json.into();
        assert!(restored.as_float().unwrap().is_nan());

        // +Infinity roundtrip
        let original = Value::Float(f64::INFINITY);
        let json: serde_json::Value = original.into();
        let restored: Value = json.into();
        assert_eq!(restored.as_float(), Some(f64::INFINITY));

        // -Infinity roundtrip
        let original = Value::Float(f64::NEG_INFINITY);
        let json: serde_json::Value = original.into();
        let restored: Value = json.into();
        assert_eq!(restored.as_float(), Some(f64::NEG_INFINITY));
    }
}
