//! Benchmarks for event system with Arc<Value> optimization.
//!
//! This benchmark measures the performance improvement from using Arc<Value>
//! in events instead of Value, which reduces clones from 3× to 1× per set().

#[cfg(codspeed)]
use codspeed_criterion_compat::{Criterion, criterion_group, criterion_main};
#[cfg(not(codspeed))]
use criterion::{Criterion, criterion_group, criterion_main};

#[cfg(feature = "events")]
use paramdef::context::Context;
#[cfg(feature = "events")]
use paramdef::core::Value;
#[cfg(feature = "events")]
use paramdef::event::EventBus;
#[cfg(feature = "events")]
use paramdef::schema::Schema;
#[cfg(feature = "events")]
use paramdef::types::leaf::{Number, Text};
#[cfg(feature = "events")]
use std::hint::black_box;
#[cfg(feature = "events")]
use std::sync::Arc;

#[cfg(feature = "events")]
fn create_benchmark_schema() -> Arc<Schema> {
    let mut builder = Schema::builder();
    for i in 0..100 {
        builder = builder
            .parameter(Text::builder(format!("text_{i}")).build())
            .parameter(Number::builder(format!("num_{i}")).build());
    }
    Arc::new(builder.build())
}

#[cfg(feature = "events")]
fn bench_set_with_events(c: &mut Criterion) {
    let schema = create_benchmark_schema();

    c.bench_function("event_set_100_values", |b| {
        b.iter(|| {
            let bus = EventBus::new(1024);
            let _sub = bus.subscribe(); // Subscribe to ensure events are sent
            let mut ctx = Context::with_event_bus(Arc::clone(&schema), bus);

            for i in 0..100 {
                let _ = black_box(ctx.set(&format!("text_{i}"), Value::text("test")));
                let _ = black_box(ctx.set(&format!("num_{i}"), Value::Int(i as i64)));
            }
        });
    });

    c.bench_function("event_set_1000_values", |b| {
        b.iter(|| {
            let bus = EventBus::new(4096);
            let _sub = bus.subscribe();
            let mut ctx = Context::with_event_bus(Arc::clone(&schema), bus);

            for _ in 0..5 {
                for i in 0..100 {
                    let _ = black_box(ctx.set(&format!("text_{i}"), Value::text("test")));
                    let _ = black_box(ctx.set(&format!("num_{i}"), Value::Int(i as i64)));
                }
            }
        });
    });
}

#[cfg(feature = "events")]
fn bench_set_without_events(c: &mut Criterion) {
    let schema = create_benchmark_schema();

    c.bench_function("no_event_set_100_values", |b| {
        b.iter(|| {
            let mut ctx = Context::new(Arc::clone(&schema));

            for i in 0..100 {
                let _ = black_box(ctx.set(&format!("text_{i}"), Value::text("test")));
                let _ = black_box(ctx.set(&format!("num_{i}"), Value::Int(i as i64)));
            }
        });
    });
}

#[cfg(feature = "events")]
fn bench_bulk_operations_with_events(c: &mut Criterion) {
    let schema = create_benchmark_schema();

    c.bench_function("event_set_many_transactional_10", |b| {
        b.iter(|| {
            let bus = EventBus::new(256);
            let _sub = bus.subscribe();
            let mut ctx = Context::with_event_bus(Arc::clone(&schema), bus);

            let values: Vec<_> = (0..10)
                .map(|i| (format!("text_{i}"), Value::text("test")))
                .collect();
            let _ = black_box(ctx.set_many_transactional(values));
        });
    });

    c.bench_function("event_set_many_transactional_50", |b| {
        b.iter(|| {
            let bus = EventBus::new(1024);
            let _sub = bus.subscribe();
            let mut ctx = Context::with_event_bus(Arc::clone(&schema), bus);

            let values: Vec<_> = (0..50)
                .map(|i| (format!("text_{i}"), Value::text("test")))
                .collect();
            let _ = black_box(ctx.set_many_transactional(values));
        });
    });
}

#[cfg(feature = "events")]
criterion_group!(
    event_benches,
    bench_set_with_events,
    bench_set_without_events,
    bench_bulk_operations_with_events
);

#[cfg(not(feature = "events"))]
fn bench_noop(c: &mut Criterion) {
    c.bench_function("noop", |b| b.iter(|| {}));
}

#[cfg(not(feature = "events"))]
criterion_group!(event_benches, bench_noop);

criterion_main!(event_benches);
