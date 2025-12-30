//! Unit tests for layout panels.

#[cfg(feature = "library-enabled")]
use winrt_xaml::layout::*;

#[cfg(feature = "library-enabled")]
#[test]
fn test_orientation() {
    assert_eq!(Orientation::Horizontal, Orientation::Horizontal);
    assert_ne!(Orientation::Horizontal, Orientation::Vertical);
}

#[cfg(feature = "library-enabled")]
#[test]
fn test_stackpanel_creation() {
    let panel = StackPanel::new();
    assert!(panel.is_ok());
}

#[cfg(feature = "library-enabled")]
#[test]
fn test_stackpanel_orientation() {
    let panel = StackPanel::new().unwrap();
    assert_eq!(panel.orientation(), Orientation::Vertical);

    panel.set_orientation(Orientation::Horizontal);
    assert_eq!(panel.orientation(), Orientation::Horizontal);
}

#[cfg(feature = "library-enabled")]
#[test]
fn test_stackpanel_spacing() {
    let panel = StackPanel::new().unwrap();
    assert_eq!(panel.spacing(), 0);

    panel.set_spacing(10);
    assert_eq!(panel.spacing(), 10);
}

#[cfg(feature = "library-enabled")]
#[test]
fn test_stackpanel_padding() {
    let panel = StackPanel::new().unwrap();
    assert_eq!(panel.padding(), (0, 0, 0, 0));

    panel.set_padding((10, 15, 10, 15));
    assert_eq!(panel.padding(), (10, 15, 10, 15));
}

#[cfg(feature = "library-enabled")]
#[test]
fn test_stackpanel_fluent_api() {
    let panel = StackPanel::new()
        .unwrap()
        .with_orientation(Orientation::Horizontal)
        .with_spacing(20)
        .with_padding((5, 5, 5, 5));

    assert_eq!(panel.orientation(), Orientation::Horizontal);
    assert_eq!(panel.spacing(), 20);
    assert_eq!(panel.padding(), (5, 5, 5, 5));
}

#[cfg(feature = "library-enabled")]
#[test]
fn test_grid_creation() {
    let grid = Grid::new();
    assert!(grid.is_ok());
}

#[cfg(feature = "library-enabled")]
#[test]
fn test_border_creation() {
    let border = Border::new();
    assert!(border.is_ok());
}

#[cfg(feature = "library-enabled")]
#[test]
fn test_border_thickness() {
    let border = Border::new().unwrap();
    assert_eq!(border.border_thickness(), (1, 1, 1, 1));

    border.set_border_thickness((2, 2, 2, 2));
    assert_eq!(border.border_thickness(), (2, 2, 2, 2));
}

#[cfg(feature = "library-enabled")]
#[test]
fn test_border_padding() {
    let border = Border::new().unwrap();
    assert_eq!(border.padding(), (0, 0, 0, 0));

    border.set_padding((10, 10, 10, 10));
    assert_eq!(border.padding(), (10, 10, 10, 10));
}

#[cfg(feature = "library-enabled")]
#[test]
fn test_scrollviewer_creation() {
    let scrollviewer = ScrollViewer::new();
    assert!(scrollviewer.is_ok());
}

