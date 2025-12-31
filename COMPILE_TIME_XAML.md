# Compile-Time XAML Parsing

This document explains the compile-time XAML parsing feature provided by the `xaml!` macro.

## Overview

The `xaml!` macro parses XAML markup at **compile time** and generates efficient Rust code to create WinRT controls. This provides significant benefits over runtime parsing.

## Benefits

### ✅ Compile-Time Validation

XAML syntax errors are caught during `cargo build`, not at runtime:

```rust
// This will FAIL TO COMPILE (not fail at runtime!)
let button = xaml! {
    r##"<InvalidElement />"##  // Error: Unsupported XAML element
}?;
```

### ⚡ Zero Runtime Overhead

No XML parsing happens at runtime - the macro generates direct Rust code:

```rust
// Input:
let button = xaml! {
    r##"<Button Content="Hello" Width="200" Height="50" />"##
}?;

// Expands to:
let button = (|| -> Result<XamlUIElement> {
    let __element = XamlButton::new()?;
    __element.set_content("Hello")?;
    __element.set_size(200.0, 50.0)?;
    Ok(__element.as_uielement())
})()?;
```

### 🔒 Full Type Safety

Generated code is fully typed and checked by the Rust compiler.

### 💡 IDE Support

Full autocomplete and error checking in your IDE.

## Usage

### Basic Example

```rust
use winrt_xaml::xaml;

fn create_button() -> Result<XamlUIElement> {
    xaml! {
        r##"<Button Content="Click Me"
                  Width="200"
                  Height="50"
                  Background="#FF0078D4"
                  Foreground="#FFFFFFFF"
                  CornerRadius="8" />"##
    }
}
```

### Complete UI Example

```rust
use winrt_xaml::xaml;
use winrt_xaml::error::Result;

fn create_login_form() -> Result<()> {
    // Title
    let title = xaml! {
        r##"<TextBlock Text="Login"
                     FontSize="32"
                     FontWeight="700"
                     Foreground="#FF0078D4" />"##
    }?;

    // Username field
    let username = xaml! {
        r##"<TextBox PlaceholderText="Username"
                   Width="300"
                   Height="40"
                   Background="#FF2D2D2D"
                   Foreground="#FFFFFFFF" />"##
    }?;

    // Password field
    let password = xaml! {
        r##"<TextBox PlaceholderText="Password"
                   Width="300"
                   Height="40"
                   Background="#FF2D2D2D"
                   Foreground="#FFFFFFFF" />"##
    }?;

    // Submit button
    let submit = xaml! {
        r##"<Button Content="Login"
                  Width="300"
                  Height="50"
                  Background="#FF28A745"
                  Foreground="#FFFFFFFF"
                  CornerRadius="10" />"##
    }?;

    // Add to panel...
    Ok(())
}
```

## Supported Elements

- `<Button>` - Interactive button
- `<TextBlock>` - Text display
- `<TextBox>` - Text input
- `<StackPanel>` - Layout panel
- `<Grid>` - Grid layout
- `<ScrollViewer>` - Scrollable container

## Supported Attributes

### Common Attributes

| Attribute | Type | Example |
|-----------|------|---------|
| `Width` | f64 | `Width="200"` |
| `Height` | f64 | `Height="50"` |
| `Background` | Color (hex) | `Background="#FF0078D4"` |
| `Foreground` | Color (hex) | `Foreground="#FFFFFFFF"` |
| `Margin` | f64 (uniform) | `Margin="10"` |
| `Padding` | f64 (uniform) | `Padding="5"` |

### Button Specific

| Attribute | Type | Example |
|-----------|------|---------|
| `Content` | String | `Content="Click Me"` |
| `CornerRadius` | f64 | `CornerRadius="8"` |

### TextBlock Specific

| Attribute | Type | Example |
|-----------|------|---------|
| `Text` | String | `Text="Hello World"` |
| `FontSize` | f64 | `FontSize="24"` |
| `FontWeight` | i32 | `FontWeight="700"` |

### TextBox Specific

| Attribute | Type | Example |
|-----------|------|---------|
| `Text` | String | `Text="Initial"` |
| `PlaceholderText` | String | `PlaceholderText="Enter..."` |

### StackPanel Specific

| Attribute | Type | Example |
|-----------|------|---------|
| `Orientation` | String | `Orientation="Vertical"` |
| `Spacing` | f64 | `Spacing="10"` |

## Color Format

Colors are specified in hex format:

- **8 digits (AARRGGBB)**: `#FF0078D4` (alpha + RGB)
- **6 digits (RRGGBB)**: `#0078D4` (RGB, alpha defaults to FF)

Examples:
- `#FF0078D4` - Blue (opaque)
- `#80FF0000` - Red (50% transparent)
- `#00FF00` - Green (opaque, alpha defaults to FF)

## Compile-Time Error Detection

Try building this code to see compile-time error detection:

```rust
// This will FAIL TO COMPILE:
let button = xaml! {
    r##"<InvalidElement />"##
}?;

// Error: XAML parse error: Unsupported XAML element: InvalidElement
```

The error appears at compile time, not runtime!

## Comparison: Runtime vs Compile-Time

### Runtime Parsing (XamlReader)

```rust
let xaml_string = r##"<Button Content="Click Me" />"##;
let button = XamlReader::parse(xaml_string)?; // Parsed at runtime
```

**Pros:**
- Dynamic XAML loading
- Can load XAML from files/network

**Cons:**
- Runtime parsing overhead
- Errors only caught at runtime
- No compile-time validation

### Compile-Time Parsing (xaml! macro)

```rust
let button = xaml! {
    r##"<Button Content="Click Me" />"##
}?; // Parsed at compile time
```

**Pros:**
- ⚡ Zero runtime overhead
- ✅ Compile-time validation
- 🔒 Full type safety
- 💡 IDE support

**Cons:**
- XAML must be known at compile time
- Cannot load dynamic XAML

## Examples

Run the demo:

```bash
cargo run --example xaml_compile_time_demo
```

See the demo source:

```bash
cat examples/xaml_compile_time_demo.rs
```

## How It Works

The `xaml!` macro is a procedural macro that:

1. **Receives XAML string** at compile time
2. **Parses XML** using `quick-xml`
3. **Validates** element names and attributes
4. **Generates Rust code** that creates WinRT controls
5. **Returns** `Result<XamlUIElement, Error>`

### Generated Code

The macro generates a closure that creates and configures the control:

```rust
(|| -> Result<XamlUIElement> {
    let __element = XamlButton::new()?;
    __element.set_content("Hello")?;
    __element.set_size(200.0, 50.0)?;
    __element.set_background(0xFF0078D4)?;
    Ok(__element.as_uielement())
})()
```

This is much faster than parsing XML at runtime!

## Performance

Compile-time XAML parsing provides:

- **0ms** XML parsing at runtime
- **Direct function calls** instead of interpreted XAML
- **No allocations** for XML parsing
- **Same performance** as hand-written Rust code

## Future Enhancements

Planned features:

- [ ] Nested elements (children)
- [ ] Event handlers in XAML
- [ ] Data binding syntax
- [ ] Resource references
- [ ] Attached properties
- [ ] Style inheritance

## License

MIT OR Apache-2.0
