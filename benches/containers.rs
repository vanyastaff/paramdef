//! Benchmarks for container types: Object, List, Mode.

#[cfg(codspeed)]
use codspeed_criterion_compat::{Criterion, criterion_group, criterion_main};
#[cfg(not(codspeed))]
use criterion::{Criterion, criterion_group, criterion_main};

use paramdef::types::container::{List, Mode, Object};
use paramdef::types::leaf::Text;
use paramdef::types::traits::Container;
use std::hint::black_box;

// =============================================================================
// Object benchmarks
// =============================================================================

fn bench_object_creation(c: &mut Criterion) {
    c.bench_function("object_empty", |b| {
        b.iter(|| {
            black_box(Object::empty("empty"));
        });
    });

    c.bench_function("object_3_fields", |b| {
        b.iter(|| {
            black_box(
                Object::builder("person")
                    .label("Person")
                    .field("name", Text::builder("name").build())
                    .field("email", Text::builder("email").build())
                    .field("phone", Text::builder("phone").build())
                    .build()
                    .unwrap(),
            );
        });
    });

    c.bench_function("object_10_fields", |b| {
        b.iter(|| {
            let mut builder = Object::builder("config").label("Configuration");
            for i in 0..10 {
                builder =
                    builder.field(format!("field_{i}"), Text::builder(format!("f{i}")).build());
            }
            black_box(builder.build().unwrap());
        });
    });

    c.bench_function("object_nested", |b| {
        b.iter(|| {
            let inner = Object::builder("inner")
                .field("value", Text::builder("value").build())
                .build()
                .unwrap();
            black_box(
                Object::builder("outer")
                    .field("nested", inner)
                    .field("name", Text::builder("name").build())
                    .build()
                    .unwrap(),
            );
        });
    });
}

fn bench_object_operations(c: &mut Criterion) {
    let obj = Object::builder("config")
        .label("Configuration")
        .field("host", Text::builder("host").build())
        .field("port", Text::builder("port").build())
        .field("username", Text::builder("username").build())
        .field("password", Text::builder("password").build())
        .field("database", Text::builder("database").build())
        .build()
        .unwrap();

    c.bench_function("object_get_field_first", |b| {
        b.iter(|| {
            black_box(obj.get_field("host"));
        });
    });

    c.bench_function("object_get_field_middle", |b| {
        b.iter(|| {
            black_box(obj.get_field("username"));
        });
    });

    c.bench_function("object_get_field_last", |b| {
        b.iter(|| {
            black_box(obj.get_field("database"));
        });
    });

    c.bench_function("object_get_field_miss", |b| {
        b.iter(|| {
            black_box(obj.get_field("nonexistent"));
        });
    });

    c.bench_function("object_has_field", |b| {
        b.iter(|| {
            black_box(obj.has_field("username"));
        });
    });

    c.bench_function("object_field_count", |b| {
        b.iter(|| {
            black_box(obj.field_count());
        });
    });

    c.bench_function("object_field_keys_iter", |b| {
        b.iter(|| {
            for key in obj.field_keys() {
                black_box(key);
            }
        });
    });

    c.bench_function("object_children", |b| {
        b.iter(|| {
            black_box(obj.children());
        });
    });

    c.bench_function("object_clone", |b| {
        b.iter(|| {
            black_box(obj.clone());
        });
    });
}

// =============================================================================
// List benchmarks
// =============================================================================

fn bench_list_creation(c: &mut Criterion) {
    c.bench_function("list_simple", |b| {
        b.iter(|| {
            black_box(
                List::builder("tags")
                    .label("Tags")
                    .item_template(Text::builder("tag").build())
                    .build()
                    .unwrap(),
            );
        });
    });

    c.bench_function("list_with_constraints", |b| {
        b.iter(|| {
            black_box(
                List::builder("items")
                    .label("Items")
                    .description("A list of items")
                    .item_template(Text::builder("item").build())
                    .min_items(1)
                    .max_items(100)
                    .unique(true)
                    .sortable(true)
                    .build()
                    .unwrap(),
            );
        });
    });

    c.bench_function("list_with_object_template", |b| {
        b.iter(|| {
            let template = Object::builder("header")
                .field("name", Text::builder("name").build())
                .field("value", Text::builder("value").build())
                .build()
                .unwrap();
            black_box(
                List::builder("headers")
                    .label("HTTP Headers")
                    .item_template(template)
                    .build()
                    .unwrap(),
            );
        });
    });
}

