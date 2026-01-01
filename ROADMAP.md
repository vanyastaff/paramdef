# paramdef - Version Roadmap

Structured roadmap organized by semantic versions with clear deliverables.

---

## v0.2.0 - Foundation (Current)

**Status:** ✅ Released

Core types and basic parameter system.

### Completed Features

- **Core Infrastructure**
  - Key, Metadata, Flags, StateFlags
  - Value enum with all primitive types
  - Error types with thiserror

- **Schema Layer**
  - Schema builder and container
  - Immutable parameter definitions via Arc

- **Runtime Layer**
  - RuntimeNode with type erasure
  - State management (dirty, touched, valid)
  - Context for parameter instances

- **Type System (24 types)**
  - Group: Group, Panel
  - Leaf: Text, Number, Boolean, Vector, Select, File
  - Container: Object, List, Mode, Matrix, Routing, Expirable, Reference
  - Decoration: Notice, Separator, Link, Code, Image, Html, Video, Progress

- **Subtype System**
  - TextSubtype: 45+ variants (Email, URL, Password, JSON, Markdown, etc.)
  - NumberSubtype: 30+ variants (Port, Percentage, Temperature, etc.)
  - VectorSubtype: 20+ variants (Position3D, ColorRgba, Quaternion, etc.)
  - FileSubtype: 17 variants (Image, PDF, Audio, Video, etc.)
  - NumberUnit: 17 categories with conversions

- **Container Features**
  - Object: extensible mode (additionalProperties)
  - List: rankable mode for priority ordering
  - Matrix: table-based data entry (rows × columns)

---

## v0.3.0 - Visibility & Expressions

**Status:** 🔜 Next

Conditional visibility and expression system.

### Planned Features

- **Visibility Expression (Expr)**
  ```rust
  pub enum Expr {
      Eq(Key, Value),        // key == value
      Ne(Key, Value),        // key != value
      Lt(Key, f64),          // key < value
      Gt(Key, f64),          // key > value
      IsSet(Key),            // key is not null
      IsEmpty(Key),          // "", [], {}
      IsTrue(Key),           // key == true
      OneOf(Key, Vec<Value>), // key in [...]
      And(Vec<Expr>),        // all must be true
      Or(Vec<Expr>),         // any must be true
      Not(Box<Expr>),        // invert
  }
  ```

- **Expression Evaluation**
  - `Expr::eval(&self, ctx: &Context) -> bool`
  - `Expr::dependencies() -> Vec<Key>`
  - Lazy compilation and caching

- **Visibility Integration**
  - Add `visibility: Option<Expr>` to all parameter types
  - Runtime visibility state in Context

### Feature Flag
```toml
[features]
visibility = []
```

---

## v0.4.0 - Event System

**Status:** 📋 Planned

Reactive event system with observers.

### Planned Features

- **Parameter Events**
  ```rust
  pub enum ParameterEvent {
      ValueChanging { key: Key, old: Value, new: Value },
      ValueChanged { key: Key, old: Value, new: Value },
      Validated { key: Key, is_valid: bool, errors: Vec<Error> },
      VisibilityChanged { key: Key, visible: bool },
      BatchBegin { description: String },
      BatchEnd { description: String },
  }
  ```

- **Event Bus**
  - Based on `tokio::broadcast`
  - Multiple subscribers support
  - Batch mode for grouping related changes

- **Built-in Observers**
  - LoggerObserver - debug logging
  - ValidationObserver - auto-validate on change
  - VisibilityObserver - reactive visibility updates

### Feature Flag
```toml
[features]
events = ["dep:tokio"]
```

---

## v0.5.0 - Computed & Triggers

**Status:** 📋 Planned

Computed values and trigger system.

### Planned Features

- **Computed Expressions**
  ```rust
  Text::builder("full_name")
      .computed("{first_name} + ' ' + {last_name}")
      .build()

  Number::builder("total")
      .computed("{price} * {quantity}")
      .build()
  ```

- **Trigger System**
  ```rust
  Trigger::set_value()
      .when(Expr::Eq("country", "US"))
      .set("currency", "USD")

  Trigger::copy_value()
      .when(Expr::IsTrue("same_address"))
      .from("billing_address")
      .to("shipping_address")

  Trigger::clear_value()
      .when(Expr::Ne("type", "custom"))
      .clear("custom_value")
  ```

- **Dependency Graph**
  - Automatic dependency tracking
  - Cycle detection
  - Topological sort for evaluation order

### Dependencies
- Requires: v0.3.0 (Visibility), v0.4.0 (Events)

---

## v0.6.0 - Validation System

**Status:** 📋 Planned

Comprehensive validation with built-in validators.

### Planned Features

- **Validator Trait**
  ```rust
  pub trait Validator: Send + Sync {
      fn validate(&self, value: &Value) -> Result<(), ValidationError>;
      fn name(&self) -> &'static str;
  }
  ```

- **Built-in Validators**
  - RequiredValidator
  - MinLengthValidator, MaxLengthValidator
  - RangeValidator (min, max)
  - PatternValidator (regex)
  - EmailValidator, UrlValidator
  - CustomValidator (closure-based)

- **Validation Modes**
  - Immediate: validate on every change
  - OnBlur: validate when field loses focus
  - OnSubmit: validate only on form submit
  - Manual: validate only when explicitly called

