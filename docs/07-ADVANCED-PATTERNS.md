# Advanced Patterns

Industry patterns from Qt, Houdini, TouchDesigner, WPF, Node-RED, Airflow, and more.

---

## 1. Reset Functionality (Qt Pattern)

Reset parameters to their default values.

```rust
// Parameter trait includes reset support
pub trait Parameter {
    fn default_value(&self) -> Option<Self::Value>;
    
    fn reset(&self) -> Self::Value {
        self.default_value().expect("Parameter must have default for reset")
    }
}

// Context methods
impl Context {
    /// Reset single parameter to default
    pub fn reset(&mut self, key: &str) -> Result<()> {
        let param = self.schema.get_parameter(key)?;
        let default = param.reset();
        self.set_value(key, default)
    }
    
    /// Reset all parameters
    pub fn reset_all(&mut self) -> Result<()> {
        for param in self.schema.parameters() {
            if let Some(default) = param.default_value() {
                self.set_value(param.metadata().key(), default)?;
            }
        }
        Ok(())
    }
}
```

---

## 2. Change Notifications (Qt/WPF Pattern)

Subscribe to parameter changes with events.

```rust
pub enum ParameterEvent {
    BeforeChange { key: String, old_value: Value, new_value: Value },
    AfterChange { key: String, old_value: Value, new_value: Value },
    ValidationStarted { key: String },
    ValidationPassed { key: String },
    ValidationFailed { key: String, errors: Vec<ValidationError> },
    Touched { key: String },
    Reset { key: String, value: Value },
    VisibilityChanged { key: String, visible: bool },
    EnabledChanged { key: String, enabled: bool },
    ActionTriggered { key: String, timestamp: Instant },
}

// Subscribe to specific parameter
let mut events = context.subscribe_parameter("username");
while let Ok(event) = events.recv().await {
    match event {
        ParameterEvent::AfterChange { key, new_value, .. } => {
            println!("{} changed to {:?}", key, new_value);
        }
        _ => {}
    }
}

// Subscribe to all events
let mut all_events = context.subscribe_all();
```

---

## 3. Conditional Visibility & Enable (Houdini Pattern)

Show/hide based on other parameters using a single `Expr` expression.

### Expr - Visibility Expression

```rust
/// Visibility expression - evaluates to bool
pub enum Expr {
    // === Comparisons ===
    Eq(Key, Value),           // key == value
    Ne(Key, Value),           // key != value
    
    // === Existence ===
    IsSet(Key),               // key is not null
    IsEmpty(Key),             // "", [], {}
    
    // === Boolean ===
    IsTrue(Key),              // key == true
    
    // === Numeric ===
    Lt(Key, f64),             // key < value
    Le(Key, f64),             // key <= value
    Gt(Key, f64),             // key > value
    Ge(Key, f64),             // key >= value
    
    // === Set ===
    OneOf(Key, Arc<[Value]>), // key in [...]
    
    // === Validation ===
    IsValid(Key),             // key passed validation
    
    // === Combinators ===
    And(Arc<[Expr]>),         // all must be true
    Or(Arc<[Expr]>),          // any must be true
    Not(Box<Expr>),           // invert
}
```

### Context for Evaluation

```rust
/// Context for evaluating visibility expressions
pub struct Context {
    values: HashMap<Key, Value>,
    validation: HashMap<Key, bool>,
}

impl Context {
    pub fn new() -> Self;
    
    pub fn with_value(self, key: impl Into<Key>, value: Value) -> Self;
    pub fn with_validation(self, key: impl Into<Key>, is_valid: bool) -> Self;
    
    pub fn get(&self, key: &Key) -> Option<&Value>;
    pub fn is_valid(&self, key: &Key) -> bool;
}

impl Expr {
    pub fn eval(&self, ctx: &Context) -> bool;
    pub fn dependencies(&self) -> Vec<Key>;
}
```

### Usage Examples

```rust
use Expr::*;

// Show API key field only when auth type is "api_key"
Text::builder("api_key")
    .visible_when(Eq("auth_type".into(), Value::text("api_key")))
    .build()

// Show advanced options when enabled AND level > 10
Number::builder::<i32>("threshold")
    .visible_when(And(Arc::from([
        IsTrue("advanced".into()),
        Gt("level".into(), 10.0),
    ])))
    .build()

// Show either admin OR superuser
Panel::builder("admin_panel")
    .visible_when(Or(Arc::from([
        Eq("role".into(), Value::text("admin")),
        IsTrue("superuser".into()),
    ])))
    .build()

// Hide when disabled (= show when NOT disabled)
Text::builder("feature")
    .visible_when(Not(Box::new(IsTrue("disabled".into()))))
    .build()

// Show error message only when email is invalid
Notice::builder("email_error")
    .visible_when(Not(Box::new(IsValid("email".into()))))
    .build()

// Show confirmation field only when password is valid
Text::builder("confirm_password")
    .visible_when(IsValid("password".into()))
    .build()

// Complex: show when valid AND not in maintenance mode
Text::builder("settings")
    .visible_when(And(Arc::from([
        IsValid("email".into()),
        Not(Box::new(IsTrue("maintenance".into()))),
    ])))
    .build()
```