fn bench_list_operations(c: &mut Criterion) {
    let list = List::builder("items")
        .label("Items")
        .item_template(Text::builder("item").build())
        .min_items(1)
        .max_items(50)
        .unique(true)
        .sortable(true)
        .build()
        .unwrap();

    c.bench_function("list_item_template", |b| {
        b.iter(|| {
            black_box(list.item_template());
        });
    });

    c.bench_function("list_min_items", |b| {
        b.iter(|| {
            black_box(list.min_items());
        });
    });

    c.bench_function("list_max_items", |b| {
        b.iter(|| {
            black_box(list.max_items());
        });
    });

    c.bench_function("list_is_unique", |b| {
        b.iter(|| {
            black_box(list.is_unique());
        });
    });

    c.bench_function("list_is_sortable", |b| {
        b.iter(|| {
            black_box(list.is_sortable());
        });
    });

    c.bench_function("list_children", |b| {
        b.iter(|| {
            black_box(list.children());
        });
    });

    c.bench_function("list_clone", |b| {
        b.iter(|| {
            black_box(list.clone());
        });
    });
}

// =============================================================================
// Mode benchmarks
// =============================================================================

fn bench_mode_creation(c: &mut Criterion) {
    c.bench_function("mode_2_variants", |b| {
        b.iter(|| {
            black_box(
                Mode::builder("toggle")
                    .label("Toggle")
                    .variant("on", "On", Object::empty("on_config"))
                    .variant("off", "Off", Object::empty("off_config"))
                    .default_variant("off")
                    .build()
                    .unwrap(),
            );
        });
    });

    c.bench_function("mode_5_variants", |b| {
        b.iter(|| {
            black_box(
                Mode::builder("auth")
                    .label("Authentication")
                    .variant("none", "No Auth", Object::empty("none"))
                    .variant(
                        "basic",
                        "Basic Auth",
                        Object::builder("basic")
                            .field("username", Text::builder("username").build())
                            .field("password", Text::builder("password").build())
                            .build()
                            .unwrap(),
                    )
                    .variant(
                        "bearer",
                        "Bearer Token",
                        Object::builder("bearer")
                            .field("token", Text::builder("token").build())
                            .build()
                            .unwrap(),
                    )
                    .variant(
                        "api_key",
                        "API Key",
                        Object::builder("api_key")
                            .field("key", Text::builder("key").build())
                            .field("header", Text::builder("header").build())
                            .build()
                            .unwrap(),
                    )
                    .variant(
                        "oauth2",
                        "OAuth 2.0",
                        Object::builder("oauth2")
                            .field("client_id", Text::builder("client_id").build())
                            .field("client_secret", Text::builder("client_secret").build())
                            .field("scope", Text::builder("scope").build())
                            .build()
                            .unwrap(),
                    )
                    .default_variant("none")
                    .build()
                    .unwrap(),
            );
        });
    });
}

fn bench_mode_operations(c: &mut Criterion) {
    let mode = Mode::builder("auth")
        .label("Authentication")
        .variant("none", "No Auth", Object::empty("none"))
        .variant(
            "basic",
            "Basic Auth",
            Object::builder("basic")
                .field("username", Text::builder("username").build())
                .field("password", Text::builder("password").build())
                .build()
                .unwrap(),
        )
        .variant(
            "bearer",
            "Bearer Token",
            Object::builder("bearer")
                .field("token", Text::builder("token").build())
                .build()
                .unwrap(),
        )
        .default_variant("none")
        .build()
        .unwrap();

    c.bench_function("mode_get_variant_first", |b| {
        b.iter(|| {
            black_box(mode.get_variant("none"));
        });
    });

    c.bench_function("mode_get_variant_last", |b| {
        b.iter(|| {
            black_box(mode.get_variant("bearer"));
        });
    });

    c.bench_function("mode_get_variant_miss", |b| {
        b.iter(|| {
            black_box(mode.get_variant("nonexistent"));
        });
    });

    c.bench_function("mode_variant_count", |b| {
        b.iter(|| {
            black_box(mode.variant_count());
        });
    });

    c.bench_function("mode_default_variant", |b| {
        b.iter(|| {
            black_box(mode.default_variant());
        });
    });

    c.bench_function("mode_variant_keys_iter", |b| {
        b.iter(|| {
            for key in mode.variant_keys() {
                black_box(key);
            }
        });
    });

    c.bench_function("mode_variants", |b| {
        b.iter(|| {
            black_box(mode.variants());
        });
    });

    c.bench_function("mode_children", |b| {
        b.iter(|| {
            black_box(mode.children());
        });
    });

    c.bench_function("mode_clone", |b| {
        b.iter(|| {
            black_box(mode.clone());
        });
    });
}

criterion_group!(
    benches,
    bench_object_creation,
    bench_object_operations,
    bench_list_creation,
    bench_list_operations,
    bench_mode_creation,
    bench_mode_operations,
);

criterion_main!(benches);
