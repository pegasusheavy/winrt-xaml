//! Canvas layout container for absolute positioning.

use crate::controls::{HorizontalAlignment, Thickness, UIElement, VerticalAlignment};
use crate::error::Result;
use crate::layout::Panel;
use std::sync::Arc;

/// A canvas panel for absolute positioning.
///
/// # Example
///
/// ```rust,no_run
/// use winrt_xaml::prelude::*;
///
/// let canvas = Canvas::new()
///     .child_at(Button::with_content("Top-Left"), 10.0, 10.0)
///     .child_at(Button::with_content("Bottom-Right"), 200.0, 150.0);
/// ```
#[derive(Clone)]
pub struct Canvas {
    inner: Arc<CanvasInner>,
}

struct CanvasInner {
    canvas: windows::UI::Xaml::Controls::Canvas,
    element: UIElement,
}

// Safety: Canvas uses thread-safe primitives
unsafe impl Send for Canvas {}
unsafe impl Sync for Canvas {}

impl Canvas {
    /// Create a new canvas.
    pub fn new() -> Self {
        let canvas = windows::UI::Xaml::Controls::Canvas::new()
            .expect("Failed to create Canvas");

        let element = UIElement::from_winrt(canvas.clone().into());

        Self {
            inner: Arc::new(CanvasInner { canvas, element }),
        }
    }

    /// Get the underlying WinRT Canvas.
    pub fn as_winrt_canvas(&self) -> &windows::UI::Xaml::Controls::Canvas {
        &self.inner.canvas
    }

    /// Add a child at the specified position.
    pub fn add_child_at(&self, child: impl Into<UIElement>, left: f64, top: f64) -> Result<()> {
        let child = child.into();
        let winrt_element = child.as_winrt_element()?;

        // Set position
        windows::UI::Xaml::Controls::Canvas::SetLeft(&winrt_element, left)?;
        windows::UI::Xaml::Controls::Canvas::SetTop(&winrt_element, top)?;

        // Add to children
        let panel: windows::UI::Xaml::Controls::Panel = self.inner.canvas.cast()?;
        let children = panel.Children()?;
        children.Append(&winrt_element)?;
        Ok(())
    }

    /// Set the left position of a child.
    pub fn set_left(element: &UIElement, left: f64) -> Result<()> {
        let winrt_element = element.as_winrt_element()?;
        windows::UI::Xaml::Controls::Canvas::SetLeft(&winrt_element, left)?;
        Ok(())
    }

    /// Set the top position of a child.
    pub fn set_top(element: &UIElement, top: f64) -> Result<()> {
        let winrt_element = element.as_winrt_element()?;
        windows::UI::Xaml::Controls::Canvas::SetTop(&winrt_element, top)?;
        Ok(())
    }

    /// Get the left position of a child.
    pub fn get_left(element: &UIElement) -> Result<f64> {
        let winrt_element = element.as_winrt_element()?;
        Ok(windows::UI::Xaml::Controls::Canvas::GetLeft(&winrt_element)?)
    }

    /// Get the top position of a child.
    pub fn get_top(element: &UIElement) -> Result<f64> {
        let winrt_element = element.as_winrt_element()?;
        Ok(windows::UI::Xaml::Controls::Canvas::GetTop(&winrt_element)?)
    }

    /// Set the z-index of a child.
    pub fn set_z_index(element: &UIElement, z_index: i32) -> Result<()> {
        let winrt_element = element.as_winrt_element()?;
        windows::UI::Xaml::Controls::Canvas::SetZIndex(&winrt_element, z_index)?;
        Ok(())
    }

    /// Get the z-index of a child.
    pub fn get_z_index(element: &UIElement) -> Result<i32> {
        let winrt_element = element.as_winrt_element()?;
        Ok(windows::UI::Xaml::Controls::Canvas::GetZIndex(&winrt_element)?)
    }

    // Fluent API methods

    /// Add a child at position (fluent API).
    pub fn child_at(self, child: impl Into<UIElement>, left: f64, top: f64) -> Self {
        let _ = self.add_child_at(child, left, top);
        self
    }

    /// Set width (fluent API).
    pub fn width(self, width: f64) -> Self {
        let _ = self.inner.element.set_width(width);
        self
    }

    /// Set height (fluent API).
    pub fn height(self, height: f64) -> Self {
        let _ = self.inner.element.set_height(height);
        self
    }

    /// Set margin (fluent API).
    pub fn margin(self, margin: Thickness) -> Self {
        let _ = self.inner.element.set_margin(margin);
        self
    }

    /// Set uniform margin (fluent API).
    pub fn margin_uniform(self, margin: f64) -> Self {
        let _ = self.inner.element.set_margin(Thickness::uniform(margin));
        self
    }

    /// Set horizontal alignment (fluent API).
    pub fn horizontal_alignment(self, alignment: HorizontalAlignment) -> Self {
        let _ = self.inner.element.set_horizontal_alignment(alignment);
        self
    }

    /// Set vertical alignment (fluent API).
    pub fn vertical_alignment(self, alignment: VerticalAlignment) -> Self {
        let _ = self.inner.element.set_vertical_alignment(alignment);
        self
    }
}

impl Default for Canvas {
    fn default() -> Self {
        Self::new()
    }
}

impl Panel for Canvas {
    fn as_ui_element(&self) -> &UIElement {
        &self.inner.element
    }

    fn into_ui_element(self) -> UIElement {
        self.inner.element.clone()
    }

    fn add_child(&self, child: impl Into<UIElement>) -> Result<()> {
        self.add_child_at(child, 0.0, 0.0)
    }

    fn remove_child(&self, child: &UIElement) -> Result<()> {
        let winrt_element = child.as_winrt_element()?;
        let panel: windows::UI::Xaml::Controls::Panel = self.inner.canvas.cast()?;
        let children = panel.Children()?;

        for i in 0..children.Size()? {
            if let Ok(item) = children.GetAt(i) {
                let item_ptr = &item as *const _ as usize;
                let child_ptr = &winrt_element as *const _ as usize;
                if item_ptr == child_ptr {
                    children.RemoveAt(i)?;
                    break;
                }
            }
        }
        Ok(())
    }

    fn clear_children(&self) -> Result<()> {
        let panel: windows::UI::Xaml::Controls::Panel = self.inner.canvas.cast()?;
        let children = panel.Children()?;
        children.Clear()?;
        Ok(())
    }

    fn child_count(&self) -> Result<u32> {
        let panel: windows::UI::Xaml::Controls::Panel = self.inner.canvas.cast()?;
        let children = panel.Children()?;
        Ok(children.Size()?)
    }
}

impl From<Canvas> for UIElement {
    fn from(c: Canvas) -> Self {
        c.inner.element.clone()
    }
}

impl std::fmt::Debug for Canvas {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Canvas")
            .field("child_count", &self.child_count().ok())
            .finish()
    }
}
