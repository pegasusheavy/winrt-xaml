//! Windows.UI.Xaml.UIElement bindings.

use crate::winrt::IInspectable;

/// Windows.UI.Xaml.UIElement - Base class for all XAML visual elements.
#[derive(Clone)]
pub struct XamlUIElement {
    inspectable: IInspectable,
}

impl XamlUIElement {
    /// Create a UIElement from an IInspectable.
    pub fn from_inspectable(inspectable: IInspectable) -> Self {
        XamlUIElement { inspectable }
    }

    /// Get the underlying IInspectable.
    pub fn as_inspectable(&self) -> &IInspectable {
        &self.inspectable
    }
}

unsafe impl Send for XamlUIElement {}
unsafe impl Sync for XamlUIElement {}


