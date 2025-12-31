//! Benchmarks for parameter creation and access.

#[cfg(not(codspeed))]
use criterion::{Criterion, criterion_group, criterion_main};
#[cfg(codspeed)]
use codspeed_criterion_compat::{Criterion, criterion_group, criterion_main};

use paramdef::node::{Leaf, Node};
use paramdef::parameter::{Boolean, Number, Select, SelectOption, Text, Vector};
use std::hint::black_box;

fn bench_text_creation(c: &mut Criterion) {
    c.bench_function("text_minimal", |b| {
        b.iter(|| {
            black_box(Text::builder("username").build());
        });
    });

    c.bench_function("text_full", |b| {
        b.iter(|| {
            black_box(
                Text::builder("username")
                    .label("Username")
                    .description("Enter your username")
                    .default("guest")
                    .required()
                    .build(),
            );
        });
    });
}

fn bench_number_creation(c: &mut Criterion) {
    c.bench_function("number_minimal", |b| {
        b.iter(|| {
            black_box(Number::builder("count").build());
        });
    });

    c.bench_function("number_full", |b| {
        b.iter(|| {
            black_box(
                Number::builder("temperature")
                    .label("Temperature")
                    .description("Current temperature")
                    .default(20.0)
                    .required()
                    .build(),
            );
        });
    });
}

fn bench_boolean_creation(c: &mut Criterion) {
    c.bench_function("boolean_minimal", |b| {
        b.iter(|| {
            black_box(Boolean::builder("enabled").build());
        });
    });

    c.bench_function("boolean_full", |b| {
        b.iter(|| {
            black_box(
                Boolean::builder("enabled")
                    .label("Enabled")
                    .description("Enable this feature")
                    .default(true)
                    .build(),
            );
        });
    });
}

fn bench_vector_creation(c: &mut Criterion) {
    c.bench_function("vector3_minimal", |b| {
        b.iter(|| {
            black_box(Vector::builder::<f64, 3>("position").build());
        });
    });

    c.bench_function("vector3_full", |b| {
        b.iter(|| {
            black_box(
                Vector::builder::<f64, 3>("position")
                    .label("Position")
                    .description("3D position")
                    .default([1.0, 2.0, 3.0])
                    .build(),
            );
        });
    });

    c.bench_function("vector4_full", |b| {
        b.iter(|| {
            black_box(
                Vector::builder::<f64, 4>("color")
                    .label("Color")
                    .default([1.0, 0.0, 0.0, 1.0])
                    .build(),
            );
        });
    });
}

fn bench_select_creation(c: &mut Criterion) {
    c.bench_function("select_minimal", |b| {
        b.iter(|| {
            black_box(Select::single("choice").build());
        });
    });

    c.bench_function("select_with_options", |b| {
        b.iter(|| {
            black_box(
                Select::single("method")
                    .label("HTTP Method")
                    .options(vec![
                        SelectOption::simple("GET"),
                        SelectOption::simple("POST"),
                        SelectOption::simple("PUT"),
                        SelectOption::simple("DELETE"),
                    ])
                    .default_single("GET")
                    .build(),
            );
        });
    });

    c.bench_function("select_many_options", |b| {
        let options: Vec<SelectOption> = (0..100)
            .map(|i| SelectOption::new(format!("opt_{i}"), format!("Option {i}")))
            .collect();

        b.iter(|| {
            black_box(
                Select::single("country")
                    .options(options.clone())
                    .searchable()
                    .build(),
            );
        });
    });
}

fn bench_parameter_access(c: &mut Criterion) {
    let text = Text::builder("username")
        .label("Username")
        .default("guest")
        .build();

    let number = Number::builder("count").default(42.0).build();

    let vector = Vector::builder::<f64, 3>("pos")
        .default([1.0, 2.0, 3.0])
        .build();

    c.bench_function("text_key_access", |b| {
        b.iter(|| {
            black_box(text.key());
        });
    });

    c.bench_function("text_metadata_access", |b| {
        b.iter(|| {
            black_box(text.metadata().label());
        });
    });

    c.bench_function("text_default_value", |b| {
        b.iter(|| {
            black_box(text.default_value());
        });
    });

    c.bench_function("number_default_value", |b| {
        b.iter(|| {
            black_box(number.default_value());
        });
    });

    c.bench_function("vector_default_value", |b| {
        b.iter(|| {
            black_box(vector.default_value());
        });
    });
}

criterion_group!(
    benches,
    bench_text_creation,
    bench_number_creation,
    bench_boolean_creation,
    bench_vector_creation,
    bench_select_creation,
    bench_parameter_access,
);

criterion_main!(benches);
