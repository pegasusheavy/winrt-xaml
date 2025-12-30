//! XAML parser benchmarks - requires complete winrt_xaml implementation
//!
//! These benchmarks are disabled until the library compilation is fixed.
//! To enable: fix the Windows crate features and library implementation.

#![allow(dead_code, unused_imports)]

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
// use winrt_xaml::prelude::*;

/// Benchmark simple XAML parsing
fn bench_simple_xaml(c: &mut Criterion) {
    let mut group = c.benchmark_group("simple_xaml");

    group.bench_function("parse_button", |b| {
        let xaml = r#"<Button Content="Click Me" />"#;
        b.iter(|| {
            black_box(load_xaml(xaml))
        })
    });

    group.bench_function("parse_textblock", |b| {
        let xaml = r#"<TextBlock Text="Hello World" FontSize="16" />"#;
        b.iter(|| {
            black_box(load_xaml(xaml))
        })
    });

    group.bench_function("parse_textbox", |b| {
        let xaml = r#"<TextBox PlaceholderText="Enter text..." Width="200" />"#;
        b.iter(|| {
            black_box(load_xaml(xaml))
        })
    });

    group.finish();
}

/// Benchmark XAML with attributes
fn bench_xaml_attributes(c: &mut Criterion) {
    let mut group = c.benchmark_group("xaml_attributes");

    group.bench_function("few_attributes", |b| {
        let xaml = r#"<Button Content="Test" Width="100" />"#;
        b.iter(|| {
            black_box(load_xaml(xaml))
        })
    });

    group.bench_function("many_attributes", |b| {
        let xaml = r#"
            <Button
                Content="Test"
                Width="100"
                Height="40"
                Padding="10"
                FontSize="14"
                FontWeight="Bold"
                HorizontalAlignment="Center"
                VerticalAlignment="Center"
            />
        "#;
        b.iter(|| {
            black_box(load_xaml(xaml))
        })
    });

    group.finish();
}

/// Benchmark nested XAML structures
fn bench_nested_xaml(c: &mut Criterion) {
    let mut group = c.benchmark_group("nested_xaml");

    group.bench_function("simple_nesting", |b| {
        let xaml = r#"
            <StackPanel Orientation="Vertical">
                <TextBlock Text="Item 1" />
                <TextBlock Text="Item 2" />
                <TextBlock Text="Item 3" />
            </StackPanel>
        "#;
        b.iter(|| {
            black_box(load_xaml(xaml))
        })
    });

    group.bench_function("deep_nesting", |b| {
        let xaml = r#"
            <Border>
                <Border>
                    <Border>
                        <Border>
                            <TextBlock Text="Deep" />
                        </Border>
                    </Border>
                </Border>
            </Border>
        "#;
        b.iter(|| {
            black_box(load_xaml(xaml))
        })
    });

    group.bench_function("complex_hierarchy", |b| {
        let xaml = r#"
            <StackPanel Orientation="Vertical" Spacing="10">
                <Border BorderThickness="1" Padding="10">
                    <StackPanel Orientation="Horizontal" Spacing="5">
                        <TextBlock Text="Label:" FontWeight="Bold" />
                        <TextBlock Text="Value" />
                    </StackPanel>
                </Border>
                <Border BorderThickness="1" Padding="10">
                    <StackPanel Orientation="Horizontal" Spacing="5">
                        <TextBlock Text="Label:" FontWeight="Bold" />
                        <TextBlock Text="Value" />
                    </StackPanel>
                </Border>
            </StackPanel>
        "#;
        b.iter(|| {
            black_box(load_xaml(xaml))
        })
    });

    group.finish();
}

/// Benchmark Grid XAML parsing
fn bench_grid_xaml(c: &mut Criterion) {
    let mut group = c.benchmark_group("grid_xaml");

    group.bench_function("simple_grid", |b| {
        let xaml = r#"
            <Grid>
                <TextBlock Text="Item" />
            </Grid>
        "#;
        b.iter(|| {
            black_box(load_xaml(xaml))
        })
    });

    group.bench_function("grid_with_definitions", |b| {
        let xaml = r#"
            <Grid RowSpacing="10" ColumnSpacing="10">
                <Grid.RowDefinitions>
                    <RowDefinition Height="Auto" />
                    <RowDefinition Height="*" />
                </Grid.RowDefinitions>
                <Grid.ColumnDefinitions>
                    <ColumnDefinition Width="100" />
                    <ColumnDefinition Width="*" />
                </Grid.ColumnDefinitions>
            </Grid>
        "#;
        b.iter(|| {
            black_box(load_xaml(xaml))
        })
    });

    group.finish();
}

/// Benchmark large XAML documents
fn bench_large_xaml(c: &mut Criterion) {
    let mut group = c.benchmark_group("large_xaml");

    // Generate XAML with varying numbers of elements
    for count in [10, 50, 100].iter() {
        let mut xaml = String::from(r#"<StackPanel Orientation="Vertical">"#);
        for i in 0..*count {
            xaml.push_str(&format!(
                r#"<TextBlock Text="Item {}" FontSize="14" />"#,
                i
            ));
        }
        xaml.push_str("</StackPanel>");

        group.bench_with_input(
            BenchmarkId::new("items", count),
            &xaml,
            |b, xaml| {
                b.iter(|| {
                    black_box(load_xaml(xaml))
                })
            }
        );
    }

    group.finish();
}

/// Benchmark XAML with complex forms
fn bench_form_xaml(c: &mut Criterion) {
    let mut group = c.benchmark_group("form_xaml");

    group.bench_function("registration_form", |b| {
        let xaml = r#"
            <StackPanel Orientation="Vertical" Spacing="15" Padding="20">
                <TextBlock Text="Registration" FontSize="24" FontWeight="Bold" />

                <StackPanel Orientation="Vertical" Spacing="5">
                    <TextBlock Text="Name" />
                    <TextBox PlaceholderText="Enter your name" />
                </StackPanel>

                <StackPanel Orientation="Vertical" Spacing="5">
                    <TextBlock Text="Email" />
                    <TextBox PlaceholderText="Enter your email" />
                </StackPanel>

                <StackPanel Orientation="Vertical" Spacing="5">
                    <TextBlock Text="Password" />
                    <TextBox PlaceholderText="Enter password" />
                </StackPanel>

                <CheckBox Content="I agree to the terms" />

                <Button Content="Register" Padding="20,10" />
            </StackPanel>
        "#;
        b.iter(|| {
            black_box(load_xaml(xaml))
        })
    });

    group.finish();
}

/// Benchmark XAML string operations
fn bench_xaml_parsing_internals(c: &mut Criterion) {
    let mut group = c.benchmark_group("xaml_internals");

    group.bench_function("attribute_parsing", |b| {
        let xaml = r#"<Button Width="100" Height="50" Content="Test" FontSize="14" />"#;
        b.iter(|| {
            black_box(load_xaml(xaml))
        })
    });

    group.bench_function("whitespace_handling", |b| {
        let xaml = r#"
            <StackPanel    Orientation="Vertical"   Spacing="10"  >
                <TextBlock    Text="Item 1"   />
                <TextBlock    Text="Item 2"   />
            </StackPanel>
        "#;
        b.iter(|| {
            black_box(load_xaml(xaml))
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_simple_xaml,
    bench_xaml_attributes,
    bench_nested_xaml,
    bench_grid_xaml,
    bench_large_xaml,
    bench_form_xaml,
    bench_xaml_parsing_internals
);

criterion_main!(benches);

