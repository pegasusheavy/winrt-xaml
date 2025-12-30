//! StackPanel layout implementation.

use crate::controls::UIElement;
use crate::error::Result;
use crate::layout::Orientation;
use parking_lot::RwLock;
use std::sync::Arc;
use windows::Win32::Foundation::*;
use windows::Win32::UI::WindowsAndMessaging::*;

/// A panel that arranges children in a stack.
#[derive(Clone)]
pub struct StackPanel {
    element: UIElement,
    inner: Arc<StackPanelInner>,
}

struct StackPanelInner {
    children: RwLock<Vec<UIElement>>,
    orientation: RwLock<Orientation>,
    spacing: RwLock<i32>,
    padding: RwLock<(i32, i32, i32, i32)>, // left, top, right, bottom
}

impl StackPanel {
    /// Create a new stack panel.
    pub fn new() -> Result<Self> {
        let inner = Arc::new(StackPanelInner {
            children: RwLock::new(Vec::new()),
            orientation: RwLock::new(Orientation::Vertical),
            spacing: RwLock::new(0),
            padding: RwLock::new((0, 0, 0, 0)),
        });

        Ok(StackPanel {
            element: UIElement::empty(),
            inner,
        })
    }

    /// Get the orientation.
    pub fn orientation(&self) -> Orientation {
        *self.inner.orientation.read()
    }

    /// Set the orientation.
    pub fn set_orientation(&self, orientation: Orientation) {
        *self.inner.orientation.write() = orientation;
    }

    /// Set the orientation (fluent API).
    pub fn with_orientation(self, orientation: Orientation) -> Self {
        self.set_orientation(orientation);
        self
    }

    /// Get the spacing between children.
    pub fn spacing(&self) -> i32 {
        *self.inner.spacing.read()
    }

    /// Set the spacing between children.
    pub fn set_spacing(&self, spacing: i32) {
        *self.inner.spacing.write() = spacing;
    }

    /// Set the spacing between children (fluent API).
    pub fn with_spacing(self, spacing: i32) -> Self {
        self.set_spacing(spacing);
        self
    }

    /// Get the padding.
    pub fn padding(&self) -> (i32, i32, i32, i32) {
        *self.inner.padding.read()
    }

    /// Set the padding (left, top, right, bottom).
    pub fn set_padding(&self, padding: (i32, i32, i32, i32)) {
        *self.inner.padding.write() = padding;
    }

    /// Set the padding (fluent API).
    pub fn with_padding(self, padding: (i32, i32, i32, i32)) -> Self {
        self.set_padding(padding);
        self
    }

    /// Add a child to the stack panel.
    pub fn add_child(&self, child: UIElement) {
        self.inner.children.write().push(child);
    }

    /// Add a child to the stack panel (fluent API).
    pub fn with_child(self, child: UIElement) -> Self {
        self.add_child(child);
        self
    }

    /// Get the children.
    pub fn children(&self) -> Vec<UIElement> {
        self.inner.children.read().clone()
    }

    /// Clear all children.
    pub fn clear_children(&self) {
        self.inner.children.write().clear();
    }

    /// Perform layout calculations.
    pub(crate) fn layout(&self, _available_width: i32, _available_height: i32) {
        let orientation = self.orientation();
        let spacing = self.spacing();
        let (pad_left, pad_top, _pad_right, _pad_bottom) = self.padding();

        let mut current_x = pad_left;
        let mut current_y = pad_top;

        let children = self.inner.children.read();

        for (i, child) in children.iter().enumerate() {
            if i > 0 {
                match orientation {
                    Orientation::Vertical => current_y += spacing,
                    Orientation::Horizontal => current_x += spacing,
                }
            }

            let child_width = child.width();
            let child_height = child.height();

            // Set position in the element
            child.set_x(current_x);
            child.set_y(current_y);

            // Actually move the Win32 child window if it exists
            let hwnd = child.hwnd();
            if !hwnd.0.is_null() {
                unsafe {
                    let _ = SetWindowPos(
                        hwnd,
                        HWND(std::ptr::null_mut()),
                        current_x,
                        current_y,
                        child_width,
                        child_height,
                        SWP_NOZORDER | SWP_NOACTIVATE,
                    );
                }
            }

            match orientation {
                Orientation::Vertical => current_y += child_height,
                Orientation::Horizontal => current_x += child_width,
            }
        }
    }

    /// Get the underlying UI element.
    pub fn element(&self) -> &UIElement {
        &self.element
    }

    /// Get the HWND of this stack panel.
    pub fn hwnd(&self) -> HWND {
        self.element.hwnd()
    }
}

impl Default for StackPanel {
    fn default() -> Self {
        Self::new().expect("Failed to create stack panel")
    }
}

impl From<StackPanel> for UIElement {
    fn from(panel: StackPanel) -> Self {
        panel.element.clone()
    }
}
