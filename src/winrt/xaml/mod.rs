//! Windows.UI.Xaml bindings - WinRT XAML types.
//!
//! This module provides bindings to the Windows.UI.Xaml namespace,
//! which are not available in the standard windows crate.

pub mod application;
pub mod window;
pub mod uielement;
pub mod controls;
pub mod panels;

pub use application::XamlApplication;
pub use window::XamlWindow;
pub use uielement::XamlUIElement;

/// Namespace constants for XAML classes.
pub mod class_names {
    pub const APPLICATION: &str = "Windows.UI.Xaml.Application";
    pub const WINDOW: &str = "Windows.UI.Xaml.Window";
    pub const BUTTON: &str = "Windows.UI.Xaml.Controls.Button";
    pub const TEXTBLOCK: &str = "Windows.UI.Xaml.Controls.TextBlock";
    pub const TEXTBOX: &str = "Windows.UI.Xaml.Controls.TextBox";
    pub const STACKPANEL: &str = "Windows.UI.Xaml.Controls.StackPanel";
    pub const GRID: &str = "Windows.UI.Xaml.Controls.Grid";
}

