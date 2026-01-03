# Event System

**Reactive parameter updates with `tokio::broadcast`**

Version: 1.0  
Feature: `events`

---

## Overview

The event system enables reactive programming patterns for paramdef. When parameters change, events are broadcast to all subscribers, allowing observers to react to changes without tight coupling.

### Design Influences

| Source | Pattern Adopted |
|--------|-----------------|
| SurveyJS | `ValueChanging`/`ValueChanged` event pairs |
| MobX | Transactions (batching) with `BatchBegin`/`BatchEnd` |
| Formik | `Touched` state for interaction tracking |
| Vue.js | Async event queue with deduplication |
| Qt | Property `NOTIFY` signals |
| Redux | Listener middleware pattern |

---

## Quick Start

```rust
use paramdef::event::{Event, EventBus};
use paramdef::context::Context;
use paramdef::schema::Schema;
use paramdef::types::leaf::Text;
use paramdef::core::Value;
use std::sync::Arc;

// Create schema
let schema = Arc::new(Schema::builder()
    .parameter(Text::builder("name").build())
    .parameter(Text::builder("email").build())
    .build());

// Create event bus and context
let bus = EventBus::new(64);
let mut sub = bus.subscribe();
let mut ctx = Context::with_event_bus(schema, bus);

// Changes emit events
ctx.set("name", Value::text("Alice"));

// Receive events asynchronously
tokio::spawn(async move {
    while let Ok(event) = sub.recv().await {
        match event {
            Event::ValueChanged { key, new_value, .. } => {
                println!("{} changed to {:?}", key, new_value);
            }
            _ => {}
        }
    }
});
```

---

## Event Types

### Value Events

Events related to parameter value changes.

#### `ValueChanging`

Emitted **before** a value change is applied.

```rust
Event::ValueChanging {
    key: Key,
    old_value: Option<Value>,
    new_value: Value,
}
```

Use cases:
- Logging value transitions
- Preparing dependent updates
- Debugging change flow

#### `ValueChanged`

Emitted **after** a value change is complete.

```rust
Event::ValueChanged {
    key: Key,
    old_value: Option<Value>,
    new_value: Value,
}
```

Use cases:
- Updating dependent values
- Triggering side effects
- Syncing with external systems
- Updating UI

#### `ValueCleared`

Emitted when a value is removed.

```rust
Event::ValueCleared {
    key: Key,
    old_value: Value,
}
```

---

### State Events

Events related to parameter state changes.

#### `Touched`

Emitted when a parameter is first interacted with.

```rust
Event::Touched { key: Key }
```

Use cases:
- Show validation errors only after interaction
- Track user engagement
- Analytics

#### `Dirtied`

Emitted when a parameter becomes dirty (value differs from initial).

```rust
Event::Dirtied { key: Key }
```

#### `Cleaned`

Emitted when a parameter is marked clean.

```rust
Event::Cleaned { key: Key }
```

#### `Reset`

Emitted when a parameter is reset to initial state.

```rust
Event::Reset { key: Key }
```

---

### Validation Events

#### `Validated`

Emitted when validation completes.

```rust
Event::Validated {
    key: Key,
    is_valid: bool,
    errors: Arc<[ValidationError]>,
}
```

The `errors` field uses `Arc` to avoid cloning error details to each subscriber.

---

### Batch Events

Events for grouping related changes.

#### `BatchBegin`

Emitted when a batch operation starts.

```rust
Event::BatchBegin {
    id: u64,
    description: Option<SmartStr>,
}
```

#### `BatchEnd`

Emitted when a batch operation ends.

```rust
Event::BatchEnd { id: u64 }
```

---

### Context Events

Events for context-wide operations.

#### `ContextReset`

Emitted when the entire context is reset.

```rust
Event::ContextReset
```

#### `AllCleaned`

Emitted when all parameters are marked clean.

```rust
Event::AllCleaned
```

---

## EventBus

The `EventBus` manages event distribution using `tokio::broadcast`.

### Creating an EventBus

```rust
use paramdef::event::EventBus;

// With custom capacity
let bus = EventBus::new(128);

// With default capacity (64)
let bus = EventBus::with_default_capacity();
```

### Emitting Events

```rust
use paramdef::event::Event;

// Emit single event
let receivers = bus.emit(Event::touched("username"));
println!("Sent to {} subscribers", receivers);

// Emit multiple events
let events = vec![
    Event::touched("a"),
    Event::touched("b"),
    Event::touched("c"),
];
bus.emit_all(events);
```

