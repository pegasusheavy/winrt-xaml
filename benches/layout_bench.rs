//! Layout benchmarks - requires complete winrt_xaml implementation
//!
//! These benchmarks are disabled until the library compilation is fixed.
//! To enable: fix the Windows crate features and library implementation.

#![allow(dead_code, unused_imports)]

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
// use winrt_xaml::prelude::*;

/// Benchmark StackPanel layout operations
fn bench_stackpanel(c: &mut Criterion) {
    let mut group = c.benchmark_group("stackpanel");

    group.bench_function("empty_vertical", |b| {
        b.iter(|| {
            black_box(StackPanel::new().orientation(Orientation::Vertical))
        })
    });

    group.bench_function("empty_horizontal", |b| {
        b.iter(|| {
            black_box(StackPanel::new().orientation(Orientation::Horizontal))
        })
    });

    group.bench_function("with_spacing", |b| {
        b.iter(|| {
            black_box(
                StackPanel::new()
                    .orientation(Orientation::Vertical)
                    .spacing(10.0)
            )
        })
    });

    // Benchmark with varying numbers of children
    for count in [10, 50, 100, 500].iter() {
        group.bench_with_input(
            BenchmarkId::new("vertical_children", count),
            count,
            |b, &count| {
                b.iter(|| {
                    let mut panel = StackPanel::new()
                        .orientation(Orientation::Vertical)
                        .spacing(5.0);

                    for i in 0..count {
                        panel = panel.child(
                            TextBlock::new()
                                .text(&format!("Item {}", i))
                                .font_size(14.0)
                        );
                    }
                    black_box(panel)
                })
            }
        );
    }

    group.bench_function("nested_stackpanels", |b| {
        b.iter(|| {
            black_box(
                StackPanel::new()
                    .orientation(Orientation::Vertical)
                    .child(
                        StackPanel::new()
                            .orientation(Orientation::Horizontal)
                            .child(TextBlock::new().text("A"))
                            .child(TextBlock::new().text("B"))
                    )
                    .child(
                        StackPanel::new()
                            .orientation(Orientation::Horizontal)
                            .child(TextBlock::new().text("C"))
                            .child(TextBlock::new().text("D"))
                    )
            )
        })
    });

    group.finish();
}

/// Benchmark Grid layout operations
fn bench_grid(c: &mut Criterion) {
    let mut group = c.benchmark_group("grid");

    group.bench_function("empty_grid", |b| {
        b.iter(|| {
            black_box(Grid::new())
        })
    });

    group.bench_function("2x2_grid", |b| {
        b.iter(|| {
            black_box(
                Grid::new()
                    .rows(vec![RowDefinition::star(1.0), RowDefinition::star(1.0)])
                    .columns(vec![ColumnDefinition::star(1.0), ColumnDefinition::star(1.0)])
            )
        })
    });

    group.bench_function("2x2_grid_with_children", |b| {
        b.iter(|| {
            black_box(
                Grid::new()
                    .rows(vec![RowDefinition::star(1.0), RowDefinition::star(1.0)])
                    .columns(vec![ColumnDefinition::star(1.0), ColumnDefinition::star(1.0)])
                    .child_at(TextBlock::new().text("A"), 0, 0)
                    .child_at(TextBlock::new().text("B"), 0, 1)
                    .child_at(TextBlock::new().text("C"), 1, 0)
                    .child_at(TextBlock::new().text("D"), 1, 1)
            )
        })
    });

    // Benchmark with varying grid sizes
    for size in [2, 4, 8, 16].iter() {
        group.bench_with_input(
            BenchmarkId::new("grid_definition", size),
            size,
            |b, &size| {
                b.iter(|| {
                    let rows: Vec<RowDefinition> = (0..size)
                        .map(|_| RowDefinition::star(1.0))
                        .collect();
                    let cols: Vec<ColumnDefinition> = (0..size)
                        .map(|_| ColumnDefinition::star(1.0))
                        .collect();

                    black_box(Grid::new().rows(rows).columns(cols))
                })
            }
        );
    }

    group.bench_function("grid_with_spacing", |b| {
        b.iter(|| {
            black_box(
                Grid::new()
                    .rows(vec![RowDefinition::star(1.0), RowDefinition::star(1.0)])
                    .columns(vec![ColumnDefinition::star(1.0), ColumnDefinition::star(1.0)])
                    .row_spacing(10.0)
                    .column_spacing(10.0)
            )
        })
    });

    group.bench_function("complex_grid", |b| {
        b.iter(|| {
            black_box(
                Grid::new()
                    .rows(vec![
                        RowDefinition::auto(),
                        RowDefinition::star(1.0),
                        RowDefinition::pixel(50.0),
                    ])
                    .columns(vec![
                        ColumnDefinition::pixel(200.0),
                        ColumnDefinition::star(2.0),
                        ColumnDefinition::star(1.0),
                    ])
                    .row_spacing(5.0)
                    .column_spacing(5.0)
            )
        })
    });

    group.finish();
}

