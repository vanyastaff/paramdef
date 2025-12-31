//! Benchmarks for Value creation and operations.

use criterion::{Criterion, criterion_group, criterion_main};
use paramdef::core::Value;
use std::hint::black_box;

fn bench_value_creation(c: &mut Criterion) {
    c.bench_function("value_null", |b| {
        b.iter(|| {
            black_box(Value::Null);
        });
    });

    c.bench_function("value_bool", |b| {
        b.iter(|| {
            black_box(Value::Bool(true));
        });
    });

    c.bench_function("value_int", |b| {
        b.iter(|| {
            black_box(Value::Int(42));
        });
    });

    c.bench_function("value_float", |b| {
        b.iter(|| {
            black_box(Value::Float(3.14));
        });
    });

    c.bench_function("value_text_short", |b| {
        b.iter(|| {
            black_box(Value::text("hello"));
        });
    });

    c.bench_function("value_text_long", |b| {
        let long_text = "a".repeat(100);
        b.iter(|| {
            black_box(Value::text(&long_text));
        });
    });
}

fn bench_value_array(c: &mut Criterion) {
    c.bench_function("array_small", |b| {
        b.iter(|| {
            black_box(Value::array([Value::Int(1), Value::Int(2), Value::Int(3)]));
        });
    });

    c.bench_function("array_medium", |b| {
        let items: Vec<Value> = (0..100).map(Value::Int).collect();
        b.iter(|| {
            black_box(Value::array(items.clone()));
        });
    });

    c.bench_function("array_large", |b| {
        let items: Vec<Value> = (0..1000).map(Value::Int).collect();
        b.iter(|| {
            black_box(Value::array(items.clone()));
        });
    });
}

fn bench_value_object(c: &mut Criterion) {
    c.bench_function("object_small", |b| {
        b.iter(|| {
            black_box(Value::object([
                ("name", Value::text("Alice")),
                ("age", Value::Int(30)),
            ]));
        });
    });

    c.bench_function("object_medium", |b| {
        let pairs: Vec<(String, Value)> = (0..50)
            .map(|i| (format!("key_{i}"), Value::Int(i)))
            .collect();

        b.iter(|| {
            let pairs_ref: Vec<(&str, Value)> =
                pairs.iter().map(|(k, v)| (k.as_str(), v.clone())).collect();
            black_box(Value::object(pairs_ref));
        });
    });

    c.bench_function("object_large", |b| {
        let pairs: Vec<(String, Value)> = (0..200)
            .map(|i| (format!("key_{i}"), Value::Int(i)))
            .collect();

        b.iter(|| {
            let pairs_ref: Vec<(&str, Value)> =
                pairs.iter().map(|(k, v)| (k.as_str(), v.clone())).collect();
            black_box(Value::object(pairs_ref));
        });
    });

    c.bench_function("object_large_with_capacity", |b| {
        let pairs: Vec<(String, Value)> = (0..200)
            .map(|i| (format!("key_{i}"), Value::Int(i)))
            .collect();

        b.iter(|| {
            let pairs_ref: Vec<(&str, Value)> =
                pairs.iter().map(|(k, v)| (k.as_str(), v.clone())).collect();
            black_box(Value::object_with_capacity(200, pairs_ref));
        });
    });
}

fn bench_value_clone(c: &mut Criterion) {
    let simple = Value::Int(42);
    let text = Value::text("hello world");
    let array = Value::array((0..100).map(Value::Int).collect::<Vec<_>>());
    let nested = Value::object([
        ("name", Value::text("Test")),
        (
            "values",
            Value::array([Value::Int(1), Value::Int(2), Value::Int(3)]),
        ),
        (
            "nested",
            Value::object([("a", Value::Int(1)), ("b", Value::Int(2))]),
        ),
    ]);

    c.bench_function("clone_simple", |b| {
        b.iter(|| {
            black_box(simple.clone());
        });
    });

    c.bench_function("clone_text", |b| {
        b.iter(|| {
            black_box(text.clone());
        });
    });

    c.bench_function("clone_array", |b| {
        b.iter(|| {
            black_box(array.clone());
        });
    });

    c.bench_function("clone_nested", |b| {
        b.iter(|| {
            black_box(nested.clone());
        });
    });
}

fn bench_value_access(c: &mut Criterion) {
    let int = Value::Int(42);
    let float = Value::Float(3.14);
    let text = Value::text("hello");
    let array = Value::array([Value::Int(1), Value::Int(2), Value::Int(3)]);

    c.bench_function("access_as_i64", |b| {
        b.iter(|| {
            black_box(int.as_i64());
        });
    });

    c.bench_function("access_as_f64", |b| {
        b.iter(|| {
            black_box(float.as_f64());
        });
    });

    c.bench_function("access_as_text", |b| {
        b.iter(|| {
            black_box(text.as_text());
        });
    });

    c.bench_function("access_as_array", |b| {
        b.iter(|| {
            black_box(array.as_array());
        });
    });

    c.bench_function("access_is_null", |b| {
        b.iter(|| {
            black_box(int.is_null());
        });
    });

    c.bench_function("access_type_name", |b| {
        b.iter(|| {
            black_box(int.type_name());
        });
    });
}

fn bench_value_conversions(c: &mut Criterion) {
    c.bench_function("from_i32", |b| {
        b.iter(|| {
            let v: Value = black_box(42i32).into();
            black_box(v);
        });
    });

    c.bench_function("from_string", |b| {
        b.iter(|| {
            let v: Value = black_box(String::from("hello")).into();
            black_box(v);
        });
    });

    c.bench_function("from_option_some", |b| {
        b.iter(|| {
            let v: Value = black_box(Some(42i32)).into();
            black_box(v);
        });
    });

    c.bench_function("from_option_none", |b| {
        b.iter(|| {
            let v: Value = black_box(Option::<i32>::None).into();
            black_box(v);
        });
    });
}

criterion_group!(
    benches,
    bench_value_creation,
    bench_value_array,
    bench_value_object,
    bench_value_clone,
    bench_value_access,
    bench_value_conversions,
);

criterion_main!(benches);