### Batching

Group related changes with batching:

```rust
// Manual batching
let batch_id = bus.begin_batch(Some("Update profile"));
bus.emit(Event::value_changed("name", None, Value::text("Alice")));
bus.emit(Event::value_changed("email", None, Value::text("alice@example.com")));
bus.end_batch(batch_id);

// Automatic batching with closure
bus.batch(Some("Update profile"), || {
    // Events emitted here are grouped
});
```

### Subscriber Count

```rust
let count = bus.subscriber_count();
let has_any = bus.has_subscribers();
```

---

## Subscription

Subscriptions receive events from the EventBus.

### Creating a Subscription

```rust
let mut sub = bus.subscribe();
```

### Receiving Events

```rust
// Async receive (waits for event)
let event = sub.recv().await?;

// Non-blocking try receive
match sub.try_recv()? {
    Some(event) => handle(event),
    None => {} // No event available
}
```

### Checking Status

```rust
// Number of pending events
let pending = sub.len();

// Check if empty
if sub.is_empty() {
    println!("No pending events");
}

// Check if bus is closed
if sub.is_closed() {
    println!("Event bus was dropped");
}
```

### Resubscribing

Create a new subscription from the current point:

```rust
let sub2 = sub.resubscribe();
// sub2 will only receive events from now on
```

---

## Error Handling

### RecvError

```rust
pub enum RecvError {
    /// The event bus was dropped
    Closed,
    /// Receiver lagged behind and missed events
    Lagged(u64),
}
```

Handle lag gracefully:

```rust
loop {
    match sub.recv().await {
        Ok(event) => handle(event),
        Err(RecvError::Lagged(n)) => {
            eprintln!("Warning: missed {} events", n);
            // Continue receiving subsequent events
        }
        Err(RecvError::Closed) => break,
    }
}
```

---

## Context Integration

### Creating Context with Events

```rust
let bus = EventBus::new(64);
let ctx = Context::with_event_bus(schema, bus);
```

### Events Emitted by Context

| Method | Events Emitted |
|--------|----------------|
| `set()` | `ValueChanging`, `ValueChanged`, `Dirtied` (if first dirty) |
| `clear()` | `ValueCleared` |
| `touch()` | `Touched` (if not already touched) |
| `reset()` | `ContextReset` |
| `mark_all_clean()` | `AllCleaned` |
| `batch()` | `BatchBegin`, ..., `BatchEnd` |

### Batching in Context

```rust
ctx.batch(Some("Update user"), |ctx| {
    ctx.set("name", Value::text("Alice"));
    ctx.set("age", Value::Int(30));
    ctx.set("email", Value::text("alice@example.com"));
});
```

Subscribers receive:
1. `BatchBegin { description: "Update user" }`
2. `ValueChanging { key: "name", ... }`
3. `ValueChanged { key: "name", ... }`
4. `Dirtied { key: "name" }`
5. `ValueChanging { key: "age", ... }`
6. `ValueChanged { key: "age", ... }`
7. `Dirtied { key: "age" }`
8. `ValueChanging { key: "email", ... }`
9. `ValueChanged { key: "email", ... }`
10. `Dirtied { key: "email" }`
11. `BatchEnd`

### Managing the EventBus

```rust
// Get reference to event bus
if let Some(bus) = ctx.event_bus() {
    let sub = bus.subscribe();
}

// Replace event bus
ctx.set_event_bus(new_bus);

// Remove event bus
let old_bus = ctx.take_event_bus();
```

---

## Event Helpers

### Constructors

```rust
// Value events
Event::value_changing("key", old_value, new_value);
Event::value_changed("key", old_value, new_value);
Event::value_cleared("key", old_value);

// Validation events
Event::validated("key", is_valid, errors);
Event::valid("key"); // Shorthand for successful validation

// State events
Event::touched("key");
Event::dirtied("key");
Event::cleaned("key");
Event::reset("key");

// Batch events
Event::batch_begin(id, Some("description"));
Event::batch_end(id);
```

### Querying Events

```rust
// Get the key (if applicable)
if let Some(key) = event.key() {
    println!("Event for: {}", key);
}

// Check event category
event.is_value_event();      // ValueChanging, ValueChanged, ValueCleared
event.is_state_event();      // Touched, Dirtied, Cleaned, Reset, etc.
event.is_validation_event(); // Validated
event.is_batch_event();      // BatchBegin, BatchEnd
```

