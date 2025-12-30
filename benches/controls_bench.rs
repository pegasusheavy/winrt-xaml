//! Control benchmarks - requires complete winrt_xaml implementation
//!
//! These benchmarks are disabled until the library compilation is fixed.
//! To enable: fix the Windows crate features and library implementation.

#![allow(dead_code, unused_imports)]

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
// use winrt_xaml::prelude::*;
use std::sync::Arc;
use parking_lot::RwLock;

/// Benchmark control creation
fn bench_control_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("control_creation");

    // Placeholder - requires winrt_xaml implementation
    group.bench_function("button_new", |b| {
        b.iter(|| {
            black_box(42)
        })
    });

    group.bench_function("button_with_content", |b| {
        b.iter(|| {
            black_box(Button::new().content("Test Button"))
        })
    });

    group.bench_function("textblock_new", |b| {
        b.iter(|| {
            black_box(TextBlock::new())
        })
    });

    group.bench_function("textblock_with_text", |b| {
        b.iter(|| {
            black_box(TextBlock::new().text("Sample Text"))
        })
    });

    group.bench_function("textbox_new", |b| {
        b.iter(|| {
            black_box(TextBox::new())
        })
    });

    group.bench_function("checkbox_new", |b| {
        b.iter(|| {
            black_box(CheckBox::new())
        })
    });

    group.bench_function("slider_new", |b| {
        b.iter(|| {
            black_box(Slider::new())
        })
    });

    group.bench_function("progressbar_new", |b| {
        b.iter(|| {
            black_box(ProgressBar::new())
        })
    });

    group.finish();
}

/// Benchmark control with event handlers
fn bench_control_events(c: &mut Criterion) {
    let mut group = c.benchmark_group("control_events");

    group.bench_function("button_with_handler", |b| {
        b.iter(|| {
            black_box(Button::new().on_click(|_| {}))
        })
    });

    group.bench_function("textbox_with_handler", |b| {
        b.iter(|| {
            black_box(TextBox::new().on_text_changed(|_| {}))
        })
    });

    group.bench_function("checkbox_with_handler", |b| {
        b.iter(|| {
            black_box(CheckBox::new().on_checked_changed(|_| {}))
        })
    });

    group.bench_function("slider_with_handler", |b| {
        b.iter(|| {
            black_box(Slider::new().on_value_changed(|_| {}))
        })
    });

    group.finish();
}

/// Benchmark control styling
fn bench_control_styling(c: &mut Criterion) {
    let mut group = c.benchmark_group("control_styling");

    group.bench_function("button_full_style", |b| {
        b.iter(|| {
            black_box(
                Button::new()
                    .content("Styled Button")
                    .padding_uniform(10.0)
                    .font_size(14.0)
                    .width(100.0)
                    .height(40.0)
            )
        })
    });

    group.bench_function("textblock_full_style", |b| {
        b.iter(|| {
            black_box(
                TextBlock::new()
                    .text("Styled Text")
                    .font_size(16.0)
                    .font_weight(FontWeight::Bold)
                    .foreground(&Brush::from_color(Color::BLACK))
            )
        })
    });

    group.bench_function("border_with_styling", |b| {
        b.iter(|| {
            black_box(
                Border::new()
                    .background(&Brush::from_color(Color::WHITE))
                    .border_thickness_uniform(1.0)
                    .border_brush(&Brush::from_color(Color::GRAY))
                    .corner_radius_uniform(5.0)
                    .padding_uniform(10.0)
            )
        })
    });

    group.finish();
}

/// Benchmark complex control hierarchies
fn bench_control_hierarchy(c: &mut Criterion) {
    let mut group = c.benchmark_group("control_hierarchy");

    group.bench_function("simple_stackpanel", |b| {
        b.iter(|| {
            black_box(
                StackPanel::new()
                    .orientation(Orientation::Vertical)
                    .child(TextBlock::new().text("Item 1"))
                    .child(TextBlock::new().text("Item 2"))
                    .child(TextBlock::new().text("Item 3"))
            )
        })
    });

    for size in [5, 10, 20, 50].iter() {
        group.bench_with_input(BenchmarkId::new("stackpanel_items", size), size, |b, &size| {
            b.iter(|| {
                let mut panel = StackPanel::new().orientation(Orientation::Vertical);
                for i in 0..size {
                    panel = panel.child(TextBlock::new().text(&format!("Item {}", i)));
                }
                black_box(panel)
            })
        });
    }

    group.bench_function("nested_borders", |b| {
        b.iter(|| {
            black_box(
                Border::new()
                    .child(
                        Border::new()
                            .child(
                                Border::new()
                                    .child(TextBlock::new().text("Nested"))
                            )
                    )
            )
        })
    });

    group.finish();
}

/// Benchmark state updates
fn bench_state_updates(c: &mut Criterion) {
    let mut group = c.benchmark_group("state_updates");

    group.bench_function("rwlock_read", |b| {
        let state = Arc::new(RwLock::new(0i32));
        b.iter(|| {
            black_box(*state.read())
        })
    });

    group.bench_function("rwlock_write", |b| {
        let state = Arc::new(RwLock::new(0i32));
        b.iter(|| {
            let mut s = state.write();
            *s += 1;
            black_box(*s)
        })
    });

    group.bench_function("rwlock_clone_read", |b| {
        let state = Arc::new(RwLock::new(vec![1, 2, 3, 4, 5]));
        b.iter(|| {
            black_box(state.read().clone())
        })
    });

    group.finish();
}

/// Benchmark color operations
fn bench_color_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("color_operations");

    group.bench_function("color_rgb_creation", |b| {
        b.iter(|| {
            black_box(Color::rgb(255, 128, 64))
        })
    });

    group.bench_function("brush_from_color", |b| {
        b.iter(|| {
            black_box(Brush::from_color(Color::rgb(255, 128, 64)))
        })
    });

    group.bench_function("brush_reuse", |b| {
        let brush = Brush::from_color(Color::rgb(255, 128, 64));
        b.iter(|| {
            black_box(&brush)
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_control_creation,
    bench_control_events,
    bench_control_styling,
    bench_control_hierarchy,
    bench_state_updates,
    bench_color_operations
);

criterion_main!(benches);

