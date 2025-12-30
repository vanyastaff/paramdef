# Project Structure

Source code organization following Rust 2024 Edition modern style.

---

## Directory Layout

```
paramdef/
├── Cargo.toml
├── src/
│   ├── lib.rs                 # Public API, re-exports
│   ├── error.rs               # Error types
│   ├── types.rs               # Marker traits (Numeric, Integer, Float, Textual)
│   ├── value.rs               # Value enum
│   ├── key.rs                 # Key type (SmartString)
│   ├── metadata.rs            # Metadata struct
│   ├── flags.rs               # Flags bitflags
│   │
│   ├── node.rs                # Node traits + re-exports
│   ├── node/
│   │   ├── group.rs           # Group + GroupBuilder
│   │   ├── panel.rs           # Panel + PanelBuilder
│   │   ├── notice.rs          # Notice + NoticeBuilder
│   │   ├── object.rs          # Object + ObjectBuilder
│   │   ├── list.rs            # List + ListBuilder
│   │   ├── mode.rs            # Mode + ModeBuilder
│   │   ├── routing.rs         # Routing + RoutingBuilder
│   │   ├── expirable.rs       # Expirable + ExpirableBuilder
│   │   ├── reference.rs       # Ref + RefBuilder
│   │   ├── text.rs            # Text + TextBuilder
│   │   ├── number.rs          # Number + NumberBuilder
│   │   ├── boolean.rs         # Boolean + BooleanBuilder
│   │   ├── vector.rs          # Vector + VectorBuilder
│   │   └── select.rs          # Select + SelectBuilder
│   │
│   ├── subtype.rs             # Subtype traits + re-exports
│   ├── subtype/
│   │   ├── number.rs          # NumberSubtype<T> implementations
│   │   ├── vector.rs          # VectorSubtype<N> implementations
│   │   └── text.rs            # TextSubtype implementations
│   │
│   ├── unit.rs                # NumberUnit enum
│   │
│   ├── transform.rs           # Transform traits + re-exports
│   ├── transform/
│   │   ├── string.rs          # StringTransform implementations
│   │   ├── number.rs          # NumericTransform<T> implementations
│   │   └── vector.rs          # VectorTransform<T, N> implementations
│   │
│   ├── runtime.rs             # Runtime layer re-exports
│   ├── runtime/
│   │   ├── context.rs         # RuntimeContext
│   │   └── state.rs           # StateFlags
│   │
│   ├── validation.rs          # Validators (#[cfg(feature = "validation")])
│   ├── ui_hints.rs            # UiHints types (#[cfg(feature = "ui")])
│   ├── display.rs             # DisplayConfig (#[cfg(feature = "display")])
│   └── events.rs              # EventBus (#[cfg(feature = "events")])
```

---

## Module Style

Modern style (Rust 2018+) — no `mod.rs` files:

```
src/
├── node.rs          # Module root: traits + pub mod + re-exports
└── node/            # Submodules
    ├── text.rs
    └── number.rs
```

**Example `node.rs`:**

```rust
//! Node types and traits

pub mod group;
pub mod panel;
pub mod notice;
pub mod object;
pub mod list;
pub mod mode;
pub mod routing;
pub mod expirable;
pub mod reference;
pub mod text;
pub mod number;
pub mod boolean;
pub mod vector;
pub mod select;

// Re-exports
pub use group::Group;
pub use panel::Panel;
pub use notice::Notice;
pub use object::Object;
pub use list::List;
pub use mode::Mode;
pub use routing::Routing;
pub use expirable::Expirable;
pub use reference::Ref;
pub use text::Text;
pub use number::Number;
pub use boolean::Boolean;
pub use vector::Vector;
pub use select::Select;

/// Base trait for all node types
pub trait Node {
    fn metadata(&self) -> &Metadata;
    fn flags(&self) -> Flags;
}

/// Trait for nodes that contain other nodes
pub trait Container: Node {
    fn children(&self) -> &[Arc<dyn Node>];
}

/// Trait for nodes with own value
pub trait Leaf: Node {
    type Value;
    fn default_value(&self) -> Option<&Self::Value>;
}
```

