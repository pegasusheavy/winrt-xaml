//! Windows.UI.Xaml.Controls.TextBox bindings.

use crate::winrt::{IInspectable, IActivationFactory};
use crate::error::Result;
use windows::core::IInspectable as CoreIInspectable;

/// Windows.UI.Xaml.Controls.TextBox - A text input control.
#[derive(Clone)]
pub struct XamlTextBox {
    inspectable: IInspectable,
}

impl XamlTextBox {
    /// Create a new TextBox using WinRT activation.
    pub fn new() -> Result<Self> {
        let factory = IActivationFactory::get("Windows.UI.Xaml.Controls.TextBox")
            .map_err(|e| crate::error::Error::control_creation(format!("Failed to get TextBox factory: {}", e)))?;

        let inspectable: CoreIInspectable = factory.activate_instance()
            .map_err(|e| crate::error::Error::control_creation(format!("Failed to activate TextBox: {}", e)))?;

        Ok(XamlTextBox {
            inspectable: IInspectable::from(inspectable),
        })
    }

    /// Get the underlying IInspectable.
    pub fn as_inspectable(&self) -> &IInspectable {
        &self.inspectable
    }
}

impl Default for XamlTextBox {
    fn default() -> Self {
        Self::new().expect("Failed to create XAML TextBox")
    }
}

unsafe impl Send for XamlTextBox {}
unsafe impl Sync for XamlTextBox {}

