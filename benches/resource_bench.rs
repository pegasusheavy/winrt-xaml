//! Resource benchmarks - requires complete winrt_xaml implementation
//!
//! These benchmarks are disabled until the library compilation is fixed.
//! To enable: fix the Windows crate features and library implementation.

#![allow(dead_code, unused_imports)]

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
// use winrt_xaml::prelude::*;

/// Benchmark resource dictionary operations
fn bench_resource_dictionary(c: &mut Criterion) {
    let mut group = c.benchmark_group("resource_dictionary");

    group.bench_function("create_empty", |b| {
        b.iter(|| {
            black_box(ResourceDictionary::new())
        })
    });

    group.bench_function("insert_single", |b| {
        b.iter(|| {
            let dict = ResourceDictionary::new();
            dict.insert("key", "value");
            black_box(dict)
        })
    });

    for count in [10, 50, 100, 500].iter() {
        group.bench_with_input(
            BenchmarkId::new("insert_multiple", count),
            count,
            |b, &count| {
                b.iter(|| {
                    let dict = ResourceDictionary::new();
                    for i in 0..count {
                        dict.insert(&format!("key{}", i), format!("value{}", i));
                    }
                    black_box(dict)
                })
            }
        );
    }

    group.bench_function("lookup_existing", |b| {
        let dict = ResourceDictionary::new();
        dict.insert("test_key", "test_value");

        b.iter(|| {
            black_box(dict.get("test_key"))
        })
    });

    group.bench_function("lookup_missing", |b| {
        let dict = ResourceDictionary::new();
        dict.insert("test_key", "test_value");

        b.iter(|| {
            black_box(dict.get("missing_key"))
        })
    });

    group.bench_function("update_existing", |b| {
        let dict = ResourceDictionary::new();
        dict.insert("test_key", "test_value");

        b.iter(|| {
            dict.insert("test_key", "new_value");
            black_box(&dict)
        })
    });

    group.finish();
}

/// Benchmark Brush creation and caching
fn bench_brush_resources(c: &mut Criterion) {
    let mut group = c.benchmark_group("brush_resources");

    group.bench_function("create_solid_brush", |b| {
        b.iter(|| {
            black_box(Brush::from_color(Color::rgb(128, 128, 128)))
        })
    });

    group.bench_function("reuse_brush", |b| {
        let brush = Brush::from_color(Color::rgb(128, 128, 128));
        b.iter(|| {
            black_box(&brush)
        })
    });

    group.bench_function("create_multiple_brushes", |b| {
        b.iter(|| {
            let brushes = vec![
                Brush::from_color(Color::RED),
                Brush::from_color(Color::GREEN),
                Brush::from_color(Color::BLUE),
                Brush::from_color(Color::YELLOW),
                Brush::from_color(Color::BLACK),
                Brush::from_color(Color::WHITE),
            ];
            black_box(brushes)
        })
    });

    group.bench_function("brush_in_dictionary", |b| {
        b.iter(|| {
            let dict = ResourceDictionary::new();
            dict.insert("PrimaryBrush", Brush::from_color(Color::rgb(0, 120, 215)));
            dict.insert("SecondaryBrush", Brush::from_color(Color::rgb(100, 100, 100)));
            dict.insert("AccentBrush", Brush::from_color(Color::rgb(255, 185, 0)));
            black_box(dict)
        })
    });

    group.finish();
}

/// Benchmark Color operations
fn bench_color_resources(c: &mut Criterion) {
    let mut group = c.benchmark_group("color_resources");

    group.bench_function("create_rgb", |b| {
        b.iter(|| {
            black_box(Color::rgb(128, 128, 128))
        })
    });

    group.bench_function("create_rgba", |b| {
        b.iter(|| {
            black_box(Color::rgba(128, 128, 128, 200))
        })
    });

    group.bench_function("predefined_colors", |b| {
        b.iter(|| {
            let colors = vec![
                Color::RED,
                Color::GREEN,
                Color::BLUE,
                Color::BLACK,
                Color::WHITE,
            ];
            black_box(colors)
        })
    });

    for count in [10, 50, 100].iter() {
        group.bench_with_input(
            BenchmarkId::new("create_colors", count),
            count,
            |b, &count| {
                b.iter(|| {
                    let colors: Vec<Color> = (0..count)
                        .map(|i| {
                            let val = ((i * 255) / count) as u8;
                            Color::rgb(val, val, val)
                        })
                        .collect();
                    black_box(colors)
                })
            }
        );
    }

    group.finish();
}

/// Benchmark FontWeight and FontStyle
fn bench_font_resources(c: &mut Criterion) {
    let mut group = c.benchmark_group("font_resources");

    group.bench_function("fontweight_normal", |b| {
        b.iter(|| {
            black_box(FontWeight::Normal)
        })
    });

    group.bench_function("fontweight_bold", |b| {
        b.iter(|| {
            black_box(FontWeight::Bold)
        })
    });

    group.bench_function("fontstyle_normal", |b| {
        b.iter(|| {
            black_box(FontStyle::Normal)
        })
    });

    group.bench_function("fontstyle_italic", |b| {
        b.iter(|| {
            black_box(FontStyle::Italic)
        })
    });

    group.bench_function("font_attributes_in_dict", |b| {
        b.iter(|| {
            let dict = ResourceDictionary::new();
            dict.insert("HeaderFontSize", 24.0);
            dict.insert("BodyFontSize", 14.0);
            dict.insert("HeaderWeight", FontWeight::Bold);
            dict.insert("BodyWeight", FontWeight::Normal);
            black_box(dict)
        })
    });

    group.finish();
}

