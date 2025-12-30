//! Windows.UI.Xaml.Controls.StackPanel bindings.

use crate::winrt::{IInspectable, IActivationFactory};
use crate::error::Result;
use windows::core::IInspectable as CoreIInspectable;

/// Windows.UI.Xaml.Controls.StackPanel - A panel that arranges children in a stack.
#[derive(Clone)]
pub struct XamlStackPanel {
    inspectable: IInspectable,
}

impl XamlStackPanel {
    /// Create a new StackPanel using WinRT activation.
    pub fn new() -> Result<Self> {
        let factory = IActivationFactory::get("Windows.UI.Xaml.Controls.StackPanel")
            .map_err(|e| crate::error::Error::control_creation(format!("Failed to get StackPanel factory: {}", e)))?;

        let inspectable: CoreIInspectable = factory.activate_instance()
            .map_err(|e| crate::error::Error::control_creation(format!("Failed to activate StackPanel: {}", e)))?;

        Ok(XamlStackPanel {
            inspectable: IInspectable::from(inspectable),
        })
    }

    /// Get the underlying IInspectable.
    pub fn as_inspectable(&self) -> &IInspectable {
        &self.inspectable
    }
}

impl Default for XamlStackPanel {
    fn default() -> Self {
        Self::new().expect("Failed to create XAML StackPanel")
    }
}

unsafe impl Send for XamlStackPanel {}
unsafe impl Sync for XamlStackPanel {}

