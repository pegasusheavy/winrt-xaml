//! XAML parsing and loading - stub implementation.

mod parser;

pub use self::parser::XamlReader;

use crate::controls::UIElement;
use crate::error::{Error, Result};

/// Load XAML from a string.
pub fn load_xaml(_xaml: &str) -> Result<UIElement> {
    // TODO: Implement XAML parsing
    Err(Error::not_implemented("XAML parsing not yet implemented"))
}

/// Parse XAML into a UI element tree.
pub fn parse_xaml(_xaml: &str) -> Result<UIElement> {
    // TODO: Implement XAML parsing
    Err(Error::not_implemented("XAML parsing not yet implemented"))
}
