//! Benchmarks for Schema and Context operations.

#[cfg(codspeed)]
use codspeed_criterion_compat::{Criterion, criterion_group, criterion_main};
#[cfg(not(codspeed))]
use criterion::{Criterion, criterion_group, criterion_main};

use paramdef::context::Context;
use paramdef::core::Value;
use paramdef::parameter::{Number, Text};
use paramdef::schema::Schema;
use std::hint::black_box;
use std::sync::Arc;

fn create_small_schema() -> Schema {
    Schema::builder()
        .parameter(Text::builder("name").label("Name").build())
        .parameter(Text::builder("email").label("Email").build())
        .parameter(Number::builder("age").label("Age").build())
        .build()
}

fn create_medium_schema() -> Schema {
    let mut builder = Schema::builder();
    for i in 0..50 {
        builder = builder.parameter(Text::builder(format!("field_{i}")).build());
    }
    builder.build()
}

fn create_large_schema() -> Schema {
    let mut builder = Schema::builder();
    for i in 0..200 {
        builder = builder.parameter(Text::builder(format!("field_{i}")).build());
    }
    builder.build()
}

fn bench_schema_creation(c: &mut Criterion) {
    c.bench_function("schema_small_3", |b| {
        b.iter(|| {
            black_box(create_small_schema());
        });
    });

    c.bench_function("schema_medium_50", |b| {
        b.iter(|| {
            black_box(create_medium_schema());
        });
    });

    c.bench_function("schema_large_200", |b| {
        b.iter(|| {
            black_box(create_large_schema());
        });
    });
}

fn bench_schema_lookup(c: &mut Criterion) {
    let small = create_small_schema();
    let medium = create_medium_schema();
    let large = create_large_schema();

    c.bench_function("schema_get_small", |b| {
        b.iter(|| {
            black_box(small.get("name"));
            black_box(small.get("email"));
            black_box(small.get("age"));
        });
    });

    c.bench_function("schema_get_medium_first", |b| {
        b.iter(|| {
            black_box(medium.get("field_0"));
        });
    });

    c.bench_function("schema_get_medium_middle", |b| {
        b.iter(|| {
            black_box(medium.get("field_25"));
        });
    });

    c.bench_function("schema_get_medium_last", |b| {
        b.iter(|| {
            black_box(medium.get("field_49"));
        });
    });

    c.bench_function("schema_get_large_miss", |b| {
        b.iter(|| {
            black_box(large.get("nonexistent"));
        });
    });
}

fn bench_schema_iteration(c: &mut Criterion) {
    let small = create_small_schema();
    let medium = create_medium_schema();
    let large = create_large_schema();

    c.bench_function("schema_iter_small", |b| {
        b.iter(|| {
            for param in small.iter() {
                black_box(param.key());
            }
        });
    });

    c.bench_function("schema_iter_medium", |b| {
        b.iter(|| {
            for param in medium.iter() {
                black_box(param.key());
            }
        });
    });

    c.bench_function("schema_iter_large", |b| {
        b.iter(|| {
            for param in large.iter() {
                black_box(param.key());
            }
        });
    });

    c.bench_function("schema_keys_large", |b| {
        b.iter(|| {
            for key in large.keys() {
                black_box(key);
            }
        });
    });
}

fn bench_context_creation(c: &mut Criterion) {
    let small = Arc::new(create_small_schema());
    let medium = Arc::new(create_medium_schema());
    let large = Arc::new(create_large_schema());

    c.bench_function("context_from_small", |b| {
        b.iter(|| {
            black_box(Context::new(Arc::clone(&small)));
        });
    });

    c.bench_function("context_from_medium", |b| {
        b.iter(|| {
            black_box(Context::new(Arc::clone(&medium)));
        });
    });

    c.bench_function("context_from_large", |b| {
        b.iter(|| {
            black_box(Context::new(Arc::clone(&large)));
        });
    });
}

fn bench_context_operations(c: &mut Criterion) {
    let schema = Arc::new(create_medium_schema());

    c.bench_function("context_set_value", |b| {
        let mut ctx = Context::new(Arc::clone(&schema));
        b.iter(|| {
            black_box(ctx.set("field_25", Value::text("test")));
        });
    });

    c.bench_function("context_get_value", |b| {
        let mut ctx = Context::new(Arc::clone(&schema));
        ctx.set("field_25", Value::text("test"));
        b.iter(|| {
            black_box(ctx.get("field_25"));
        });
    });

    c.bench_function("context_get_miss", |b| {
        let ctx = Context::new(Arc::clone(&schema));
        b.iter(|| {
            black_box(ctx.get("nonexistent"));
        });
    });

    c.bench_function("context_is_dirty", |b| {
        let mut ctx = Context::new(Arc::clone(&schema));
        ctx.set("field_0", Value::text("a"));
        ctx.set("field_25", Value::text("b"));
        ctx.set("field_49", Value::text("c"));
        b.iter(|| {
            black_box(ctx.is_dirty());
        });
    });

    c.bench_function("context_collect_values", |b| {
        let mut ctx = Context::new(Arc::clone(&schema));
        for i in 0..10 {
            ctx.set(&format!("field_{i}"), Value::text("value"));
        }
        b.iter(|| {
            black_box(ctx.collect_values());
        });
    });

    c.bench_function("context_collect_dirty", |b| {
        let mut ctx = Context::new(Arc::clone(&schema));
        for i in 0..10 {
            ctx.set(&format!("field_{i}"), Value::text("value"));
        }
        // Mark some as clean
        for i in 0..5 {
            if let Some(node) = ctx.node_mut(&format!("field_{i}")) {
                node.state_mut().mark_clean();
            }
        }
        b.iter(|| {
            black_box(ctx.collect_dirty_values());
        });
    });
}

fn bench_context_bulk(c: &mut Criterion) {
    let schema = Arc::new(create_large_schema());

    c.bench_function("context_set_all_200", |b| {
        b.iter(|| {
            let mut ctx = Context::new(Arc::clone(&schema));
            for i in 0..200 {
                ctx.set(&format!("field_{i}"), Value::text("value"));
            }
            black_box(ctx);
        });
    });

    c.bench_function("context_mark_all_clean", |b| {
        let mut ctx = Context::new(Arc::clone(&schema));
        for i in 0..200 {
            ctx.set(&format!("field_{i}"), Value::text("value"));
        }
        b.iter(|| {
            ctx.mark_all_clean();
            // Re-dirty for next iteration
            ctx.set("field_0", Value::text("x"));
        });
    });

    c.bench_function("context_reset", |b| {
        let mut ctx = Context::new(Arc::clone(&schema));
        for i in 0..50 {
            ctx.set(&format!("field_{i}"), Value::text("value"));
        }
        b.iter(|| {
            ctx.reset();
            // Re-set for next iteration
            ctx.set("field_0", Value::text("x"));
        });
    });
}

criterion_group!(
    benches,
    bench_schema_creation,
    bench_schema_lookup,
    bench_schema_iteration,
    bench_context_creation,
    bench_context_operations,
    bench_context_bulk,
);

criterion_main!(benches);
