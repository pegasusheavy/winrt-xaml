//! UI controls using Win32 APIs.

use crate::error::Result;
use windows::Win32::Foundation::HWND;
use std::fmt::Debug;

mod uielement;
mod button;
mod textblock;
mod textbox;
mod checkbox;
mod combobox;
mod slider;
mod progressbar;
mod toggle;
mod image;
mod listview;

pub use self::button::Button;
pub use self::checkbox::CheckBox;
pub use self::combobox::ComboBox;
pub use self::image::{Image, Stretch};
pub use self::listview::ListView;
pub use self::progressbar::ProgressBar;
pub use self::slider::{Slider, SliderOrientation};
pub use self::textblock::{TextAlignment, TextBlock};
pub use self::textbox::TextBox;
pub use self::toggle::ToggleSwitch;
pub use self::uielement::UIElement;

// Re-export orientation type (needed for XAML compatibility)
pub use SliderOrientation as Orientation;

/// Trait for controls that can be created as Win32 windows.
///
/// This trait allows polymorphic handling of different control types,
/// enabling them to be stored in collections and created uniformly.
pub trait Control: Send + Sync + Debug {
    /// Create the Win32 control with the given parent window.
    ///
    /// This method creates the actual Win32 window for the control,
    /// using the parent window handle and the control's stored properties.
    fn create_control(&self, parent: HWND) -> Result<()>;

    /// Get the underlying UIElement.
    ///
    /// All controls wrap a UIElement which stores common properties
    /// like position, size, visibility, etc.
    fn as_element(&self) -> &UIElement;

    /// Get the HWND of this control (convenience method).
    fn hwnd(&self) -> HWND {
        self.as_element().hwnd()
    }

    /// Check if this control has been created (has a valid HWND).
    fn is_created(&self) -> bool {
        !self.hwnd().0.is_null()
    }

    /// Get a mutable reference as a trait object for downcasting.
    fn as_any(&self) -> &dyn std::any::Any;
}
