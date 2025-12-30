//! Memory profiling for WinRT-XAML
//!
//! This uses dhat for heap profiling to detect memory leaks and inefficient allocations.
//!
//! Run with: cargo run --release --features dhat-heap --bin memory_profile
//!
//! NOTE: Currently disabled pending library compilation fixes.

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

// use winrt_xaml::prelude::*;
use std::sync::Arc;
use parking_lot::RwLock;

fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    println!("Starting memory profiling...\n");

    profile_control_creation();
    profile_layout_creation();
    profile_state_management();
    profile_xaml_parsing();
    profile_resource_management();
    profile_complex_ui();

    println!("\nMemory profiling complete!");
    println!("Check dhat-heap.json for detailed results");
}

fn profile_control_creation() {
    println!("Profiling control creation...");

    // Placeholder - requires winrt_xaml implementation
    let controls: Vec<_> = (0..1000)
        .map(|i| {
            format!("Button {}", i)
        })
        .collect();

    drop(controls);
    println!("  ✓ Created and dropped 1000 buttons");

    // Create controls with handlers
    let buttons: Vec<_> = (0..1000)
        .map(|i| {
            let idx = i;
            Button::new()
                .content(&format!("Button {}", i))
                .on_click(move |_| {
                    println!("Clicked {}", idx);
                })
        })
        .collect();

    drop(buttons);
    println!("  ✓ Created and dropped 1000 buttons with handlers");
}

fn profile_layout_creation() {
    println!("\nProfiling layout creation...");

    // Create large StackPanel
    let mut panel = StackPanel::new().orientation(Orientation::Vertical);
    for i in 0..500 {
        panel = panel.child(
            Border::new()
                .padding_uniform(5.0)
                .child(TextBlock::new().text(&format!("Item {}", i)))
        );
    }
    drop(panel);
    println!("  ✓ Created and dropped StackPanel with 500 items");

    // Create large Grid
    let rows: Vec<_> = (0..20).map(|_| RowDefinition::star(1.0)).collect();
    let cols: Vec<_> = (0..20).map(|_| ColumnDefinition::star(1.0)).collect();
    let mut grid = Grid::new()
        .rows(rows)
        .columns(cols);

    for row in 0..20 {
        for col in 0..20 {
            grid = grid.child_at(
                TextBlock::new().text(&format!("R{}C{}", row, col)),
                row,
                col
            );
        }
    }
    drop(grid);
    println!("  ✓ Created and dropped 20x20 Grid with 400 cells");
}

fn profile_state_management() {
    println!("\nProfiling state management...");

    // Test Arc<RwLock<T>> patterns
    let state = Arc::new(RwLock::new(Vec::<String>::new()));

    // Simulate many state updates
    for i in 0..1000 {
        state.write().push(format!("Item {}", i));
    }

    // Simulate many reads
    for _ in 0..1000 {
        let _data = state.read().clone();
    }

    drop(state);
    println!("  ✓ Performed 1000 writes and 1000 reads with Arc<RwLock<Vec<String>>>");

    // Test with complex state
    #[derive(Clone)]
    struct ComplexState {
        items: Vec<String>,
        count: i32,
        enabled: bool,
        metadata: Vec<(String, String)>,
    }

    let complex_state = Arc::new(RwLock::new(ComplexState {
        items: Vec::new(),
        count: 0,
        enabled: true,
        metadata: Vec::new(),
    }));

    for i in 0..500 {
        let mut s = complex_state.write();
        s.items.push(format!("Item {}", i));
        s.count += 1;
        s.metadata.push((format!("key{}", i), format!("value{}", i)));
    }

    drop(complex_state);
    println!("  ✓ Managed complex state with 500 updates");
}

fn profile_xaml_parsing() {
    println!("\nProfiling XAML parsing...");

    // Parse simple XAML repeatedly
    for _ in 0..100 {
        let _ui = load_xaml(r#"
            <StackPanel Orientation="Vertical" Spacing="10">
                <TextBlock Text="Hello" FontSize="16" />
                <Button Content="Click Me" Padding="10" />
                <TextBox PlaceholderText="Enter text..." />
            </StackPanel>
        "#);
    }
    println!("  ✓ Parsed simple XAML 100 times");

    // Parse complex XAML
    for _ in 0..50 {
        let xaml = generate_large_xaml(50);
        let _ui = load_xaml(&xaml);
    }
    println!("  ✓ Parsed complex XAML with 50 items, 50 times");
}

fn profile_resource_management() {
    println!("\nProfiling resource management...");

    // Create many resource dictionaries
    let mut dicts = Vec::new();
    for _ in 0..100 {
        let dict = ResourceDictionary::new();
        for i in 0..50 {
            dict.insert(&format!("key{}", i), format!("value{}", i));
        }
        dicts.push(dict);
    }
    drop(dicts);
    println!("  ✓ Created and dropped 100 dictionaries with 50 entries each");

    // Create many brushes
    let brushes: Vec<_> = (0..1000)
        .map(|i| {
            let val = (i % 256) as u8;
            Brush::from_color(Color::rgb(val, val, val))
        })
        .collect();
    drop(brushes);
    println!("  ✓ Created and dropped 1000 brushes");
}

fn profile_complex_ui() {
    println!("\nProfiling complex UI scenarios...");

    // Shopping cart scenario
    for _ in 0..10 {
        let cart_items: Vec<_> = (0..100)
            .map(|i| {
                Border::new()
                    .padding_uniform(10.0)
                    .child(
                        Grid::new()
                            .columns(vec![
                                ColumnDefinition::star(1.0),
                                ColumnDefinition::auto(),
                            ])
                            .child_at(
                                TextBlock::new().text(&format!("Product {}", i)),
                                0, 0
                            )
                            .child_at(
                                TextBlock::new().text(&format!("${:.2}", i as f64 * 9.99)),
                                0, 1
                            )
                    )
            })
            .collect();
        drop(cart_items);
    }
    println!("  ✓ Created complex shopping cart UI 10 times");

    // Chat interface scenario
    for _ in 0..10 {
        let messages: Vec<_> = (0..200)
            .map(|i| {
                Border::new()
                    .padding(Thickness::symmetric(10.0, 5.0))
                    .background(&Brush::from_color(if i % 2 == 0 {
                        Color::rgb(230, 240, 255)
                    } else {
                        Color::WHITE
                    }))
                    .corner_radius_uniform(8.0)
                    .child(
                        StackPanel::new()
                            .orientation(Orientation::Vertical)
                            .spacing(4.0)
                            .child(TextBlock::new().text(&format!("Message {}", i)))
                            .child(
                                TextBlock::new()
                                    .text(&format!("{}:{:02}", i / 60, i % 60))
                                    .font_size(10.0)
                            )
                    )
            })
            .collect();
        drop(messages);
    }
    println!("  ✓ Created complex chat interface 10 times");
}

fn generate_large_xaml(item_count: usize) -> String {
    let mut xaml = String::from(r#"<StackPanel Orientation="Vertical" Spacing="10">"#);
    for i in 0..item_count {
        xaml.push_str(&format!(
            r#"
            <Border BorderThickness="1" Padding="10">
                <Grid>
                    <TextBlock Text="Item {}" FontSize="14" />
                </Grid>
            </Border>
            "#,
            i
        ));
    }
    xaml.push_str("</StackPanel>");
    xaml
}

