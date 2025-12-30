//! Integration tests for the library.

use winrt_xaml::prelude::*;

#[test]
fn test_application_creation() {
    let app = Application::new();
    assert!(app.is_ok());
}

#[test]
fn test_window_builder() {
    let window = Window::builder()
        .title("Test Window")
        .size(800, 600)
        .build();

    assert!(window.is_ok());

    if let Ok(window) = window {
        assert_eq!(window.title(), "Test Window");
        assert_eq!(window.size(), (800, 600));
    }
}

#[test]
fn test_window_position() {
    let window = Window::new().unwrap();

    window.set_position(100, 100).unwrap();
    // Position might be default on unshown window
    // Just test that it doesn't panic
}

#[test]
fn test_window_size() {
    let window = Window::new().unwrap();

    window.set_size(1024, 768).unwrap();
    assert_eq!(window.size(), (1024, 768));
}

#[test]
fn test_window_title() {
    let window = Window::new().unwrap();

    window.set_title("My Application").unwrap();
    assert_eq!(window.title(), "My Application");
}

#[test]
fn test_window_visibility() {
    let window = Window::new().unwrap();

    assert!(!window.is_visible());
    // Note: Can't actually show window in tests
}

#[test]
fn test_control_hierarchy() {
    // Create a simple control hierarchy
    let button = Button::new().unwrap();
    button.set_content("Test Button").unwrap();

    let textblock = TextBlock::new().unwrap();
    textblock.set_text("Test Text").unwrap();

    // Controls created successfully
    assert_eq!(button.content(), "Test Button");
    assert_eq!(textblock.text(), "Test Text");
}

#[test]
fn test_event_workflow() {
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;

    let button = Button::new().unwrap();
    let clicked = Arc::new(AtomicBool::new(false));
    let clicked_clone = clicked.clone();

    button.click().subscribe(move |_| {
        clicked_clone.store(true, Ordering::SeqCst);
    });

    // Test that event subscription worked
    assert_eq!(button.click().subscriber_count(), 1);
}

#[test]
fn test_resource_dictionary_usage() {
    let resources = ResourceDictionary::new();

    resources.insert("AppTitle", ResourceValue::String("My App".to_string()));
    resources.insert("MaxItems", ResourceValue::Integer(100));

    match resources.get("AppTitle") {
        Some(ResourceValue::String(title)) => assert_eq!(title, "My App"),
        _ => panic!("Expected string resource"),
    }
}

#[test]
fn test_color_brush_workflow() {
    let color = Color::from_rgb(128, 128, 255);
    let brush = Brush::new(color);

    assert_eq!(brush.color(), color);
}

