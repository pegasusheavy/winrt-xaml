# WinRT-XAML Benchmarks and Profiling

This directory contains comprehensive benchmarks and profiling tools for the WinRT-XAML library.

## Running Benchmarks

### Run All Benchmarks

```bash
cargo bench
```

### Run Specific Benchmark Suite

```bash
# Control benchmarks
cargo bench --bench controls_bench

# Layout benchmarks
cargo bench --bench layout_bench

# XAML parser benchmarks
cargo bench --bench xaml_parser_bench

# Resource benchmarks
cargo bench --bench resource_bench
```

### Run Specific Benchmark

```bash
# Run only button creation benchmarks
cargo bench --bench controls_bench -- "button"

# Run only grid benchmarks
cargo bench --bench layout_bench -- "grid"
```

## Benchmark Suites

### 1. `controls_bench.rs` - Control Performance
Measures performance of UI control creation, styling, and event handling.

**Benchmarks:**
- `control_creation` - Creating various controls (Button, TextBlock, TextBox, etc.)
- `control_events` - Controls with event handlers
- `control_styling` - Fully styled controls
- `control_hierarchy` - Nested control structures
- `state_updates` - Arc<RwLock<T>> read/write performance
- `color_operations` - Color and Brush creation

**Key Metrics:**
- Control creation time
- Memory overhead per control
- Event handler attachment cost
- Styling impact

### 2. `layout_bench.rs` - Layout Performance
Measures layout system performance with varying complexity.

**Benchmarks:**
- `stackpanel` - StackPanel with varying child counts
- `grid` - Grid with varying dimensions
- `border` - Border nesting and styling
- `canvas` - Canvas with positioned children
- `scrollviewer` - ScrollViewer with large content
- `complex_layouts` - Real-world layout patterns

**Key Metrics:**
- Layout calculation time
- Scaling with child count
- Nesting overhead
- Complex layout composition

### 3. `xaml_parser_bench.rs` - XAML Parsing Performance
Measures XAML parsing and UI construction from markup.

**Benchmarks:**
- `simple_xaml` - Basic XAML elements
- `xaml_attributes` - Attribute parsing
- `nested_xaml` - Nested structures
- `grid_xaml` - Grid definitions
- `large_xaml` - Documents with many elements
- `form_xaml` - Complex forms

**Key Metrics:**
- Parse time per element
- Attribute parsing overhead
- Nesting depth impact
- Memory allocation patterns

### 4. `resource_bench.rs` - Resource Management
Measures resource dictionary and theme resource performance.

**Benchmarks:**
- `resource_dictionary` - Dictionary operations (insert, lookup, update)
- `brush_resources` - Brush creation and caching
- `color_resources` - Color operations
- `font_resources` - Font attributes
- `thickness_resources` - Thickness operations
- `corner_radius_resources` - CornerRadius operations
- `resource_lookup` - Lookup patterns and performance
- `theme_resources` - Complete theme creation

**Key Metrics:**
- Dictionary lookup time
- Resource creation overhead
- Caching effectiveness
- Theme switching cost

## Memory Profiling

### Using dhat for Heap Profiling

dhat is a heap profiler that provides detailed information about memory allocations.

#### Setup

1. Add the dhat feature to your run:

```bash
# This will generate dhat-heap.json
cargo run --release --features dhat-heap --bin memory_profile
```

