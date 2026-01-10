# paramdef - Implementation Roadmap

**Step-by-step guide to implementation**

Version: 1.2  
Status: Phase 6 Complete ✅

---

## Overview

This roadmap provides a structured approach to implementing paramdef, organized into phases with clear deliverables and dependencies.

**Total Estimated Effort:** 8-12 weeks  
**Team Size:** 1-2 developers  
**Prerequisites:** Rust 1.85+, familiarity with Arc, trait objects

---

## Current Progress

| Phase | Status | Description |
|-------|--------|-------------|
| Phase 1: Foundation | ✅ Complete | Core types, Key, Metadata, Flags, Value |
| Phase 2: Parameter Types | ✅ Complete | All 23 node types (Leaf, Container, Group, Decoration) |
| Phase 3: Schema & Runtime | ✅ Complete | Schema, Context, RuntimeNode, ErasedRuntimeNode |
| Phase 4.1: Event System | ✅ Complete | Event, EventBus, Subscription, Context integration |
| Phase 4.2: Validation | ✅ Complete | Hybrid Expr + Validator trait, built-in validators |
| Phase 4.3: Transformers | ✅ Complete | Hybrid Transform + Transformer trait, built-in transformers |
| Phase 4.4: Unified Expressions | ✅ Complete | ExprTarget + Rule, unified expr system, when() API |
| Phase 5: Visibility | ✅ Complete | Fluent when() API, Rule-based visibility |
| Phase 6: Polish | 🔄 In Progress | Documentation updates, examples polish |
| Phase 7: Expression Parser | 🔲 Future (v0.3.0) | Lexer + parser for string-based rules (2-3 days) |
| Phase 8: UI Integration | 🔲 Future | egui example |

---

## Phase 1: Foundation ✅

### Goal: Core infrastructure and basic types

**Status: COMPLETE**

### 1.1 Project Setup ✅

- [x] Create project structure
- [x] Set up Cargo.toml with dependencies
- [x] Configure CI/CD (GitHub Actions)
- [x] Set up documentation structure

**Files:**
```
paramdef/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── core/
│   │   ├── mod.rs
│   │   ├── key.rs
│   │   ├── metadata.rs
│   │   ├── flags.rs
│   │   ├── value/
│   │   └── error.rs
│   └── ...
```

### 1.2 Core Types ✅

- [x] Key - Parameter identifier with SmartString
- [x] Metadata - Label, description, group, tags
- [x] Flags - Schema-level bitflags (REQUIRED, READONLY, etc.)
- [x] StateFlags - Runtime state (DIRTY, TOUCHED, VALID)
- [x] Value - Unified runtime representation
- [x] Error - thiserror-based error types

### 1.3 Subtypes ✅

- [x] TextSubtype - 56 variants with helper methods
- [x] NumberSubtype - Type-safe with Integer/Float constraints via macro
- [x] VectorSubtype - Size-aware with component names
- [x] FileSubtype - File type categories
- [x] NumberUnit - 17 unit categories with conversion

---

## Phase 2: Parameter Types ✅

### Goal: Implement all 23 node types

**Status: COMPLETE**

### Node Hierarchy

| Category | Count | Types |
|----------|-------|-------|
| Group | 2 | Group, Panel |
| Decoration | 8 | Notice, Separator, Link, Code, Image, Html, Video, Progress |
| Container | 7 | Object, List, Mode, Matrix, Routing, Expirable, Reference |
| Leaf | 6 | Text, Number, Boolean, Vector, Select, File |

### 2.1 Node Trait ✅

```rust
pub trait Node: Send + Sync + Debug {
    fn key(&self) -> &Key;
    fn metadata(&self) -> &Metadata;
    fn flags(&self) -> Flags;
    fn node_kind(&self) -> NodeKind;
}
```

### 2.2 Leaf Types ✅

