//! # WinRT-XAML
//!
//! A Rust library for creating UIs using WinRT and XAML.
//!
//! This library provides a safe, idiomatic Rust interface to Windows Runtime (WinRT)
//! XAML APIs, enabling the creation of modern Windows applications with XAML-based UIs.
//!
//! ## Features
//!
//! - **Application Management**: Easy application lifecycle management
//! - **Window Management**: Create and manage windows with XAML content
//! - **XAML Controls**: Rich set of UI controls (Button, TextBlock, TextBox, etc.)
//! - **Layout Panels**: Flexible layout system (StackPanel, Grid, Canvas)
//! - **Event Handling**: Type-safe event subscription and handling
//! - **Data Binding**: Reactive data binding support
//! - **XAML Parsing**: Runtime and compile-time XAML parsing
//! - **Compile-Time XAML**: `xaml!` macro for zero-cost abstractions
//! - **Styling**: Resource dictionaries and style management
//!
//! ## Example
//!
//! ```rust,no_run
//! use winrt_xaml::prelude::*;
//!
//! fn main() -> Result<()> {
//!     let app = Application::new()?;
//!
//!     let window = Window::new()?
//!         .title("My XAML App")
//!         .size(800, 600);
//!
//!     let button = Button::new()
//!         .content("Click Me!")
//!         .on_click(|_| {
//!             println!("Button clicked!");
//!         });
//!
//!     window.set_content(button)?;
//!     app.run()
//! }
//! ```
//!
//! ## XAML Islands
//!
//! For desktop (Win32) applications, this library uses XAML Islands to host
//! XAML content within traditional windows. Enable the `xaml-islands` feature
//! (enabled by default) to use this functionality.

#![cfg(windows)]
#![warn(missing_docs)]
#![warn(rust_2018_idioms)]

#[cfg(any(feature = "xaml-islands", feature = "uwp"))]
pub mod winrt;
pub mod xaml_native;

#[cfg(any(feature = "xaml-islands", feature = "uwp"))]
pub mod xaml_islands;
#[cfg(any(feature = "xaml-islands", feature = "uwp"))]
pub mod app;
#[cfg(any(feature = "xaml-islands", feature = "uwp"))]
pub mod controls;
#[cfg(any(feature = "xaml-islands", feature = "uwp"))]
pub mod error;
#[cfg(any(feature = "xaml-islands", feature = "uwp"))]
pub mod events;
#[cfg(any(feature = "xaml-islands", feature = "uwp"))]
pub mod layout;
#[cfg(any(feature = "xaml-islands", feature = "uwp"))]
pub mod media;
#[cfg(any(feature = "xaml-islands", feature = "uwp"))]
pub mod resources;
#[cfg(any(feature = "xaml-islands", feature = "uwp"))]
pub mod window;

/// Reactive state management and data binding.
#[cfg(any(feature = "xaml-islands", feature = "uwp"))]
pub mod reactive;

// Re-export windows crate for advanced usage
#[cfg(any(feature = "xaml-islands", feature = "uwp"))]
pub use windows;

/// Prelude module for convenient imports
#[cfg(any(feature = "xaml-islands", feature = "uwp"))]
pub mod prelude {
    pub use crate::app::Application;
    pub use crate::controls::*;
    pub use crate::error::{Error, Result};
    pub use crate::events::*;
    pub use crate::layout::{self, *};
    pub use crate::media::*;
    pub use crate::resources::*;
    pub use crate::window::Window;

    // Re-export WinRT XAML types
    #[cfg(feature = "xaml-islands")]
    pub use crate::xaml_native::{
        ImageStretch, ListViewSelectionMode, ScrollBarVisibility, ScrollMode, XamlButton,
        XamlCheckBox, XamlComboBox, XamlGrid, XamlImage, XamlListView, XamlManager,
        XamlProgressBar, XamlRadioButton, XamlScrollViewer, XamlSlider, XamlSource,
        XamlStackPanel, XamlTextBlock, XamlTextBox, XamlUIElement,
    };

    // Re-export reactive types
    #[cfg(any(feature = "xaml-islands", feature = "uwp"))]
    pub use crate::reactive::{Property, ObservableCollection, Computed, CollectionChange};
}

/// Re-export of the Result type with our Error
#[cfg(any(feature = "xaml-islands", feature = "uwp"))]
pub type Result<T> = std::result::Result<T, error::Error>;

// Re-export the compile-time XAML macro
/// Compile-time XAML parsing macro.
///
/// Parses XAML at compile time and generates Rust code to create WinRT controls.
///
/// # Example
///
/// ```rust,ignore
/// use winrt_xaml::xaml;
///
/// let button = xaml! {
///     r#"<Button Content="Click Me"
///               Width="200"
///               Height="50"
///               Background="#FF0078D4"
///               Foreground="#FFFFFFFF"
///               CornerRadius="8" />"#
/// };
/// ```
pub use winrt_xaml_macros::xaml;