2. View the results:
   - Open [https://nnethercote.github.io/dh_view/dh_view.html](https://nnethercote.github.io/dh_view/dh_view.html)
   - Upload the generated `dhat-heap.json` file

#### What to Look For

**Memory Leaks:**
- Look for allocations that are never freed
- Check "Total bytes" vs "At t-end bytes"
- High "At t-end bytes" indicates potential leaks

**Inefficient Allocations:**
- Many small allocations (fragmentation)
- Large allocations that could be reused
- Repeated allocations in loops

**Common Issues:**
```rust
// BAD: Allocates on every call
fn get_color() -> Brush {
    Brush::from_color(Color::rgb(0, 120, 215))
}

// GOOD: Reuse the brush
lazy_static! {
    static ref PRIMARY_BRUSH: Brush = Brush::from_color(Color::rgb(0, 120, 215));
}
```

### Using Valgrind (Linux only)

```bash
# Install valgrind
sudo apt-get install valgrind

# Run with memcheck
cargo build --release
valgrind --leak-check=full --show-leak-kinds=all \
    ./target/release/memory_profile
```

### Using Windows Performance Analyzer

On Windows, use the built-in tools:

```powershell
# Using Windows Performance Recorder
# 1. Install Windows Performance Toolkit
# 2. Start recording
wpr -start GeneralProfile

# 3. Run your benchmark
cargo bench

# 4. Stop recording
wpr -stop output.etl

# 5. Analyze with Windows Performance Analyzer
wpa output.etl
```

## Performance Targets

### Control Creation
- Button creation: < 50 µs
- TextBlock creation: < 30 µs
- Complex control: < 100 µs

### Layout
- StackPanel with 100 children: < 1 ms
- 10x10 Grid: < 500 µs
- Complex dashboard layout: < 2 ms

### XAML Parsing
- Simple element: < 100 µs
- 100-element document: < 10 ms
- Complex form: < 5 ms

### Resource Operations
- Dictionary lookup: < 1 µs
- Brush creation: < 10 µs
- Theme creation: < 500 µs

## Common Performance Issues and Fixes

### 1. Excessive Cloning

**Problem:**
```rust
// BAD: Clones entire vector on every read
let items = state.read().clone();
```

**Solution:**
```rust
// GOOD: Only clone what you need
let item_count = state.read().len();
```

### 2. Unnecessary Control Recreation

**Problem:**
```rust
// BAD: Recreates UI on every update
fn update_ui(count: i32) -> UIElement {
    StackPanel::new()
        .child(TextBlock::new().text(&format!("Count: {}", count)))
        .into()
}
```

**Solution:**
```rust
// GOOD: Update only the changing part
// Use data binding or state management to update text
```

### 3. Large Resource Dictionaries

**Problem:**
```rust
// BAD: Linear search through all resources
dict.get("key") // O(n) with HashMap implementation
```

**Solution:**
- Use HashMap with good hashing for O(1) lookups
- Cache frequently used resources
- Split into smaller, scoped dictionaries

### 4. Memory Leaks in Event Handlers

**Problem:**
```rust
// BAD: Circular reference prevents cleanup
let state = Arc::new(RwLock::new(data));
let state_clone = state.clone();
button.on_click(move |_| {
    // state_clone keeps state alive forever
    state_clone.write().update();
});
```

**Solution:**
```rust
// GOOD: Use Weak references when appropriate
let state_weak = Arc::downgrade(&state);
button.on_click(move |_| {
    if let Some(state) = state_weak.upgrade() {
        state.write().update();
    }
});
```

### 5. Inefficient String Operations

**Problem:**
```rust
// BAD: Many small allocations
let text = format!("Item {}", i);
```

**Solution:**
```rust
// GOOD: Reuse string buffers or use static strings
use std::fmt::Write;
let mut text = String::with_capacity(20);
write!(&mut text, "Item {}", i).unwrap();
```

## Continuous Performance Monitoring

### Setting Up CI Benchmarks

Add to your `.github/workflows/bench.yml`:

```yaml
name: Benchmarks

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

jobs:
  benchmark:
    runs-on: windows-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable

      - name: Run benchmarks
        run: cargo bench --no-fail-fast

      - name: Store benchmark results
        uses: benchmark-action/github-action-benchmark@v1
        with:
          tool: 'cargo'
          output-file-path: target/criterion/*/base/estimates.json
```

### Tracking Regressions

Use criterion's baseline comparison:

```bash
# Save current results as baseline
cargo bench -- --save-baseline main

# Make changes...

# Compare against baseline
cargo bench -- --baseline main
```

## Optimization Tips

1. **Profile Before Optimizing**
   - Always measure first
   - Focus on hot paths
   - Use flamegraphs to identify bottlenecks

2. **Reduce Allocations**
   - Reuse objects when possible
   - Use object pools for frequently created objects
   - Pre-allocate with `Vec::with_capacity()`

3. **Optimize State Management**
   - Minimize lock contention
   - Use `RwLock` for read-heavy workloads
   - Consider lock-free data structures for hot paths

4. **Cache Expensive Operations**
   - Cache layout calculations
   - Reuse brushes and colors
   - Memoize complex computations

5. **Batch Operations**
   - Group multiple updates together
   - Use bulk operations when available
   - Minimize lock acquisitions

## Tools

### Criterion Features
- HTML reports in `target/criterion/report/index.html`
- Statistical analysis with confidence intervals
- Automatic outlier detection
- Comparison between runs

### Recommended Profilers
- **dhat** - Heap profiling (all platforms)
- **perf** - CPU profiling (Linux)
- **Instruments** - CPU and memory profiling (macOS)
- **Windows Performance Analyzer** - Comprehensive profiling (Windows)
- **cargo-flamegraph** - Visual call stacks

### Installing Tools

```bash
# Flamegraph
cargo install flamegraph

# Run with flamegraph
cargo flamegraph --bench controls_bench

# Opens flamegraph.svg in browser
```

## Contributing

When adding new benchmarks:

1. Follow existing naming conventions
2. Use `black_box()` to prevent compiler optimizations
3. Include benchmarks with varying sizes/complexity
4. Document what is being measured
5. Set appropriate sample sizes for accuracy
6. Add to this README

## Questions?

- Check the [main documentation](../README.md)
- Review existing benchmarks for patterns
- Open an issue for performance problems

