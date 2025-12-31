//! Benchmarks for core types: Key, Metadata, Flags.

#[cfg(codspeed)]
use codspeed_criterion_compat::{Criterion, criterion_group, criterion_main};
#[cfg(not(codspeed))]
use criterion::{Criterion, criterion_group, criterion_main};

use paramdef::core::{Flags, Key, Metadata, StateFlags};
use std::collections::HashMap;
use std::hint::black_box;

fn bench_key_creation(c: &mut Criterion) {
    c.bench_function("key_short", |b| {
        b.iter(|| {
            black_box(Key::new("name"));
        });
    });

    c.bench_function("key_medium", |b| {
        b.iter(|| {
            black_box(Key::new("user_profile_settings"));
        });
    });

    c.bench_function("key_long", |b| {
        let long_key = "a".repeat(50);
        b.iter(|| {
            black_box(Key::new(&long_key));
        });
    });

    c.bench_function("key_from_string", |b| {
        b.iter(|| {
            black_box(Key::from(String::from("username")));
        });
    });
}

fn bench_key_operations(c: &mut Criterion) {
    let key1 = Key::new("username");
    let key2 = Key::new("username");
    let key3 = Key::new("password");

    c.bench_function("key_eq_same", |b| {
        b.iter(|| {
            black_box(key1 == key2);
        });
    });

    c.bench_function("key_eq_different", |b| {
        b.iter(|| {
            black_box(key1 == key3);
        });
    });

    c.bench_function("key_eq_str", |b| {
        b.iter(|| {
            black_box(key1 == "username");
        });
    });

    c.bench_function("key_clone", |b| {
        b.iter(|| {
            black_box(key1.clone());
        });
    });

    c.bench_function("key_as_str", |b| {
        b.iter(|| {
            black_box(key1.as_str());
        });
    });

    c.bench_function("key_is_empty", |b| {
        b.iter(|| {
            black_box(key1.is_empty());
        });
    });
}

fn bench_key_hash(c: &mut Criterion) {
    let keys: Vec<Key> = (0..100).map(|i| Key::new(format!("key_{i}"))).collect();
    let mut map: HashMap<Key, i32> = HashMap::new();
    for (i, key) in keys.iter().enumerate() {
        map.insert(key.clone(), i as i32);
    }

    c.bench_function("key_hash_insert", |b| {
        b.iter(|| {
            let mut m = HashMap::new();
            for key in &keys {
                m.insert(key.clone(), 1);
            }
            black_box(m);
        });
    });

    c.bench_function("key_hash_lookup", |b| {
        b.iter(|| {
            black_box(map.get(&keys[50]));
        });
    });

    c.bench_function("key_hash_lookup_miss", |b| {
        let missing = Key::new("nonexistent");
        b.iter(|| {
            black_box(map.get(&missing));
        });
    });
}

fn bench_metadata_creation(c: &mut Criterion) {
    c.bench_function("metadata_minimal", |b| {
        b.iter(|| {
            black_box(Metadata::new("field"));
        });
    });

    c.bench_function("metadata_with_label", |b| {
        b.iter(|| {
            black_box(Metadata::builder("field").label("Field Label").build());
        });
    });

    c.bench_function("metadata_full", |b| {
        b.iter(|| {
            black_box(
                Metadata::builder("field")
                    .label("Field Label")
                    .description("A detailed description of this field")
                    .group("settings")
                    .tag("important")
                    .tag("user")
                    .build(),
            );
        });
    });
}

fn bench_metadata_access(c: &mut Criterion) {
    let meta = Metadata::builder("field")
        .label("Field Label")
        .description("Description")
        .group("settings")
        .tag("important")
        .build();

    c.bench_function("metadata_key", |b| {
        b.iter(|| {
            black_box(meta.key());
        });
    });

    c.bench_function("metadata_label", |b| {
        b.iter(|| {
            black_box(meta.label());
        });
    });

    c.bench_function("metadata_description", |b| {
        b.iter(|| {
            black_box(meta.description());
        });
    });

    c.bench_function("metadata_group", |b| {
        b.iter(|| {
            black_box(meta.group());
        });
    });

    c.bench_function("metadata_tags", |b| {
        b.iter(|| {
            black_box(meta.tags());
        });
    });

    c.bench_function("metadata_display_label", |b| {
        b.iter(|| {
            black_box(meta.display_label());
        });
    });

    c.bench_function("metadata_clone", |b| {
        b.iter(|| {
            black_box(meta.clone());
        });
    });
}

fn bench_flags(c: &mut Criterion) {
    c.bench_function("flags_default", |b| {
        b.iter(|| {
            black_box(Flags::default());
        });
    });

    c.bench_function("flags_required", |b| {
        b.iter(|| {
            black_box(Flags::REQUIRED);
        });
    });

    c.bench_function("flags_combine", |b| {
        b.iter(|| {
            black_box(Flags::REQUIRED | Flags::READONLY | Flags::HIDDEN);
        });
    });

    let flags = Flags::REQUIRED | Flags::SENSITIVE;

    c.bench_function("flags_contains", |b| {
        b.iter(|| {
            black_box(flags.contains(Flags::REQUIRED));
            black_box(flags.contains(Flags::READONLY));
        });
    });

    c.bench_function("flags_is_required", |b| {
        b.iter(|| {
            black_box(flags.is_required());
        });
    });

    c.bench_function("flags_is_empty", |b| {
        b.iter(|| {
            black_box(flags.is_empty());
        });
    });
}

fn bench_state_flags(c: &mut Criterion) {
    c.bench_function("state_flags_default", |b| {
        b.iter(|| {
            black_box(StateFlags::default());
        });
    });

    c.bench_function("state_flags_initial", |b| {
        b.iter(|| {
            black_box(StateFlags::initial());
        });
    });

    let mut flags = StateFlags::initial();

    c.bench_function("state_flags_modify", |b| {
        b.iter(|| {
            flags.insert(StateFlags::DIRTY);
            flags.insert(StateFlags::TOUCHED);
            flags.remove(StateFlags::DIRTY);
            black_box(&flags);
        });
    });

    c.bench_function("state_flags_check", |b| {
        b.iter(|| {
            black_box(flags.contains(StateFlags::VALID));
            black_box(flags.contains(StateFlags::DIRTY));
            black_box(flags.contains(StateFlags::TOUCHED));
        });
    });
}

criterion_group!(
    benches,
    bench_key_creation,
    bench_key_operations,
    bench_key_hash,
    bench_metadata_creation,
    bench_metadata_access,
    bench_flags,
    bench_state_flags,
);

criterion_main!(benches);
