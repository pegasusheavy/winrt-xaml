//! ScrollViewer layout - stub implementation.

use crate::controls::UIElement;
use crate::error::Result;
use parking_lot::RwLock;
use std::sync::Arc;
use windows::Win32::Foundation::*;

/// ScrollViewer layout control - stub.
#[derive(Clone)]
pub struct ScrollViewer {
    element: UIElement,
    inner: Arc<ScrollViewerInner>,
}

struct ScrollViewerInner {
    child: RwLock<Option<UIElement>>,
}

impl ScrollViewer {
    /// Create a new scroll viewer.
    pub fn new() -> Result<Self> {
        let inner = Arc::new(ScrollViewerInner {
            child: RwLock::new(None),
        });

        Ok(ScrollViewer {
            element: UIElement::empty(),
            inner,
        })
    }

    /// Set the child element.
    pub fn set_child(&self, child: Option<UIElement>) {
        *self.inner.child.write() = child;
    }

    /// Get the child element.
    pub fn child(&self) -> Option<UIElement> {
        self.inner.child.read().clone()
    }

    /// Get the underlying UI element.
    pub fn element(&self) -> &UIElement {
        &self.element
    }

    /// Get the HWND of this scroll viewer.
    pub fn hwnd(&self) -> HWND {
        self.element.hwnd()
    }
}

impl Default for ScrollViewer {
    fn default() -> Self {
        Self::new().expect("Failed to create scroll viewer")
    }
}

impl From<ScrollViewer> for UIElement {
    fn from(viewer: ScrollViewer) -> Self {
        viewer.element.clone()
    }
}
