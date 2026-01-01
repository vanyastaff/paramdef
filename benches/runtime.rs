//! Benchmarks for RuntimeNode operations.

#[cfg(codspeed)]
use codspeed_criterion_compat::{Criterion, criterion_group, criterion_main};
#[cfg(not(codspeed))]
use criterion::{Criterion, criterion_group, criterion_main};

use paramdef::core::Value;
use paramdef::runtime::{ErasedRuntimeNode, RuntimeNode};
use paramdef::types::leaf::{Number, Text};
use paramdef::types::traits::Node;
use std::hint::black_box;
use std::sync::Arc;

fn bench_runtime_node_creation(c: &mut Criterion) {
    let text_schema = Arc::new(Text::builder("name").label("Name").build());
    let number_schema = Arc::new(Number::builder("count").build());

    c.bench_function("runtime_node_new_text", |b| {
        b.iter(|| {
            black_box(RuntimeNode::new(Arc::clone(&text_schema)));
        });
    });

    c.bench_function("runtime_node_new_number", |b| {
        b.iter(|| {
            black_box(RuntimeNode::new(Arc::clone(&number_schema)));
        });
    });

    c.bench_function("runtime_node_clone", |b| {
        let node = RuntimeNode::new(Arc::clone(&text_schema));
        b.iter(|| {
            black_box(node.clone());
        });
    });
}

fn bench_erased_runtime_node(c: &mut Criterion) {
    let text_schema = Arc::new(Text::builder("name").build());

    c.bench_function("erased_from_typed", |b| {
        b.iter(|| {
            let typed = RuntimeNode::new(Arc::clone(&text_schema));
            black_box(ErasedRuntimeNode::new(typed));
        });
    });

    c.bench_function("erased_from_arc", |b| {
        let schema: Arc<dyn Node> = Arc::new(Text::builder("name").build());
        b.iter(|| {
            black_box(ErasedRuntimeNode::from_arc(Arc::clone(&schema)));
        });
    });

    c.bench_function("erased_clone", |b| {
        let schema: Arc<dyn Node> = Arc::new(Text::builder("name").build());
        let erased = ErasedRuntimeNode::from_arc(schema);
        b.iter(|| {
            black_box(erased.clone());
        });
    });
}

fn bench_runtime_node_operations(c: &mut Criterion) {
    let schema = Arc::new(Text::builder("name").build());

    c.bench_function("runtime_set_value", |b| {
        let mut node = RuntimeNode::new(Arc::clone(&schema));
        b.iter(|| {
            node.set_value(Value::text("test"));
            black_box(&node);
        });
    });

    c.bench_function("runtime_get_value", |b| {
        let mut node = RuntimeNode::new(Arc::clone(&schema));
        node.set_value(Value::text("test"));
        b.iter(|| {
            black_box(node.value());
        });
    });

    c.bench_function("runtime_clear_value", |b| {
        let mut node = RuntimeNode::new(Arc::clone(&schema));
        b.iter(|| {
            node.set_value(Value::text("test"));
            node.clear_value();
            black_box(&node);
        });
    });

    c.bench_function("runtime_state_access", |b| {
        let mut node = RuntimeNode::new(Arc::clone(&schema));
        node.set_value(Value::text("test"));
        b.iter(|| {
            black_box(node.state().is_dirty());
            black_box(node.state().is_touched());
            black_box(node.state().is_valid());
        });
    });

    c.bench_function("runtime_state_modify", |b| {
        let mut node = RuntimeNode::new(Arc::clone(&schema));
        b.iter(|| {
            node.state_mut().mark_dirty();
            node.state_mut().mark_touched();
            node.state_mut().mark_clean();
            black_box(&node);
        });
    });

    c.bench_function("runtime_reset", |b| {
        let mut node = RuntimeNode::new(Arc::clone(&schema));
        b.iter(|| {
            node.set_value(Value::text("test"));
            node.state_mut().mark_touched();
            node.reset();
            black_box(&node);
        });
    });
}

fn bench_runtime_node_schema_access(c: &mut Criterion) {
    let schema = Arc::new(
        Text::builder("username")
            .label("Username")
            .description("Enter your username")
            .build(),
    );
    let node = RuntimeNode::new(Arc::clone(&schema));

    c.bench_function("runtime_node_key", |b| {
        b.iter(|| {
            black_box(node.node().key());
        });
    });

    c.bench_function("runtime_node_metadata", |b| {
        b.iter(|| {
            black_box(node.node().metadata());
        });
    });

    c.bench_function("runtime_node_label", |b| {
        b.iter(|| {
            black_box(node.node().metadata().label());
        });
    });

    c.bench_function("runtime_node_kind", |b| {
        b.iter(|| {
            black_box(node.node().kind());
        });
    });
}

criterion_group!(
    benches,
    bench_runtime_node_creation,
    bench_erased_runtime_node,
    bench_runtime_node_operations,
    bench_runtime_node_schema_access,
);

criterion_main!(benches);
