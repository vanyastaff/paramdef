//! Context state management example.
//!
//! Demonstrates:
//! - Tracking dirty state
//! - Collecting dirty values
//! - Resetting context

use paramdef::context::Context;
use paramdef::core::Value;
use paramdef::schema::Schema;
use paramdef::types::leaf::Text;
use std::sync::Arc;

fn main() {
    println!("=== Context State Management Example ===\n");

    let schema = Schema::builder()
        .parameter(Text::builder("field1").build())
        .parameter(Text::builder("field2").build())
        .parameter(Text::builder("field3").build())
        .build();

    let mut ctx = Context::new(Arc::new(schema));

    // Initial state
    println!("Initial state:");
    println!("  Is dirty: {}", ctx.is_dirty());
    println!("  Is valid: {}\n", ctx.is_valid());

    // Set some values
    ctx.set("field1", Value::text("value1")).unwrap();
    ctx.set("field2", Value::text("value2")).unwrap();

    println!("After setting values:");
    println!("  Is dirty: {}", ctx.is_dirty());

    let dirty = ctx.collect_dirty_values();
    println!("  Dirty fields: {:?}\n", dirty.keys().collect::<Vec<_>>());

    // Mark all clean
    ctx.mark_all_clean();
    println!("After marking clean:");
    println!("  Is dirty: {}\n", ctx.is_dirty());

    // Modify one field
    ctx.set("field1", Value::text("modified")).unwrap();
    println!("After modifying field1:");
    println!("  Is dirty: {}", ctx.is_dirty());

    let dirty = ctx.collect_dirty_values();
    println!("  Dirty fields: {:?}\n", dirty.keys().collect::<Vec<_>>());

    // Clear a field
    ctx.clear("field2").unwrap();
    println!("After clearing field2:");
    println!("  field2 value: {:?}", ctx.get("field2"));

    let dirty = ctx.collect_dirty_values();
    println!("  Dirty fields: {:?}\n", dirty.keys().collect::<Vec<_>>());

    // Collect all values
    let all_values = ctx.collect_values();
    println!("All values ({} total):", all_values.len());
    for (key, value) in all_values {
        println!("  {}: {:?}", key, value);
    }

    // Reset context
    println!("\nAfter reset:");
    ctx.reset();
    println!("  Is dirty: {}", ctx.is_dirty());
    println!("  Values count: {}", ctx.collect_values().len());
}
