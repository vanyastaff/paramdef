//! Throughput benchmarks for large-scale context operations.
//!
//! Measures performance at scale: 1M parameter contexts, bulk operations,
//! and high-frequency updates typical in workflow engines and no-code platforms.

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

fn create_large_schema(size: usize) -> Arc<Schema> {
    let mut builder = Schema::builder();
    for i in 0..size {
        builder = builder
            .parameter(Text::builder(format!("text_{i}")).build())
            .parameter(Number::builder(format!("num_{i}")).build())
            .parameter(Text::builder(format!("status_{i}")).build());
    }
    Arc::new(builder.build())
}

fn bench_context_creation(c: &mut Criterion) {
    c.bench_function("context_create_100_params", |b| {
        let schema = create_large_schema(33); // 33 * 3 = 99 params
        b.iter(|| {
            black_box(Context::new(Arc::clone(&schema)));
        });
    });

    c.bench_function("context_create_1000_params", |b| {
        let schema = create_large_schema(333); // 333 * 3 = 999 params
        b.iter(|| {
            black_box(Context::new(Arc::clone(&schema)));
        });
    });

    c.bench_function("context_create_10000_params", |b| {
        let schema = create_large_schema(3333); // 3333 * 3 = 9999 params
        b.iter(|| {
            black_box(Context::new(Arc::clone(&schema)));
        });
    });
}

fn bench_bulk_set_throughput(c: &mut Criterion) {
    let schema = create_large_schema(1000); // 3000 params

    c.bench_function("bulk_set_100_sequential", |b| {
        b.iter(|| {
            let mut ctx = Context::new(Arc::clone(&schema));
            for i in 0..100 {
                let _ = black_box(ctx.set(&format!("text_{i}"), Value::text("test")));
            }
        });
    });

    c.bench_function("bulk_set_1000_sequential", |b| {
        b.iter(|| {
            let mut ctx = Context::new(Arc::clone(&schema));
            for i in 0..1000 {
                let _ = black_box(ctx.set(&format!("text_{i}"), Value::text("test")));
            }
        });
    });

    c.bench_function("bulk_set_100_transactional", |b| {
        b.iter(|| {
            let mut ctx = Context::new(Arc::clone(&schema));
            let values: Vec<_> = (0..100)
                .map(|i| (format!("text_{i}"), Value::text("test")))
                .collect();
            let _ = black_box(ctx.set_many_transactional(values));
        });
    });
}

fn bench_bulk_get_throughput(c: &mut Criterion) {
    let schema = create_large_schema(1000); // 3000 params
    let mut ctx = Context::new(Arc::clone(&schema));

    // Pre-populate context
    for i in 0..1000 {
        let _ = ctx.set(&format!("text_{i}"), Value::text("test"));
        let _ = ctx.set(&format!("num_{i}"), Value::Int(i as i64));
    }

    c.bench_function("bulk_get_100_sequential", |b| {
        b.iter(|| {
            for i in 0..100 {
                let _ = black_box(ctx.get(&format!("text_{i}")));
            }
        });
    });

    c.bench_function("bulk_get_100_batch", |b| {
        let keys: Vec<_> = (0..100).map(|i| format!("text_{i}")).collect();
        b.iter(|| {
            let _ = black_box(ctx.get_many(&keys));
        });
    });

    c.bench_function("bulk_get_1000_sequential", |b| {
        b.iter(|| {
            for i in 0..1000 {
                let _ = black_box(ctx.get(&format!("text_{i}")));
            }
        });
    });

    c.bench_function("bulk_get_1000_batch", |b| {
        let keys: Vec<_> = (0..1000).map(|i| format!("text_{i}")).collect();
        b.iter(|| {
            let _ = black_box(ctx.get_many(&keys));
        });
    });
}

fn bench_iteration_throughput(c: &mut Criterion) {
    let schema = create_large_schema(1000); // 3000 params
    let mut ctx = Context::new(Arc::clone(&schema));

    // Pre-populate context
    for i in 0..1000 {
        let _ = ctx.set(&format!("text_{i}"), Value::text("test"));
        let _ = ctx.set(&format!("num_{i}"), Value::Int(i as i64));
    }

    c.bench_function("iterate_values_2000", |b| {
        b.iter(|| {
            let count = black_box(ctx.values().count());
            assert!(count > 0);
        });
    });

    c.bench_function("iterate_values_collect_2000", |b| {
        b.iter(|| {
            let values: Vec<_> = black_box(ctx.values().collect());
            assert!(!values.is_empty());
        });
    });

    c.bench_function("iterate_dirty_values_2000", |b| {
        b.iter(|| {
            let count = black_box(ctx.dirty_values().count());
            assert!(count > 0);
        });
    });
}

fn bench_state_operations(c: &mut Criterion) {
    let schema = create_large_schema(1000); // 3000 params
    let mut ctx = Context::new(Arc::clone(&schema));

    // Pre-populate context
    for i in 0..1000 {
        let _ = ctx.set(&format!("text_{i}"), Value::text("test"));
    }

    c.bench_function("check_is_dirty_1000", |b| {
        b.iter(|| {
            black_box(ctx.is_dirty());
        });
    });

    c.bench_function("check_is_valid_1000", |b| {
        b.iter(|| {
            black_box(ctx.is_valid());
        });
    });

    c.bench_function("mark_all_clean_1000", |b| {
        b.iter(|| {
            ctx.mark_all_clean();
        });
    });

    c.bench_function("save_dirty_to_map_1000", |b| {
        b.iter(|| {
            let _ = black_box(ctx.save_dirty_to_map());
        });
    });
}

fn bench_workflow_simulation(c: &mut Criterion) {
    // Simulate a typical workflow engine scenario:
    // 1. Create context with 500 parameters
    // 2. Set 50 input values
    // 3. Read 100 times during execution
    // 4. Update 30 intermediate values
    // 5. Extract 20 output values

    let schema = create_large_schema(166); // 166 * 3 = 498 params

    c.bench_function("workflow_simulation_500_params", |b| {
        b.iter(|| {
            let mut ctx = Context::new(Arc::clone(&schema));

            // Set inputs (50 values)
            for i in 0..50 {
                let _ = ctx.set(&format!("text_{i}"), Value::text("input"));
            }

            // Simulate execution (100 reads)
            for _ in 0..10 {
                for i in 0..10 {
                    let _ = black_box(ctx.get(&format!("text_{i}")));
                }
            }

            // Update intermediate values (30 updates)
            for i in 50..80 {
                let _ = ctx.set(&format!("num_{i}"), Value::Int(i as i64));
            }

            // Extract outputs (20 values)
            let output_keys: Vec<_> = (0..20).map(|i| format!("text_{i}")).collect();
            let _ = black_box(ctx.get_many(&output_keys));

            // Check if dirty
            black_box(ctx.is_dirty());
        });
    });
}

criterion_group!(
    throughput_benches,
    bench_context_creation,
    bench_bulk_set_throughput,
    bench_bulk_get_throughput,
    bench_iteration_throughput,
    bench_state_operations,
    bench_workflow_simulation
);

criterion_main!(throughput_benches);
