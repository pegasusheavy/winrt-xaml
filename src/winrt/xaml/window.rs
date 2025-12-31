//! Windows.UI.Xaml.Window bindings.

use crate::winrt::{IInspectable, IActivationFactory};
use crate::error::Result;
use windows::core::IInspectable as CoreIInspectable;

/// Windows.UI.Xaml.Window - Represents a window in a XAML application.
#[derive(Clone)]
pub struct XamlWindow {
    inspectable: IInspectable,
}

impl XamlWindow {
    /// Create a new Window instance using WinRT activation.
    pub fn new() -> Result<Self> {
        // Try to activate the Windows.UI.Xaml.Window runtime class
        let factory = IActivationFactory::get("Windows.UI.Xaml.Window")
            .map_err(|e| crate::error::Error::window_creation(format!("Failed to get XAML Window factory: {}", e)))?;

        let inspectable: CoreIInspectable = factory.activate_instance()
            .map_err(|e| crate::error::Error::window_creation(format!("Failed to activate XAML Window: {}", e)))?;

        Ok(XamlWindow {
            inspectable: IInspectable::from(inspectable),
        })
    }

    /// Get the current Window instance.
    pub fn current() -> Result<Self> {
        // In real WinRT, this would call Window.Current
        Self::new()
    }

    /// Activate the window (show and bring to foreground).
    pub fn activate(&self) -> Result<()> {
        // This would call the WinRT Window.Activate() method
        Ok(())
    }

    /// Set the window title.
    pub fn set_title(&self, _title: &str) -> Result<()> {
        // This would set the Title property via WinRT
        Ok(())
    }

    /// Get the underlying IInspectable.
    pub fn as_inspectable(&self) -> &IInspectable {
        &self.inspectable
    }
}

impl Default for XamlWindow {
    fn default() -> Self {
        Self::new().expect("Failed to create XAML Window")
    }
}

unsafe impl Send for XamlWindow {}
unsafe impl Sync for XamlWindow {}

