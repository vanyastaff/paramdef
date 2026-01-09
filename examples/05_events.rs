//! Event system example.
//!
//! Demonstrates:
//! - Creating an event bus
//! - Subscribing to events
//! - Emitting and receiving events

#[cfg(feature = "events")]
use paramdef::context::Context;
#[cfg(feature = "events")]
use paramdef::core::Value;
#[cfg(feature = "events")]
use paramdef::event::{Event, EventBus, RecvError};
#[cfg(feature = "events")]
use paramdef::schema::Schema;
#[cfg(feature = "events")]
use paramdef::types::leaf::Text;
#[cfg(feature = "events")]
use std::sync::Arc;
#[cfg(feature = "events")]
use tokio::time::{Duration, sleep};

#[cfg(feature = "events")]
#[tokio::main]
async fn main() {
    println!("=== Event System Example ===\n");

    // Create event bus and context
    let bus = EventBus::new(32);
    let schema = Schema::builder()
        .parameter(Text::builder("username").build())
        .parameter(Text::builder("email").build())
        .build();

    let mut ctx = Context::with_event_bus(Arc::new(schema), bus.clone());

    // Subscribe to events
    let mut sub = bus.subscribe();

    // Spawn a task to listen for events
    let listener = tokio::spawn(async move {
        let mut count = 0;
        loop {
            match sub.recv().await {
                Ok(event) => {
                    count += 1;
                    match event {
                        Event::ValueChanged { key, new_value, .. } => {
                            println!("  [Event] Value changed: {} = {:?}", key, new_value);
                        }
                        Event::Touched { key } => {
                            println!("  [Event] Field touched: {}", key);
                        }
                        Event::Dirtied { key } => {
                            println!("  [Event] Field dirtied: {}", key);
                        }
                        _ => {
                            println!("  [Event] {:?}", event);
                        }
                    }
                    if count >= 6 {
                        break;
                    }
                }
                Err(RecvError::Lagged(n)) => {
                    println!("  [Warning] Receiver lagged, missed {} events", n);
                    // Continue receiving subsequent events
                }
                Err(RecvError::Closed) => {
                    println!("  [Info] Event bus closed");
                    break;
                }
            }
        }
    });

    // Give listener time to start
    sleep(Duration::from_millis(10)).await;

    // Perform operations that emit events
    println!("Setting values...\n");
    ctx.set("username", Value::text("alice"));
    ctx.set("email", Value::text("alice@example.com"));

    println!("\nModifying values...\n");
    ctx.set("username", Value::text("bob"));

    println!("\nClearing value...\n");
    ctx.clear("email");

    // Wait for listener to finish
    if let Err(e) = listener.await {
        eprintln!("Listener task panicked: {:?}", e);
    }

    println!("\nEvent processing complete!");
}

#[cfg(not(feature = "events"))]
fn main() {
    println!("This example requires the 'events' feature.");
    println!("Run with: cargo run --example 05_events --features events");
}
