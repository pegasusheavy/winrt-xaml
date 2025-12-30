//! Base UI element functionality.

use parking_lot::RwLock;
use std::sync::Arc;
use windows::Win32::Foundation::*;

/// A base UI element that wraps a Win32 HWND.
#[derive(Clone, Debug)]
pub struct UIElement {
    inner: Arc<UIElementInner>,
}

#[derive(Debug)]
struct UIElementInner {
    hwnd: RwLock<isize>,
    width: RwLock<i32>,
    height: RwLock<i32>,
    x: RwLock<i32>,
    y: RwLock<i32>,
    visible: RwLock<bool>,
    enabled: RwLock<bool>,
    tag: RwLock<Option<String>>,
}

impl UIElement {
    /// Create a new UI element with the given HWND.
    pub fn new(hwnd: HWND) -> Self {
        Self {
            inner: Arc::new(UIElementInner {
                hwnd: RwLock::new(hwnd.0 as isize),
                width: RwLock::new(0),
                height: RwLock::new(0),
                x: RwLock::new(0),
                y: RwLock::new(0),
                visible: RwLock::new(true),
                enabled: RwLock::new(true),
                tag: RwLock::new(None),
            }),
        }
    }

    /// Create an empty UI element (null HWND).
    pub fn empty() -> Self {
        Self::new(HWND(std::ptr::null_mut()))
    }

    /// Get the HWND of this element.
    pub fn hwnd(&self) -> HWND {
        HWND(*self.inner.hwnd.read() as *mut core::ffi::c_void)
    }

    /// Set the HWND of this element.
    pub fn set_hwnd(&self, hwnd: HWND) {
        *self.inner.hwnd.write() = hwnd.0 as isize;
    }

    /// Get the position of this element.
    pub fn position(&self) -> (i32, i32) {
        (*self.inner.x.read(), *self.inner.y.read())
    }

    /// Get the size of this element.
    pub fn size(&self) -> (i32, i32) {
        (*self.inner.width.read(), *self.inner.height.read())
    }

    /// Get the width of this element.
    pub fn width(&self) -> i32 {
        *self.inner.width.read()
    }

    /// Set the width of this element.
    pub fn set_width(&self, width: i32) {
        *self.inner.width.write() = width;
    }

    /// Get the height of this element.
    pub fn height(&self) -> i32 {
        *self.inner.height.read()
    }

    /// Set the height of this element.
    pub fn set_height(&self, height: i32) {
        *self.inner.height.write() = height;
    }

    /// Get the X position of this element.
    pub fn x(&self) -> i32 {
        *self.inner.x.read()
    }

    /// Set the X position of this element.
    pub fn set_x(&self, x: i32) {
        *self.inner.x.write() = x;
    }

    /// Get the Y position of this element.
    pub fn y(&self) -> i32 {
        *self.inner.y.read()
    }

    /// Set the Y position of this element.
    pub fn set_y(&self, y: i32) {
        *self.inner.y.write() = y;
    }

    /// Check if this element is visible.
    pub fn is_visible(&self) -> bool {
        *self.inner.visible.read()
    }

    /// Set the visibility of this element.
    pub fn set_visible(&self, visible: bool) {
        *self.inner.visible.write() = visible;
    }

    /// Check if this element is enabled.
    pub fn is_enabled(&self) -> bool {
        *self.inner.enabled.read()
    }

    /// Set the enabled state of this element.
    pub fn set_enabled(&self, enabled: bool) {
        *self.inner.enabled.write() = enabled;
    }

    /// Get the tag associated with this element.
    pub fn tag(&self) -> Option<String> {
        self.inner.tag.read().clone()
    }

    /// Set a tag for this element.
    pub fn set_tag(&self, tag: Option<String>) {
        *self.inner.tag.write() = tag;
    }

    /// Check if this is an empty element.
    pub fn is_empty(&self) -> bool {
        self.hwnd().0.is_null()
    }
}

impl Default for UIElement {
    fn default() -> Self {
        Self::empty()
    }
}

impl From<HWND> for UIElement {
    fn from(hwnd: HWND) -> Self {
        UIElement::new(hwnd)
    }
}
