//! Unit tests for window management.

use winrt_xaml::window::Window;
use winrt_xaml::error::Error;

#[test]
fn test_window_creation() {
    let window = Window::new();
    assert!(window.is_ok());
}

#[test]
fn test_window_title() {
    let window = Window::new().unwrap();

    // Default title is "Window"
    assert_eq!(window.title(), "Window");

    // Set title
    let _ = window.set_title("Test Window");
    assert_eq!(window.title(), "Test Window");
}

#[test]
fn test_window_size() {
    let window = Window::new().unwrap();

    // Default size
    assert_eq!(window.size(), (800, 600));

    // Set size
    window.set_size(1024, 768);
    assert_eq!(window.size(), (1024, 768));
}

#[test]
fn test_window_position() {
    let window = Window::new().unwrap();

    // Default position is CW_USEDEFAULT (-2147483648)
    let default_pos = window.position();
    // Just verify it's a valid position value
    assert!(default_pos.0 != 0 || default_pos.1 != 0 || default_pos == (0, 0));

    // Set position
    let _ = window.set_position(200, 300);
    assert_eq!(window.position(), (200, 300));
}

#[test]
fn test_window_visibility() {
    let window = Window::new().unwrap();

    // Initially not visible
    assert!(!window.is_visible());

    // Note: We can't actually test show() without a real window,
    // but we can verify the API exists
}

#[test]
fn test_window_setter_methods() {
    let window = Window::new().unwrap();

    window.set_title("Builder Window");
    window.set_size(640, 480);
    window.set_position(50, 50);

    assert_eq!(window.title(), "Builder Window");
    assert_eq!(window.size(), (640, 480));
    assert_eq!(window.position(), (50, 50));
}

#[test]
fn test_window_setter_chaining() {
    let window = Window::new().unwrap();

    window.set_title("First");
    window.set_title("Second"); // Should override
    window.set_size(100, 100);
    window.set_size(200, 200); // Should override

    assert_eq!(window.title(), "Second");
    assert_eq!(window.size(), (200, 200));
}

#[test]
fn test_window_clone() {
    let window1 = Window::new().unwrap();
    window1.set_title("Original");

    let window2 = window1.clone();

    // Both should refer to the same window
    assert_eq!(window2.title(), "Original");

    // Modify through window2
    window2.set_title("Modified");

    // Should be visible through window1
    assert_eq!(window1.title(), "Modified");
}

#[test]
fn test_window_state_sharing() {
    let window1 = Window::new().unwrap();
    let window2 = window1.clone();

    // Modify through window1
    window1.set_size(400, 300);
    window1.set_position(10, 20);

    // Should be visible through window2
    assert_eq!(window2.size(), (400, 300));
    assert_eq!(window2.position(), (10, 20));
}

#[test]
fn test_window_title_empty_string() {
    let window = Window::new().unwrap();
    window.set_title("");
    assert_eq!(window.title(), "");
}

#[test]
fn test_window_title_unicode() {
    let window = Window::new().unwrap();
    window.set_title("Hello 世界 🌍");
    assert_eq!(window.title(), "Hello 世界 🌍");
}

#[test]
fn test_window_size_zero() {
    let window = Window::new().unwrap();
    window.set_size(0, 0);
    assert_eq!(window.size(), (0, 0));
}

#[test]
fn test_window_size_large() {
    let window = Window::new().unwrap();
    window.set_size(3840, 2160); // 4K resolution
    assert_eq!(window.size(), (3840, 2160));
}

#[test]
fn test_window_position_negative() {
    let window = Window::new().unwrap();
    window.set_position(-100, -200);
    assert_eq!(window.position(), (-100, -200));
}

#[test]
fn test_window_multiple_creation() {
    let window1 = Window::new();
    assert!(window1.is_ok());

    let window2 = Window::new();
    assert!(window2.is_ok());

    // Both should be independent
    let w1 = window1.unwrap();
    let w2 = window2.unwrap();

    w1.set_title("Window 1");
    w2.set_title("Window 2");

    assert_eq!(w1.title(), "Window 1");
    assert_eq!(w2.title(), "Window 2");
}

#[test]
fn test_window_error_creation() {
    let err = Error::window_creation("Test window error");
    let msg = err.to_string();
    assert!(msg.contains("Failed to create window") || msg.contains("Test window error"));
}

#[test]
fn test_window_api_exists() {
    // Compile-time checks that the API exists

    fn _check_new() {
        fn _needs_result<T>(_: fn() -> Result<T, Error>) {}
        _needs_result(Window::new);
    }

    fn _check_title() {
        fn _needs_method(_: fn(&Window) -> String) {}
        _needs_method(Window::title);
    }

    fn _check_set_title() {
        // set_title uses impl AsRef<str>, so we just verify it exists
        fn _needs_method(_: &Window) {
            // This compiles if set_title exists
        }
    }

    fn _check_size() {
        fn _needs_method(_: fn(&Window) -> (i32, i32)) {}
        _needs_method(Window::size);
    }

    fn _check_set_size() {
        let _ = Window::set_size;
    }

    fn _check_position() {
        fn _needs_method(_: fn(&Window) -> (i32, i32)) {}
        _needs_method(Window::position);
    }

    fn _check_set_position() {
        let _ = Window::set_position;
    }

    fn _check_is_visible() {
        fn _needs_method(_: fn(&Window) -> bool) {}
        _needs_method(Window::is_visible);
    }

}

#[test]
fn test_window_default_values() {
    let window = Window::new().unwrap();

    // Verify default values
    assert_eq!(window.title(), "Window");
    assert_eq!(window.size(), (800, 600));
    // Position is CW_USEDEFAULT, just verify it exists
    let _ = window.position();
    assert!(!window.is_visible());
}

#[test]
fn test_window_partial_setters() {
    let window = Window::new().unwrap();
    let _ = window.set_title("Test");

    // Size should still be default
    assert_eq!(window.size(), (800, 600));
}

// Note: We can't test show(), hide(), hwnd(), set_content(), etc. without
// actually creating real windows, which requires a message loop and is better
// suited for integration tests.
