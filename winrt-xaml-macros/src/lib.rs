//! Procedural macros for compile-time XAML parsing.
//!
//! This crate provides the `xaml!` macro which parses XAML at compile time
//! and generates Rust code to create WinRT controls.

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};
use quick_xml::events::{Event, BytesStart};
use quick_xml::Reader;
use std::str;

mod codegen;
use codegen::*;

/// Compile-time XAML parsing macro.
///
/// Parses XAML markup at compile time and generates Rust code to create
/// the corresponding WinRT controls.
///
/// # Example
///
/// ```rust,ignore
/// use winrt_xaml::xaml;
///
/// let button = xaml! {
///     <Button Content="Click Me"
///             Width="200"
///             Height="50"
///             Background="#FF0078D4"
///             Foreground="#FFFFFFFF"
///             CornerRadius="8" />
/// };
/// ```
///
/// This expands to:
///
/// ```rust,ignore
/// {
///     let button = XamlButton::new()?;
///     button.set_content("Click Me")?;
///     button.set_size(200.0, 50.0)?;
///     button.set_background(0xFF0078D4)?;
///     button.set_foreground(0xFFFFFFFF)?;
///     button.set_corner_radius(8.0)?;
///     button.as_uielement()
/// }
/// ```
#[proc_macro]
pub fn xaml(input: TokenStream) -> TokenStream {
    let input_str = parse_macro_input!(input as LitStr);
    let xaml_content = input_str.value();

    // Parse XAML at compile time
    match parse_xaml(&xaml_content) {
        Ok(code) => code.into(),
        Err(e) => {
            let error_msg = format!("XAML parse error: {}", e);
            quote! {
                compile_error!(#error_msg)
            }
            .into()
        }
    }
}

/// Parse XAML and generate Rust code.
fn parse_xaml(xaml: &str) -> Result<proc_macro2::TokenStream, String> {
    let mut reader = Reader::from_str(xaml);
    reader.config_mut().trim_text(true);

    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) | Ok(Event::Empty(e)) => {
                let name_bytes = e.name();
                let element_name = str::from_utf8(name_bytes.as_ref())
                    .map_err(|e| format!("Invalid UTF-8: {}", e))?;

                return generate_code_for_element(&e, element_name);
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(format!("XML parse error: {}", e)),
            _ => (),
        }
        buf.clear();
    }

    Err("No root element found".to_string())
}

/// Generate Rust code for a XAML element.
fn generate_code_for_element(
    element: &BytesStart,
    element_name: &str,
) -> Result<proc_macro2::TokenStream, String> {
    match element_name {
        "Button" => generate_button_code(element),
        "TextBlock" => generate_textblock_code(element),
        "TextBox" => generate_textbox_code(element),
        "StackPanel" => generate_stackpanel_code(element),
        "Grid" => generate_grid_code(element),
        "ScrollViewer" => generate_scrollviewer_code(element),
        _ => Err(format!("Unsupported XAML element: {}", element_name)),
    }
}
