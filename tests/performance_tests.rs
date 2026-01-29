//! Performance optimization tests for paramdef.
//!
//! Tests Arc<Value> usage in events, RollbackStorage optimization,
//! and bulk operation performance.

#![cfg(feature = "events")]

use paramdef::context::Context;
use paramdef::core::Value;
use paramdef::event::{Event, EventBus};
use paramdef::schema::Schema;
use paramdef::types::Text;
use std::sync::Arc;

/// Tests for Event Arc<Value> optimization (T051)
mod event_arc_value {
    use super::*;

    #[test]
    fn test_event_value_changed_uses_arc() {
        // Create context with event bus
        let schema = Schema::builder()
            .parameter(Text::builder("name").build())
            .build();

        let bus = EventBus::new(64);
        let mut rx = bus.subscribe();
        let mut ctx = Context::with_event_bus(Arc::new(schema), bus);

        // Set a value
        ctx.set("name", Value::text("Alice")).unwrap();

        // Receive the ValueChanged event
        let event = rx.try_recv().ok().flatten();

        // Verify the event exists
        assert!(event.is_some(), "Should receive ValueChanged event");

        if let Some(Event::ValueChanged { new_value, .. }) = event {
            // The new_value is now Arc<Value>
            // We can verify by checking Arc::strong_count works
            let count = Arc::strong_count(&new_value);
            assert!(count >= 1, "Arc strong count should be at least 1");
        } else {
            panic!("Expected ValueChanged event");
        }
    }

    #[test]
    fn test_event_value_changing_uses_arc() {
        // Similar test for ValueChanging event
        let schema = Schema::builder()
            .parameter(Text::builder("email").build())
            .build();

        let bus = EventBus::new(64);
        let mut rx = bus.subscribe();
        let mut ctx = Context::with_event_bus(Arc::new(schema), bus);

        // Set initial value
        ctx.set("email", Value::text("old@example.com")).unwrap();

        // Clear the old events
        while rx.try_recv().is_ok() {}

        // Set a new value (should emit ValueChanging)
        ctx.set("email", Value::text("new@example.com")).unwrap();

        // Should receive ValueChanging event first
        let event = rx.try_recv().ok().flatten();
        assert!(event.is_some(), "Should receive ValueChanging event");

        if let Some(Event::ValueChanging {
            new_value,
            old_value,
            ..
        }) = event
        {
            // Both should be Arc<Value>
            let _new_count = Arc::strong_count(&new_value);
            if let Some(old) = old_value {
                let _old_count = Arc::strong_count(&old);
            }
        } else {
            panic!("Expected ValueChanging event");
        }
    }

    #[test]
    #[test]
    fn test_multiple_subscribers_share_arc() {
        // Multiple subscribers should share the same Arc<Value>
        // without cloning the underlying Value

        let schema = Schema::builder()
            .parameter(Text::builder("shared").build())
            .build();

        let bus = EventBus::new(64);
        let mut rx1 = bus.subscribe();
        let mut rx2 = bus.subscribe();
        let mut rx3 = bus.subscribe();

        let mut ctx = Context::with_event_bus(Arc::new(schema), bus);

        // Set a value that will be broadcast to all 3 subscribers
        ctx.set("shared", Value::text("broadcast")).unwrap();

        // All subscribers should receive the event
        // try_recv() returns Result<Option<Event>, RecvError>
        let event1 = rx1.try_recv().ok().flatten();
        let event2 = rx2.try_recv().ok().flatten();
        let event3 = rx3.try_recv().ok().flatten();

        assert!(event1.is_some());
        assert!(event2.is_some());
        assert!(event3.is_some());

        // Extract Arc<Value> from events
        let v1 = if let Some(Event::ValueChanged { new_value, .. }) = event1 {
            new_value
        } else {
            panic!("Expected ValueChanged event from rx1");
        };

        let v2 = if let Some(Event::ValueChanged { new_value, .. }) = event2 {
            new_value
        } else {
            panic!("Expected ValueChanged event from rx2");
        };

        let v3 = if let Some(Event::ValueChanged { new_value, .. }) = event3 {
            new_value
        } else {
            panic!("Expected ValueChanged event from rx3");
        };

        // All three should point to the same Arc
        // Strong count should be at least 3 (one for each subscriber)
        let count = Arc::strong_count(&v1);
        assert!(count >= 3, "Arc should be shared, count: {}", count);

        // Verify they're actually the same Arc by comparing pointers
        assert!(Arc::ptr_eq(&v1, &v2));
        assert!(Arc::ptr_eq(&v2, &v3));
    }
}

/// Tests for RollbackStorage optimization (T055)
#[cfg(test)]
#[allow(unused_mut)] // Windows doesn't detect unused mut, Linux does
mod rollback_storage {
    use super::*;
    use paramdef::context::rollback::RollbackStorage;
    use paramdef::core::Key;

    #[test]
    fn test_rollback_storage_small_uses_stack() {
        // Test that small transactions (1-8 fields) use stack buffer
        let mut storage = RollbackStorage::new();

        // Store up to 8 values
        for i in 0..8 {
            let key = Key::from(format!("field{}", i));
            storage.store(key, Some(Value::Int(i)));
        }

        // Should still be Small variant
        assert!(
            storage.is_small(),
            "Storage with 8 items should use Small variant"
        );
        assert_eq!(storage.len(), 8);
    }

