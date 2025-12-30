//! ToggleSwitch control - stub implementation.

use crate::controls::UIElement;
use crate::error::Result;
use crate::events::{CheckedEventArgs, EventHandler};
use parking_lot::RwLock;
use std::sync::Arc;
use windows::Win32::Foundation::*;

/// A toggle switch control.
#[derive(Clone)]
pub struct ToggleSwitch {
    element: UIElement,
    inner: Arc<ToggleSwitchInner>,
}

struct ToggleSwitchInner {
    is_on: RwLock<bool>,
    header: RwLock<String>,
    on_content: RwLock<String>,
    off_content: RwLock<String>,
    toggled: EventHandler<CheckedEventArgs>,
}

impl ToggleSwitch {
    /// Create a new toggle switch.
    pub fn new() -> Result<Self> {
        let inner = Arc::new(ToggleSwitchInner {
            is_on: RwLock::new(false),
            header: RwLock::new(String::new()),
            on_content: RwLock::new("On".to_string()),
            off_content: RwLock::new("Off".to_string()),
            toggled: EventHandler::new(),
        });

        Ok(ToggleSwitch {
            element: UIElement::empty(),
            inner,
        })
    }

    /// Check if the toggle is on.
    pub fn is_on(&self) -> bool {
        *self.inner.is_on.read()
    }

    /// Set the toggle state.
    pub fn set_is_on(&self, is_on: bool) {
        *self.inner.is_on.write() = is_on;
    }

    /// Get the header text.
    pub fn header(&self) -> String {
        self.inner.header.read().clone()
    }

    /// Set the header text.
    pub fn set_header(&self, header: impl Into<String>) {
        *self.inner.header.write() = header.into();
    }

    /// Set the header text (fluent API).
    pub fn with_header(self, header: impl Into<String>) -> Self {
        self.set_header(header);
        self
    }

    /// Subscribe to the toggled event.
    pub fn toggled(&self) -> &EventHandler<CheckedEventArgs> {
        &self.inner.toggled
    }

    /// Get the underlying UI element.
    pub fn element(&self) -> &UIElement {
        &self.element
    }

    /// Get the HWND of this toggle switch.
    pub fn hwnd(&self) -> HWND {
        self.element.hwnd()
    }
}

impl Default for ToggleSwitch {
    fn default() -> Self {
        Self::new().expect("Failed to create toggle switch")
    }
}

impl From<ToggleSwitch> for UIElement {
    fn from(toggle: ToggleSwitch) -> Self {
        toggle.element.clone()
    }
}