- [x] Text - String values with TextSubtype
- [x] Number - Numeric values with NumberSubtype + Unit
- [x] Boolean - True/false toggles
- [x] Vector - Fixed-size numeric arrays with VectorSubtype
- [x] Select - Single/multi selection with options
- [x] File - File uploads with FileSubtype

### 2.3 Container Types ✅

- [x] Object - Named field collection
- [x] List - Dynamic array with item template
- [x] Mode - Discriminated union (sum type)
- [x] Matrix - Table-based data entry
- [x] Routing - Connection/reference wrapper
- [x] Expirable - TTL-based wrapper
- [x] Reference - Template reference

### 2.4 Group Types ✅

- [x] Group - Root parameter group
- [x] Panel - UI organization panel

### 2.5 Decoration Types ✅

- [x] Notice - Info/warning/error messages
- [x] Separator - Visual dividers
- [x] Link - Clickable references
- [x] Code - Syntax-highlighted code
- [x] Image - Static images
- [x] Html - Rich HTML content
- [x] Video - Embedded video
- [x] Progress - Progress indicators

---

## Phase 3: Schema and Runtime ✅

### Goal: Schema definition and runtime state

**Status: COMPLETE**

### 3.1 Schema ✅

```rust
pub struct Schema {
    parameters: IndexMap<Key, Arc<dyn Node>>,
}

impl Schema {
    pub fn builder() -> SchemaBuilder;
    pub fn get(&self, key: &str) -> Option<&Arc<dyn Node>>;
    pub fn iter(&self) -> impl Iterator<Item = &Arc<dyn Node>>;
}
```

### 3.2 RuntimeNode ✅

```rust
pub struct RuntimeNode<T: Node> {
    node: Arc<T>,
    state: State,
}

pub struct State {
    value: Option<Value>,
    flags: StateFlags,
}
```

### 3.3 ErasedRuntimeNode ✅

Type-erased runtime node for heterogeneous collections.

### 3.4 Context ✅

```rust
pub struct Context {
    schema: Arc<Schema>,
    nodes: FxHashMap<Key, ErasedRuntimeNode>,
    #[cfg(feature = "events")]
    event_bus: Option<EventBus>,
}

impl Context {
    pub fn new(schema: Arc<Schema>) -> Self;
    pub fn with_event_bus(schema: Arc<Schema>, bus: EventBus) -> Self;
    pub fn get(&self, key: &str) -> Option<&Value>;
    pub fn set(&mut self, key: &str, value: Value) -> bool;
    pub fn touch(&mut self, key: &str) -> bool;
    pub fn batch<F, R>(&mut self, description: Option<impl Into<SmartStr>>, f: F) -> R;
}
```

---

## Phase 4: Reactive Systems (In Progress)

### Goal: Events, observers, validation, undo/redo

### 4.1 Event System ✅

**Status: COMPLETE**

**Files:**
```
src/event/
├── mod.rs      # Module exports and documentation
├── types.rs    # Event enum, ValidationError
└── bus.rs      # EventBus, Subscription, RecvError
```

**Event Types:**
```rust
pub enum Event {
    // Value events
    ValueChanging { key, old_value, new_value },
    ValueChanged { key, old_value, new_value },
    ValueCleared { key, old_value },
    
    // Validation events
    Validated { key, is_valid, errors },
    
    // State events
    Touched { key },
    Dirtied { key },
    Cleaned { key },
    Reset { key },
    
    // Batch events
    BatchBegin { id, description },
    BatchEnd { id },
    
    // Context events
    ContextReset,
    AllCleaned,
}
```

**EventBus:**
```rust
pub struct EventBus {
    tx: broadcast::Sender<Event>,
    batch_counter: AtomicU64,
}

impl EventBus {
    pub fn new(capacity: usize) -> Self;
    pub fn subscribe(&self) -> Subscription;
    pub fn emit(&self, event: Event) -> usize;
    pub fn batch<F, R>(&self, description: Option<impl Into<SmartStr>>, f: F) -> R;
}
```

