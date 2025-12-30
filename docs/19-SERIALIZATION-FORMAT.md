# Serialization Format

**JSON schema specification for paramdef serialization**

> **Requires:** `features = ["serde"]`

---

## Table of Contents

1. [Overview](#overview)
2. [Value Serialization](#value-serialization)
3. [Schema Serialization](#schema-serialization)
4. [Context Serialization](#context-serialization)
5. [Metadata Serialization](#metadata-serialization)
6. [Display Config Serialization](#display-config-serialization)
7. [Validation Config Serialization](#validation-config-serialization)
8. [Complete Examples](#complete-examples)
9. [Versioning](#versioning)
10. [Best Practices](#best-practices)

---

## Overview

paramdef uses `serde` for serialization with JSON as the primary format. The serialization format is designed to be:

- **Human-readable** - Easy to inspect and debug
- **Compact** - Omit null/empty fields
- **Forward-compatible** - Unknown fields are ignored
- **Round-trip safe** - Deserialize what you serialize

### Feature Flag

```toml
[dependencies]
paramdef = { version = "0.1", features = ["serde"] }
```

### Basic Usage

```rust
use paramdef::prelude::*;
use serde_json;

// Serialize schema
let json = serde_json::to_string_pretty(&schema)?;

// Deserialize schema
let schema: Schema = serde_json::from_str(&json)?;

// Serialize context values
let values_json = serde_json::to_string(&context.values())?;
```

---

## Value Serialization

The `Value` enum serializes to JSON primitives directly.

### Value → JSON Mapping

| Value Variant | JSON Type | Example |
|---------------|-----------|---------|
| `Value::Null` | `null` | `null` |
| `Value::Bool(b)` | boolean | `true` |
| `Value::Int(i)` | number | `42` |
| `Value::Float(f)` | number | `3.14` |
| `Value::Text(s)` | string | `"hello"` |
| `Value::Array(arr)` | array | `[1, 2, 3]` |
| `Value::Object(map)` | object | `{"a": 1}` |

### Examples

```rust
// Primitives
Value::Null           // → null
Value::Bool(true)     // → true
Value::Int(42)        // → 42
Value::Float(3.14)    // → 3.14
Value::Text("hello")  // → "hello"

// Array (for Vector, List)
Value::Array(vec![
    Value::Float(1.0),
    Value::Float(0.5),
    Value::Float(0.0),
])  // → [1.0, 0.5, 0.0]

// Object (for Object, Mode)
Value::Object(HashMap::from([
    ("name".into(), Value::Text("Alice".into())),
    ("age".into(), Value::Int(30)),
]))  // → {"name": "Alice", "age": 30}
```

### Mode Value Format

Mode (discriminated union) serializes with `mode` and `value` fields:

```json
{
  "mode": "basic",
  "value": {
    "username": "admin",
    "password": "secret"
  }
}
```

### Vector Value Format

Vectors serialize as JSON arrays:

```json
{
  "position": [1.0, 2.0, 3.0],
  "color": [1.0, 0.5, 0.0, 1.0]
}
```

---

## Schema Serialization

Schema serializes as a JSON object with parameter definitions.

### Schema JSON Structure

```json
{
  "version": "1.0",
  "parameters": [
    {
      "kind": "Text",
      "key": "name",
      "metadata": { ... },
      "subtype": "SingleLine",
      "constraints": { ... },
      "visibility": { ... },
      "validation": { ... }
    },
    {
      "kind": "Number",
      "key": "age",
      ...
    }
  ],
  "groups": [
    {
      "key": "personal",
      "label": "Personal Info",
      "children": ["name", "age"]
    }
  ]
}
```

### Node Kind Values

| Node Type | `kind` Value |
|-----------|--------------|
| Group | `"Group"` |
| Panel | `"Panel"` |
| Notice | `"Notice"` |
| Object | `"Object"` |
| List | `"List"` |
| Mode | `"Mode"` |
| Routing | `"Routing"` |
| Expirable | `"Expirable"` |
| Text | `"Text"` |
| Number | `"Number"` |
| Boolean | `"Boolean"` |
| Vector | `"Vector"` |
| Select | `"Select"` |

### Text Node Example

```json
{
  "kind": "Text",
  "key": "email",
  "metadata": {
    "label": "Email Address",
    "description": "Your email for notifications",
    "placeholder": "user@example.com"
  },
  "subtype": "Email",
  "flags": ["REQUIRED"],
  "constraints": {
    "min_length": 5,
    "max_length": 255
  }
}
```

### Number Node Example

```json
{
  "kind": "Number",
  "key": "temperature",
  "metadata": {
    "label": "Temperature"
  },
  "subtype": "Temperature",
  "unit": {
    "category": "Temperature",
    "unit": "Celsius"
  },
  "constraints": {
    "min": -273.15,
    "max": 1000.0,
    "step": 0.1
  },
  "default": 20.0
}
```

### Vector Node Example

```json
{
  "kind": "Vector",
  "key": "position",
  "metadata": {
    "label": "Position"
  },
  "subtype": "Position3D",
  "size": 3,
  "default": [0.0, 0.0, 0.0]
}
```

### Select Node Example

```json
{
  "kind": "Select",
  "key": "theme",
  "metadata": {
    "label": "Theme"
  },
  "mode": "Single",
  "options": [
    { "value": "light", "label": "Light Theme" },
    { "value": "dark", "label": "Dark Theme" },
    { "value": "auto", "label": "Auto" }
  ],
  "default": "auto"
}
```

### Mode Node Example

```json
{
  "kind": "Mode",
  "key": "auth",
  "metadata": {
    "label": "Authentication"
  },
  "variants": [
    {
      "key": "none",
      "label": "No Authentication",
      "schema": null
    },
    {
      "key": "basic",
      "label": "Basic Auth",
      "schema": {
        "parameters": [
          { "kind": "Text", "key": "username", ... },
          { "kind": "Text", "key": "password", "subtype": "Secret", ... }
        ]
      }
    },
    {
      "key": "bearer",
      "label": "Bearer Token",
      "schema": {
        "parameters": [
          { "kind": "Text", "key": "token", "subtype": "Secret", ... }
        ]
      }
    }
  ],
  "default_variant": "none"
}
```

---

## Context Serialization

Context can be serialized in two modes: **values only** or **full state**.

### Values Only (Recommended for Storage)

```json
{
  "name": "Alice",
  "age": 30,
  "email": "alice@example.com",
  "settings": {
    "theme": "dark",
    "notifications": true
  }
}
```

### Full State (For Debugging/Snapshots)

```json
{
  "schema_version": "1.0",
  "values": {
    "name": "Alice",
    "age": 30
  },
  "state": {
    "name": {
      "dirty": true,
      "touched": true,
      "valid": true
    },
    "age": {
      "dirty": false,
      "touched": false,
      "valid": true
    }
  },
  "errors": {}
}
```

### Serialization Methods

```rust
// Values only (compact)
let values = context.collect_values();
let json = serde_json::to_string(&values)?;

// Full state (debug)
let snapshot = context.snapshot();
let json = serde_json::to_string(&snapshot)?;

// Restore values
let values: HashMap<Key, Value> = serde_json::from_str(&json)?;
context.set_values(values)?;
```

---

## Metadata Serialization

### Metadata JSON Structure

```json
{
  "label": "Email Address",
  "description": "Your email for notifications",
  "placeholder": "user@example.com",
  "help": "We'll never share your email",
  "group": "contact",
  "page": "Personal",
  "order": 10,
  "tags": ["required", "contact"],
  "fluent_id": "email-field"
}
```

### Optional Fields

All metadata fields are optional except `key` (which is the parameter key):

```json
{
  "label": "Name"
}
```

### Localization Keys

When using i18n feature, fluent IDs are serialized:

```json
{
  "label": "Database Host",
  "fluent_id": "db-host",
  "description_fluent_id": "db-host-desc"
}
```

---

## Display Config Serialization

### Display Rules

```json
{
  "show_when": {
    "type": "Equals",
    "field": "auth_type",
    "value": "api_key"
  }
}
```

### Complex Conditions

```json
{
  "show_when": {
    "type": "And",
    "conditions": [
      {
        "type": "Equals",
        "field": "use_ssl",
        "value": true
      },
      {
        "type": "NotEquals",
        "field": "protocol",
        "value": "http"
      }
    ]
  }
}
```

### Condition Types

| Type | JSON | Description |
|------|------|-------------|
| Equals | `{"type": "Equals", "field": "x", "value": v}` | field == value |
| NotEquals | `{"type": "NotEquals", "field": "x", "value": v}` | field != value |
| IsSet | `{"type": "IsSet", "field": "x"}` | field is not null |
| IsNull | `{"type": "IsNull", "field": "x"}` | field is null |
| IsEmpty | `{"type": "IsEmpty", "field": "x"}` | field is empty |
| IsNotEmpty | `{"type": "IsNotEmpty", "field": "x"}` | field is not empty |
| IsTrue | `{"type": "IsTrue", "field": "x"}` | field == true |
| IsFalse | `{"type": "IsFalse", "field": "x"}` | field == false |
| GreaterThan | `{"type": "GreaterThan", "field": "x", "value": n}` | field > n |
| LessThan | `{"type": "LessThan", "field": "x", "value": n}` | field < n |
| InRange | `{"type": "InRange", "field": "x", "min": a, "max": b}` | a <= field <= b |
| Contains | `{"type": "Contains", "field": "x", "value": s}` | field contains s |
| OneOf | `{"type": "OneOf", "field": "x", "values": [...]}` | field in values |
| And | `{"type": "And", "conditions": [...]}` | all conditions |
| Or | `{"type": "Or", "conditions": [...]}` | any condition |
| Not | `{"type": "Not", "condition": {...}}` | negate |

---

## Validation Config Serialization

Validation rules serialize declaratively:

```json
{
  "validation": {
    "required": true,
    "rules": [
      { "type": "MinLength", "value": 3 },
      { "type": "MaxLength", "value": 100 },
      { "type": "Pattern", "regex": "^[a-z0-9]+$", "message": "Only lowercase letters and numbers" }
    ]
  }
}
```

### Built-in Validation Rules

| Rule | JSON |
|------|------|
| Required | `{"type": "Required"}` |
| MinLength | `{"type": "MinLength", "value": n}` |
| MaxLength | `{"type": "MaxLength", "value": n}` |
| Min | `{"type": "Min", "value": n}` |
| Max | `{"type": "Max", "value": n}` |
| Range | `{"type": "Range", "min": a, "max": b}` |
| Pattern | `{"type": "Pattern", "regex": "...", "message": "..."}` |
| Email | `{"type": "Email"}` |
| Url | `{"type": "Url"}` |
| Uuid | `{"type": "Uuid"}` |

**Note:** Custom validators (closures, async validators) cannot be serialized. They must be re-attached after deserialization.

---

## Complete Examples

### User Profile Schema

```json
{
  "version": "1.0",
  "parameters": [
    {
      "kind": "Text",
      "key": "username",
      "metadata": {
        "label": "Username",
        "placeholder": "johndoe"
      },
      "subtype": "Username",
      "flags": ["REQUIRED"],
      "constraints": {
        "min_length": 3,
        "max_length": 20,
        "pattern": "^[a-z0-9_]+$"
      }
    },
    {
      "kind": "Text",
      "key": "email",
      "metadata": {
        "label": "Email Address"
      },
      "subtype": "Email",
      "flags": ["REQUIRED"]
    },
    {
      "kind": "Text",
      "key": "bio",
      "metadata": {
        "label": "Bio",
        "placeholder": "Tell us about yourself..."
      },
      "subtype": "MultiLine",
      "constraints": {
        "max_length": 500
      }
    },
    {
      "kind": "Number",
      "key": "age",
      "metadata": {
        "label": "Age"
      },
      "subtype": "Age",
      "constraints": {
        "min": 13,
        "max": 150
      }
    },
    {
      "kind": "Vector",
      "key": "avatar_color",
      "metadata": {
        "label": "Avatar Color"
      },
      "subtype": "ColorRgb",
      "size": 3,
      "default": [0.2, 0.5, 0.8]
    }
  ]
}
```

### User Profile Values

```json
{
  "username": "johndoe",
  "email": "john@example.com",
  "bio": "Software developer from Seattle",
  "age": 28,
  "avatar_color": [0.2, 0.5, 0.8]
}
```

### API Client Config Schema

```json
{
  "version": "1.0",
  "parameters": [
    {
      "kind": "Text",
      "key": "base_url",
      "metadata": {
        "label": "Base URL",
        "group": "Connection"
      },
      "subtype": "Url",
      "flags": ["REQUIRED"]
    },
    {
      "kind": "Mode",
      "key": "auth",
      "metadata": {
        "label": "Authentication",
        "group": "Connection"
      },
      "variants": [
        {
          "key": "none",
          "label": "None",
          "schema": null
        },
        {
          "key": "api_key",
          "label": "API Key",
          "schema": {
            "parameters": [
              {
                "kind": "Text",
                "key": "api_key",
                "metadata": { "label": "API Key" },
                "subtype": "Secret",
                "flags": ["REQUIRED", "SENSITIVE"]
              },
              {
                "kind": "Text",
                "key": "header_name",
                "metadata": { "label": "Header Name" },
                "subtype": "SingleLine",
                "default": "X-API-Key"
              }
            ]
          }
        },
        {
          "key": "bearer",
          "label": "Bearer Token",
          "schema": {
            "parameters": [
              {
                "kind": "Text",
                "key": "token",
                "metadata": { "label": "Token" },
                "subtype": "Secret",
                "flags": ["REQUIRED", "SENSITIVE"]
              }
            ]
          }
        }
      ],
      "default_variant": "none"
    },
    {
      "kind": "Number",
      "key": "timeout",
      "metadata": {
        "label": "Timeout",
        "group": "Advanced"
      },
      "subtype": "DurationSeconds",
      "constraints": {
        "min": 1,
        "max": 300
      },
      "default": 30
    }
  ],
  "groups": [
    { "key": "Connection", "label": "Connection Settings", "order": 0 },
    { "key": "Advanced", "label": "Advanced Settings", "order": 1 }
  ]
}
```

---

## Versioning

### Schema Version Field

Always include version for forward compatibility:

```json
{
  "version": "1.0",
  "parameters": [...]
}
```

### Version Migration

```rust
fn migrate_schema(json: &str) -> Result<Schema> {
    let value: serde_json::Value = serde_json::from_str(json)?;
    
    let version = value.get("version")
        .and_then(|v| v.as_str())
        .unwrap_or("1.0");
    
    match version {
        "1.0" => serde_json::from_value(value),
        "0.9" => migrate_v09_to_v10(value),
        _ => Err(Error::UnsupportedVersion(version.into())),
    }
}
```

---

## Best Practices

### 1. Omit Default Values

```json
// ✅ Good - omit defaults
{
  "kind": "Text",
  "key": "name",
  "metadata": { "label": "Name" }
}

// ❌ Bad - unnecessary defaults
{
  "kind": "Text",
  "key": "name",
  "metadata": { "label": "Name" },
  "subtype": "Generic",
  "flags": [],
  "constraints": {},
  "default": null
}
```

### 2. Use Subtype Instead of Validation

```json
// ✅ Good - subtype implies validation
{
  "kind": "Text",
  "key": "email",
  "subtype": "Email"
}

// ❌ Bad - redundant
{
  "kind": "Text",
  "key": "email",
  "subtype": "Generic",
  "validation": {
    "rules": [{ "type": "Email" }]
  }
}
```

### 3. Sensitive Data Handling

**Never serialize sensitive values in plain text:**

```rust
// Filter sensitive values before serialization
let safe_values = context.collect_values_filtered(|param| {
    !param.flags().contains(Flags::SENSITIVE)
});

let json = serde_json::to_string(&safe_values)?;
```

### 4. Pretty Print for Config Files

```rust
// For config files (human-readable)
let json = serde_json::to_string_pretty(&schema)?;

// For API/storage (compact)
let json = serde_json::to_string(&schema)?;
```

### 5. Validate After Deserialization

```rust
let schema: Schema = serde_json::from_str(&json)?;

// Always validate after loading
schema.validate()?;

// Create context and validate values
let mut context = Context::new(Arc::new(schema));
context.set_values(loaded_values)?;
context.validate_all().await?;
```

---

## See Also

- [02-TYPE-SYSTEM](02-TYPE-SYSTEM.md) - Node types and Value enum
- [05-VALIDATION](05-VALIDATION.md) - Validation rules
- [12-SCHEMA-CONTEXT](12-SCHEMA-CONTEXT.md) - Schema vs Context architecture
- [20-THREADING-SAFETY](20-THREADING-SAFETY.md) - Thread-safe serialization patterns