- **Cross-Field Validation**
  ```rust
  Validator::cross_field()
      .fields(["password", "confirm_password"])
      .rule(|values| values[0] == values[1])
      .message("Passwords must match")
  ```

### Feature Flag
```toml
[features]
validation = []
```

---

## v0.7.0 - Transformers

**Status:** 📋 Planned

Value transformation pipeline.

### Planned Features

- **Transformer Trait**
  ```rust
  pub trait Transformer: Send + Sync {
      fn transform(&self, value: Value) -> Value;
      fn name(&self) -> &'static str;
  }
  ```

- **Built-in Transformers**
  - TrimTransformer
  - LowercaseTransformer, UppercaseTransformer
  - StripWhitespaceTransformer
  - NormalizeUnicodeTransformer
  - SanitizeHtmlTransformer
  - NumberFormatTransformer

- **Transform Pipeline**
  ```rust
  Text::builder("username")
      .transform(TrimTransformer)
      .transform(LowercaseTransformer)
      .build()
  ```

---

## v0.8.0 - History & Undo/Redo

**Status:** 📋 Planned

Command-based history system.

### Planned Features

- **Command Pattern**
  ```rust
  pub trait Command: Send + Sync {
      fn execute(&mut self, ctx: &mut Context) -> Result<()>;
      fn undo(&mut self, ctx: &mut Context) -> Result<()>;
      fn redo(&mut self, ctx: &mut Context) -> Result<()>;
      fn merge(&mut self, other: &dyn Command) -> bool;
      fn description(&self) -> &str;
  }
  ```

- **Built-in Commands**
  - SetValueCommand
  - ClearValueCommand
  - BatchCommand (transactions)

- **History Manager**
  ```rust
  pub struct HistoryManager {
      undo_stack: VecDeque<Box<dyn Command>>,
      redo_stack: VecDeque<Box<dyn Command>>,
      max_history: usize,
  }
  ```

- **Transaction Support**
  ```rust
  ctx.begin_transaction("Update user");
  ctx.set_value("name", "John");
  ctx.set_value("email", "john@example.com");
  ctx.commit_transaction(); // Single undo step
  ```

### Dependencies
- Requires: v0.4.0 (Events)

---

## v0.9.0 - Serialization

**Status:** 📋 Planned

Full serialization support.

### Planned Features

- **Schema Serialization**
  - Serialize/Deserialize for all schema types
  - JSON Schema export
  - TypeScript type generation

- **Value Serialization**
  - Custom date/time formats
  - Binary encoding options
  - Streaming support for large schemas

- **Import/Export**
  ```rust
  // Export to JSON Schema
  let json_schema = schema.to_json_schema()?;

  // Export to TypeScript
  let ts_types = schema.to_typescript()?;

  // Import from JSON
  let schema = Schema::from_json(json)?;
  ```

### Feature Flag
```toml
[features]
serde = ["dep:serde", "dep:serde_json"]
```

---

## v1.0.0 - Production Ready

**Status:** 📋 Planned

Stable API with full documentation.

### Requirements

- All features complete and tested
- 90%+ test coverage
- Complete API documentation
- Performance benchmarks
- Migration guide from v0.x
- Examples for common use cases

### Stability Guarantees

- Semantic versioning
- Deprecation warnings before removal
- MSRV policy (minimum 1.85)

---

## v1.1.0 - I18n Support

**Status:** 📋 Future

Internationalization with Fluent.

### Planned Features

- **Fluent Integration**
  ```rust
  Text::builder("name")
      .label_fluent("field-name-label")
      .description_fluent("field-name-description")
      .build()
  ```

- **User-Managed Translations**
  - Library provides Fluent keys
  - Users provide translation files
  - No embedded translations (zero bloat)

### Feature Flag
```toml
[features]
i18n = ["dep:fluent"]
```

---

## v1.2.0 - Async Validation

**Status:** 📋 Future

Async validators for remote validation.

### Planned Features

- **Async Validator Trait**
  ```rust
  #[async_trait]
  pub trait AsyncValidator: Send + Sync {
      async fn validate(&self, value: &Value) -> Result<(), ValidationError>;
  }
  ```

- **Use Cases**
  - Username availability check
  - Email verification
  - Remote API validation
  - Database lookups

---

## v2.0.0 - Next Generation

**Status:** 🔮 Future

Major improvements and breaking changes.

### Potential Features

- WebAssembly support
- Plugin system for custom types
- Visual schema editor export
- GraphQL schema generation
- React/Vue component generation

---

## Version Dependencies

```
v0.2.0 (Foundation)
    │
    ├── v0.3.0 (Visibility)
    │       │
    │       └── v0.5.0 (Computed/Triggers)
    │
    ├── v0.4.0 (Events)
    │       │
    │       ├── v0.5.0 (Computed/Triggers)
    │       │
    │       └── v0.8.0 (History)
    │
    ├── v0.6.0 (Validation)
    │
    ├── v0.7.0 (Transformers)
    │
    └── v0.9.0 (Serialization)
            │
            └── v1.0.0 (Production Ready)
                    │
                    ├── v1.1.0 (I18n)
                    │
                    └── v1.2.0 (Async Validation)
```

---

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for contribution guidelines.

Each version has a corresponding GitHub milestone for tracking issues and PRs.