**Industry Patterns Implemented:**
| Pattern | Source |
|---------|--------|
| `ValueChanging`/`ValueChanged` pair | SurveyJS |
| Batching with `BatchBegin`/`BatchEnd` | MobX transactions |
| `Touched` state tracking | Formik |
| RAII Subscription cleanup | MobX disposer |
| `tokio::broadcast` channel | Vue async queue |

---

### 4.2 Validation System ✅

**Status: COMPLETE**

**Files:**
```
src/validation/
├── mod.rs       # Module exports and documentation
├── context.rs   # ValidationContext, ValueAccess, NoValues
├── expr.rs      # Declarative Expr enum (~30 variants)
├── result.rs    # ValidationResult, ValidationOutcome, Error
├── rule.rs      # Rule enum (Expr + Fn), Rules collection
├── traits.rs    # Validator trait, FnValidator
└── validators.rs # Built-in validators
```

**Hybrid Design (Expr + Validator):**
```rust
/// Declarative validation (80% of cases)
pub enum Expr {
    Required,
    MinLength(usize), MaxLength(usize), Length(usize),
    Pattern(SmartStr), Email, Url, Uuid,
    Min(f64), Max(f64), ExclusiveMin(f64), ExclusiveMax(f64),
    Positive, Negative, NonNegative, Integer, MultipleOf(f64),
    MinItems(usize), MaxItems(usize), UniqueItems,
    OneOf(Vec<Value>), Const(Value),
    And(Vec<Expr>), Or(Vec<Expr>), Not(Box<Expr>),
    If { condition, then, otherwise },
    EqualTo(SmartStr), NotEqualTo(SmartStr),
    LessThan(SmartStr), GreaterThan(SmartStr),
    // ...
}

/// Programmatic validation (20% complex cases)
pub trait Validator: Send + Sync + Debug {
    fn validate(&self, value: &Value, ctx: &ValidationContext<'_>) -> ValidationResult;
}

/// Hybrid rule combining both approaches
pub enum Rule {
    Expr(Expr),           // Declarative, serializable
    Fn(Arc<dyn Validator>) // Flexible, Rust-native
}
```

**Built-in Validators:**
```rust
pub struct Required;
pub struct Length { min: Option<usize>, max: Option<usize> }
pub struct Range { min: Option<f64>, max: Option<f64>, exclusive_min: bool, exclusive_max: bool }
pub struct Match { other_key: SmartStr }
pub struct PasswordStrength { min_length, require_uppercase, require_lowercase, require_digit, require_special }
pub struct When<V: Validator> { condition_key, expected_value, then_validator }
```

**Industry Patterns Implemented:**
| Pattern | Source |
|---------|--------|
| Declarative expressions | JSON Schema, Zod |
| Cross-field validation | Yup `.when()` |
| Resolver pattern | React Hook Form |
| Expression language | CEL (Kubernetes) |
| Thread-local regex cache | Performance optimization |

---

### 4.3 Transformer System ✅

**Status: COMPLETE**

**Files:**
```
src/transform/
├── mod.rs         # Module exports and documentation
├── expr.rs        # Declarative Transform enum (~20 variants)
├── traits.rs      # Transformer trait, FnTransformer
├── transforms.rs  # Transforms collection for chaining
└── transformers.rs # Built-in struct transformers
```

**Hybrid Design (Transform + Transformer):**
```rust
/// Declarative transformation (80% of cases)
pub enum Transform {
    // String transformations
    Trim, TrimStart, TrimEnd,
    Lowercase, Uppercase, Capitalize,
    CollapseWhitespace, RemoveWhitespace,
    Replace { from, to }, Truncate { max_length, suffix },
    Pad { min_length, char, start },
    
    // Numeric transformations
    Abs, Ceil, Floor, Round, RoundTo { decimals },
    Clamp { min, max },
    
    // Null handling
    DefaultTo { value }, NullIf { value }, NullIfEmpty,
    
    // Composition
    Sequence(Vec<Transform>),
}

/// Programmatic transformation (20% complex cases)
pub trait Transformer: Send + Sync {
    fn name(&self) -> &'static str;
    fn transform(&self, value: &Value) -> Value;
}

/// Collection for chaining transformations
pub struct Transforms {
    transforms: Vec<TransformItem>,
}
```

