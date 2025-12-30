//! Windows.UI.Xaml.Controls.TextBlock bindings.

use crate::winrt::{IInspectable, IActivationFactory, PropertyAccess};
use crate::error::Result;
use windows::core::IInspectable as CoreIInspectable;

/// Windows.UI.Xaml.Controls.TextBlock - A text display control.
#[derive(Clone)]
pub struct XamlTextBlock {
    inspectable: IInspectable,
}

impl XamlTextBlock {
    /// Create a new TextBlock using WinRT activation.
    pub fn new() -> Result<Self> {
        let factory = IActivationFactory::get("Windows.UI.Xaml.Controls.TextBlock")
            .map_err(|e| crate::error::Error::control_creation(format!("Failed to get TextBlock factory: {}", e)))?;

        let inspectable: CoreIInspectable = factory.activate_instance()
            .map_err(|e| crate::error::Error::control_creation(format!("Failed to activate TextBlock: {}", e)))?;

        Ok(XamlTextBlock {
            inspectable: IInspectable::from(inspectable),
        })
    }

    /// Get the underlying IInspectable.
    pub fn as_inspectable(&self) -> &IInspectable {
        &self.inspectable
    }

    /// Set the text content.
    pub fn set_text(&self, text: &str) -> Result<()> {
        self.set_string_property("Text", text)
    }

    /// Get the text content.
    pub fn get_text(&self) -> Result<String> {
        self.get_string_property("Text")
    }

    /// Set the font size.
    pub fn set_font_size(&self, size: f64) -> Result<()> {
        self.set_numeric_property("FontSize", size)
    }
}

impl PropertyAccess for XamlTextBlock {
    fn inspectable(&self) -> &IInspectable {
        &self.inspectable
    }
}

impl Default for XamlTextBlock {
    fn default() -> Self {
        Self::new().expect("Failed to create XAML TextBlock")
    }
}

unsafe impl Send for XamlTextBlock {}
unsafe impl Sync for XamlTextBlock {}

