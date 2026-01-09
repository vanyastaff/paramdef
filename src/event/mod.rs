//! Event system for reactive parameter updates.
//!
//! This module provides the event infrastructure for the paramdef library,
//! enabling reactive programming patterns inspired by:
//!
//! - `SurveyJS`: `onValueChanging`/`onValueChanged` event pairs
//! - `MobX`: Reactions and transactions (batching)
//! - `Vue.js`: Computed dependencies and async queue
//! - Qt: Property `NOTIFY` signals
//!
//! # Architecture
//!
//! The event system uses `tokio::broadcast` for efficient multi-producer,
//! multi-consumer event distribution:
//!
//! ```text
//! ┌─────────┐     ┌──────────┐     ┌──────────────┐
//! │ Context │────▶│ EventBus │────▶│ Subscription │
//! └─────────┘     └──────────┘     └──────────────┘
//!                      │                  ▲
//!                      │           ┌──────┴───────┐
//!                      ▼           │              │
//!                 ┌─────────┐  ┌─────────┐  ┌─────────┐
//!                 │Observer1│  │Observer2│  │Observer3│
//!                 └─────────┘  └─────────┘  └─────────┘
//! ```
//!
//! # Event Types
//!
//! | Event | Description |
//! |-------|-------------|
//! | [`Event::ValueChanging`] | Before value change (notification) |
//! | [`Event::ValueChanged`] | After value changed |
//! | [`Event::ValueCleared`] | Value was removed |
//! | [`Event::Validated`] | Validation completed |
//! | [`Event::Touched`] | Parameter was interacted with |
//! | [`Event::Dirtied`] | Parameter became dirty |
//! | [`Event::Cleaned`] | Parameter marked clean |
//! | [`Event::Reset`] | Parameter reset to initial state |
//! | [`Event::BatchBegin`] | Batch operation started |
//! | [`Event::BatchEnd`] | Batch operation ended |
//!
//! # Quick Start
//!
//! ```ignore
//! use paramdef::event::{Event, EventBus};
//! use paramdef::core::Value;
//!
//! // Create event bus
//! let bus = EventBus::new(64);
//!
//! // Subscribe to events
//! let mut sub = bus.subscribe();
//!
//! // Emit events
//! bus.emit(Event::value_changed("username", None, Value::text("alice")));
//!
//! // Receive events (async)
//! tokio::spawn(async move {
//!     while let Ok(event) = sub.recv().await {
//!         match event {
//!             Event::ValueChanged { key, new_value, .. } => {
//!                 println!("{} = {:?}", key, new_value);
//!             }
//!             _ => {}
//!         }
//!     }
//! });
//! ```
//!
//! # Batching
//!
//! Use batching to group related changes:
//!
//! ```ignore
//! bus.batch(Some("Update user profile"), || {
//!     ctx.set("name", Value::text("Alice"));
//!     ctx.set("email", Value::text("alice@example.com"));
//!     ctx.set("age", Value::Int(30));
//! });
//!
//! // Subscribers receive:
//! // 1. BatchBegin { description: "Update user profile" }
//! // 2. ValueChanged { key: "name", ... }
//! // 3. ValueChanged { key: "email", ... }
//! // 4. ValueChanged { key: "age", ... }
//! // 5. BatchEnd
//! ```
//!
//! # Feature Flag
//!
//! This module requires the `events` feature:
//!
//! ```toml
//! [dependencies]
//! paramdef = { version = "0.2", features = ["events"] }
//! ```

mod bus;
mod types;

pub use bus::{DEFAULT_CAPACITY, EventBus, RecvError, Subscription};
pub use types::{Event, ValidationError};