**Built-in Transformers:**
```rust
pub struct Clamp { min: f64, max: f64 }
pub struct Round { decimals: u32 }
pub struct Truncate { max_length: usize, suffix: Option<String> }
pub struct Replace { from: String, to: String }
pub struct Default { value: Value }
```

**Industry Patterns Implemented:**
| Pattern | Source |
|---------|--------|
| `parse`/`format` pipeline | React Final Form |
| Method chaining | Express Validator |
| Normalize before validate | OWASP Guidelines |
| Idempotent transformations | Functional programming |

---

### 4.4 History System (Undo/Redo) ✅

**Status: COMPLETE**

**Files:**
```
src/history/
├── mod.rs       # Module exports and documentation
├── command.rs   # Command trait, CommandResult
├── manager.rs   # HistoryManager with undo/redo stacks
└── commands.rs  # Built-in commands (SetValue, Clear, Touch, Macro)
```

**Command Trait:**
```rust
pub trait Command: Send + Sync + Debug {
    fn as_any(&self) -> &dyn Any;
    fn execute(&mut self, ctx: &mut Context) -> CommandResult;
    fn undo(&mut self, ctx: &mut Context) -> CommandResult;
    fn redo(&mut self, ctx: &mut Context) -> CommandResult;
    fn merge(&mut self, other: &dyn Command) -> bool;
    fn can_merge_with(&self, other: &dyn Command) -> bool;
    fn description(&self) -> &str;
}
```

**HistoryManager:**
```rust
pub struct HistoryManager {
    undo_stack: VecDeque<Box<dyn Command>>,
    redo_stack: VecDeque<Box<dyn Command>>,
    max_history: usize,
    enable_merging: bool,
}

impl HistoryManager {
    pub fn new() -> Self;
    pub fn execute<C: Command + 'static>(&mut self, cmd: C, ctx: &mut Context) -> CommandResult;
    pub fn undo(&mut self, ctx: &mut Context) -> CommandResult;
    pub fn redo(&mut self, ctx: &mut Context) -> CommandResult;
    pub fn can_undo(&self) -> bool;
    pub fn can_redo(&self) -> bool;
}
```

**Built-in Commands:**
```rust
pub struct SetValueCommand { key, old_value, new_value }  // Supports merging
pub struct ClearValueCommand { key, old_value }
pub struct TouchCommand { key, was_touched }
pub struct MacroCommand { commands: Vec<Box<dyn Command>>, description }
```

**Industry Patterns Implemented:**
| Pattern | Source |
|---------|--------|
| Command pattern | Qt Undo Framework |
| Command merging | Photoshop |
| Transaction grouping | Text Editors |
| Memory-efficient deltas | Game Engines (100 bytes vs 10KB snapshots) |

---

## Phase 5: Visibility System ✅

### Goal: Conditional visibility

**Status: COMPLETE**

**Files:**
```
src/visibility/
├── mod.rs       # Module exports and documentation
└── expr.rs      # Expr enum with evaluation logic
```

**Expr Enum:**
```rust
pub enum Expr {
    // Value comparisons
    Eq(Key, Value), Ne(Key, Value),
    Lt(Key, f64), Gt(Key, f64), Lte(Key, f64), Gte(Key, f64),
    
    // State checks
    IsSet(Key), IsEmpty(Key), IsTrue(Key), IsFalse(Key), IsValid(Key),
    
    // Collection operations
    OneOf(Key, Arc<[Value]>), Contains(Key, Value),
    
    // Logical operators
    And(Arc<[Expr]>), Or(Arc<[Expr]>), Not(Box<Expr>),
}

impl Expr {
    pub fn eval(&self, ctx: &Context) -> bool;
    pub fn dependencies(&self) -> Vec<Key>;
    
    // Builder methods: eq(), ne(), is_true(), and(), or(), negate(), etc.
}
```

