//! Native XAML Islands interop via C++ helper DLL.

pub mod ffi;

pub use ffi::{
    XamlManager, XamlSource, XamlButton,
    initialize_xaml, create_xaml_source, create_xaml_button,
};