### Evaluating Visibility

```rust
use Expr::*;

let expr = And(Arc::from([
    IsTrue("enabled".into()),
    Not(Box::new(IsTrue("maintenance".into()))),
]));

let ctx = Context::new()
    .with_value("enabled", Value::boolean(true))
    .with_value("maintenance", Value::boolean(false));

assert!(expr.eval(&ctx));  // true: enabled=true AND NOT maintenance=false
```

### Builder Helpers

```rust
impl<T: Node> Builder<T> {
    /// Show when expression is true
    fn visible_when(self, expr: Expr) -> Self;
    
    /// Convenience: show when key equals value
    fn visible_when_eq(self, key: impl Into<Key>, value: Value) -> Self;
    
    /// Convenience: show when key is true
    fn visible_when_true(self, key: impl Into<Key>) -> Self;
    
    /// Convenience: hide when expression is true (inverts)
    fn hidden_when(self, expr: Expr) -> Self;
}

---

## 4. Parameter Organization (TouchDesigner/Houdini Pattern)

Organize parameters into pages, groups, and subgroups.

```rust
pub struct ParameterLocation {
    pub page: Option<String>,
    pub group: Option<String>,
    pub subgroup: Option<String>,
    pub order: i32,
}

// Usage
Number::builder("pos_x")
    .label("X Position")
    .page("Transform")
    .group("Position")
    .order(0)
    .build()

Number::builder("pos_y")
    .label("Y Position")
    .page("Transform")
    .group("Position")
    .order(1)
    .build()

Vector::builder("color")
    .label("Color")
    .page("Appearance")
    .order(0)
    .build()
```

---

## 5. Storage Flags (Qt Pattern)

Control serialization behavior.

```rust
bitflags! {
    pub struct Flags: u32 {
        // ... existing flags ...
        
        const STORED      = 1 << 8;   // Save to file (default: true)
        const DESIGNABLE  = 1 << 9;   // Visible in designer
        const SCRIPTABLE  = 1 << 10;  // Accessible via scripts
        const USER        = 1 << 11;  // Primary user property
        const CONSTANT    = 1 << 12;  // Value never changes
    }
}

// Don't serialize temporary values
Text::builder("cached_value")
    .not_stored()
    .build()

// Session-only identifier
Text::builder("session_id")
    .constant()
    .not_stored()
    .build()

// Serialization respects flags
impl Context {
    pub fn serialize(&self) -> HashMap<String, Value> {
        self.schema.parameters()
            .filter(|p| p.flags().contains(Flags::STORED))
            .map(|p| (p.key(), self.get_value(p.key())))
            .collect()
    }
}
```

---

## 6. Template Support (Airflow Pattern)

Enable template expressions in parameter values.

```rust
bitflags! {
    pub struct Flags: u32 {
        const TEMPLATABLE = 1 << 13;  // Supports templates
    }
}

pub struct TemplateContext {
    pub variables: HashMap<String, Value>,
    pub functions: HashMap<String, TemplateFn>,
}

// Mark parameter as templatable
Text::builder("message")
    .templatable()
    .default("Hello {{user.name}}!")
    .build()

// Render at runtime
let mut template_ctx = TemplateContext::new();
template_ctx.add_variable("user", user_value);

let rendered = context.get_rendered("message", &template_ctx)?;
// Result: "Hello Alice!"
```

---

## 7. Typed Input Pattern (Node-RED Style)

Use `Mode` for values that can be one of multiple types (discriminated union).

```rust
// Value can be literal OR expression OR reference
Mode::builder("value")
    .label("Value")
    .variant("literal", "Literal", Schema::new()
        .with(Text::builder("value").build()))
    .variant("expression", "Expression", Schema::new()
        .with(Text::builder("value")
            .subtype(TextSubtype::Expression)
            .build()))
    .variant("reference", "Reference", Schema::new()
        .with(Text::builder("value")
            .subtype(TextSubtype::JsonPath)
            .build()))
    .default_variant("literal")
    .build()

// Serialized as:
// { "mode": "expression", "value": { "value": "{{$json.data}}" } }
```

This is a common pattern in workflow tools where a field can accept different input types.

---

## 8. Expression Engine

First-class expression support with dependency tracking.

```rust
pub enum Value {
    // ... other variants ...
    Expression {
        template: String,
        compiled: Option<Arc<CompiledExpression>>,
    },
}

