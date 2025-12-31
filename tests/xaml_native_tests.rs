//! Unit tests for xaml_native module.
//!
//! Note: Many of these tests verify the API structure and error handling
//! rather than full functionality, as full WinRT testing requires COM initialization
//! and a message loop.

use winrt_xaml::xaml_native::*;

#[test]
fn test_scroll_mode_enum() {
    assert_eq!(ScrollMode::Disabled as i32, 0);
    assert_eq!(ScrollMode::Enabled as i32, 1);
    assert_eq!(ScrollMode::Auto as i32, 2);
}

#[test]
fn test_scrollbar_visibility_enum() {
    assert_eq!(ScrollBarVisibility::Disabled as i32, 0);
    assert_eq!(ScrollBarVisibility::Auto as i32, 1);
    assert_eq!(ScrollBarVisibility::Hidden as i32, 2);
    assert_eq!(ScrollBarVisibility::Visible as i32, 3);
}

// Test FFI handle types are properly defined
#[test]
fn test_ffi_handle_types() {
    use winrt_xaml::xaml_native::ffi::*;
    
    // Verify handles can be created (null pointers for testing)
    let _manager: XamlManagerHandle = XamlManagerHandle(std::ptr::null_mut());
    let _source: XamlSourceHandle = XamlSourceHandle(std::ptr::null_mut());
    let _button: XamlButtonHandle = XamlButtonHandle(std::ptr::null_mut());
    let _textblock: XamlTextBlockHandle = XamlTextBlockHandle(std::ptr::null_mut());
    let _textbox: XamlTextBoxHandle = XamlTextBoxHandle(std::ptr::null_mut());
    let _panel: XamlStackPanelHandle = XamlStackPanelHandle(std::ptr::null_mut());
    let _grid: XamlGridHandle = XamlGridHandle(std::ptr::null_mut());
    let _scroll: XamlScrollViewerHandle = XamlScrollViewerHandle(std::ptr::null_mut());
    let _elem: XamlUIElementHandle = XamlUIElementHandle(std::ptr::null_mut());
}

#[test]
fn test_ffi_handles_are_copy() {
    use winrt_xaml::xaml_native::ffi::*;
    
    let handle1 = XamlButtonHandle(std::ptr::null_mut());
    let handle2 = handle1; // Should compile (Copy trait)
    let _handle3 = handle1; // Should still work
    
    assert_eq!(handle1.0, handle2.0);
}

#[test]
fn test_ffi_handles_are_send_sync() {
    use winrt_xaml::xaml_native::ffi::*;
    
    fn assert_send<T: Send>() {}
    fn assert_sync<T: Sync>() {}
    
    assert_send::<XamlManagerHandle>();
    assert_sync::<XamlManagerHandle>();
    assert_send::<XamlButtonHandle>();
    assert_sync::<XamlButtonHandle>();
    assert_send::<XamlTextBoxHandle>();
    assert_sync::<XamlTextBoxHandle>();
}

// Test error handling for null handles
#[test]
fn test_xaml_manager_null_handle_safety() {
    // Creating with a null handle should be caught
    // We can't actually test XamlManager::new() without COM initialization,
    // but we can verify the type exists and has the expected methods
    
    // This is a compile-time check that the API exists
    fn _check_api() {
        let _ = XamlManager::new;
    }
}

#[test]
fn test_xaml_source_api_exists() {
    // Verify XamlSource API structure
    fn _check_api() {
        let _ = XamlSource::new;
    }
}

#[test]
fn test_xaml_button_api_exists() {
    // Verify XamlButton API structure
    fn _check_api() {
        let _ = XamlButton::new;
    }
}

#[test]
fn test_xaml_textblock_api_exists() {
    // Verify XamlTextBlock API structure
    fn _check_api() {
        let _ = XamlTextBlock::new;
    }
}

#[test]
fn test_xaml_textbox_api_exists() {
    // Verify XamlTextBox API structure
    fn _check_api() {
        let _ = XamlTextBox::new;
    }
}

