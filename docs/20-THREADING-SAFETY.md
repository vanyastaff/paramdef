# Threading Safety

**Thread-safety guarantees and concurrent usage patterns**

---

## Table of Contents

1. [Overview](#overview)
2. [Send and Sync Bounds](#send-and-sync-bounds)
3. [Schema Sharing](#schema-sharing)
4. [Context Thread Safety](#context-thread-safety)
5. [Event System Concurrency](#event-system-concurrency)
6. [Async Validation](#async-validation)
7. [Common Patterns](#common-patterns)
8. [Anti-Patterns](#anti-patterns)
9. [Performance Considerations](#performance-considerations)

---

## Overview

paramdef is designed for concurrent usage in multi-threaded applications. The library follows Rust's ownership model strictly:

- **Schema** is immutable and freely shareable (`Arc<Schema>`)
- **Context** is mutable and owned by one thread at a time
- **Events** are broadcast-safe via `tokio::broadcast`
- **Values** are `Send + Sync` when contained types are

### Design Philosophy

```
┌─────────────────────────────────────────────────────────┐
│  SCHEMA (Arc<Schema>)                                   │
│  - Immutable after creation                             │
│  - Send + Sync                                          │
│  - Clone is cheap (Arc::clone)                          │
│  - Shared between all contexts                          │
└─────────────────────────────────────────────────────────┘
                    │
         ┌─────────┴─────────┐
         │                   │
         ▼                   ▼
┌─────────────────┐  ┌─────────────────┐
│  Context A      │  │  Context B      │
│  (Thread 1)     │  │  (Thread 2)     │
│  - Mutable      │  │  - Mutable      │
│  - NOT Sync     │  │  - NOT Sync     │
│  - Own values   │  │  - Own values   │
└─────────────────┘  └─────────────────┘
```

---

## Send and Sync Bounds

### Core Types

| Type | Send | Sync | Notes |
|------|------|------|-------|
| `Schema` | Yes | Yes | Immutable, Arc-wrapped internally |
| `Arc<Schema>` | Yes | Yes | Cheap to clone, share freely |
| `Context` | Yes | **No** | Contains mutable state |
| `Value` | Yes | Yes | All variants are thread-safe |
| `Metadata` | Yes | Yes | Immutable |
| `Flags` | Yes | Yes | Copy type (u64) |
| `Key` | Yes | Yes | SmartString-backed |

### Node Types

All 13 node types are `Send + Sync`:

```rust
// All node types implement these bounds
impl Send for Text {}
impl Sync for Text {}

impl Send for Number {}
impl Sync for Number {}

// ... same for all 13 types
```

### Value Enum

```rust
pub enum Value {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    Text(String),
    Array(Arc<[Value]>),      // Arc for cheap cloning
    Object(Arc<HashMap<String, Value>>),  // Arc for cheap cloning
}

// Value is Send + Sync because:
// - Primitives are Copy
// - String is Send + Sync
// - Arc<T> is Send + Sync when T is Send + Sync
```

---

## Schema Sharing

Schema is designed to be shared across threads using `Arc`.

### Creating Shared Schema

```rust
use std::sync::Arc;
use paramdef::prelude::*;

// Create schema once
let schema = Arc::new(Schema::new()
    .with(Text::builder("name").required().build())
    .with(Number::builder("age").range(0, 150).build())
);

// Clone Arc for each thread (cheap - just increments ref count)
let schema_for_thread1 = schema.clone();
let schema_for_thread2 = schema.clone();
```

### Sharing Across Threads

```rust
use std::thread;

let schema = Arc::new(create_user_schema());

let handles: Vec<_> = (0..4).map(|i| {
    let schema = schema.clone();
    
    thread::spawn(move || {
        // Each thread gets its own Context
        let mut context = Context::new(schema);
        context.set_value("name", format!("User {}", i).into())?;
        context.validate_all()
    })
}).collect();

for handle in handles {
    handle.join().unwrap()?;
}
```

### Sharing Across Async Tasks

```rust
use tokio::task;

let schema = Arc::new(create_user_schema());

let handles: Vec<_> = (0..4).map(|i| {
    let schema = schema.clone();
    
    task::spawn(async move {
        let mut context = Context::new(schema);
        context.set_value("name", format!("User {}", i).into())?;
        context.validate_all().await
    })
}).collect();

for handle in handles {
    handle.await??;
}
```

---

## Context Thread Safety

`Context` is `Send` but **NOT `Sync`**. This means:

- You CAN move a Context to another thread
- You CANNOT share a Context reference between threads
- You CANNOT use `&Context` from multiple threads

### Why Context is Not Sync

Context contains mutable state:

```rust
pub struct Context {
    schema: Arc<Schema>,
    values: HashMap<Key, Value>,        // Mutable
    state: HashMap<Key, ParameterState>, // Mutable
    errors: HashMap<Key, Vec<ValidationError>>, // Mutable
    event_bus: EventBus,                // Internal synchronization
}
```

### Moving Context Between Threads

```rust
// ✅ OK: Move context to another thread
let context = Context::new(schema);

thread::spawn(move || {
    // Context is now owned by this thread
    context.set_value("name", "Alice".into())?;
    Ok::<_, Error>(())
}).join().unwrap()?;
```

### Sharing Context State (Not the Context Itself)

If you need to share state, extract values:

```rust
// Thread 1: Prepare data
let mut context = Context::new(schema.clone());
context.set_value("name", "Alice".into())?;

// Extract values (they are Send + Sync)
let values = context.collect_values();

// Thread 2: Use values
thread::spawn(move || {
    // Create new context with same schema
    let mut context = Context::new(schema);
    context.set_values(values)?;
    // Continue processing...
    Ok::<_, Error>(())
});
```

### Using Mutex for Shared Context

If you truly need shared mutable access (rare):

```rust
use std::sync::Mutex;

let shared_context = Arc::new(Mutex::new(Context::new(schema)));

// Thread 1
{
    let mut ctx = shared_context.lock().unwrap();
    ctx.set_value("count", Value::Int(1))?;
}

// Thread 2
{
    let mut ctx = shared_context.lock().unwrap();
    let count = ctx.get_int("count")?;
    ctx.set_value("count", Value::Int(count + 1))?;
}
```

**Warning:** Mutex-wrapped Context is usually a code smell. Consider using message passing instead.

---

## Event System Concurrency

The event system uses `tokio::broadcast` which is designed for concurrent access.

### EventBus Thread Safety

```rust
pub struct EventBus {
    tx: broadcast::Sender<ParameterEvent>,
    // Sender is Clone + Send + Sync
}

impl EventBus {
    // Safe to call from any thread
    pub fn subscribe(&self) -> broadcast::Receiver<ParameterEvent> {
        self.tx.subscribe()
    }
    
    // Safe to call from any thread
    pub fn emit(&self, event: ParameterEvent) {
        let _ = self.tx.send(event);
    }
}
```

### Subscribing from Multiple Threads

```rust
let event_bus = context.event_bus().clone();

// Thread 1: UI updates
let mut rx1 = event_bus.subscribe();
thread::spawn(move || {
    while let Ok(event) = rx1.blocking_recv() {
        update_ui(&event);
    }
});

// Thread 2: Logging
let mut rx2 = event_bus.subscribe();
thread::spawn(move || {
    while let Ok(event) = rx2.blocking_recv() {
        log_event(&event);
    }
});

// Thread 3: Persistence
let mut rx3 = event_bus.subscribe();
thread::spawn(move || {
    while let Ok(event) = rx3.blocking_recv() {
        if matches!(event, ParameterEvent::ValueChanged { .. }) {
            save_to_disk(&event);
        }
    }
});
```

### Async Event Handling

```rust
let event_bus = context.event_bus().clone();
let mut rx = event_bus.subscribe();

tokio::spawn(async move {
    while let Ok(event) = rx.recv().await {
        match event {
            ParameterEvent::ValueChanged { key, new_value, .. } => {
                println!("{} changed to {:?}", key, new_value);
            }
            ParameterEvent::Validated { key, is_valid, .. } => {
                if !is_valid {
                    show_error(&key).await;
                }
            }
            _ => {}
        }
    }
});
```

---

## Async Validation

Async validators run concurrently using Tokio.

### Validator Thread Safety

```rust
#[async_trait]
pub trait AsyncValidator: Send + Sync {
    async fn validate(&self, value: &Value) -> Result<(), ValidationError>;
}
```

**Requirements:**
- Validators must be `Send + Sync`
- Internal state must be thread-safe (use `Arc<Mutex<_>>` if needed)
- Async operations should not block

### Thread-Safe Validator Example

```rust
pub struct DatabaseUniqueValidator {
    pool: Arc<Pool>,  // Connection pool is already thread-safe
    table: String,
    column: String,
}

#[async_trait]
impl AsyncValidator for DatabaseUniqueValidator {
    async fn validate(&self, value: &Value) -> Result<(), ValidationError> {
        let text = value.as_text().ok_or(ValidationError::type_mismatch())?;
        
        // Pool handles connection safely
        let exists = sqlx::query_scalar!(
            "SELECT EXISTS(SELECT 1 FROM {} WHERE {} = $1)",
            self.table,
            self.column,
            text
        )
        .fetch_one(self.pool.as_ref())
        .await?;
        
        if exists {
            Err(ValidationError::new("not_unique", "Value already exists"))
        } else {
            Ok(())
        }
    }
}
```

### Concurrent Validation

```rust
// Validate multiple parameters concurrently
let results = context.validate_all().await;

// Under the hood, this runs validators concurrently:
// - Sync validators run sequentially (fast)
// - Async validators run concurrently (parallel I/O)
```

---

## Common Patterns

### Pattern 1: One Context Per Request (Web Server)

```rust
async fn handle_request(
    schema: Arc<Schema>,
    body: Json<HashMap<String, Value>>,
) -> Result<Response> {
    // Create context for this request
    let mut context = Context::new(schema);
    
    // Set values from request
    context.set_values(body.0)?;
    
    // Validate
    context.validate_all().await?;
    
    // Process...
    Ok(Response::ok())
}
```

### Pattern 2: Long-Lived Context with Worker Thread

```rust
struct Editor {
    context: Context,
}

impl Editor {
    fn spawn_autosave(self: Arc<Mutex<Self>>) -> JoinHandle<()> {
        let editor = self.clone();
        
        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_secs(30));
                
                let values = {
                    let editor = editor.lock().unwrap();
                    editor.context.collect_values()
                };
                
                // Save values (no lock held)
                save_to_file(&values);
            }
        })
    }
}
```

### Pattern 3: Message Passing (Recommended)

```rust
enum ContextMessage {
    SetValue(Key, Value),
    GetValue(Key, oneshot::Sender<Option<Value>>),
    Validate(oneshot::Sender<ValidationResult>),
}

async fn context_actor(
    schema: Arc<Schema>,
    mut rx: mpsc::Receiver<ContextMessage>,
) {
    let mut context = Context::new(schema);
    
    while let Some(msg) = rx.recv().await {
        match msg {
            ContextMessage::SetValue(key, value) => {
                let _ = context.set_value(&key, value);
            }
            ContextMessage::GetValue(key, reply) => {
                let value = context.get_value(&key).cloned();
                let _ = reply.send(value);
            }
            ContextMessage::Validate(reply) => {
                let result = context.validate_all().await;
                let _ = reply.send(result);
            }
        }
    }
}
```

### Pattern 4: Read-Write Lock for Read-Heavy Workloads

```rust
use tokio::sync::RwLock;

struct SharedContext {
    inner: RwLock<Context>,
}

impl SharedContext {
    // Multiple readers allowed
    async fn get_value(&self, key: &Key) -> Option<Value> {
        let ctx = self.inner.read().await;
        ctx.get_value(key).cloned()
    }
    
    // Exclusive writer
    async fn set_value(&self, key: Key, value: Value) -> Result<()> {
        let mut ctx = self.inner.write().await;
        ctx.set_value(&key, value)
    }
}
```

---

## Anti-Patterns

### Anti-Pattern 1: Sharing &Context Across Threads

```rust
// ❌ WON'T COMPILE: Context is not Sync
let context = Context::new(schema);

thread::scope(|s| {
    s.spawn(|| {
        context.get_value("name")  // Error: &Context is not Send
    });
});
```

### Anti-Pattern 2: Blocking in Async Validators

```rust
// ❌ BAD: Blocks the async runtime
#[async_trait]
impl AsyncValidator for BadValidator {
    async fn validate(&self, value: &Value) -> Result<(), ValidationError> {
        // This blocks the executor thread!
        std::thread::sleep(Duration::from_secs(5));
        Ok(())
    }
}

// ✅ GOOD: Use async sleep
#[async_trait]
impl AsyncValidator for GoodValidator {
    async fn validate(&self, value: &Value) -> Result<(), ValidationError> {
        tokio::time::sleep(Duration::from_secs(5)).await;
        Ok(())
    }
}
```

### Anti-Pattern 3: Long Mutex Holds

```rust
// ❌ BAD: Hold mutex during I/O
let shared = Arc::new(Mutex::new(Context::new(schema)));

async fn bad_save(shared: Arc<Mutex<Context>>) {
    let ctx = shared.lock().unwrap();
    // Mutex held during slow I/O!
    save_to_disk(ctx.collect_values()).await;
}

// ✅ GOOD: Release mutex before I/O
async fn good_save(shared: Arc<Mutex<Context>>) {
    let values = {
        let ctx = shared.lock().unwrap();
        ctx.collect_values()
        // Mutex released here
    };
    
    // No lock held during I/O
    save_to_disk(values).await;
}
```

### Anti-Pattern 4: Clone Context Instead of Arc<Schema>

```rust
// ❌ BAD: Cloning context is expensive
fn process(context: Context) {
    let context_copy = context.clone();  // Deep copy of all values!
}

// ✅ GOOD: Share schema, create new contexts
fn process(schema: Arc<Schema>, values: HashMap<Key, Value>) {
    let mut context = Context::new(schema);  // Cheap
    context.set_values(values)?;
}
```

---

## Performance Considerations

### Arc Overhead

`Arc::clone()` is very cheap (atomic increment), but accessing Arc contents requires dereferencing:

```rust
// Clone is cheap (~10ns)
let schema2 = schema.clone();

// Deref is negligible but present
let param = schema.get_parameter("name")?;
```

### Contention Points

| Operation | Contention | Mitigation |
|-----------|------------|------------|
| Schema access | None | Immutable, no locks |
| Context mutation | Per-context | One owner only |
| Event broadcast | Low | Lock-free channel |
| Async validation | Per-validator | Stateless validators |

### Batch Operations

For bulk updates, use batch methods to reduce overhead:

```rust
// ❌ Slow: Many individual updates
for (key, value) in values {
    context.set_value(&key, value)?;  // Event per update
}

// ✅ Fast: Single batch update
context.set_values(values)?;  // Single batch event
```

### Memory Layout

Values use `Arc<[Value]>` and `Arc<HashMap>` for arrays and objects to enable cheap cloning:

```rust
// Cloning large arrays is cheap
let array = Value::Array(Arc::new([Value::Int(1); 1000]));
let array2 = array.clone();  // Just Arc::clone, no data copy
```

---

## See Also

- [08-EVENT-SYSTEM](08-EVENT-SYSTEM.md) - Event system details
- [12-SCHEMA-CONTEXT](12-SCHEMA-CONTEXT.md) - Schema vs Context architecture
- [19-SERIALIZATION-FORMAT](19-SERIALIZATION-FORMAT.md) - Serialization for cross-thread transfer
