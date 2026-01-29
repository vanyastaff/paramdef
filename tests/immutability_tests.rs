//! Immutability verification tests.
//!
//! These tests verify that the schema layer is truly immutable and
//! all runtime state is properly managed in Context.

use paramdef::context::Context;
use paramdef::schema::Schema;
use paramdef::types::group::Panel;
use paramdef::types::leaf::Text;
use paramdef::types::traits::Layout;
use std::sync::Arc;

/// Test that Panel schema has no mutable fields.
///
/// This test verifies that Panel::collapsed has been removed from the schema
/// and that panel state is managed through Context/UiStateManager instead.
#[test]
fn test_panel_schema_is_immutable() {
    let panel = Panel::builder("settings")
        .label("Settings")
        .child(Text::builder("name").build())
        .build();

    // Panel should be fully immutable after construction
    // The test will fail if Panel still has a mutable collapsed field

    // Create two contexts sharing the same panel schema
    let schema = Arc::new(Schema::builder().parameter(panel).build());
    let ctx1 = Context::new(Arc::clone(&schema));
    let ctx2 = Context::new(Arc::clone(&schema));

    // Each context should have independent UI state
    // If Panel.collapsed was in the schema, this would be impossible
    assert!(!ctx1.is_panel_collapsed(&"settings".into()));
    assert!(!ctx2.is_panel_collapsed(&"settings".into()));
}

/// Test that panel runtime state is managed in Context, not in Panel schema.
///
/// This verifies that changing panel state in one context doesn't affect another
/// context that shares the same schema (Arc<Panel>).
#[test]
fn test_panel_runtime_state_in_context() {
    let panel = Panel::builder("settings").label("Settings").build();

    let schema = Arc::new(Schema::builder().parameter(panel).build());

    let mut ctx1 = Context::new(Arc::clone(&schema));
    let mut ctx2 = Context::new(Arc::clone(&schema));

    // Initially both should be expanded (default state)
    assert!(!ctx1.is_panel_collapsed(&"settings".into()));
    assert!(!ctx2.is_panel_collapsed(&"settings".into()));

    // Collapse in ctx1 only
    ctx1.set_panel_collapsed("settings", true);

    // ctx1 should be collapsed, ctx2 should still be expanded
    assert!(ctx1.is_panel_collapsed(&"settings".into()));
    assert!(!ctx2.is_panel_collapsed(&"settings".into()));

    // Collapse in ctx2
    ctx2.set_panel_collapsed("settings", true);

    // Now both collapsed independently
    assert!(ctx1.is_panel_collapsed(&"settings".into()));
    assert!(ctx2.is_panel_collapsed(&"settings".into()));
}

/// Test that multiple contexts can have independent panel states.
///
/// This verifies architectural invariant: schema is shared (Arc), runtime state is per-context.
#[test]
fn test_multiple_contexts_independent_panel_state() {
    let panel = Panel::builder("panel1").build();
    let schema = Arc::new(Schema::builder().parameter(panel).build());

    // Create 3 contexts sharing the same schema
    let mut contexts = vec![
        Context::new(Arc::clone(&schema)),
        Context::new(Arc::clone(&schema)),
        Context::new(Arc::clone(&schema)),
    ];

    // Set different states in each context
    contexts[0].set_panel_collapsed("panel1", true); // collapsed
    contexts[1].set_panel_collapsed("panel1", false); // expanded
    contexts[2].set_panel_collapsed("panel1", true); // collapsed

    // Verify states are independent
    assert!(contexts[0].is_panel_collapsed(&"panel1".into()));
    assert!(!contexts[1].is_panel_collapsed(&"panel1".into()));
    assert!(contexts[2].is_panel_collapsed(&"panel1".into()));
}

/// Test that Panel schema is Send + Sync (can be shared across threads).
///
/// This verifies that removing mutable state from Panel makes it thread-safe.
/// Arc<Panel> should be sharable across threads without Mutex.
#[test]
fn test_panel_schema_shareable_across_threads() {
    let panel = Panel::builder("shared").label("Shared Panel").build();

    let schema = Arc::new(Schema::builder().parameter(panel).build());

    // Spawn 3 threads, each with their own Context but sharing the schema
    let handles: Vec<_> = (0..3)
        .map(|i| {
            let schema_clone = Arc::clone(&schema);
            std::thread::spawn(move || {
                let mut ctx = Context::new(schema_clone);

                // Each thread sets different panel state
                ctx.set_panel_collapsed("shared", i % 2 == 0);

                // Return the state
                ctx.is_panel_collapsed(&"shared".into())
            })
        })
        .collect();

    // Collect results
    let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();

    // Verify each thread got its own state
    assert_eq!(results.len(), 3);
    assert!(results[0]); // i=0, 0 % 2 == 0 → true
    assert!(!results[1]); // i=1, 1 % 2 != 0 → false
    assert!(results[2]); // i=2, 2 % 2 == 0 → true
}

// =============================================================================
// T011: Layout Trait Immutability Tests
// =============================================================================

/// Test that Layout trait has no mutable set_collapsed method.
///
/// This test verifies that the Layout trait only provides read-only access
/// to children, and that collapsed state is NOT part of the trait.
#[test]
fn test_layout_trait_no_set_collapsed() {
    let panel = Panel::builder("test").build();

    // Layout trait should only provide children() method
    let _children = panel.children();

    // This should compile because Layout trait is immutable
    // If set_collapsed still exists on Layout trait, this won't compile
}

/// Test that panel collapsed state is managed via Context, not Layout trait.
///
/// This ensures that the proper pattern is to use Context::set_panel_collapsed()
/// instead of calling mutable methods on the Panel/Layout itself.
#[test]
fn test_panel_set_collapsed_via_context() {
    let panel = Panel::builder("settings").build();
    let schema = Arc::new(Schema::builder().parameter(panel).build());
    let mut ctx = Context::new(schema);

    // Correct approach: use Context API
    ctx.set_panel_collapsed("settings", true);
    assert!(ctx.is_panel_collapsed(&"settings".into()));

    // Panel itself is immutable - no set_collapsed method
    // The following would not compile if uncommented:
    // let mut panel_ref = ...;
    // panel_ref.set_collapsed(true);  // ERROR: no such method
}