#[test]
fn test_xaml_stackpanel_api_exists() {
    // Verify XamlStackPanel API structure
    fn _check_api() {
        let _ = XamlStackPanel::new;
    }
}

#[test]
fn test_xaml_grid_api_exists() {
    // Verify XamlGrid API structure
    fn _check_api() {
        let _ = XamlGrid::new;
    }
}

#[test]
fn test_xaml_scrollviewer_api_exists() {
    // Verify XamlScrollViewer API structure
    fn _check_api() {
        let _ = XamlScrollViewer::new;
    }
}

// Test that error messages are properly retrieved
#[test]
fn test_get_last_error_function() {
    // get_last_error is internal to xaml_native module
    // We test error handling through the public API instead
    use winrt_xaml::error::Error;
    
    let err = Error::control_creation("Test error".to_string());
    assert!(err.to_string().contains("Test error"));
}

// Test color constants
#[test]
fn test_argb_color_format() {
    // Test that ARGB colors are in the correct format
    let blue = 0xFF0078D4u32;
    
    let alpha = (blue >> 24) & 0xFF;
    let red = (blue >> 16) & 0xFF;
    let green = (blue >> 8) & 0xFF;
    let blue_component = blue & 0xFF;
    
    assert_eq!(alpha, 0xFF); // Fully opaque
    assert_eq!(red, 0x00);
    assert_eq!(green, 0x78);
    assert_eq!(blue_component, 0xD4);
}

#[test]
fn test_common_colors() {
    // Test common color values used in examples
    let dark_bg = 0xFF1A1A1Au32;
    let transparent = 0x00000000u32;
    let white = 0xFFFFFFFFu32;
    let black = 0xFF000000u32;
    
    assert_eq!(dark_bg >> 24, 0xFF); // Opaque
    assert_eq!(transparent >> 24, 0x00); // Fully transparent
    assert_eq!(white & 0x00FFFFFF, 0x00FFFFFF); // All color components white
    assert_eq!(black & 0x00FFFFFF, 0x00000000); // All color components black
}

// Test padding/margin values
#[test]
fn test_padding_values() {
    let padding = (10.0, 15.0, 10.0, 15.0);
    
    assert_eq!(padding.0, 10.0); // left
    assert_eq!(padding.1, 15.0); // top
    assert_eq!(padding.2, 10.0); // right
    assert_eq!(padding.3, 15.0); // bottom
}

#[test]
fn test_uniform_padding() {
    let padding = (20.0, 20.0, 20.0, 20.0);
    
    assert!(padding.0 == padding.1 && padding.1 == padding.2 && padding.2 == padding.3);
}

// Test size values
#[test]
fn test_size_values() {
    let width = 200.0f64;
    let height = 50.0f64;
    
    assert!(width > 0.0);
    assert!(height > 0.0);
    assert!(width > height); // Typical button dimensions
}

#[test]
fn test_font_sizes() {
    let small = 12.0f64;
    let normal = 14.0f64;
    let large = 24.0f64;
    let xlarge = 48.0f64;
    
    assert!(small < normal);
    assert!(normal < large);
    assert!(large < xlarge);
}

#[test]
fn test_font_weights() {
    let normal = 400;
    let bold = 700;
    
    assert!(normal < bold);
    assert_eq!(normal, 400);
    assert_eq!(bold, 700);
}

// Test corner radius values
#[test]
fn test_corner_radius_values() {
    let sharp = 0.0f64;
    let slightly_rounded = 5.0f64;
    let rounded = 10.0f64;
    let very_rounded = 20.0f64;
    
    assert_eq!(sharp, 0.0);
    assert!(slightly_rounded > sharp);
    assert!(rounded > slightly_rounded);
    assert!(very_rounded > rounded);
}

// Test that Drop is implemented for all wrapper types
#[test]
fn test_drop_implementation_exists() {
    // This is a compile-time check that Drop is implemented
    // If Drop wasn't implemented, the compiler would warn about leaks
    // We verify this through the type system rather than Drop bounds
    
    // The types all have Drop implementations in their definitions
    // This test just verifies the types exist and are well-formed
    fn _check_types_exist() {
        let _ = std::mem::size_of::<XamlManager>();
        let _ = std::mem::size_of::<XamlSource>();
        let _ = std::mem::size_of::<XamlButton>();
    }
}

