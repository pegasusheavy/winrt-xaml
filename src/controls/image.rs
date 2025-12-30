//! Image control - stub implementation.

use crate::controls::UIElement;
use crate::error::Result;
use parking_lot::RwLock;
use std::sync::Arc;
use windows::Win32::Foundation::*;

/// Image stretch modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stretch {
    /// No stretching.
    None,
    /// Fill available space.
    Fill,
    /// Uniform scaling.
    Uniform,
    /// Uniform scaling to fill.
    UniformToFill,
}

/// An image control.
#[derive(Clone)]
pub struct Image {
    element: UIElement,
    inner: Arc<ImageInner>,
}

struct ImageInner {
    source: RwLock<Option<String>>,
    stretch: RwLock<Stretch>,
}

impl Image {
    /// Create a new image.
    pub fn new() -> Result<Self> {
        let inner = Arc::new(ImageInner {
            source: RwLock::new(None),
            stretch: RwLock::new(Stretch::Uniform),
        });

        Ok(Image {
            element: UIElement::empty(),
            inner,
        })
    }

    /// Get the image source.
    pub fn source(&self) -> Option<String> {
        self.inner.source.read().clone()
    }

    /// Set the image source.
    pub fn set_source(&self, source: Option<String>) {
        *self.inner.source.write() = source;
    }

    /// Get the stretch mode.
    pub fn stretch(&self) -> Stretch {
        *self.inner.stretch.read()
    }

    /// Set the stretch mode.
    pub fn set_stretch(&self, stretch: Stretch) {
        *self.inner.stretch.write() = stretch;
    }

    /// Set the stretch mode (fluent API).
    pub fn with_stretch(self, stretch: Stretch) -> Self {
        self.set_stretch(stretch);
        self
    }

    /// Get the underlying UI element.
    pub fn element(&self) -> &UIElement {
        &self.element
    }

    /// Get the HWND of this image.
    pub fn hwnd(&self) -> HWND {
        self.element.hwnd()
    }
}

impl Default for Image {
    fn default() -> Self {
        Self::new().expect("Failed to create image")
    }
}

impl From<Image> for UIElement {
    fn from(image: Image) -> Self {
        image.element.clone()
    }
}
