//! Unit tests for media types (colors, brushes).

use winrt_xaml::media::*;

#[test]
fn test_color_creation() {
    let color = Color::from_rgb(255, 128, 64);
    assert_eq!(color.r, 255);
    assert_eq!(color.g, 128);
    assert_eq!(color.b, 64);
    assert_eq!(color.a, 255);

    let color = Color::from_rgba(100, 150, 200, 128);
    assert_eq!(color.r, 100);
    assert_eq!(color.g, 150);
    assert_eq!(color.b, 200);
    assert_eq!(color.a, 128);
}

#[test]
fn test_color_constants() {
    assert_eq!(Color::BLACK, Color::from_rgb(0, 0, 0));
    assert_eq!(Color::WHITE, Color::from_rgb(255, 255, 255));
    assert_eq!(Color::RED, Color::from_rgb(255, 0, 0));
    assert_eq!(Color::GREEN, Color::from_rgb(0, 255, 0));
    assert_eq!(Color::BLUE, Color::from_rgb(0, 0, 255));

    let transparent = Color::TRANSPARENT;
    assert_eq!(transparent.a, 0);
}

#[test]
fn test_colorref_conversion() {
    let color = Color::from_rgb(255, 128, 64);
    let colorref = color.to_colorref();
    let converted_back = Color::from_colorref(colorref);

    assert_eq!(converted_back.r, color.r);
    assert_eq!(converted_back.g, color.g);
    assert_eq!(converted_back.b, color.b);
}

#[test]
fn test_brush_creation() {
    let color = Color::from_rgb(100, 150, 200);
    let brush = Brush::new(color);

    assert_eq!(brush.color().r, 100);
    assert_eq!(brush.color().g, 150);
    assert_eq!(brush.color().b, 200);
}

#[test]
fn test_brush_clone() {
    let color = Color::from_rgb(50, 100, 150);
    let brush1 = Brush::new(color);
    let brush2 = brush1.clone();

    assert_eq!(brush1.color(), brush2.color());
}

#[test]
fn test_color_equality() {
    let color1 = Color::from_rgb(100, 100, 100);
    let color2 = Color::from_rgb(100, 100, 100);
    let color3 = Color::from_rgb(100, 100, 101);

    assert_eq!(color1, color2);
    assert_ne!(color1, color3);
}

#[test]
fn test_solid_color_brush() {
    let color = Color::RED;
    let brush: SolidColorBrush = Brush::new(color);

    assert_eq!(brush.color(), Color::RED);
}

