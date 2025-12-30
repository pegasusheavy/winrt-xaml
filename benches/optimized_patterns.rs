//! Optimized pattern benchmarks
//!
//! This demonstrates optimized versions of common patterns
//! to compare against the baseline infrastructure test.

use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;
use std::sync::Arc;
use std::fmt::Write;
use parking_lot::RwLock;

/// Benchmark optimized string operations
fn bench_optimized_strings(c: &mut Criterion) {
    let mut group = c.benchmark_group("optimized_strings");
    
    // Baseline: repeated format! calls
    group.bench_function("baseline_repeated_format", |b| {
        b.iter(|| {
            let items: Vec<String> = (0..100)
                .map(|i| format!("Item {}", i))
                .collect();
            black_box(items)
        })
    });
    
    // Optimized: reuse string buffer
    group.bench_function("optimized_string_reuse", |b| {
        b.iter(|| {
            let mut items = Vec::with_capacity(100);
            let mut buffer = String::with_capacity(20);
            for i in 0..100 {
                buffer.clear();
                write!(&mut buffer, "Item {}", i).unwrap();
                items.push(buffer.clone());
            }
            black_box(items)
        })
    });
    
    // Super optimized: pre-allocate and reuse
    group.bench_function("super_optimized_string_pool", |b| {
        b.iter(|| {
            let items: Vec<String> = (0..100)
                .map(|i| {
                    let mut s = String::with_capacity(10);
                    write!(&mut s, "Item {}", i).unwrap();
                    s
                })
                .collect();
            black_box(items)
        })
    });
    
    group.finish();
}

/// Benchmark optimized state management
fn bench_optimized_state(c: &mut Criterion) {
    let mut group = c.benchmark_group("optimized_state");
    
    #[derive(Clone)]
    struct State {
        count: i32,
        items: Vec<String>,
        enabled: bool,
    }
    
    let state = Arc::new(RwLock::new(State {
        count: 0,
        items: vec!["a".to_string(), "b".to_string(), "c".to_string()],
        enabled: true,
    }));
    
    // Baseline: clone entire state
    group.bench_function("baseline_full_clone", |b| {
        b.iter(|| {
            black_box(state.read().clone())
        })
    });
    
    // Optimized: read only what you need
    group.bench_function("optimized_selective_read", |b| {
        b.iter(|| {
            let s = state.read();
            black_box((s.count, s.enabled))
        })
    });
    
    // Super optimized: cache read lock
    group.bench_function("super_optimized_batch_read", |b| {
        b.iter(|| {
            let s = state.read();
            let results = (s.count, s.enabled, s.items.len());
            black_box(results)
        })
    });
    
    group.finish();
}

/// Benchmark optimized vec operations
fn bench_optimized_vec(c: &mut Criterion) {
    let mut group = c.benchmark_group("optimized_vec");
    
    // Baseline: no pre-allocation
    group.bench_function("baseline_vec_push_1000", |b| {
        b.iter(|| {
            let mut v = Vec::new();
            for i in 0..1000 {
                v.push(black_box(i));
            }
            black_box(v)
        })
    });
    
    // Optimized: with_capacity
    group.bench_function("optimized_vec_with_capacity", |b| {
        b.iter(|| {
            let mut v = Vec::with_capacity(1000);
            for i in 0..1000 {
                v.push(black_box(i));
            }
            black_box(v)
        })
    });
    
    // Super optimized: collect with known size
    group.bench_function("super_optimized_vec_collect", |b| {
        b.iter(|| {
            let v: Vec<i32> = (0..1000).collect();
            black_box(v)
        })
    });
    
    group.finish();
}

/// Benchmark optimized collection operations
fn bench_optimized_collections(c: &mut Criterion) {
    let mut group = c.benchmark_group("optimized_collections");
    
    let large_vec: Vec<i32> = (0..1000).collect();
    
    // Baseline: clone entire vec
    group.bench_function("baseline_vec_clone", |b| {
        b.iter(|| {
            black_box(large_vec.clone())
        })
    });
    
    // Optimized: only clone slice you need
    group.bench_function("optimized_slice_clone", |b| {
        b.iter(|| {
            black_box(large_vec[0..10].to_vec())
        })
    });
    
    // Super optimized: use references when possible
    group.bench_function("super_optimized_slice_ref", |b| {
        b.iter(|| {
            black_box(&large_vec[0..10])
        })
    });
    
    group.finish();
}

/// Benchmark Arc optimization patterns
fn bench_optimized_arc(c: &mut Criterion) {
    let mut group = c.benchmark_group("optimized_arc");
    
    // Baseline: create new Arc every time
    group.bench_function("baseline_arc_new_each_time", |b| {
        b.iter(|| {
            let data = vec![1, 2, 3, 4, 5];
            black_box(Arc::new(data))
        })
    });
    
    // Optimized: reuse Arc
    group.bench_function("optimized_arc_reuse", |b| {
        let arc_data = Arc::new(vec![1, 2, 3, 4, 5]);
        b.iter(|| {
            black_box(&arc_data)
        })
    });
    
    // Clone cost
    group.bench_function("arc_clone_cost", |b| {
        let arc_data = Arc::new(vec![1, 2, 3, 4, 5]);
        b.iter(|| {
            black_box(arc_data.clone())
        })
    });
    
    group.finish();
}

/// Benchmark lock-free read patterns
fn bench_optimized_locking(c: &mut Criterion) {
    let mut group = c.benchmark_group("optimized_locking");
    
    let state = Arc::new(RwLock::new(vec![1, 2, 3, 4, 5]));
    
    // Baseline: lock for each access
    group.bench_function("baseline_lock_per_access", |b| {
        b.iter(|| {
            let sum: i32 = (0..10)
                .map(|_| state.read().iter().sum::<i32>())
                .sum();
            black_box(sum)
        })
    });
    
    // Optimized: lock once, access many times
    group.bench_function("optimized_lock_once", |b| {
        b.iter(|| {
            let guard = state.read();
            let sum: i32 = (0..10)
                .map(|_| guard.iter().sum::<i32>())
                .sum();
            black_box(sum)
        })
    });
    
    group.finish();
}

criterion_group!(
    benches,
    bench_optimized_strings,
    bench_optimized_state,
    bench_optimized_vec,
    bench_optimized_collections,
    bench_optimized_arc,
    bench_optimized_locking
);

criterion_main!(benches);

