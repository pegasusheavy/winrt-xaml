# winrt-xaml-macros

Procedural macros for compile-time XAML parsing in Rust.

## Overview

This crate provides the `xaml!` macro which parses XAML markup at **compile time** and generates Rust code to create WinRT controls. This provides:

- ✅ **Compile-time validation**: XAML errors are caught during compilation
- ⚡ **Zero runtime overhead**: No parsing at runtime
- 🔒 **Type safety**: Generated code is fully typed
- 💡 **IDE support**: Full autocomplete and error checking

## Usage

```rust
use winrt_xaml::xaml;

// This XAML is parsed at compile time!
let button = xaml! {
    r##"<Button Content="Click Me"
              Width="200"
              Height="50"
              Background="#FF0078D4"
              Foreground="#FFFFFFFF"
              CornerRadius="8" />"##
}?;

// button is a Result<XamlUIElement, Error>
```

## Supported Elements

- `<Button>` - Interactive button control
- `<TextBlock>` - Text display
- `<TextBox>` - Text input
- `<StackPanel>` - Layout panel
- `<Grid>` - Grid layout
- `<ScrollViewer>` - Scrollable container

## Supported Attributes

### Common Attributes

- `Width`, `Height` - Dimensions (f64)
- `Background`, `Foreground` - Colors (hex: `#AARRGGBB` or `#RRGGBB`)
- `Margin`, `Padding` - Spacing (uniform f64)

### Button Attributes

- `Content` - Button text
- `CornerRadius` - Rounded corners (f64)

### TextBlock Attributes

- `Text` - Display text
- `FontSize` - Font size (f64)
- `FontWeight` - Font weight (i32, e.g., 700 for bold)

### TextBox Attributes

- `Text` - Initial text
- `PlaceholderText` - Placeholder text

### StackPanel Attributes

- `Orientation` - "Vertical" or "Horizontal"
- `Spacing` - Space between children (f64)

## Example

```rust
use winrt_xaml::xaml;
use winrt_xaml::error::Result;

fn create_ui() -> Result<()> {
    // Title
    let title = xaml! {
        r##"<TextBlock Text="My App"
                      FontSize="24"
                      FontWeight="700"
                      Foreground="#FF00D7FF" />"##
    }?;

    // Input field
    let input = xaml! {
        r##"<TextBox PlaceholderText="Enter text..."
                   Width="300"
                   Height="40"
                   Background="#FF2D2D2D"
                   Foreground="#FFFFFFFF" />"##
    }?;

    // Submit button
    let button = xaml! {
        r##"<Button Content="Submit"
                  Width="300"
                  Height="50"
                  Background="#FF28A745"
                  Foreground="#FFFFFFFF"
                  CornerRadius="10" />"##
    }?;

    Ok(())
}
```

## How It Works

The `xaml!` macro:

1. Parses XAML at compile time using `quick-xml`
2. Validates element names and attributes
3. Generates Rust code that creates WinRT controls
4. Returns a `Result<XamlUIElement, Error>`

### Generated Code

The macro expands to efficient Rust code:

```rust
// Input:
let button = xaml! {
    r##"<Button Content="Hello" Width="200" Height="50" />"##
}?;

// Expands to:
let button = (|| -> winrt_xaml::error::Result<winrt_xaml::xaml_native::XamlUIElement> {
    let __element = winrt_xaml::xaml_native::XamlButton::new()?;
    __element.set_content("Hello")?;
    __element.set_size(200.0, 50.0)?;
    Ok(__element.as_uielement())
})()?;
```

## Compile-Time Validation

Invalid XAML is caught at compile time:

```rust
// This will fail to compile:
let button = xaml! {
    r##"<InvalidElement />"##  // Error: Unsupported XAML element: InvalidElement
}?;
```

## License

MIT OR Apache-2.0
