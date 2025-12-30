//! Windows Runtime (WinRT) infrastructure for XAML bindings.
//!
//! This module provides the low-level WinRT ABI bindings needed to interact
//! with Windows.UI.Xaml APIs that are not available in the windows crate.

pub mod abi;
pub mod hstring;
pub mod inspectable;
pub mod factory;
pub mod xaml;
pub mod properties;
pub mod events;
pub mod xaml_manager;

pub use hstring::HString;
pub use inspectable::IInspectable;
pub use factory::IActivationFactory;
pub use properties::{PropertyAccess, WinRTObject};
pub use events::{
    EventToken, WinRTEvent, WinRTRoutedEventArgs, WinRTClickEventArgs,
    WinRTPropertyChangedEventArgs,
};
pub use xaml_manager::WindowsXamlManager;

