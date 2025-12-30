# WinRT XAML Examples Guide

## Overview

This library now has full support for pure WinRT XAML controls through XAML Islands. All examples have been updated (or new examples created) to demonstrate the WinRT architecture.

## Available WinRT Examples

### ✅ Fully Working Examples

1. **`pure_winrt_button.rs`** - Single button demonstration
   - Shows basic XAML Islands setup
   - Demonstrates button creation and properties
   - Complete COM initialization

2. **`winrt_controls_demo.rs`** - Comprehensive controls showcase
   - Button, TextBlock, TextBox
   - StackPanel layout with spacing
   - Font size customization
   - Placeholder text
   - **Run**: `cargo run --example winrt_controls_demo`

3. **`winrt_counter.rs`** - Counter application
   - Multiple buttons
   - Text display
   - StackPanel layout
   - **Run**: `cargo run --example winrt_counter`
   - **Note**: Event handling to be added in future update

4. **`winrt_simple_window.rs`** - Minimal example
   - Just a TextBlock in a window
   - Simplest possible WinRT app
   - **Run**: `cargo run --example winrt_simple_window`

### 🔧 Reference Examples (WinRT Architecture Demos)

5. **`winrt_activation_test.rs`** - Tests WinRT class activation
6. **`winrt_activation_with_com.rs`** - COM initialization demo
7. **`winrt_mvp.rs`** - Hybrid Win32+WinRT demonstration
8. **`xaml_islands_demo.rs`** - XAML Islands technical demo

## WinRT Example Template

All WinRT examples follow this pattern:

```rust
use winrt_xaml::error::Result;
use winrt_xaml::xaml_native::*;
use windows::core::w;
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::System::Com::{CoInitializeEx, CoUninitialize, COINIT_APARTMENTTHREADED};
use std::ptr;

fn main() -> Result<()> {
    // 1. Initialize COM
    unsafe {
        CoInitializeEx(None, COINIT_APARTMENTTHREADED)
            .map_err(|e| winrt_xaml::error::Error::initialization(format!("COM init failed: {:?}", e)))?;
    }

    // 2. Initialize XAML Manager
    let _manager = XamlManager::new()?;

    // 3. Create Win32 host window
    let host_hwnd = create_host_window()?;

    // 4. Create and attach XAML source
    let mut xaml_source = XamlSource::new()?;
    let island_hwnd = xaml_source.attach_to_window(host_hwnd)?;

    // 5. Build UI with WinRT controls
    let panel = XamlStackPanel::new()?;
    panel.set_vertical(true)?;
    panel.set_spacing(10.0)?;

    let text = XamlTextBlock::new()?;
    text.set_text("Hello WinRT!")?;
    text.set_font_size(24.0)?;
    panel.add_child(&text.as_uielement())?;

    let button = XamlButton::new()?;
    button.set_content("Click Me")?;
    button.set_size(200.0, 50.0)?;
    panel.add_child(&button.as_uielement())?;

    // 6. Set content
    xaml_source.set_content_element(&panel.as_uielement())?;

    // 7. Size the XAML island
    unsafe {
        let mut rect = windows::Win32::Foundation::RECT::default();
        let _ = GetClientRect(host_hwnd, &mut rect);
        let _ = SetWindowPos(
            island_hwnd, None, 0, 0,
            rect.right - rect.left, rect.bottom - rect.top,
            SWP_NOZORDER | SWP_NOACTIVATE,
        );
    }

    // 8. Message loop
    unsafe {
        let mut msg = MSG::default();
        while GetMessageW(&mut msg, None, 0, 0).as_bool() {
            let _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
        CoUninitialize();
    }

    Ok(())
}

// Helper function for creating Win32 host window
fn create_host_window() -> Result<HWND> {
    unsafe {
        let class_name = w!("MyAppClass");
        let wc = WNDCLASSW {
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(window_proc),
            hInstance: GetModuleHandleW(None)?.into(),
            lpszClassName: class_name,
            hCursor: LoadCursorW(None, IDC_ARROW)?,
            ..Default::default()
        };
        let _ = RegisterClassW(&wc);

        CreateWindowExW(
            WINDOW_EX_STYLE(0), class_name, w!("My App"),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT, CW_USEDEFAULT, 800, 600,
            None, None, GetModuleHandleW(None)?,
            Some(ptr::null()),
        ).map_err(|e| winrt_xaml::error::Error::window_creation(format!("{:?}", e)))
    }
}

unsafe extern "system" fn window_proc(
    hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM
) -> LRESULT {
    match msg {
        WM_SIZE => {
            // Resize XAML island when window resizes
            if let Ok(child) = GetWindow(hwnd, GW_CHILD) {
                if !child.0.is_null() {
                    let width = (lparam.0 & 0xFFFF) as i32;
                    let height = ((lparam.0 >> 16) & 0xFFFF) as i32;
                    let _ = SetWindowPos(child, None, 0, 0, width, height,
                        SWP_NOZORDER | SWP_NOACTIVATE);
                }
            }
            LRESULT(0)
        }
        WM_DESTROY => {
            PostQuitMessage(0);
            LRESULT(0)
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}
```

## Available WinRT Controls

