//! Windows.UI.Xaml.Controls.Button bindings.

use crate::winrt::{IInspectable, IActivationFactory, PropertyAccess, WinRTEvent, WinRTClickEventArgs};
use crate::error::Result;
use windows::core::IInspectable as CoreIInspectable;

/// Windows.UI.Xaml.Controls.Button - A button control.
#[derive(Clone)]
pub struct XamlButton {
    inspectable: IInspectable,
    click_event: WinRTEvent<WinRTClickEventArgs>,
}

impl XamlButton {
    /// Create a new Button using WinRT activation.
    pub fn new() -> Result<Self> {
        let factory = IActivationFactory::get("Windows.UI.Xaml.Controls.Button")
            .map_err(|e| crate::error::Error::control_creation(format!("Failed to get Button factory: {}", e)))?;

        let inspectable: CoreIInspectable = factory.activate_instance()
            .map_err(|e| crate::error::Error::control_creation(format!("Failed to activate Button: {}", e)))?;

        Ok(XamlButton {
            inspectable: IInspectable::from(inspectable),
            click_event: WinRTEvent::new(),
        })
    }

    /// Get the underlying IInspectable.
    pub fn as_inspectable(&self) -> &IInspectable {
        &self.inspectable
    }

    /// Set the button content (text or object).
    pub fn set_content(&self, content: &str) -> Result<()> {
        self.set_string_property("Content", content)
    }

    /// Get the button content.
    pub fn get_content(&self) -> Result<String> {
        self.get_string_property("Content")
    }

    /// Set the button width.
    pub fn set_width(&self, width: f64) -> Result<()> {
        self.set_numeric_property("Width", width)
    }

    /// Set the button height.
    pub fn set_height(&self, height: f64) -> Result<()> {
        self.set_numeric_property("Height", height)
    }

    /// Get the Click event for subscribing to button clicks.
    pub fn click(&self) -> &WinRTEvent<WinRTClickEventArgs> {
        &self.click_event
    }

    /// Trigger a click event (for testing or manual invocation).
    pub fn trigger_click(&self) -> Result<()> {
        let args = WinRTClickEventArgs::new("XamlButton");
        self.click_event.invoke(&args)
    }
}

impl PropertyAccess for XamlButton {
    fn inspectable(&self) -> &IInspectable {
        &self.inspectable
    }
}

impl Default for XamlButton {
    fn default() -> Self {
        Self::new().expect("Failed to create XAML Button")
    }
}

unsafe impl Send for XamlButton {}
unsafe impl Sync for XamlButton {}

