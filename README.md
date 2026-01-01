# paramdef

[![Crates.io](https://img.shields.io/crates/v/paramdef.svg)](https://crates.io/crates/paramdef)
[![Documentation](https://docs.rs/paramdef/badge.svg)](https://docs.rs/paramdef)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE-MIT)

**Type-safe parameter definition system for Rust** — The "serde of parameter schemas"

Inspired by Blender RNA, Unreal Engine UPROPERTY, Qt Property System, and Houdini Parameters.

## Overview

`paramdef` provides a comprehensive system for defining, validating, and managing parameters in applications. Perfect for:

- 🎯 **Workflow Engines** - Define node parameters with validation
- 🎨 **Visual Programming Tools** - Type-safe parameter schemas
- 🔧 **No-Code Platforms** - Dynamic form generation
- 🎮 **Game Engines** - Component properties and settings
- ⚙️ **CLI Tools** - Configuration with validation

## Quick Start

```rust
use paramdef::prelude::*;

// Define parameter schema
let schema = Schema::builder()
    .parameter(Text::builder("username")
        .label("Username")
        .required()
        .build())
    .parameter(Number::builder("age")
        .label("Age")
        .default(18.0)
        .build())
    .parameter(Boolean::builder("active")
        .label("Active")
        .default(true)
        .build())
    .build();

// Create runtime context
let mut ctx = Context::new(Arc::new(schema));

// Set and get values
ctx.set("username", Value::text("alice"));
ctx.set("age", Value::Float(25.0));

assert_eq!(ctx.get("username").and_then(|v| v.as_text()), Some("alice"));
```

## Key Features

### 🏗️ Three-Layer Architecture

```
┌─────────────────────────────────────┐
│  Schema Layer (Immutable)           │  ← Shared definitions (Arc)
│  - Metadata, flags, validators      │
├─────────────────────────────────────┤
│  Runtime Layer (Mutable)            │  ← Per-instance state
│  - Current values, dirty flags      │
├─────────────────────────────────────┤
│  Value Layer                        │  ← Runtime representation
│  - Unified Value enum               │
└─────────────────────────────────────┘
```

### 📊 14 Core Types

| Category   | Own Value | Children | Types |
|------------|-----------|----------|-------|
| **Group**      | ❌ | ✅ | 1 - Root aggregator |
| **Layout**     | ❌ | ✅ | 1 - UI organization |
| **Decoration** | ❌ | ❌ | 5 - Display elements |
| **Container**  | ✅ | ✅ | 6 - Structured data |
| **Leaf**       | ✅ | ❌ | 5 - Terminal values |

**Leaf Types:** Text, Number, Boolean, Vector, Select
**Containers:** Object, List, Mode, Routing, Expirable, Reference
**Decorations:** Notice, Separator, Link, Code, Image

### 🎯 Type-Safe Subtypes

Compile-time constraints for specialized parameters:

```rust
use paramdef::types::leaf::{Text, Number, Vector};
use paramdef::subtype::{Email, Port, Percentage};

// Email validation (compile-time enforced)
let email: Text<Email> = Text::email("contact");

// Port numbers (integer-only)
let port: Number<Port> = Number::port("http_port")
    .default(8080.0)
    .build();

// Percentage (float-only, 0-100 range)
let opacity: Number<Percentage> = Number::percentage("alpha")
    .default(100.0)
    .build();

// Fixed-size vectors (compile-time size)
let position = Vector::builder::<f64, 3>("pos")
    .default([0.0, 0.0, 0.0])
    .build();
```

### 🔧 Blender-Style Subtype + Unit Pattern

Separate semantic meaning from measurement system:

```rust
use paramdef::subtype::NumberUnit;

// Subtype = WHAT it is (semantic)
// Unit = HOW to measure (system)
let distance = Number::builder("length")
    .unit(NumberUnit::Meters)
    .default(10.0)
    .build();

// 60 subtypes × 17 unit categories = powerful combinations!
```

### 🚀 Performance

Excellent performance characteristics:

- **Schema creation**: ~100-500ns per parameter
- **Context (100 params)**: ~50µs initialization
- **Runtime node**: ~200ns creation
- **Container ops**: ~2-10µs for nested structures

Optimizations:
- `SmartString` for stack-allocated short strings (<23 bytes)
- `Arc` for cheap cloning of immutable data
- Const generics for fixed-size vectors (on stack, no heap)

## Feature Flags

```toml
[dependencies]
paramdef = { version = "0.2", features = ["serde", "validation"] }
```

| Feature | Description |
|---------|-------------|
| `serde` | Serialization/deserialization support |
| `validation` | Validation system with custom validators |
| `visibility` | Visibility conditions and expressions |
| `events` | Event system with tokio channels |
| `i18n` | Internationalization with Fluent |
| `chrono` | Chrono type conversions |
| `full` | Enable all features |

**Core library has zero UI dependencies** - works headless (servers, CLI).

## Examples

### Complex Nested Schemas

```rust
use paramdef::types::container::Object;
use paramdef::types::leaf::{Text, Number, Boolean};

let address = Object::builder("address")
    .field("street", Text::builder("street").required().build())
    .field("city", Text::builder("city").required().build())
    .field("zip", Text::builder("zip").build())
    .build()
    .unwrap();

let user = Object::builder("user")
    .field("name", Text::builder("name").required().build())
    .field("email", Text::email("email"))
    .field("age", Number::builder("age").build())
    .field("address", address)
    .build()
    .unwrap();
```

### Mode Container (Discriminated Unions)

```rust
use paramdef::types::container::Mode;

// Output can be file, database, or API
let output = Mode::builder("output")
    .variant("file", file_params)
    .variant("database", db_params)
    .variant("api", api_params)
    .build()
    .unwrap();

// Runtime value: {"mode": "database", "value": {...}}
```

### Using Flags

```rust
use paramdef::core::Flags;

let password = Text::builder("password")
    .flags(Flags::REQUIRED | Flags::SENSITIVE)
    .build();

assert!(password.flags().contains(Flags::REQUIRED));
assert!(password.flags().contains(Flags::SENSITIVE));
```

## Architecture

### Node Categories

**Group** (1 type)
- Root aggregator with NO own value
- Provides `ValueAccess` at runtime
- Can contain: Layout, Decoration, Container, Leaf

**Layout** (1 type)
- UI organization (Panel)
- NO own value, has `ValueAccess`
- Can contain: Decoration, Container, Leaf

**Decoration** (5 types)
- Display-only, NO value, NO children
- Types: Notice, Separator, Link, Code, Image

**Container** (6 types)
- HAS own value + children
- Provides `ValueAccess` at runtime
- Types: Object, List, Mode, Routing, Expirable, Reference

**Leaf** (5 types)
- Terminal values, NO children
- Types: Text, Number, Boolean, Vector, Select

## Current Status

**Version 0.2.0** - Active Development

✅ **Complete:**
- Core type system (14 types)
- Three-layer architecture
- Subtype system with compile-time constraints
- Comprehensive benchmarks
- Zero-warning build

🚧 **In Progress:**
- Validation system
- Event system (undo/redo)
- Visibility/conditional logic
- i18n integration

📚 **Documentation:**
- 18 comprehensive design documents in `docs/`
- Full API documentation
- Architecture guide

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
paramdef = "0.2"
```

## Documentation

- [API Documentation](https://docs.rs/paramdef)
- [Architecture Guide](docs/01-ARCHITECTURE.md)
- [Type System Reference](docs/02-TYPE-SYSTEM.md)
- [Design Decisions](docs/17-DESIGN-DECISIONS.md)

## MSRV

Minimum Supported Rust Version: **1.85**

Uses Rust 2024 Edition.

## Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
