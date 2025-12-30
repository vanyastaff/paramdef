# paramdef - Final Design Decisions

**Key architectural decisions with rationale**

---

## Table of Contents

1. [Subtype System Decisions](#subtype-system-decisions)
2. [Type System Decisions](#type-system-decisions)
3. [Runtime Architecture](#runtime-architecture)
4. [Event System](#event-system)
5. [Integration Decisions](#integration-decisions)
6. [Rejected Alternatives](#rejected-alternatives)

---

## Subtype System Decisions

### ✅ DECISION: Create TextSubtype, NumberSubtype, VectorSubtype

**Rationale:**
- Industry standard (Blender RNA, Unreal Engine)
- Semantic meaning is valuable
- Enables proper validation
- UI hints without coupling
- 150+ real-world use cases identified

**Evidence:**
- Blender: FloatProperty has 15+ subtypes (FACTOR, ANGLE, DISTANCE, etc.)
- Unreal: UPROPERTY has metadata specifiers (ClampMin, UIMin, etc.)
- Qt: QVariant has type system with semantic meaning

**Impact:**
- Better DX (Developer Experience)
- Type-safe validation
- Clear documentation
- Proper UI rendering hints

---

### ❌ DECISION: NO BooleanSubtype

**Rationale:**
- Blender RNA does NOT have Boolean subtypes
- Boolean is too simple (only 2 states)
- UI variations handled by layout, not property type
- Semantic meaning in naming conventions (`show_`, `use_`, `is_`)

**Evidence from Blender:**
```python
# Blender uses naming, not subtypes
show_wireframe = BoolProperty()    # "show_" prefix
use_smooth = BoolProperty()        # "use_" prefix
hide_viewport = BoolProperty()     # "hide_" prefix

# UI control via layout, not property
layout.prop(obj, "show_name", toggle=True)  # Toggle button
layout.prop(obj, "show_name")               # Checkbox
```

**Alternative Approach:**
- Use naming conventions
- UI hints in metadata
- Keep Boolean simple

**Impact:**
- Simpler API
- Industry-standard approach
- Less code to maintain
- Follows KISS principle

---

### ❌ DECISION: NO ChoiceSubtype

**Rationale:**
- YAGNI (You Ain't Gonna Need It)
- Choice already has `mode` (Single/Multi)
- UI variations (Dropdown vs Radio vs Tags) are presentation, not semantics
- Can be added later without breaking changes

**Why NOT needed:**
```rust
// Choice already has semantic distinction
pub enum ChoiceMode {
    Single,    // Single selection
    Multi,     // Multiple selection
}

// UI variations in metadata, not subtype
Select::builder("color")
    .mode(ChoiceMode::Single)
    .with_ui_hint(UIHint::Dropdown)  // ← Metadata, not subtype
    .build()
```

**Impact:**
- Simpler API
- Less code
- Can add later if needed

---

### 📊 DECISION: Helper Methods Over Category Enum

**Question:** Should we have `TextCategory` enum?

**Decision:** NO, use helper methods instead.

**Rationale:**
```rust
// ❌ BAD: Category enum
pub enum TextCategory {
    Code, Web, FileSystem, ...
}

impl TextSubtype {
    pub fn category(&self) -> TextCategory {
        match self {
            Self::Json | Self::Xml => TextCategory::Code,
            // ... 50+ lines of mapping
        }
    }
}

// ✅ GOOD: Helper methods
impl TextSubtype {
    pub fn is_code(&self) -> bool {
        matches!(self, Self::Json | Self::Xml | Self::Code | ...)
    }
    
    pub fn is_structured(&self) -> bool {
        matches!(self, Self::Json | Self::Xml | Self::Yaml | Self::Toml)
    }
}
```

**Benefits:**
- More flexible (one subtype can be in multiple "categories")
- Less code (no big match statement)
- More discoverable (autocomplete shows `is_*` methods)
- KISS principle

---

## Type System Decisions

### ✅ DECISION: Runtime Vector Size (NOT Const Generics)

**Question:** Should we use `Vector<const N: usize>`?

**Decision:** NO, use runtime size.

**Rationale:**

#### Problem 1: Schema Cannot Be Generic
```rust
// ❌ IMPOSSIBLE:
pub struct Schema {
    // Cannot store different N in same collection!
    parameters: Vec<Arc<Vector<???>>>,
}

// ✅ REQUIRED:
pub struct Schema {
    parameters: Vec<Arc<dyn Parameter>>,  // Type erasure
}
```

**Type erasure kills const generics benefits!**

#### Problem 2: VectorSubtype Already Encodes Size
```rust
impl VectorSubtype {
    pub fn component_count(&self) -> Option<usize> {
        match self {
            Self::Vector2 => Some(2),
            Self::Vector3 => Some(3),
            Self::Vector4 => Some(4),
            Self::ColorRgb => Some(3),
            Self::ColorRgba => Some(4),
            Self::Matrix4x4 => Some(16),
        }
    }
}
```

**Having BOTH const generic AND subtype is redundant!**

#### Industry Evidence:

**Blender:**
```python
# Runtime size parameter
location = FloatVectorProperty(size=3)
color = FloatVectorProperty(size=4)
matrix = FloatVectorProperty(size=16)
```

**Unreal Engine:**
```cpp
// Specific types, but system uses type erasure
FVector Location;       // 3 components
FVector4 Tangent;       // 4 components
// All stored as UProperty* (type-erased)
```

#### Solution: Type-Safe API Without Generics

```rust
// Type-safe builders
let position = Vector::vector3("position")
    .default_vec3([0.0, 0.0, 0.0])  // ✅ Enforces [f64; 3]
    .build();

// Type-safe getters
let pos: [f64; 3] = context.get_vec3("position")?;  // ✅ Type-safe

// Runtime validation
context.set_vec3("position", [1.0, 2.0, 3.0])?;  // ✅ Validated
```

**Benefits:**
- ✅ Type safety where it matters (builders, getters)
- ✅ Flexibility where needed (schema storage)
- ✅ Industry standard approach
- ✅ Simple API (no generic explosion)

---

### ✅ DECISION: Separate Unit System

**Question:** Should units be part of NumberSubtype?

**Decision:** NO, separate `Unit` enum.

**Rationale:**

#### Problem: Subtype Explosion
```rust
// ❌ BAD: Units in subtype
pub enum NumberSubtype {
    DistanceInMeters,
    DistanceInKilometers,
    DistanceInMiles,
    TemperatureInCelsius,
    TemperatureInFahrenheit,
    // ... 500+ combinations!
}

// ✅ GOOD: Separate concerns
pub enum NumberSubtype {
    Distance,      // What it represents
    Temperature,
    // ... ~60 subtypes
}

pub enum Unit {
    Distance(DistanceUnit),  // How it's measured
    Temperature(TemperatureUnit),
    // ... ~17 categories
}
```

**Benefits:**
- Separation of concerns (WHAT vs HOW)
- 60 subtypes × 17 unit categories = manageable
- Unit conversion built-in
- User can choose preferred units

---

## Runtime Architecture

### ✅ DECISION: Three-Layer Architecture

**Layers:**
1. **Schema** - Immutable definitions (Arc-shared)
2. **Runtime** - Mutable state (per-instance)
3. **Context** - Orchestration (manages collection)

**Rationale:**
- Clean separation of concerns
- Memory efficient (Arc sharing)
- Blender RNA proven pattern
- Enables undo/redo
- Supports reactive updates

**Implementation:**
```rust
// Layer 1: Schema (immutable)
pub struct Text {
    metadata: Metadata,
    flags: Flags,
    validators: Vec<Validator>,
}

// Layer 2: Runtime (mutable)
pub struct RuntimeParameter<T> {
    schema: Arc<T>,              // ← Shared
    state: ParameterState,       // ← Owned
    value: Value,                // ← Owned
}

// Layer 3: Context (orchestration)
pub struct Context {
    schema: Arc<Schema>,
    parameters: HashMap<Key, RuntimeParameter>,
    event_bus: EventBus,
    history: HistoryManager,
}
```

---

### ✅ DECISION: RuntimeParameter<T> Generic Pattern

**Question:** Should RuntimeParameter be generic over parameter type?

**Decision:** YES, generic is best.

**Rationale:**

**Alternative 1: Trait Object (NO)**
```rust
// ❌ BAD: Loses type information
pub struct RuntimeParameter {
    schema: Arc<dyn Parameter>,  // Type-erased
}
```

**Alternative 2: Enum (NO)**
```rust
// ❌ BAD: Exhaustive matching everywhere
pub enum RuntimeParameter {
    Text(RuntimeText),
    Number(RuntimeNumber),
    // ...
}
```

**Chosen: Generic (YES)**
```rust
// ✅ GOOD: Type-safe + flexible
pub struct RuntimeParameter<T: Parameter> {
    schema: Arc<T>,
    state: ParameterState,
    value: Value,
}

impl RuntimeParameter<Text> {
    // Type-specific methods
    pub fn as_text(&self) -> Option<&str>;
}

impl RuntimeParameter<Number> {
    // Type-specific methods
    pub fn as_f64(&self) -> Option<f64>;
}
```

**Benefits:**
- Type-safe access
- Type-specific methods
- No exhaustive matching
- Extensible

---

## Event System

### ✅ DECISION: tokio::broadcast for EventBus

**Question:** Custom EventBus or library?

**Decision:** Use `tokio::broadcast`.

**Rationale:**

**Options Evaluated:**
1. **tokio::broadcast** - 5M events/sec, async+sync ✅
2. **flume** - 8M events/sec, modern API
3. **crossbeam-channel** - 10M events/sec, sync-only
4. **custom** - Full control, high maintenance

**Decision Matrix:**
- Already have tokio in dependencies ✅
- Async + sync support needed ✅
- Battle-tested by Tokio team ✅
- Multiple subscribers built-in ✅
- Backpressure handling ✅

**Usage:**
```rust
pub struct EventBus {
    tx: broadcast::Sender<ParameterEvent>,
    observers: Mutex<HashMap<SubscriptionId, Box<dyn Observer>>>,
}

// Sync callback observers
event_bus.subscribe(LoggerObserver::new());

// Async receivers
let mut rx = event_bus.receiver();
tokio::spawn(async move {
    while let Ok(event) = rx.recv().await {
        update_ui(&event).await;
    }
});
```

**Separation:**
- `tokio::broadcast` → Events
- `crossbeam` → Work queues, lock-free structures

---

### ✅ DECISION: Command Pattern for Undo/Redo

**Question:** Undo/Redo strategy?

**Options:**
1. **Command Pattern** - Low memory, fast, extensible ✅
2. **Snapshot Pattern** - High memory, simple
3. **Event Sourcing** - Complex, audit trail
4. **Diff-based** - Complex, slow

**Decision:** Command Pattern.

**Rationale:**
- Industry standard (Photoshop, Blender, VS Code)
- Memory efficient (only stores changes)
- Supports command merging
- Extensible (custom commands)
- Enables transactions (MacroCommand)

**Implementation:**
```rust
pub trait Command: Send + Sync {
    fn execute(&mut self, ctx: &mut Context) -> Result<()>;
    fn undo(&mut self, ctx: &mut Context) -> Result<()>;
    fn redo(&mut self, ctx: &mut Context) -> Result<()>;
    fn merge(&mut self, other: &dyn Command) -> bool;  // Optimization!
}

// Command merging example:
// Type "hello" character by character → merged into 1 command
// Not 5 separate commands for 'h', 'e', 'l', 'l', 'o'
```

**Benefits:**
- ~100 bytes per command vs ~10KB per snapshot
- Efficient for large schemas
- Natural transaction support
- Can serialize history

---

## Integration Decisions

### ✅ DECISION: Mozilla Fluent for Localization

**Question:** Custom localization or library?

**Decision:** Use Mozilla Fluent.

**Rationale:**
- Industry standard (Firefox, Thunderbird)
- Battle-tested
- Powerful (pluralization, gender, etc.)
- Don't reinvent the wheel
- Better than building custom

**Integration:**
```rust
// In metadata
pub struct Metadata {
    key: Key,
    label_fluent_key: Option<String>,
    description_fluent_key: Option<String>,
}

// Usage
parameter.label_fluent_key = Some("param-email-label");
// → Resolved: "Email Address" (en), "Adresse e-mail" (fr)
```

---

### ✅ DECISION: Display System Integrated with Events

**Question:** Separate display system or integrated?

**Decision:** Integrated with events (reactive).

**Rationale:**
- Automatic updates when values change
- No manual refresh needed
- Efficient (only recalculates affected parameters)
- Industry pattern (Qt signals/slots, React)

**Implementation:**
```rust
impl DisplayObserver {
    fn on_event(&mut self, event: &ParameterEvent) {
        match event {
            ValueChanged { key, new_value, .. } => {
                // 1. Update context
                self.context.insert(key, new_value);
                
                // 2. Find dependent parameters
                if let Some(deps) = self.dependencies.get(key) {
                    // 3. Recalculate visibility
                    for dep in deps {
                        let visible = self.should_show(dep);
                        emit(VisibilityChanged { dep, visible });
                    }
                }
            }
        }
    }
}
```

**Benefits:**
- Reactive updates
- Efficient (only affected params)
- Declarative rules
- Type-safe

---

## Rejected Alternatives

### ❌ REJECTED: Subtype in Value Enum

**Proposal:** Store subtype in Value enum.

```rust
// ❌ REJECTED
pub enum Value {
    Text { value: String, subtype: TextSubtype },
    Number { value: f64, subtype: NumberSubtype },
}
```

**Why Rejected:**
- Subtype belongs to schema, not value
- Value should be data-only
- Violates separation of concerns
- Schema already has subtype
- Runtime overhead

**Correct Design:**
```rust
// ✅ CORRECT
pub enum Value {
    Text(String),
    Number(f64),
}

pub struct Text {
    subtype: TextSubtype,  // ← In schema
}
```

---

### ❌ REJECTED: Validation in Value

**Proposal:** Values self-validate.

```rust
// ❌ REJECTED
impl Value {
    pub fn validate(&self) -> ValidationResult<()>;
}
```

**Why Rejected:**
- Value doesn't know its context
- Validation rules in schema, not value
- Different parameters can have different validation
- Separation of concerns

**Correct Design:**
```rust
// ✅ CORRECT
impl Text {
    pub fn validate(&self, value: &str) -> ValidationResult<()> {
        for validator in &self.validators {
            validator.validate(value)?;
        }
        Ok(())
    }
}
```

---

### ❌ REJECTED: UI Coupling in Core

**Proposal:** UI widgets in parameter definitions.

```rust
// ❌ REJECTED
pub struct Text {
    widget: Widget,  // egui::TextEdit, iced::TextInput, etc.
}
```

**Why Rejected:**
- Core should be UI-agnostic
- Multiple UI frameworks need support
- Headless use cases (CLI, server)
- Violates separation of concerns

**Correct Design:**
```rust
// ✅ CORRECT: UI hints, not widgets
pub struct Text {
    ui_hints: Option<UIHints>,  // Optional, feature-gated
}

pub struct UIHints {
    widget_type: WidgetType,    // Hint, not implementation
    placeholder: Option<String>,
}
```

---

### ❌ REJECTED: Multiple Inheritance / Mixins

**Proposal:** Parameters inherit from base classes.

```rust
// ❌ REJECTED
pub trait Validatable {
    fn validate(&self, value: &Value) -> ValidationResult<()>;
}

pub trait Transformable {
    fn transform(&self, value: Value) -> Value;
}

pub struct Text: Validatable + Transformable + ... {
    // Multiple trait implementations
}
```

**Why Rejected:**
- Rust doesn't have inheritance
- Trait combinatorics explosion
- Complex API surface
- Hard to understand

**Correct Design:**
```rust
// ✅ CORRECT: Composition
pub struct Text {
    validators: Vec<Arc<dyn Validator>>,      // Composition
    transformers: Vec<Arc<dyn Transformer>>,  // Composition
}
```

---

## Summary

### Key Decisions

| Decision | Rationale | Impact |
|----------|-----------|--------|
| TextSubtype, NumberSubtype, VectorSubtype | Industry standard, semantic value | +150 subtypes |
| NO BooleanSubtype | Too simple, Blender doesn't use | Simpler API |
| NO ChoiceSubtype | YAGNI, can add later | Less code |
| Helper methods over Category | More flexible, KISS | Better DX |
| Runtime vector size | Schema storage, industry standard | Type-safe API |
| Separate Unit system | Separation of concerns | Clean design |
| Three-layer architecture | Proven pattern, memory efficient | Maintainable |
| RuntimeParameter<T> generic | Type-safe, extensible | Better API |
| tokio::broadcast for events | Already in deps, battle-tested | Reliable |
| Command pattern for undo/redo | Industry standard, efficient | Memory efficient |
| Mozilla Fluent for i18n | Don't reinvent, proven | Less work |
| Reactive display system | Automatic updates, efficient | Great UX |

### Design Philosophy

1. **Industry Standards First** - Follow Blender, Unreal, Qt
2. **KISS Principle** - Simple solutions over complex
3. **YAGNI** - Don't build what you don't need
4. **Separation of Concerns** - Schema vs Runtime vs UI
5. **Type Safety** - Where it matters, with runtime flexibility
6. **Zero-Cost** - Efficient abstractions, Arc sharing

---

## Additional Decisions (Review Session)

### ✅ DECISION: Schema as Group Trait Implementation

**Question:** How do Schema and Group relate?

**Decision:** `Group` is a trait, `Schema` is its implementation.

```rust
// Group - trait for root aggregator
pub trait Group: Node + ValueAccess {
    fn children(&self) -> &[GroupChild];
}

// Schema - concrete implementation
pub struct Schema {
    metadata: Metadata,
    children: Vec<GroupChild>,
}

impl Group for Schema { ... }
impl Node for Schema { ... }
impl ValueAccess for Schema { ... }
```

**Benefits:**
- Clear separation of interface and implementation
- Type-safe child nesting rules via GroupChild enum
- Schema is the single root type

---

### ✅ DECISION: Macro for Child Enums

**Question:** How to enforce node nesting rules?

**Decision:** Use macro to generate child enums with From impls.

```rust
macro_rules! define_child_enums {
    (
        layouts: [$($layout:ident),*],
        decorations: [$($decoration:ident),*],
        containers: [$($container:ident),*],
        leaves: [$($leaf:ident),*]
    ) => {
        pub enum GroupChild {
            $($layout(Arc<$layout>),)*
            $($decoration(Arc<$decoration>),)*
            $($container(Arc<$container>),)*
            $($leaf(Arc<$leaf>),)*
        }

        pub enum LayoutChild {
            $($decoration(Arc<$decoration>),)*
            $($container(Arc<$container>),)*
            $($leaf(Arc<$leaf>),)*
        }

        pub enum ContainerChild {
            $($decoration(Arc<$decoration>),)*
            $($container(Arc<$container>),)*
            $($leaf(Arc<$leaf>),)*
        }

        // From impls generated automatically...
    };
}

define_child_enums! {
    layouts: [Panel],
    decorations: [Notice],
    containers: [Object, List, Mode, Routing, Expirable],
    leaves: [Text, Number, Boolean, Vector, Select]
}
```

**Benefits:**
- Single source of truth for all types
- Compile-time nesting enforcement
- Easy to add new types
- No orphan rule issues

---

### ✅ DECISION: SoA Storage + RuntimeNode View

**Question:** How to store runtime state?

**Decision:** Hybrid approach - SoA for storage, RuntimeNode for UI access.

```rust
// Storage: Structure of Arrays (efficient)
pub struct Context {
    schema: Arc<Schema>,
    values: HashMap<Key, Value>,
    states: HashMap<Key, StateFlags>,
    errors: HashMap<Key, Vec<ValidationError>>,
}

// View: RuntimeNode for UI (convenient)
pub struct RuntimeNode<'ctx, T: Node> {
    node: &'ctx T,
    value: &'ctx mut Value,
    state: &'ctx mut StateFlags,
    errors: &'ctx [ValidationError],
}

impl Context {
    pub fn get_runtime<T: Node>(&mut self, key: &Key) -> Option<RuntimeNode<'_, T>> {
        // Assembles view from HashMaps
    }
}
```

**Benefits:**
- SoA: Cache-friendly for batch operations
- SoA: Easy snapshots for undo/redo
- RuntimeNode: Convenient for UI integration
- Best of both worlds

---

### ✅ DECISION: Key = SmartString (No Typed Keys)

**Question:** Should keys be typed like `PropertyKey<T>`?

**Decision:** NO, use simple SmartString keys.

```rust
pub type Key = SmartString<LazyCompact>;

context.get_value("email")?;
context.set_value("port", Value::Int(8080))?;
```

**Why NOT typed keys:**
- Simpler API
- Dynamic schemas work naturally
- SmartString gives stack allocation for short keys (<23 bytes)
- Type safety achieved through builders and getters

---

### ✅ DECISION: validators + async_validators

**Question:** How to handle sync and async validation?

**Decision:** Two separate vectors, async feature-gated.

```rust
pub struct ValidationConfig {
    validators: Vec<Arc<dyn Fn(&Value) -> Result<(), ValidationError> + Send + Sync>>,
    
    #[cfg(feature = "async")]
    async_validators: Vec<Arc<dyn Fn(&Value) -> BoxFuture<'static, Result<(), ValidationError>> + Send + Sync>>,
}
```

**Usage with async closures (Rust 1.85+):**
```rust
Text::builder("username")
    .validate(|v| { /* sync */ Ok(()) })
    .validate_async(async |v| { /* async */ Ok(()) })
    .build()
```

**Benefits:**
- Zero cost when async not needed
- Clean feature separation
- Native async closures (no macros)

---

### ✅ DECISION: Vector with Generic Builder

**Question:** How to handle Vector type safety with runtime storage?

**Decision:** Generic builder, runtime storage.

```rust
// Schema type (stored in Schema)
pub struct Vector {
    metadata: Metadata,
    element_type: ElementType,
    size: usize,
}

// Generic builder (compile-time type safety)
pub struct VectorBuilder<T, const N: usize> { ... }

impl<T: VectorElement, const N: usize> VectorBuilder<T, N> {
    pub fn default(mut self, value: [T; N]) -> Self { ... }
    pub fn build(self) -> Vector { ... }
}

// Usage
let position = Vector::builder::<f64, 3>("position")
    .default([0.0, 0.0, 0.0])
    .build();
```

**Benefits:**
- Type-safe at construction time
- Uniform storage in Schema
- Supports any size, not just predefined subtypes

---

### ✅ DECISION: Callbacks + Optional Async Broadcast for Events

**Question:** How to implement EventBus?

**Decision:** Sync callbacks always, tokio broadcast with feature.

```rust
pub enum Event {
    BeforeChange { key: Key, old_value: Value, new_value: Value },
    AfterChange { key: Key, old_value: Value, new_value: Value },
    ValidationPassed { key: Key },
    ValidationFailed { key: Key, errors: Vec<ValidationError> },
    // ...
}

pub struct EventBus {
    // Always available (no dependencies)
    callbacks: Vec<Box<dyn Fn(&Event) + Send + Sync>>,
    
    // Optional async broadcast
    #[cfg(feature = "async")]
    broadcast: tokio::sync::broadcast::Sender<Event>,
}
```

**Also:** Renamed `ParameterEvent` → `Event`.

**Benefits:**
- Zero dependencies for sync use
- Async broadcast for reactive apps
- Simple naming