**Key Features:**
- ✅ 15 expression types covering all common visibility needs
- ✅ Type-safe evaluation with graceful fallback (returns false on mismatch)
- ✅ Automatic dependency tracking via `dependencies()`
- ✅ Composable with `And`, `Or`, `Not` operators
- ✅ Builder methods for ergonomic construction
- ✅ Serializable with serde feature
- ✅ 13 comprehensive tests

**Industry Patterns Implemented:**
| Pattern | Source |
|---------|--------|
| Conditional schemas | JSON Schema |
| Field dependencies | React Hook Form |
| Dynamic form controls | Angular Forms |
| Type-safe evaluation | TypeScript strict mode |

**Example:**
```rust
use paramdef::visibility::Expr;

// Simple: show if premium
let expr = Expr::is_true("premium");

// Complex: (premium AND age >= 18) OR admin
let expr = Expr::or(vec![
    Expr::and(vec![
        Expr::is_true("premium"),
        Expr::gte("age", 18.0),
    ]),
    Expr::is_true("admin"),
]);

// Evaluate
assert_eq!(expr.eval(&ctx), true);

// Get dependencies
let deps = expr.dependencies(); // ["premium", "age", "admin"]
```

---

## Phase 6: Polish and Optimization ✅

### Goal: Performance, docs, examples

**Status: COMPLETE**

- [x] Benchmark infrastructure (criterion with 6 benchmark suites)
- [x] Fix serde support for Expr (Arc<[T]> serialization)
- [x] Complete API documentation (zero warnings)
- [x] Create 10+ examples (10 examples covering all major features)
- [ ] Property-based tests (proptest) - deferred to Phase 7+

**Completed Work:**
- Added arc_slice_serde helper for Arc<[T]> serialization
- Fixed all documentation warnings
- Created 10 comprehensive examples demonstrating core functionality
- All examples compile and run successfully
- Zero clippy warnings, zero documentation warnings

---

## Phase 7: UI Integration 🔲

### Goal: egui integration example

**Status: OPTIONAL**

---

## Feature Flags

| Feature | Status | Description |
|---------|--------|-------------|
| `default` | ✅ | Core types only |
| `serde` | ✅ | Serialization + JSON |
| `events` | ✅ | Event system with tokio |
| `visibility` | 🔲 | Visibility expressions |
| `validation` | ✅ | Validation system with Expr + Validator |
| `i18n` | 🔲 | Fluent localization |
| `chrono` | ✅ | Chrono type conversions |
| `full` | 🔲 | All features |

---

## Code Statistics

| Metric | Value |
|--------|-------|
| Lines of Code | ~16,700 |
| Node Types | 23 |
| Test Coverage | 90%+ |
| Clippy Warnings | 0 |

---

## Next Steps

1. **Phase 6: Polish (In Progress)**
   - ✅ Unified expression system complete
   - ✅ Fluent when() API for visibility
   - 🔄 Documentation updates in progress
   - 🔲 Additional examples

2. **Phase 7: Expression Parser (Future, v0.3.0)**
   - String-based expression parsing
   - Lexer + recursive descent parser
   - Support for config files (TOML, JSON, YAML)
   - **Estimated effort:** 2-3 days
   - See: `docs/23-EXPRESSION-PARSER.md`

3. **Phase 8: UI Integration (Future)**
   - egui example implementation
   - Form builder patterns
   - Reactive UI bindings

4. **Future Enhancements**
   - Cross-field validation improvements
   - History/undo system (Command pattern)
   - Performance benchmarks
