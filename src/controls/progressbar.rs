//! ProgressBar control - stub implementation.

use crate::controls::UIElement;
use crate::error::Result;
use parking_lot::RwLock;
use std::sync::Arc;
use windows::Win32::Foundation::*;

/// A progress bar control.
#[derive(Clone)]
pub struct ProgressBar {
    element: UIElement,
    inner: Arc<ProgressBarInner>,
}

struct ProgressBarInner {
    value: RwLock<f64>,
    minimum: RwLock<f64>,
    maximum: RwLock<f64>,
    is_indeterminate: RwLock<bool>,
}

impl ProgressBar {
    /// Create a new progress bar.
    pub fn new() -> Result<Self> {
        let inner = Arc::new(ProgressBarInner {
            value: RwLock::new(0.0),
            minimum: RwLock::new(0.0),
            maximum: RwLock::new(100.0),
            is_indeterminate: RwLock::new(false),
        });

        Ok(ProgressBar {
            element: UIElement::empty(),
            inner,
        })
    }

    /// Get the current value.
    pub fn value(&self) -> f64 {
        *self.inner.value.read()
    }

    /// Set the value.
    pub fn set_value(&self, value: f64) {
        *self.inner.value.write() = value;
    }

    /// Set the value (fluent API).
    pub fn with_value(self, value: f64) -> Self {
        self.set_value(value);
        self
    }

    /// Get the minimum value.
    pub fn minimum(&self) -> f64 {
        *self.inner.minimum.read()
    }

    /// Set the minimum value.
    pub fn set_minimum(&self, minimum: f64) {
        *self.inner.minimum.write() = minimum;
    }

    /// Set the minimum value (fluent API).
    pub fn with_minimum(self, minimum: f64) -> Self {
        self.set_minimum(minimum);
        self
    }

    /// Get the maximum value.
    pub fn maximum(&self) -> f64 {
        *self.inner.maximum.read()
    }

    /// Set the maximum value.
    pub fn set_maximum(&self, maximum: f64) {
        *self.inner.maximum.write() = maximum;
    }

    /// Set the maximum value (fluent API).
    pub fn with_maximum(self, maximum: f64) -> Self {
        self.set_maximum(maximum);
        self
    }

    /// Check if the progress bar is in indeterminate mode.
    pub fn is_indeterminate(&self) -> bool {
        *self.inner.is_indeterminate.read()
    }

    /// Set the indeterminate mode.
    pub fn set_indeterminate(&self, indeterminate: bool) {
        *self.inner.is_indeterminate.write() = indeterminate;
    }

    /// Set the indeterminate mode (fluent API).
    pub fn with_is_indeterminate(self, indeterminate: bool) -> Self {
        self.set_indeterminate(indeterminate);
        self
    }

    /// Get the underlying UI element.
    pub fn element(&self) -> &UIElement {
        &self.element
    }

    /// Get the HWND of this progress bar.
    pub fn hwnd(&self) -> HWND {
        self.element.hwnd()
    }
}

impl Default for ProgressBar {
    fn default() -> Self {
        Self::new().expect("Failed to create progress bar")
    }
}

impl From<ProgressBar> for UIElement {
    fn from(progress: ProgressBar) -> Self {
        progress.element.clone()
    }
}
