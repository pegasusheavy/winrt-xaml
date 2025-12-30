# Examples Status

## Working Examples

### ✅ simple_window
**Status**: Fully functional
**Description**: Minimal Win32 window with a single button
**Run**: `cargo run --example simple_window --features library-enabled`

```rust
// Creates a basic Win32 window with a button
let app = Application::new()?;
let window = Window::builder()
    .title("Simple Win32 Window")
    .size(400, 300)
    .build()?;

let button = Button::new()
    .with_content("Hello, Win32!")?
    .with_width(150)
    .with_height(50)
    .with_x(125)
    .with_y(100)
    .on_click(|_| {
        println!("Button clicked!");
    });

window.set_content(button)?;
app.run()
```

## Examples Needing Updates

The following examples were written for the original WinRT XAML API and need to be updated for the Win32 implementation:

### ⚠️ basic_window
**Issues**:
- Uses old fluent API (`.content()` instead of `.with_content()`)
- Uses `FontWeight`, `HorizontalAlignment`, `TextWrapping` enums not yet implemented
- Uses chained builder pattern not compatible with `Result` returns

**Required Changes**:
- Update to use `.with_*()` fluent methods
- Handle `Result` types with `?` operator
- Implement missing enums or remove their usage

### ⚠️ controls_demo
**Issues**: Similar to basic_window - needs API updates

### ⚠️ data_binding
**Issues**: Data binding not yet implemented in Win32 version

### ⚠️ todo_app
**Issues**: Needs API updates + data binding

### ⚠️ counter
**Issues**: Needs API updates

### ⚠️ calculator
**Issues**: Needs API updates + grid layout implementation

### ⚠️ form_demo
**Issues**: Needs API updates

### ⚠️ settings_panel
**Issues**: Needs API updates

### ⚠️ color_picker
**Issues**: Needs API updates + custom controls

### ⚠️ timer_stopwatch
**Issues**: Needs API updates + timer implementation

### ⚠️ image_gallery
**Issues**: Needs API updates + image loading

### ⚠️ music_player
**Issues**: Needs API updates + media playback

### ⚠️ weather_dashboard
**Issues**: Needs API updates + HTTP client

### ⚠️ shopping_cart
**Issues**: Needs API updates + data binding

### ⚠️ chat_interface
**Issues**: Needs API updates + scrolling

## Migration Guide

To update examples for Win32 implementation:

### 1. Update Control Creation

**Old (WinRT style)**:
```rust
Button::new()
    .content("Click")
    .width(100)
```

**New (Win32 style)**:
```rust
Button::new()?
    .with_content("Click")?
    .with_width(100)
```

### 2. Handle Result Types

All control creation now returns `Result<T>`:

```rust
// Use ? operator
let button = Button::new()?;

// Or handle explicitly
let button = match Button::new() {
    Ok(b) => b,
    Err(e) => {
        eprintln!("Failed to create button: {}", e);
        return Err(e);
    }
};
```

### 3. Manual Layout (Temporary)

Until layout panels are fully implemented, use manual positioning:

```rust
let button = Button::new()?
    .with_x(100)
    .with_y(100)
    .with_width(150)
    .with_height(50);
```

### 4. Event Handling

Event handling remains similar:

```rust
button.click().subscribe(|_| {
    println!("Clicked!");
});
```

### 5. Simplified Features

Some features are not yet available in Win32 version:
- Data binding (stub only)
- XAML parsing (stub only)
- Complex layouts (Grid, Canvas)
- Animations
- Styles and templates

## Creating New Examples

Template for new Win32-compatible examples:

```rust
//! Example description

use winrt_xaml::prelude::*;

fn main() -> Result<()> {
    env_logger::init();

    let app = Application::new()?;

    let window = Window::builder()
        .title("My Example")
        .size(800, 600)
        .build()?;

    // Create controls
    let button = Button::new()?
        .with_content("Click Me")?
        .with_x(300)
        .with_y(250)
        .with_width(200)
        .with_height(100);

    button.click().subscribe(|_| {
        println!("Button clicked!");
    });

    window.set_content(button)?;

    app.run()
}
```

## Testing Examples

Run all examples (will show compilation errors for broken ones):

```powershell
# List all examples
cargo build --examples --features library-enabled

# Run specific example
cargo run --example simple_window --features library-enabled
```

## Roadmap

### Phase 1: Core Examples (Now)
- [x] simple_window - Basic window with button

### Phase 2: Control Examples (Next)
- [ ] Update basic_window
- [ ] Update controls_demo
- [ ] Update counter

### Phase 3: Layout Examples
- [ ] Update calculator (needs Grid)
- [ ] Update form_demo (needs StackPanel)

### Phase 4: Advanced Examples
- [ ] Update todo_app (needs data binding)
- [ ] Update shopping_cart (needs data binding)

---

**Last Updated**: December 30, 2025
**Working Examples**: 1/16
**Next Priority**: Update basic_window and controls_demo

