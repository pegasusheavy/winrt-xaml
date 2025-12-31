//! Benchmarks for reactive state management.

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use winrt_xaml::reactive::{Property, ObservableCollection, Computed};

fn property_creation(c: &mut Criterion) {
    c.bench_function("property_new", |b| {
        b.iter(|| {
            black_box(Property::new(42));
        });
    });
}

fn property_get_set(c: &mut Criterion) {
    let prop = Property::new(0);
    
    c.bench_function("property_get", |b| {
        b.iter(|| {
            black_box(prop.get());
        });
    });
    
    c.bench_function("property_set", |b| {
        let mut counter = 0;
        b.iter(|| {
            prop.set(black_box(counter));
            counter += 1;
        });
    });
}

fn property_subscribe(c: &mut Criterion) {
    let prop = Property::new(0);
    
    c.bench_function("property_subscribe_1", |b| {
        b.iter(|| {
            let _id = prop.subscribe(|_| {});
        });
    });
    
    // Benchmark with multiple subscribers
    let mut group = c.benchmark_group("property_notify");
    for subscriber_count in [1, 5, 10, 50].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(subscriber_count),
            subscriber_count,
            |b, &count| {
                let prop = Property::new(0);
                for _ in 0..count {
                    prop.subscribe(|_| {});
                }
                b.iter(|| {
                    prop.set(black_box(42));
                });
            },
        );
    }
    group.finish();
}

fn property_update(c: &mut Criterion) {
    let prop = Property::new(0);
    
    c.bench_function("property_update", |b| {
        b.iter(|| {
            prop.update(|v| *v += 1);
        });
    });
}

fn collection_operations(c: &mut Criterion) {
    c.bench_function("collection_new", |b| {
        b.iter(|| {
            black_box(ObservableCollection::<i32>::new());
        });
    });
    
    c.bench_function("collection_push", |b| {
        let col = ObservableCollection::new();
        let mut counter = 0;
        b.iter(|| {
            col.push(black_box(counter));
            counter += 1;
        });
    });
    
    c.bench_function("collection_get", |b| {
        let col = ObservableCollection::new();
        for i in 0..100 {
            col.push(i);
        }
        b.iter(|| {
            black_box(col.get(50));
        });
    });
}

fn collection_subscribe(c: &mut Criterion) {
    let mut group = c.benchmark_group("collection_notify");
    for subscriber_count in [1, 5, 10].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(subscriber_count),
            subscriber_count,
            |b, &count| {
                let col = ObservableCollection::new();
                for _ in 0..count {
                    col.subscribe(|_| {});
                }
                b.iter(|| {
                    col.push(black_box(42));
                });
            },
        );
    }
    group.finish();
}

fn computed_operations(c: &mut Criterion) {
    let prop = Property::new(5);
    
    c.bench_function("computed_from_property", |b| {
        b.iter(|| {
            black_box(Computed::from_property(&prop, |n| n * 2));
        });
    });
    
    c.bench_function("computed_get", |b| {
        let computed = Computed::from_property(&prop, |n| n * 2);
        b.iter(|| {
            black_box(computed.get());
        });
    });
}

fn computed_update_propagation(c: &mut Criterion) {
    let prop1 = Property::new(3);
    let prop2 = Property::new(4);
    let sum = Computed::from_properties2(&prop1, &prop2, |a, b| a + b);
    
    c.bench_function("computed_propagate", |b| {
        b.iter(|| {
            prop1.set(black_box(10));
            black_box(sum.get());
        });
    });
}

criterion_group!(
    reactive_benches,
    property_creation,
    property_get_set,
    property_subscribe,
    property_update,
    collection_operations,
    collection_subscribe,
    computed_operations,
    computed_update_propagation,
);

criterion_main!(reactive_benches);