/// Benchmark Border operations
fn bench_border(c: &mut Criterion) {
    let mut group = c.benchmark_group("border");

    group.bench_function("empty_border", |b| {
        b.iter(|| {
            black_box(Border::new())
        })
    });

    group.bench_function("border_with_child", |b| {
        b.iter(|| {
            black_box(
                Border::new()
                    .child(TextBlock::new().text("Content"))
            )
        })
    });

    group.bench_function("styled_border", |b| {
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

    group.bench_function("nested_borders", |b| {
        b.iter(|| {
            let mut current = TextBlock::new().text("Deep").into();
            for _ in 0..10 {
                current = Border::new()
                    .padding_uniform(2.0)
                    .child(current)
                    .into();
            }
            black_box(current)
        })
    });

    group.finish();
}

/// Benchmark Canvas operations
fn bench_canvas(c: &mut Criterion) {
    let mut group = c.benchmark_group("canvas");

    group.bench_function("empty_canvas", |b| {
        b.iter(|| {
            black_box(Canvas::new())
        })
    });

    for count in [10, 50, 100].iter() {
        group.bench_with_input(
            BenchmarkId::new("canvas_children", count),
            count,
            |b, &count| {
                b.iter(|| {
                    let mut canvas = Canvas::new();
                    for i in 0..count {
                        let x = (i * 10) as f64;
                        let y = (i * 5) as f64;
                        canvas = canvas.child_at(
                            Border::new()
                                .width(50.0)
                                .height(50.0)
                                .background(&Brush::from_color(Color::BLUE)),
                            x,
                            y,
                        );
                    }
                    black_box(canvas)
                })
            }
        );
    }

    group.finish();
}

/// Benchmark ScrollViewer operations
fn bench_scrollviewer(c: &mut Criterion) {
    let mut group = c.benchmark_group("scrollviewer");

    group.bench_function("empty_scrollviewer", |b| {
        b.iter(|| {
            black_box(ScrollViewer::new())
        })
    });

    group.bench_function("scrollviewer_with_content", |b| {
        b.iter(|| {
            black_box(
                ScrollViewer::new()
                    .vertical_scroll_bar_visibility(ScrollBarVisibility::Auto)
                    .content(
                        StackPanel::new()
                            .orientation(Orientation::Vertical)
                            .child(TextBlock::new().text("Item 1"))
                            .child(TextBlock::new().text("Item 2"))
                    )
            )
        })
    });

    for count in [50, 100, 500].iter() {
        group.bench_with_input(
            BenchmarkId::new("scrollviewer_items", count),
            count,
            |b, &count| {
                b.iter(|| {
                    let mut panel = StackPanel::new().orientation(Orientation::Vertical);
                    for i in 0..count {
                        panel = panel.child(TextBlock::new().text(&format!("Item {}", i)));
                    }

                    black_box(
                        ScrollViewer::new()
                            .vertical_scroll_bar_visibility(ScrollBarVisibility::Auto)
                            .content(panel)
                    )
                })
            }
        );
    }

    group.finish();
}

/// Benchmark complex layout compositions
fn bench_complex_layouts(c: &mut Criterion) {
    let mut group = c.benchmark_group("complex_layouts");

    group.bench_function("dashboard_layout", |b| {
        b.iter(|| {
            black_box(
                Grid::new()
                    .rows(vec![
                        RowDefinition::auto(),
                        RowDefinition::star(1.0),
                        RowDefinition::auto(),
                    ])
                    .columns(vec![
                        ColumnDefinition::pixel(250.0),
                        ColumnDefinition::star(1.0),
                    ])
                    // Header
                    .child_at(
                        Border::new()
                            .background(&Brush::from_color(Color::BLUE))
                            .padding_uniform(15.0),
                        0, 0
                    )
                    // Sidebar
                    .child_at(
                        Border::new()
                            .background(&Brush::from_color(Color::LIGHT_GRAY))
                            .child(
                                StackPanel::new()
                                    .orientation(Orientation::Vertical)
                                    .child(Button::new().content("Menu 1"))
                                    .child(Button::new().content("Menu 2"))
                                    .child(Button::new().content("Menu 3"))
                            ),
                        1, 0
                    )
                    // Content
                    .child_at(
                        ScrollViewer::new()
                            .content(
                                StackPanel::new()
                                    .orientation(Orientation::Vertical)
                                    .padding_uniform(20.0)
                            ),
                        1, 1
                    )
            )
        })
    });

    group.bench_function("form_layout", |b| {
        b.iter(|| {
            black_box(
                StackPanel::new()
                    .orientation(Orientation::Vertical)
                    .spacing(15.0)
                    .padding_uniform(20.0)
                    .child(
                        Grid::new()
                            .columns(vec![
                                ColumnDefinition::pixel(100.0),
                                ColumnDefinition::star(1.0),
                            ])
                            .child_at(TextBlock::new().text("Name:"), 0, 0)
                            .child_at(TextBox::new(), 0, 1)
                    )
                    .child(
                        Grid::new()
                            .columns(vec![
                                ColumnDefinition::pixel(100.0),
                                ColumnDefinition::star(1.0),
                            ])
                            .child_at(TextBlock::new().text("Email:"), 0, 0)
                            .child_at(TextBox::new(), 0, 1)
                    )
                    .child(Button::new().content("Submit"))
            )
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_stackpanel,
    bench_grid,
    bench_border,
    bench_canvas,
    bench_scrollviewer,
    bench_complex_layouts
);

criterion_main!(benches);

