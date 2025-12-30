//! Border layout - stub implementation.

use crate::controls::UIElement;
use crate::error::Result;
use parking_lot::RwLock;
use std::sync::Arc;
use windows::Win32::Foundation::*;

/// Border layout control - stub.
#[derive(Clone)]
pub struct Border {
    element: UIElement,
    inner: Arc<BorderInner>,
}

struct BorderInner {
    child: RwLock<Option<UIElement>>,
    border_thickness: RwLock<(i32, i32, i32, i32)>,
    padding: RwLock<(i32, i32, i32, i32)>,
}

impl Border {
    /// Create a new border.
    pub fn new() -> Result<Self> {
        let inner = Arc::new(BorderInner {
            child: RwLock::new(None),
            border_thickness: RwLock::new((1, 1, 1, 1)),
            padding: RwLock::new((0, 0, 0, 0)),
        });

        Ok(Border {
            element: UIElement::empty(),
            inner,
        })
    }

    /// Get the border thickness.
    pub fn border_thickness(&self) -> (i32, i32, i32, i32) {
        *self.inner.border_thickness.read()
    }

    /// Set the border thickness.
    pub fn set_border_thickness(&self, thickness: (i32, i32, i32, i32)) {
        *self.inner.border_thickness.write() = thickness;
    }

    /// Set the border thickness (fluent API).
    pub fn with_border_thickness(self, thickness: (i32, i32, i32, i32)) -> Self {
        self.set_border_thickness(thickness);
        self
    }

    /// Get the padding.
    pub fn padding(&self) -> (i32, i32, i32, i32) {
        *self.inner.padding.read()
    }

    /// Set the padding.
    pub fn set_padding(&self, padding: (i32, i32, i32, i32)) {
        *self.inner.padding.write() = padding;
    }

    /// Set the padding (fluent API).
    pub fn with_padding(self, padding: (i32, i32, i32, i32)) -> Self {
        self.set_padding(padding);
        self
    }

    /// Set the child element.
    pub fn set_child(&self, child: Option<UIElement>) {
        *self.inner.child.write() = child;
    }

    /// Set the child element (fluent API).
    pub fn with_child(self, child: UIElement) -> Self {
        *self.inner.child.write() = Some(child);
        self
    }

    /// Get the child element.
    pub fn child(&self) -> Option<UIElement> {
        self.inner.child.read().clone()
    }

    /// Get the underlying UI element.
    pub fn element(&self) -> &UIElement {
        &self.element
    }

    /// Get the HWND of this border.
    pub fn hwnd(&self) -> HWND {
        self.element.hwnd()
    }
}

impl Default for Border {
    fn default() -> Self {
        Self::new().expect("Failed to create border")
    }
}

impl From<Border> for UIElement {
    fn from(border: Border) -> Self {
        border.element.clone()
    }
}