pub struct CompiledExpression {
    pub ast: Ast,
    pub dependencies: Vec<String>,  // Auto-tracked
}

// Expression syntax
// {{$json.user.name}}           - Access data
// {{$json.age ?? 18}}           - Default value
// {{len($json.items)}}          - Function call
// "Hello {{$json.name}}!"       - String interpolation

// Built-in functions
// len(), upper(), lower(), trim(), json(), get()

// Lazy compilation
impl ExpressionEngine {
    pub fn evaluate(&self, template: &str, context: &Context) -> Result<Value> {
        let compiled = self.compile(template)?;  // Cached
        self.evaluate_compiled(&compiled, context)
    }
}
```

---

## 9. Snapshot/Restore (Undo/Redo)

Support undo/redo operations.

```rust
pub struct Snapshot {
    values: HashMap<String, Value>,
    timestamp: Instant,
}

impl Context {
    /// Take snapshot of current state
    pub fn snapshot(&self) -> Snapshot {
        Snapshot {
            values: self.values.clone(),
            timestamp: Instant::now(),
        }
    }
    
    /// Restore from snapshot
    pub fn restore(&mut self, snapshot: &Snapshot) {
        self.values = snapshot.values.clone();
        
        // Emit batch update event
        self.emit_event(ParameterEvent::BatchUpdate {
            keys: self.values.keys().cloned().collect(),
        });
    }
}

// Usage
let before = context.snapshot();

// Make changes...
context.set_value("name", new_value)?;

// Undo
context.restore(&before);
```

---

## 10. Complete Example: Database Connection

```rust
fn database_connection_schema() -> Schema {
    Schema::new()
        // Connection mode (discriminated union)
        .with(Mode::builder("connection")
            .label("Connection")
            .page("Connection")
            
            .variant("uri", "Connection String", Schema::new()
                .with(Text::builder("uri")
                    .label("Connection URI")
                    .subtype(TextSubtype::Url)
                    .required()
                    .templatable()
                    .placeholder("postgresql://user:pass@localhost:5432/db")
                    .build())
            )
            
            .variant("details", "Connection Details", Schema::new()
                .with(Text::builder("host")
                    .label("Host")
                    .required()
                    .default("localhost")
                    .page("Connection")
                    .group("Server")
                    .order(0)
                    .build())
                
                .with(Number::builder::<i64>("port")
                    .label("Port")
                    .range(1, 65535)
                    .default(5432)
                    .page("Connection")
                    .group("Server")
                    .order(1)
                    .build())
                
                .with(Text::builder("database")
                    .label("Database")
                    .required()
                    .page("Connection")
                    .group("Server")
                    .order(2)
                    .build())
                
                .with(Text::builder("username")
                    .label("Username")
                    .required()
                    .page("Connection")
                    .group("Credentials")
                    .order(0)
                    .build())
                
                .with(Text::builder("password")
                    .label("Password")
                    .required()
                    .page("Connection")
                    .group("Credentials")
                    .order(1)
                    .build())
            )
            .build())
        
        // SSL options (conditional)
        .with(Boolean::builder("use_ssl")
            .label("Use SSL")
            .default(true)
            .page("Security")
            .build())
        
        .with(Text::builder("ssl_cert")
            .label("SSL Certificate")
            .page("Security")
            .visible_when(Expr::IsTrue("use_ssl".into()))
            .build())
        
        // Timeouts
        .with(Number::builder("connection_timeout")
            .label("Connection Timeout")
            .page("Advanced")
            .group("Timeouts")
            .range(1.0, 300.0)
            .default(30.0)
            .build())
        
        .with(Number::builder("query_timeout")
            .label("Query Timeout")
            .page("Advanced")
            .group("Timeouts")
            .range(1.0, 3600.0)
            .default(300.0)
            .build())
        
        // Pool settings
        .with(Number::builder::<i64>("pool_size")
            .label("Pool Size")
            .page("Advanced")
            .group("Performance")
            .range(1, 100)
            .default(10)
            .build())
}
```

---

## Industry Comparison

| Pattern | Qt | Houdini | WPF | Airflow | Node-RED | paramdef |
|---------|-----|---------|-----|---------|----------|----------|
| Reset | Yes | Yes | - | - | - | **Yes** |
| Notifications | Yes | Yes | Yes | - | - | **Yes** |
| Visibility | - | Yes | - | - | - | **Yes** |
| Pages/Groups | - | Yes | - | - | - | **Yes** |
| Storage Flags | Yes | - | - | - | - | **Yes** |
| Templates | - | - | - | Yes | - | **Yes** |
| Typed Input | - | - | - | - | Yes | **Yes** (Mode) |
| Expressions | - | Yes | Yes | Yes | Yes | **Yes** |
| Undo/Redo | - | Yes | - | - | - | **Yes** |

paramdef combines best patterns from across the industry!
