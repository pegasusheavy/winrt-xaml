//! Windows.UI.Xaml.Controls.Grid bindings.

use crate::winrt::{IInspectable, IActivationFactory};
use crate::error::Result;
use windows::core::IInspectable as CoreIInspectable;

/// Windows.UI.Xaml.Controls.Grid - A panel that arranges children in a grid.
#[derive(Clone)]
pub struct XamlGrid {
    inspectable: IInspectable,
}

impl XamlGrid {
    /// Create a new Grid using WinRT activation.
    pub fn new() -> Result<Self> {
        let factory = IActivationFactory::get("Windows.UI.Xaml.Controls.Grid")
            .map_err(|e| crate::error::Error::control_creation(format!("Failed to get Grid factory: {}", e)))?;

        let inspectable: CoreIInspectable = factory.activate_instance()
            .map_err(|e| crate::error::Error::control_creation(format!("Failed to activate Grid: {}", e)))?;

        Ok(XamlGrid {
            inspectable: IInspectable::from(inspectable),
        })
    }

    /// Get the underlying IInspectable.
    pub fn as_inspectable(&self) -> &IInspectable {
        &self.inspectable
    }
}

impl Default for XamlGrid {
    fn default() -> Self {
        Self::new().expect("Failed to create XAML Grid")
    }
}

unsafe impl Send for XamlGrid {}
unsafe impl Sync for XamlGrid {}