---

## File Organization

Each node file contains:

1. **Type definition** — the node struct
2. **Builder** — builder pattern for construction
3. **Trait implementations** — Node, Container/Leaf, serde
4. **Tests** — unit tests at bottom

**Example `node/number.rs`:**

```rust
//! Number parameter type

use crate::{Metadata, Flags, Key};
use crate::types::{Numeric, Float};
use crate::subtype::NumberSubtype;

/// Number parameter with type-safe numeric value
pub struct Number<T: Numeric, S: NumberSubtype<T> = Generic> {
    pub metadata: Metadata,
    pub subtype: S,
    pub default: Option<T>,
    
    #[cfg(feature = "validation")]
    pub validators: Vec<Arc<dyn NumericValidator<T>>>,
    
    #[cfg(feature = "ui")]
    pub ui_hints: NumberUiHints<T>,
}

impl<T: Numeric, S: NumberSubtype<T>> Number<T, S> {
    pub fn builder(key: impl Into<Key>) -> NumberBuilder<T, Generic> {
        NumberBuilder::new(key)
    }
}

// === Builder ===

pub struct NumberBuilder<T: Numeric, S: NumberSubtype<T>> {
    metadata: Metadata,
    subtype: S,
    default: Option<T>,
    // ...
}

impl<T: Numeric, S: NumberSubtype<T>> NumberBuilder<T, S> {
    pub fn subtype<S2: NumberSubtype<T>>(self, subtype: S2) -> NumberBuilder<T, S2> {
        // ...
    }
    
    pub fn default(mut self, value: T) -> Self {
        self.default = Some(value);
        self
    }
    
    pub fn build(self) -> Number<T, S> {
        // ...
    }
}

// === Trait Implementations ===

impl<T: Numeric, S: NumberSubtype<T>> Node for Number<T, S> {
    fn metadata(&self) -> &Metadata { &self.metadata }
    fn flags(&self) -> Flags { self.metadata.flags }
}

impl<T: Numeric, S: NumberSubtype<T>> Leaf for Number<T, S> {
    type Value = T;
    fn default_value(&self) -> Option<&T> { self.default.as_ref() }
}

// === Serde ===

#[cfg(feature = "serde")]
impl<T: Numeric + serde::Serialize, S: NumberSubtype<T>> serde::Serialize for Number<T, S> {
    // ...
}

// === Tests ===

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_number_builder() {
        let num = Number::<i32>::builder("count")
            .default(42)
            .build();
        assert_eq!(num.default, Some(42));
    }
}
```

---

## Marker Traits

Type-safe constraints in `types.rs`:

```rust
/// Marker trait for numeric types
pub trait Numeric: Copy + Default + PartialOrd + 'static {}

impl Numeric for i8 {}
impl Numeric for i16 {}
impl Numeric for i32 {}
impl Numeric for i64 {}
impl Numeric for i128 {}
impl Numeric for u8 {}
impl Numeric for u16 {}
impl Numeric for u32 {}
impl Numeric for u64 {}
impl Numeric for u128 {}
impl Numeric for f32 {}
impl Numeric for f64 {}

/// Marker trait for integer types
pub trait Integer: Numeric {}

impl Integer for i8 {}
impl Integer for i16 {}
impl Integer for i32 {}
impl Integer for i64 {}
impl Integer for i128 {}
impl Integer for u8 {}
impl Integer for u16 {}
impl Integer for u32 {}
impl Integer for u64 {}
impl Integer for u128 {}

/// Marker trait for float types
pub trait Float: Numeric {}

impl Float for f32 {}
impl Float for f64 {}

/// Marker trait for text types
pub trait Textual: AsRef<str> + 'static {}

impl Textual for String {}
impl Textual for &'static str {}
impl Textual for smartstring::SmartString<smartstring::LazyCompact> {}
```

