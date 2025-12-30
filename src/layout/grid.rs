//! Grid layout - stub implementation.

use crate::controls::UIElement;
use crate::error::Result;
use parking_lot::RwLock;
use std::sync::Arc;
use windows::Win32::Foundation::*;

/// Grid layout control - stub.
#[derive(Clone)]
pub struct Grid {
    element: UIElement,
    inner: Arc<GridInner>,
}

struct GridInner {
    children: RwLock<Vec<UIElement>>,
}

impl Grid {
    /// Create a new grid.
    pub fn new() -> Result<Self> {
        let inner = Arc::new(GridInner {
            children: RwLock::new(Vec::new()),
        });

        Ok(Grid {
            element: UIElement::empty(),
            inner,
        })
    }

    /// Add a child to the grid.
    pub fn add_child(&self, child: UIElement) {
        self.inner.children.write().push(child);
    }

    /// Get the children.
    pub fn children(&self) -> Vec<UIElement> {
        self.inner.children.read().clone()
    }

    /// Get the underlying UI element.
    pub fn element(&self) -> &UIElement {
        &self.element
    }

    /// Get the HWND of this grid.
    pub fn hwnd(&self) -> HWND {
        self.element.hwnd()
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self::new().expect("Failed to create grid")
    }
}

impl From<Grid> for UIElement {
    fn from(grid: Grid) -> Self {
        grid.element.clone()
    }
}