    #[test]
    fn test_rollback_storage_large_uses_heap() {
        // Test that large transactions (>8 fields) use heap
        let mut storage = RollbackStorage::new();

        // Store 9 values (triggers upgrade)
        for i in 0..9 {
            let key = Key::from(format!("field{}", i));
            storage.store(key, Some(Value::Int(i)));
        }

        // Should be Large variant now
        assert!(
            storage.is_large(),
            "Storage with 9 items should use Large variant"
        );
        assert_eq!(storage.len(), 9);
    }

    #[test]
    fn test_rollback_storage_upgrade_small_to_large() {
        // Test automatic upgrade from Small to Large variant
        let mut storage = RollbackStorage::new();

        // Add 8 items (stays Small)
        for i in 0..8 {
            let key = Key::from(format!("field{}", i));
            storage.store(key, Some(Value::Int(i)));
        }
        assert!(storage.is_small());

        // Add 9th item (triggers upgrade)
        storage.store("field8".into(), Some(Value::Int(8)));
        assert!(storage.is_large());

        // All 9 items should still be accessible
        assert_eq!(storage.len(), 9);
    }

    #[test]
    fn test_rollback_storage_iter() {
        // Test iterator over stored values
        let mut storage = RollbackStorage::new();

        storage.store("a".into(), Some(Value::Int(1)));
        storage.store("b".into(), Some(Value::Int(2)));
        storage.store("c".into(), None);

        let items: Vec<_> = storage.iter().collect();
        assert_eq!(items.len(), 3);

        // Check that all keys are present
        let keys: Vec<&Key> = items.iter().map(|(k, _)| *k).collect();
        assert!(keys.iter().any(|k| k.as_str() == "a"));
        assert!(keys.iter().any(|k| k.as_str() == "b"));
        assert!(keys.iter().any(|k| k.as_str() == "c"));
    }

    #[test]
    fn test_rollback_storage_clear() {
        // Test clear operation
        let mut storage = RollbackStorage::new();

        storage.store("key1".into(), Some(Value::Int(1)));
        storage.store("key2".into(), Some(Value::Int(2)));
        assert_eq!(storage.len(), 2);

        storage.clear();
        assert_eq!(storage.len(), 0);
        assert!(storage.is_empty());
    }

    #[test]
    fn test_rollback_storage_with_capacity() {
        // Test creating storage with known capacity
        let mut storage = RollbackStorage::with_capacity(10);

        // Should start as Large variant for capacity > 8
        assert!(storage.is_large());

        // Small capacity should start as Small
        let small_storage = RollbackStorage::with_capacity(5);
        assert!(small_storage.is_small());
    }
}

/// Tests for Context::set_many_transactional (T057)
#[cfg(test)]
mod transactional_updates {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    #[ignore = "T058 not yet implemented"]
    fn test_set_many_transactional_success() {
        // Test successful transactional update
        unimplemented!("Context::set_many_transactional not yet implemented");
    }

    #[test]
    #[ignore = "T058 not yet implemented"]
    fn test_set_many_transactional_rollback_on_validation_error() {
        // Test rollback when validation fails
        unimplemented!("Context::set_many_transactional not yet implemented");
    }

    #[test]
    #[ignore = "T058 not yet implemented"]
    fn test_set_many_transactional_small_storage() {
        // Test that <8 fields use stack storage
        unimplemented!("Context::set_many_transactional not yet implemented");
    }

    #[test]
    #[ignore = "T058 not yet implemented"]
    fn test_set_many_transactional_large_storage() {
        // Test that >8 fields use heap storage
        unimplemented!("Context::set_many_transactional not yet implemented");
    }
}

/// Tests for Context::get_many (T059)
#[cfg(test)]
mod bulk_getters {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    #[ignore = "T060 not yet implemented"]
    fn test_context_get_many_existing_keys() {
        // Test retrieving multiple existing keys
        unimplemented!("Context::get_many not yet implemented");
    }

    #[test]
    #[ignore = "T060 not yet implemented"]
    fn test_context_get_many_mixed_existing_missing() {
        // Test mix of existing and missing keys
        unimplemented!("Context::get_many not yet implemented");
    }

    #[test]
    #[ignore = "T060 not yet implemented"]
    fn test_context_get_many_empty_keys() {
        // Test empty key list
        unimplemented!("Context::get_many not yet implemented");
    }

    #[test]
    #[ignore = "T060 not yet implemented"]
    fn test_context_get_many_single_lookup() {
        // Verify only 1 batch operation occurs
        unimplemented!("Context::get_many not yet implemented");
    }
}

/// Tests for zero-copy values() iterator (T061)
#[cfg(test)]
mod zero_copy_iteration {
    #[allow(unused_imports)]
    use super::*;
    #[allow(unused_imports)]
    use paramdef::core::Key;

    #[test]
    fn test_context_values_zero_copy() {
        // Verify Context::values() returns references, not clones
        // This is a documentation task, but we can verify the behavior

        let schema = Schema::builder()
            .parameter(Text::builder("field1").build())
            .parameter(Text::builder("field2").build())
            .build();

        let mut ctx = Context::new(Arc::new(schema));
        ctx.set("field1", Value::text("value1")).unwrap();
        ctx.set("field2", Value::text("value2")).unwrap();

        // Iterate over values
        let values: Vec<_> = ctx.values().collect();

        // Should have 2 values
        assert_eq!(values.len(), 2);

        // Values should be (&Key, &Value) tuples
        // The compiler ensures this by the signature of values()
        for (key, value) in values {
            // These should be references, not owned values
            let _: &Key = key;
            let _: &Value = value;
        }
    }
}