### UI Controls

- **`XamlButton`** - Fluent Design button
  - `new()` - Create button
  - `set_content(text)` - Set button text
  - `set_size(width, height)` - Set dimensions
  - `as_uielement()` - Convert for use in layouts

- **`XamlTextBlock`** - Text display
  - `new()` - Create text block
  - `set_text(text)` - Set text content
  - `set_font_size(size)` - Set font size in points
  - `as_uielement()` - Convert for use in layouts

- **`XamlTextBox`** - Text input
  - `new()` - Create text box
  - `set_text(text)` - Set initial text
  - `set_placeholder(text)` - Set placeholder text
  - `set_size(width, height)` - Set dimensions
  - `as_uielement()` - Convert for use in layouts

### Layout Containers

- **`XamlStackPanel`** - Stack layout
  - `new()` - Create stack panel
  - `add_child(element)` - Add child element
  - `set_vertical(bool)` - Set orientation (true = vertical, false = horizontal)
  - `set_spacing(spacing)` - Set spacing between children
  - `as_uielement()` - Convert for use in layouts

- **`XamlGrid`** - Grid layout
  - `new()` - Create grid
  - `add_child(element)` - Add child element
  - `as_uielement()` - Convert for use in layouts

## Converting Old Examples to WinRT

To convert an old Win32 example to WinRT:

1. **Replace imports**: Use `winrt_xaml::xaml_native::*`
2. **Add COM init**: Call `CoInitializeEx` at startup
3. **Create XAML manager**: `XamlManager::new()?`
4. **Use Win32 for hosting only**: Create minimal Win32 window for hosting XAML Island
5. **Replace Win32 controls**:
   - `Button` → `XamlButton`
   - `TextBlock` → `XamlTextBlock`
   - `TextBox` → `XamlTextBox`
   - `StackPanel` → `XamlStackPanel`
6. **Use layouts**: StackPanel or Grid for arranging controls
7. **Convert to UIElement**: Call `.as_uielement()` before adding to layouts
8. **Set as content**: `xaml_source.set_content_element(&root.as_uielement())?`

## Common Patterns

### Creating a Form Layout

```rust
let panel = XamlStackPanel::new()?;
panel.set_vertical(true)?;
panel.set_spacing(10.0)?;

let label = XamlTextBlock::new()?;
label.set_text("Enter your name:")?;
panel.add_child(&label.as_uielement())?;

let input = XamlTextBox::new()?;
input.set_placeholder("Type here...")?;
input.set_size(300.0, 36.0)?;
panel.add_child(&input.as_uielement())?;

let button = XamlButton::new()?;
button.set_content("Submit")?;
button.set_size(120.0, 40.0)?;
panel.add_child(&button.as_uielement())?;
```

### Creating a Card-like UI

```rust
let panel = XamlStackPanel::new()?;
panel.set_vertical(true)?;
panel.set_spacing(15.0)?;

let title = XamlTextBlock::new()?;
title.set_text("Card Title")?;
title.set_font_size(24.0)?;
panel.add_child(&title.as_uielement())?;

let body = XamlTextBlock::new()?;
body.set_text("Card content goes here...")?;
body.set_font_size(14.0)?;
panel.add_child(&body.as_uielement())?;

let action = XamlButton::new()?;
action.set_content("Action")?;
action.set_size(150.0, 40.0)?;
panel.add_child(&action.as_uielement())?;
```

## Known Limitations

1. **Event Handling**: Event handling (button clicks, text changes) requires additional C++ bridge work. The infrastructure is in place but callbacks need to be implemented.

2. **Grid Advanced Features**: Row/column definitions for Grid are not yet exposed through the FFI layer.

3. **More Controls**: Additional controls (CheckBox, RadioButton, ComboBox, etc.) can be added following the same pattern in the C++ bridge.

4. **Styling**: Direct XAML styling APIs are not yet exposed. Controls use default Fluent Design theme.

5. **Data Binding**: WinRT data binding is not yet implemented.

## Next Steps

1. **Add Event Handling**: Implement callback system for button clicks and other events
2. **Expand Controls**: Add CheckBox, RadioButton, ComboBox, Slider, etc.
3. **Grid Enhancements**: Expose row/column definitions and grid properties
4. **Styling API**: Add support for colors, brushes, and custom styles
5. **Data Binding**: Implement WinRT data binding for MVVM patterns

## Testing

Run all WinRT examples:

```bash
cargo run --example pure_winrt_button
cargo run --example winrt_controls_demo
cargo run --example winrt_counter
cargo run --example winrt_simple_window
```

## Architecture

```
Rust Application
    ↓
winrt_xaml::xaml_native (Safe Rust API)
    ↓
FFI Layer (ffi.rs)
    ↓
C++ XAML Islands Helper DLL (xaml_islands_helper.dll)
    ↓
C++/WinRT Projections
    ↓
Windows.UI.Xaml.* (WinRT APIs)
    ↓
Visual Rendering (Fluent Design)
```

The Win32 window is only used as a hosting container - all UI rendering is done by true WinRT XAML controls with Fluent Design styling.

---

**Last Updated**: December 30, 2025
**Status**: Core WinRT support complete, event handling in progress