**Usage for type-safe transforms:**

```rust
// Works with any Numeric
pub struct Clamp<T: Numeric> { min: T, max: T }

// Only for Float types
pub struct Round<T: Float> { precision: u8 }

// Only for Integer types  
pub struct Modulo<T: Integer> { divisor: T }
```

---

## Transform Organization

```rust
// src/transform.rs

pub mod string;
pub mod number;
pub mod vector;

pub use string::*;
pub use number::*;
pub use vector::*;

/// Base transform trait
pub trait Transform<T> {
    fn transform(&self, value: T) -> T;
}
```

```rust
// src/transform/number.rs

use crate::types::{Numeric, Float, Integer};

pub trait NumericTransform<T: Numeric>: Transform<T> {}

// Available for any Numeric
pub struct Clamp<T: Numeric> { pub min: T, pub max: T }
pub struct Abs<T: Numeric>;

// Only for Float
pub struct Round<T: Float> { pub precision: u8 }
pub struct Ceil<T: Float>;
pub struct Floor<T: Float>;

// Only for Integer
pub struct Modulo<T: Integer> { pub divisor: T }
```

```rust
// src/transform/string.rs

pub trait StringTransform: Transform<String> {}

pub struct Trim;
pub struct Lowercase;
pub struct Uppercase;
pub struct Truncate { pub max_len: usize }
pub struct Replace { pub from: String, pub to: String }
```

```rust
// src/transform/vector.rs

use crate::types::{Numeric, Float};

pub trait VectorTransform<T: Numeric, const N: usize>: Transform<[T; N]> {}

// Any Numeric
pub struct ClampComponents<T: Numeric> { pub min: T, pub max: T }

// Only Float
pub struct Normalize<T: Float>;
```

---

## Feature Flags

Conditional compilation per feature:

| Feature | Files Affected |
|---------|----------------|
| `validation` | `validation.rs`, validators in node files |
| `ui` | `ui_hints.rs`, ui_hints in node files |
| `display` | `display.rs` |
| `events` | `events.rs` |
| `serde` | serde impls in all type files |

**Pattern:**

```rust
// In node/number.rs

pub struct Number<T: Numeric, S: NumberSubtype<T>> {
    pub metadata: Metadata,
    pub subtype: S,
    pub default: Option<T>,
    
    #[cfg(feature = "validation")]
    pub validators: Vec<Arc<dyn NumericValidator<T>>>,
    
    #[cfg(feature = "ui")]
    pub ui_hints: NumberUiHints<T>,
}

#[cfg(feature = "serde")]
impl<T, S> serde::Serialize for Number<T, S>
where
    T: Numeric + serde::Serialize,
    S: NumberSubtype<T>,
{
    // ...
}
```

---

## Public API

`lib.rs` provides clean public interface:

```rust
//! Type-safe parameter definition system

pub mod error;
pub mod types;
pub mod value;
pub mod key;
pub mod metadata;
pub mod flags;
pub mod node;
pub mod subtype;
pub mod unit;
pub mod transform;
pub mod runtime;

#[cfg(feature = "validation")]
pub mod validation;

#[cfg(feature = "ui")]
pub mod ui_hints;

#[cfg(feature = "display")]
pub mod display;

#[cfg(feature = "events")]
pub mod events;

// Prelude for common imports
pub mod prelude {
    pub use crate::error::{Error, Result};
    pub use crate::types::{Numeric, Integer, Float, Textual};
    pub use crate::value::Value;
    pub use crate::key::Key;
    pub use crate::metadata::Metadata;
    pub use crate::flags::Flags;
    pub use crate::node::*;
    pub use crate::subtype::*;
    pub use crate::unit::NumberUnit;
}
```

**User imports:**

```rust
use paramdef::prelude::*;

let port = Port.builder("http_port").default(8080).build();
let color = ColorRgba.builder("tint").default([1.0, 1.0, 1.0, 1.0]).build();
```