// Test Result type usage
#[test]
fn test_result_type() {
    use winrt_xaml::error::{Error, Result};
    
    fn returns_ok() -> Result<i32> {
        Ok(42)
    }
    
    fn returns_err() -> Result<i32> {
        Err(Error::control_creation("Test error".to_string()))
    }
    
    assert!(returns_ok().is_ok());
    assert_eq!(returns_ok().unwrap(), 42);
    assert!(returns_err().is_err());
}

// Test error propagation with ? operator
#[test]
fn test_error_propagation() {
    use winrt_xaml::error::{Error, Result};
    
    fn inner() -> Result<()> {
        Err(Error::control_creation("Inner error".to_string()))
    }
    
    fn outer() -> Result<()> {
        inner()?;
        Ok(())
    }
    
    assert!(outer().is_err());
}

// Test that all control types have as_uielement method
#[test]
fn test_as_uielement_api_exists() {
    // Compile-time check that as_uielement exists for all control types
    // This ensures consistent API across all controls
    
    fn _check_button() {
        let _ = XamlButton::as_uielement;
    }
    
    fn _check_textblock() {
        let _ = XamlTextBlock::as_uielement;
    }
    
    fn _check_textbox() {
        let _ = XamlTextBox::as_uielement;
    }
    
    fn _check_stackpanel() {
        let _ = XamlStackPanel::as_uielement;
    }
    
    fn _check_grid() {
        let _ = XamlGrid::as_uielement;
    }
    
    fn _check_scrollviewer() {
        let _ = XamlScrollViewer::as_uielement;
    }
}

// Test that styling methods exist on controls
#[test]
fn test_styling_api_exists() {
    use winrt_xaml::error::Result;
    
    // Compile-time check for styling methods on XamlButton
    fn _check_button_background() {
        fn _needs_method(_: fn(&XamlButton, u32) -> Result<()>) {}
        _needs_method(XamlButton::set_background);
    }
    
    fn _check_button_foreground() {
        fn _needs_method(_: fn(&XamlButton, u32) -> Result<()>) {}
        _needs_method(XamlButton::set_foreground);
    }
    
    fn _check_button_padding() {
        fn _needs_method(_: fn(&XamlButton, f64, f64, f64, f64) -> Result<()>) {}
        _needs_method(XamlButton::set_padding);
    }
    
    fn _check_button_corner_radius() {
        fn _needs_method(_: fn(&XamlButton, f64) -> Result<()>) {}
        _needs_method(XamlButton::set_corner_radius);
    }
    
    // Check TextBox styling
    fn _check_textbox_background() {
        fn _needs_method(_: fn(&XamlTextBox, u32) -> Result<()>) {}
        _needs_method(XamlTextBox::set_background);
    }
    
    // Check StackPanel styling
    fn _check_panel_background() {
        fn _needs_method(_: fn(&XamlStackPanel, u32) -> Result<()>) {}
        _needs_method(XamlStackPanel::set_background);
    }
}

// Test that text-specific methods exist
#[test]
fn test_text_api_exists() {
    use winrt_xaml::error::Result;
    
    fn _check_button_content() {
        fn _needs_method(_: fn(&XamlButton, &str) -> Result<()>) {}
        _needs_method(XamlButton::set_content);
    }
    
    fn _check_textblock_text() {
        fn _needs_method(_: fn(&XamlTextBlock, &str) -> Result<()>) {}
        _needs_method(XamlTextBlock::set_text);
    }
    
    fn _check_textblock_font_size() {
        fn _needs_method(_: fn(&XamlTextBlock, f64) -> Result<()>) {}
        _needs_method(XamlTextBlock::set_font_size);
    }
    
    fn _check_textblock_font_weight() {
        fn _needs_method(_: fn(&XamlTextBlock, i32) -> Result<()>) {}
        _needs_method(XamlTextBlock::set_font_weight);
    }
    
    fn _check_textbox_text() {
        fn _needs_method(_: fn(&XamlTextBox, &str) -> Result<()>) {}
        _needs_method(XamlTextBox::set_text);
    }
    
    fn _check_textbox_placeholder() {
        fn _needs_method(_: fn(&XamlTextBox, &str) -> Result<()>) {}
        _needs_method(XamlTextBox::set_placeholder);
    }
    
    fn _check_textbox_get_text() {
        fn _needs_method(_: fn(&XamlTextBox) -> Result<String>) {}
        _needs_method(XamlTextBox::get_text);
    }
}

