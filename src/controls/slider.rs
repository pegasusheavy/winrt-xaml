//! Slider control implementation - stub for Win32 TRACKBAR.

use crate::controls::UIElement;
use crate::error::Result;
use crate::events::{EventHandler, ValueChangedEventArgs};
use parking_lot::RwLock;
use std::sync::Arc;
use windows::Win32::Foundation::*;

/// Slider orientation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SliderOrientation {
    /// Horizontal slider.
    Horizontal,
    /// Vertical slider.
    Vertical,
}

/// A slider control.
#[derive(Clone)]
pub struct Slider {
    element: UIElement,
    inner: Arc<SliderInner>,
}

struct SliderInner {
    value: RwLock<f64>,
    minimum: RwLock<f64>,
    maximum: RwLock<f64>,
    step_frequency: RwLock<f64>,
    orientation: RwLock<SliderOrientation>,
    value_changed: EventHandler<ValueChangedEventArgs<f64>>,
}

impl Slider {
    /// Create a new slider.
    pub fn new() -> Result<Self> {
        let inner = Arc::new(SliderInner {
            value: RwLock::new(0.0),
            minimum: RwLock::new(0.0),
            maximum: RwLock::new(100.0),
            step_frequency: RwLock::new(1.0),
            orientation: RwLock::new(SliderOrientation::Horizontal),
            value_changed: EventHandler::new(),
        });

        Ok(Slider {
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

    /// Get the step frequency.
    pub fn step_frequency(&self) -> f64 {
        *self.inner.step_frequency.read()
    }

    /// Set the step frequency.
    pub fn set_step_frequency(&self, step: f64) {
        *self.inner.step_frequency.write() = step;
    }

    /// Set the step frequency (fluent API).
    pub fn with_step_frequency(self, step: f64) -> Self {
        self.set_step_frequency(step);
        self
    }

    /// Get the orientation.
    pub fn orientation(&self) -> SliderOrientation {
        *self.inner.orientation.read()
    }

    /// Set the orientation.
    pub fn set_orientation(&self, orientation: SliderOrientation) {
        *self.inner.orientation.write() = orientation;
    }

    /// Set the orientation (fluent API).
    pub fn with_orientation(self, orientation: SliderOrientation) -> Self {
        self.set_orientation(orientation);
        self
    }

    /// Subscribe to the value changed event.
    pub fn value_changed(&self) -> &EventHandler<ValueChangedEventArgs<f64>> {
        &self.inner.value_changed
    }

    /// Get the underlying UI element.
    pub fn element(&self) -> &UIElement {
        &self.element
    }

    /// Get the HWND of this slider.
    pub fn hwnd(&self) -> HWND {
        self.element.hwnd()
    }
}

impl Default for Slider {
    fn default() -> Self {
        Self::new().expect("Failed to create slider")
    }
}

impl From<Slider> for UIElement {
    fn from(slider: Slider) -> Self {
        slider.element.clone()
    }
}