---

## ValidationError

Lightweight validation error for event broadcasting.

```rust
pub struct ValidationError {
    pub code: SmartStr,
    pub message: SmartStr,
}

// Constructors
ValidationError::new("code", "message");
ValidationError::required();
ValidationError::min_length(5, 3);
ValidationError::max_length(100, 150);
ValidationError::min_value(0.0, -5.0);
ValidationError::max_value(100.0, 150.0);
ValidationError::pattern(r"^\d+$");
ValidationError::custom("invalid_format", "Value must be a valid UUID");
```

---

## Performance Considerations

### Channel Capacity

Choose capacity based on:
- Expected event rate
- Subscriber processing speed
- Acceptable lag tolerance

```rust
// High-frequency updates
let bus = EventBus::new(256);

// Low-frequency, critical events
let bus = EventBus::new(32);
```

### Arc Sharing

Events use `Arc` for large data to minimize clone overhead:

```rust
Event::Validated {
    errors: Arc<[ValidationError]>, // Shared across subscribers
}
```

### Conditional Emission

Events are only emitted when there are subscribers:

```rust
let count = bus.emit(event);
// count == 0 means no subscribers, event was not buffered
```

---

## Example: Logging Observer

```rust
use paramdef::event::{Event, EventBus, Subscription};

async fn logging_observer(mut sub: Subscription) {
    while let Ok(event) = sub.recv().await {
        match &event {
            Event::ValueChanged { key, old_value, new_value } => {
                println!("[CHANGE] {} = {:?} -> {:?}", key, old_value, new_value);
            }
            Event::Validated { key, is_valid, errors } => {
                if *is_valid {
                    println!("[VALID] {}", key);
                } else {
                    println!("[INVALID] {}: {:?}", key, errors);
                }
            }
            Event::BatchBegin { description, .. } => {
                println!("[BATCH] Start: {:?}", description);
            }
            Event::BatchEnd { .. } => {
                println!("[BATCH] End");
            }
            _ => {}
        }
    }
}

// Usage
let bus = EventBus::new(64);
let sub = bus.subscribe();
tokio::spawn(logging_observer(sub));
```

---

## Example: Dirty Tracking

```rust
use std::collections::HashSet;
use paramdef::event::{Event, Subscription};
use paramdef::core::Key;

async fn dirty_tracker(mut sub: Subscription) -> HashSet<Key> {
    let mut dirty_keys = HashSet::new();
    
    while let Ok(event) = sub.recv().await {
        match event {
            Event::Dirtied { key } => {
                dirty_keys.insert(key);
            }
            Event::Cleaned { key } => {
                dirty_keys.remove(&key);
            }
            Event::AllCleaned | Event::ContextReset => {
                dirty_keys.clear();
            }
            _ => {}
        }
    }
    
    dirty_keys
}
```

---

## Example: Debounced Auto-Save

```rust
use tokio::time::{sleep, Duration};
use paramdef::event::{Event, Subscription};

async fn auto_save(mut sub: Subscription, save_fn: impl Fn()) {
    let mut pending_save = false;
    
    loop {
        tokio::select! {
            result = sub.recv() => {
                match result {
                    Ok(Event::ValueChanged { .. }) => {
                        pending_save = true;
                    }
                    Err(_) => break,
                    _ => {}
                }
            }
            _ = sleep(Duration::from_secs(2)), if pending_save => {
                save_fn();
                pending_save = false;
            }
        }
    }
}
```

---

## Thread Safety

All event types are `Send + Sync`:

```rust
fn assert_send_sync<T: Send + Sync>() {}

assert_send_sync::<Event>();
assert_send_sync::<EventBus>();
assert_send_sync::<Subscription>();
assert_send_sync::<ValidationError>();
```

`EventBus` can be shared across threads via `Arc`:

```rust
let bus = Arc::new(EventBus::new(64));
let bus2 = Arc::clone(&bus);

tokio::spawn(async move {
    bus2.emit(Event::touched("field"));
});
```

---

## Feature Flag

Enable the event system with:

```toml
[dependencies]
paramdef = { version = "0.2", features = ["events"] }
```

Or with all features:

```toml
paramdef = { version = "0.2", features = ["full"] }
```