/// Benchmark Thickness operations
fn bench_thickness_resources(c: &mut Criterion) {
    let mut group = c.benchmark_group("thickness_resources");

    group.bench_function("thickness_new", |b| {
        b.iter(|| {
            black_box(Thickness::new(10.0, 20.0, 10.0, 20.0))
        })
    });

    group.bench_function("thickness_uniform", |b| {
        b.iter(|| {
            black_box(Thickness::uniform(10.0))
        })
    });

    group.bench_function("thickness_symmetric", |b| {
        b.iter(|| {
            black_box(Thickness::symmetric(15.0, 10.0))
        })
    });

    group.finish();
}

/// Benchmark CornerRadius operations
fn bench_corner_radius_resources(c: &mut Criterion) {
    let mut group = c.benchmark_group("corner_radius_resources");

    group.bench_function("corner_radius_new", |b| {
        b.iter(|| {
            black_box(CornerRadius::new(5.0, 5.0, 5.0, 5.0))
        })
    });

    group.bench_function("corner_radius_uniform", |b| {
        b.iter(|| {
            black_box(CornerRadius::uniform(8.0))
        })
    });

    group.finish();
}

/// Benchmark resource lookup patterns
fn bench_resource_lookup_patterns(c: &mut Criterion) {
    let mut group = c.benchmark_group("resource_lookup");

    // Small dictionary
    group.bench_function("small_dict_lookup", |b| {
        let dict = ResourceDictionary::new();
        dict.insert("key1", "value1");
        dict.insert("key2", "value2");
        dict.insert("key3", "value3");

        b.iter(|| {
            black_box(dict.get("key2"))
        })
    });

    // Large dictionary
    group.bench_function("large_dict_lookup", |b| {
        let dict = ResourceDictionary::new();
        for i in 0..1000 {
            dict.insert(&format!("key{}", i), format!("value{}", i));
        }

        b.iter(|| {
            black_box(dict.get("key500"))
        })
    });

    // Nested lookups
    group.bench_function("nested_resource_access", |b| {
        let dict = ResourceDictionary::new();
        dict.insert("PrimaryColor", Color::rgb(0, 120, 215));
        dict.insert("SecondaryColor", Color::rgb(100, 100, 100));

        b.iter(|| {
            let primary = dict.get("PrimaryColor");
            let secondary = dict.get("SecondaryColor");
            black_box((primary, secondary))
        })
    });

    group.finish();
}

/// Benchmark theme resource patterns
fn bench_theme_resources(c: &mut Criterion) {
    let mut group = c.benchmark_group("theme_resources");

    group.bench_function("light_theme", |b| {
        b.iter(|| {
            let dict = ResourceDictionary::new();
            dict.insert("BackgroundBrush", Brush::from_color(Color::WHITE));
            dict.insert("ForegroundBrush", Brush::from_color(Color::BLACK));
            dict.insert("AccentBrush", Brush::from_color(Color::rgb(0, 120, 215)));
            dict.insert("BorderBrush", Brush::from_color(Color::LIGHT_GRAY));
            black_box(dict)
        })
    });

    group.bench_function("dark_theme", |b| {
        b.iter(|| {
            let dict = ResourceDictionary::new();
            dict.insert("BackgroundBrush", Brush::from_color(Color::BLACK));
            dict.insert("ForegroundBrush", Brush::from_color(Color::WHITE));
            dict.insert("AccentBrush", Brush::from_color(Color::rgb(0, 120, 215)));
            dict.insert("BorderBrush", Brush::from_color(Color::DARK_GRAY));
            black_box(dict)
        })
    });

    group.bench_function("complete_theme", |b| {
        b.iter(|| {
            let dict = ResourceDictionary::new();
            // Colors
            dict.insert("BackgroundColor", Color::WHITE);
            dict.insert("ForegroundColor", Color::BLACK);
            dict.insert("AccentColor", Color::rgb(0, 120, 215));
            // Brushes
            dict.insert("BackgroundBrush", Brush::from_color(Color::WHITE));
            dict.insert("ForegroundBrush", Brush::from_color(Color::BLACK));
            // Sizes
            dict.insert("HeaderFontSize", 24.0);
            dict.insert("BodyFontSize", 14.0);
            dict.insert("SmallFontSize", 12.0);
            // Spacing
            dict.insert("DefaultPadding", Thickness::uniform(10.0));
            dict.insert("DefaultMargin", Thickness::uniform(5.0));
            // Fonts
            dict.insert("HeaderWeight", FontWeight::Bold);
            dict.insert("BodyWeight", FontWeight::Normal);
            black_box(dict)
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_resource_dictionary,
    bench_brush_resources,
    bench_color_resources,
    bench_font_resources,
    bench_thickness_resources,
    bench_corner_radius_resources,
    bench_resource_lookup_patterns,
    bench_theme_resources
);

criterion_main!(benches);