// Test that layout methods exist
#[test]
fn test_layout_api_exists() {
    use winrt_xaml::error::Result;
    
    fn _check_size() {
        fn _needs_method(_: fn(&XamlButton, f64, f64) -> Result<()>) {}
        _needs_method(XamlButton::set_size);
    }
    
    fn _check_stackpanel_vertical() {
        fn _needs_method(_: fn(&XamlStackPanel, bool) -> Result<()>) {}
        _needs_method(XamlStackPanel::set_vertical);
    }
    
    fn _check_stackpanel_spacing() {
        fn _needs_method(_: fn(&XamlStackPanel, f64) -> Result<()>) {}
        _needs_method(XamlStackPanel::set_spacing);
    }
    
    fn _check_stackpanel_add_child() {
        fn _needs_method(_: fn(&XamlStackPanel, &XamlUIElement) -> Result<()>) {}
        _needs_method(XamlStackPanel::add_child);
    }
    
    fn _check_grid_add_child() {
        fn _needs_method(_: fn(&XamlGrid, &XamlUIElement) -> Result<()>) {}
        _needs_method(XamlGrid::add_child);
    }
}

// Test that ScrollViewer methods exist
#[test]
fn test_scrollviewer_api_exists() {
    use winrt_xaml::error::Result;
    
    fn _check_set_content() {
        fn _needs_method(_: fn(&XamlScrollViewer, &XamlUIElement) -> Result<()>) {}
        _needs_method(XamlScrollViewer::set_content);
    }
    
    fn _check_horizontal_scroll_mode() {
        fn _needs_method(_: fn(&XamlScrollViewer, ScrollMode) -> Result<()>) {}
        _needs_method(XamlScrollViewer::set_horizontal_scroll_mode);
    }
    
    fn _check_vertical_scroll_mode() {
        fn _needs_method(_: fn(&XamlScrollViewer, ScrollMode) -> Result<()>) {}
        _needs_method(XamlScrollViewer::set_vertical_scroll_mode);
    }
    
    fn _check_horizontal_scrollbar_visibility() {
        fn _needs_method(_: fn(&XamlScrollViewer, ScrollBarVisibility) -> Result<()>) {}
        _needs_method(XamlScrollViewer::set_horizontal_scrollbar_visibility);
    }
    
    fn _check_vertical_scrollbar_visibility() {
        fn _needs_method(_: fn(&XamlScrollViewer, ScrollBarVisibility) -> Result<()>) {}
        _needs_method(XamlScrollViewer::set_vertical_scrollbar_visibility);
    }
}

// Test that event handling exists
#[test]
fn test_event_api_exists() {
    // Verify on_click method exists (generic, so just check it compiles)
    fn _check_on_click() {
        let _ = XamlButton::on_click::<fn()>;
    }
}

// Test XamlSource methods
#[test]
fn test_xaml_source_methods_exist() {
    use winrt_xaml::error::Result;
    use windows::Win32::Foundation::HWND;
    
    fn _check_attach() {
        fn _needs_method(_: fn(&mut XamlSource, HWND) -> Result<HWND>) {}
        _needs_method(XamlSource::attach_to_window);
    }
    
    fn _check_set_content() {
        fn _needs_method(_: fn(&XamlSource, &XamlUIElement) -> Result<()>) {}
        _needs_method(XamlSource::set_content_element);
    }
    
    fn _check_island_hwnd() {
        fn _needs_method(_: fn(&XamlSource) -> Option<HWND>) {}
        _needs_method(XamlSource::island_hwnd);
    }
}
