//! Benchmarks for transactional updates with RollbackStorage optimization.
//!
//! This benchmark measures the performance improvement from using stack-allocated
//! RollbackStorage for small transactions (≤8 fields) versus heap-allocated for large.

#[cfg(codspeed)]
use codspeed_criterion_compat::{Criterion, criterion_group, criterion_main};
#[cfg(not(codspeed))]
use criterion::{Criterion, criterion_group, criterion_main};

use paramdef::context::Context;
use paramdef::core::Value;
use paramdef::schema::Schema;
use paramdef::types::leaf::{Number, Text};
use std::hint::black_box;
use std::sync::Arc;

fn create_benchmark_schema(size: usize) -> Arc<Schema> {
    let mut builder = Schema::builder();
    for i in 0..size {
        builder = builder
            .parameter(Text::builder(format!("field_{i}")).build())
            .parameter(Number::builder(format!("num_{i}")).build());
    }
    Arc::new(builder.build())
}

fn bench_transactional_small(c: &mut Criterion) {
    // Small transactions (1-8 fields) should use stack buffer
    let schema = create_benchmark_schema(10);

    c.bench_function("transactional_1_field", |b| {
        b.iter(|| {
            let mut ctx = Context::new(Arc::clone(&schema));
            let values = vec![("field_0".to_string(), Value::text("test"))];
            let _ = black_box(ctx.set_many_transactional(values));
        });
    });

    c.bench_function("transactional_4_fields", |b| {
        b.iter(|| {
            let mut ctx = Context::new(Arc::clone(&schema));
            let values: Vec<_> = (0..4)
                .map(|i| (format!("field_{i}"), Value::text("test")))
                .collect();
            let _ = black_box(ctx.set_many_transactional(values));
        });
    });

    c.bench_function("transactional_8_fields", |b| {
        b.iter(|| {
            let mut ctx = Context::new(Arc::clone(&schema));
            let values: Vec<_> = (0..8)
                .map(|i| (format!("field_{i}"), Value::text("test")))
                .collect();
            let _ = black_box(ctx.set_many_transactional(values));
        });
    });
}

fn bench_transactional_large(c: &mut Criterion) {
    // Large transactions (>8 fields) should use heap HashMap
    let schema = create_benchmark_schema(100);

    c.bench_function("transactional_10_fields", |b| {
        b.iter(|| {
            let mut ctx = Context::new(Arc::clone(&schema));
            let values: Vec<_> = (0..10)
                .map(|i| (format!("field_{i}"), Value::text("test")))
                .collect();
            let _ = black_box(ctx.set_many_transactional(values));
        });
    });

    c.bench_function("transactional_20_fields", |b| {
        b.iter(|| {
            let mut ctx = Context::new(Arc::clone(&schema));
            let values: Vec<_> = (0..20)
                .map(|i| (format!("field_{i}"), Value::text("test")))
                .collect();
            let _ = black_box(ctx.set_many_transactional(values));
        });
    });

    c.bench_function("transactional_50_fields", |b| {
        b.iter(|| {
            let mut ctx = Context::new(Arc::clone(&schema));
            let values: Vec<_> = (0..50)
                .map(|i| (format!("field_{i}"), Value::text("test")))
                .collect();
            let _ = black_box(ctx.set_many_transactional(values));
        });
    });

    c.bench_function("transactional_100_fields", |b| {
        b.iter(|| {
            let mut ctx = Context::new(Arc::clone(&schema));
            let values: Vec<_> = (0..100)
                .map(|i| (format!("field_{i}"), Value::text("test")))
                .collect();
            let _ = black_box(ctx.set_many_transactional(values));
        });
    });
}

fn bench_transactional_rollback(c: &mut Criterion) {
    // Benchmark rollback performance (error case)
    let schema = create_benchmark_schema(20);

    c.bench_function("transactional_rollback_8_fields", |b| {
        b.iter(|| {
            let mut ctx = Context::new(Arc::clone(&schema));
            // Set initial values
            for i in 0..8 {
                let _ = ctx.set(&format!("field_{i}"), Value::text("original"));
            }

            // Try to set with invalid key (should rollback)
            let values: Vec<_> = (0..7)
                .map(|i| (format!("field_{i}"), Value::text("new")))
                .chain(std::iter::once((
                    "invalid_key".to_string(),
                    Value::text("fail"),
                )))
                .collect();

            let _ = black_box(ctx.set_many_transactional(values));
        });
    });

    c.bench_function("transactional_rollback_20_fields", |b| {
        b.iter(|| {
            let mut ctx = Context::new(Arc::clone(&schema));
            // Set initial values
            for i in 0..20 {
                let _ = ctx.set(&format!("field_{i}"), Value::text("original"));
            }

            // Try to set with invalid key (should rollback)
            let values: Vec<_> = (0..19)
                .map(|i| (format!("field_{i}"), Value::text("new")))
                .chain(std::iter::once((
                    "invalid_key".to_string(),
                    Value::text("fail"),
                )))
                .collect();

            let _ = black_box(ctx.set_many_transactional(values));
        });
    });
}

fn bench_partial_vs_transactional(c: &mut Criterion) {
    // Compare partial vs transactional for success cases
    let schema = create_benchmark_schema(30);

    c.bench_function("partial_20_fields", |b| {
        b.iter(|| {
            let mut ctx = Context::new(Arc::clone(&schema));
            let values: Vec<_> = (0..20)
                .map(|i| (format!("field_{i}"), Value::text("test")))
                .collect();
            let _ = black_box(ctx.set_many_partial(values));
        });
    });

    c.bench_function("transactional_20_fields_vs_partial", |b| {
        b.iter(|| {
            let mut ctx = Context::new(Arc::clone(&schema));
            let values: Vec<_> = (0..20)
                .map(|i| (format!("field_{i}"), Value::text("test")))
                .collect();
            let _ = black_box(ctx.set_many_transactional(values));
        });
    });
}

criterion_group!(
    transactional_benches,
    bench_transactional_small,
    bench_transactional_large,
    bench_transactional_rollback,
    bench_partial_vs_transactional
);

criterion_main!(transactional_benches);
