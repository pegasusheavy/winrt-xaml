//! Benchmarking infrastructure verification
//!
//! This benchmark tests the criterion infrastructure itself
//! and can run without the full WinRT XAML implementation.
//!
//! Run with: cargo bench --bench infrastructure_test

use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use std::hint::black_box;
use std::sync::Arc;
use parking_lot::RwLock;

/// Test basic Rust operations to verify benchmark infrastructure
fn bench_infrastructure(c: &mut Criterion) {
    let mut group = c.benchmark_group("infrastructure");

    group.bench_function("vec_creation", |b| {
        b.iter(|| {
            black_box(Vec::<i32>::new())
        })
    });

    group.bench_function("vec_with_capacity", |b| {
        b.iter(|| {
            black_box(Vec::<i32>::with_capacity(100))
        })
    });

    for size in [10, 100, 1000].iter() {
        group.bench_with_input(BenchmarkId::new("vec_push", size), size, |b, &size| {
            b.iter(|| {
                let mut v = Vec::new();
                for i in 0..size {
                    v.push(black_box(i));
                }
                black_box(v)
            })
        });
    }

    group.finish();
}

/// Test Arc and RwLock patterns used throughout the library
fn bench_concurrency_primitives(c: &mut Criterion) {
    let mut group = c.benchmark_group("concurrency");

    group.bench_function("arc_new", |b| {
        b.iter(|| {
            black_box(Arc::new(0i32))
        })
    });

    group.bench_function("arc_clone", |b| {
        let arc = Arc::new(0i32);
        b.iter(|| {
            black_box(arc.clone())
        })
    });

    group.bench_function("rwlock_new", |b| {
        b.iter(|| {
            black_box(RwLock::new(0i32))
        })
    });

    group.bench_function("rwlock_read", |b| {
        let lock = RwLock::new(0i32);
        b.iter(|| {
            black_box(*lock.read())
        })
    });

    group.bench_function("rwlock_write", |b| {
        let lock = RwLock::new(0i32);
        b.iter(|| {
            let mut guard = lock.write();
            *guard += 1;
            black_box(*guard)
        })
    });

    group.bench_function("arc_rwlock_read", |b| {
        let state = Arc::new(RwLock::new(0i32));
        b.iter(|| {
            black_box(*state.read())
        })
    });

    group.bench_function("arc_rwlock_write", |b| {
        let state = Arc::new(RwLock::new(0i32));
        b.iter(|| {
            let mut guard = state.write();
            *guard += 1;
            black_box(*guard)
        })
    });

    group.finish();
}

/// Test string operations commonly used in UI
fn bench_string_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("strings");

    group.bench_function("string_new", |b| {
        b.iter(|| {
            black_box(String::new())
        })
    });

    group.bench_function("string_with_capacity", |b| {
        b.iter(|| {
            black_box(String::with_capacity(50))
        })
    });

    group.bench_function("format_simple", |b| {
        b.iter(|| {
            black_box(format!("Item {}", 42))
        })
    });

    group.bench_function("format_complex", |b| {
        b.iter(|| {
            black_box(format!("Item {} of {} ({}%)", 42, 100, 42.0))
        })
    });

    for count in [10, 50, 100].iter() {
        group.bench_with_input(
            BenchmarkId::new("repeated_format", count),
            count,
            |b, &count| {
                b.iter(|| {
                    let items: Vec<String> = (0..count)
                        .map(|i| format!("Item {}", i))
                        .collect();
                    black_box(items)
                })
            }
        );
    }

    group.finish();
}

/// Test collection operations
fn bench_collections(c: &mut Criterion) {
    let mut group = c.benchmark_group("collections");

    group.bench_function("vec_clone_small", |b| {
        let v = vec![1, 2, 3, 4, 5];
        b.iter(|| {
            black_box(v.clone())
        })
    });

    group.bench_function("vec_clone_large", |b| {
        let v: Vec<i32> = (0..1000).collect();
        b.iter(|| {
            black_box(v.clone())
        })
    });

    group.bench_function("vec_iter", |b| {
        let v: Vec<i32> = (0..1000).collect();
        b.iter(|| {
            let sum: i32 = v.iter().sum();
            black_box(sum)
        })
    });

    group.finish();
}

/// Simulate UI state patterns
fn bench_ui_patterns(c: &mut Criterion) {
    let mut group = c.benchmark_group("ui_patterns");

    #[derive(Clone)]
    struct UIState {
        count: i32,
        #[allow(dead_code)]
        text: String,
        enabled: bool,
    }

    group.bench_function("state_read", |b| {
        let state = Arc::new(RwLock::new(UIState {
            count: 0,
            text: "Hello".to_string(),
            enabled: true,
        }));

        b.iter(|| {
            let s = state.read();
            black_box((s.count, s.enabled))
        })
    });

    group.bench_function("state_update", |b| {
        let state = Arc::new(RwLock::new(UIState {
            count: 0,
            text: "Hello".to_string(),
            enabled: true,
        }));

        b.iter(|| {
            let mut s = state.write();
            s.count += 1;
            black_box(s.count)
        })
    });

    group.bench_function("state_clone", |b| {
        let state = Arc::new(RwLock::new(UIState {
            count: 0,
            text: "Hello".to_string(),
            enabled: true,
        }));

        b.iter(|| {
            black_box(state.read().clone())
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_infrastructure,
    bench_concurrency_primitives,
    bench_string_operations,
    bench_collections,
    bench_ui_patterns
);

criterion_main!(benches);

