//! Code generation for XAML elements.

use proc_macro2::TokenStream;
use quote::quote;
use quick_xml::events::BytesStart;
use std::str;

/// Parse a color string like "#AARRGGBB" or "#RRGGBB" into a u32 literal.
pub fn parse_color(color_str: &str) -> Result<u32, String> {
    if !color_str.starts_with('#') {
        return Err(format!("Invalid color format: {}", color_str));
    }

    let hex = &color_str[1..];
    let value = u32::from_str_radix(hex, 16)
        .map_err(|e| format!("Invalid hex color: {}", e))?;

    // If 6 digits (RGB), add full alpha
    if hex.len() == 6 {
        Ok(0xFF000000 | value)
    } else if hex.len() == 8 {
        Ok(value)
    } else {
        Err(format!("Invalid color length: {}", color_str))
    }
}

/// Generate code for a Button element.
pub fn generate_button_code(element: &BytesStart) -> Result<TokenStream, String> {
    let mut content: Option<String> = None;
    let mut width: Option<f64> = None;
    let mut height: Option<f64> = None;
    let mut background: Option<u32> = None;
    let mut foreground: Option<u32> = None;
    let mut corner_radius: Option<f64> = None;
    let mut padding: Option<(f64, f64, f64, f64)> = None;
    let mut margin: Option<(f64, f64, f64, f64)> = None;

    // Parse attributes
    for attr in element.attributes() {
        let attr = attr.map_err(|e| format!("Attribute error: {}", e))?;
        let key = str::from_utf8(attr.key.as_ref())
            .map_err(|e| format!("Invalid UTF-8: {}", e))?;
        let value = attr
            .unescape_value()
            .map_err(|e| format!("Invalid attribute value: {}", e))?;

        match key {
            "Content" => content = Some(value.to_string()),
            "Width" => width = value.parse::<f64>().ok(),
            "Height" => height = value.parse::<f64>().ok(),
            "Background" => background = parse_color(&value).ok(),
            "Foreground" => foreground = parse_color(&value).ok(),
            "CornerRadius" => corner_radius = value.parse::<f64>().ok(),
            "Padding" => {
                // Simple uniform padding for now
                if let Ok(p) = value.parse::<f64>() {
                    padding = Some((p, p, p, p));
                }
            }
            "Margin" => {
                // Simple uniform margin for now
                if let Ok(m) = value.parse::<f64>() {
                    margin = Some((m, m, m, m));
                }
            }
            _ => {} // Ignore unknown attributes
        }
    }

    // Generate code
    let content_code = if let Some(c) = content {
        quote! { __element.set_content(#c)?; }
    } else {
        quote! {}
    };

    let size_code = if let (Some(w), Some(h)) = (width, height) {
        quote! { __element.set_size(#w, #h)?; }
    } else {
        quote! {}
    };

    let background_code = if let Some(bg) = background {
        quote! { __element.set_background(#bg)?; }
    } else {
        quote! {}
    };

    let foreground_code = if let Some(fg) = foreground {
        quote! { __element.set_foreground(#fg)?; }
    } else {
        quote! {}
    };

    let corner_code = if let Some(r) = corner_radius {
        quote! { __element.set_corner_radius(#r)?; }
    } else {
        quote! {}
    };

    let padding_code = if let Some((l, t, r, b)) = padding {
        quote! { __element.set_padding(#l, #t, #r, #b)?; }
    } else {
        quote! {}
    };

    let margin_code = if let Some((l, t, r, b)) = margin {
        quote! { __element.as_uielement().set_margin(#l, #t, #r, #b)?; }
    } else {
        quote! {}
    };

    Ok(quote! {
        (|| -> winrt_xaml::error::Result<winrt_xaml::xaml_native::XamlUIElement> {
            let __element = winrt_xaml::xaml_native::XamlButton::new()?;
            #content_code
            #size_code
            #background_code
            #foreground_code
            #corner_code
            #padding_code
            #margin_code
            Ok(__element.as_uielement())
        })()
    })
}

/// Generate code for a TextBlock element.
pub fn generate_textblock_code(element: &BytesStart) -> Result<TokenStream, String> {
    let mut text: Option<String> = None;
    let mut font_size: Option<f64> = None;
    let mut font_weight: Option<i32> = None;
    let mut foreground: Option<u32> = None;
    let mut margin: Option<(f64, f64, f64, f64)> = None;

    for attr in element.attributes() {
        let attr = attr.map_err(|e| format!("Attribute error: {}", e))?;
        let key = str::from_utf8(attr.key.as_ref())
            .map_err(|e| format!("Invalid UTF-8: {}", e))?;
        let value = attr
            .unescape_value()
            .map_err(|e| format!("Invalid attribute value: {}", e))?;

        match key {
            "Text" => text = Some(value.to_string()),
            "FontSize" => font_size = value.parse::<f64>().ok(),
            "FontWeight" => font_weight = value.parse::<i32>().ok(),
            "Foreground" => foreground = parse_color(&value).ok(),
            "Margin" => {
                if let Ok(m) = value.parse::<f64>() {
                    margin = Some((m, m, m, m));
                }
            }
            _ => {}
        }
    }

    let text_code = if let Some(t) = text {
        quote! { __element.set_text(#t)?; }
    } else {
        quote! {}
    };

    let font_size_code = if let Some(fs) = font_size {
        quote! { __element.set_font_size(#fs)?; }
    } else {
        quote! {}
    };

    let font_weight_code = if let Some(fw) = font_weight {
        quote! { __element.set_font_weight(#fw)?; }
    } else {
        quote! {}
    };

    let foreground_code = if let Some(fg) = foreground {
        quote! { __element.set_foreground(#fg)?; }
    } else {
        quote! {}
    };

    let margin_code = if let Some((l, t, r, b)) = margin {
        quote! { __element.as_uielement().set_margin(#l, #t, #r, #b)?; }
    } else {
        quote! {}
    };

    Ok(quote! {
        (|| -> winrt_xaml::error::Result<winrt_xaml::xaml_native::XamlUIElement> {
            let __element = winrt_xaml::xaml_native::XamlTextBlock::new()?;
            #text_code
            #font_size_code
            #font_weight_code
            #foreground_code
            #margin_code
            Ok(__element.as_uielement())
        })()
    })
}

/// Generate code for a TextBox element.
pub fn generate_textbox_code(element: &BytesStart) -> Result<TokenStream, String> {
    let mut text: Option<String> = None;
    let mut placeholder: Option<String> = None;
    let mut width: Option<f64> = None;
    let mut height: Option<f64> = None;
    let mut background: Option<u32> = None;
    let mut foreground: Option<u32> = None;

    for attr in element.attributes() {
        let attr = attr.map_err(|e| format!("Attribute error: {}", e))?;
        let key = str::from_utf8(attr.key.as_ref())
            .map_err(|e| format!("Invalid UTF-8: {}", e))?;
        let value = attr
            .unescape_value()
            .map_err(|e| format!("Invalid attribute value: {}", e))?;

        match key {
            "Text" => text = Some(value.to_string()),
            "PlaceholderText" => placeholder = Some(value.to_string()),
            "Width" => width = value.parse::<f64>().ok(),
            "Height" => height = value.parse::<f64>().ok(),
            "Background" => background = parse_color(&value).ok(),
            "Foreground" => foreground = parse_color(&value).ok(),
            _ => {}
        }
    }

    let text_code = if let Some(t) = text {
        quote! { __element.set_text(#t)?; }
    } else {
        quote! {}
    };

    let placeholder_code = if let Some(p) = placeholder {
        quote! { __element.set_placeholder(#p)?; }
    } else {
        quote! {}
    };

    let size_code = if let (Some(w), Some(h)) = (width, height) {
        quote! { __element.set_size(#w, #h)?; }
    } else {
        quote! {}
    };

    let background_code = if let Some(bg) = background {
        quote! { __element.set_background(#bg)?; }
    } else {
        quote! {}
    };

    let foreground_code = if let Some(fg) = foreground {
        quote! { __element.set_foreground(#fg)?; }
    } else {
        quote! {}
    };

    Ok(quote! {
        (|| -> winrt_xaml::error::Result<winrt_xaml::xaml_native::XamlUIElement> {
            let __element = winrt_xaml::xaml_native::XamlTextBox::new()?;
            #text_code
            #placeholder_code
            #size_code
            #background_code
            #foreground_code
            Ok(__element.as_uielement())
        })()
    })
}

/// Generate code for a StackPanel element.
pub fn generate_stackpanel_code(element: &BytesStart) -> Result<TokenStream, String> {
    let mut orientation_vertical = true;
    let mut spacing: Option<f64> = None;
    let mut background: Option<u32> = None;

    for attr in element.attributes() {
        let attr = attr.map_err(|e| format!("Attribute error: {}", e))?;
        let key = str::from_utf8(attr.key.as_ref())
            .map_err(|e| format!("Invalid UTF-8: {}", e))?;
        let value = attr
            .unescape_value()
            .map_err(|e| format!("Invalid attribute value: {}", e))?;

        match key {
            "Orientation" => {
                orientation_vertical = match value.as_ref() {
                    "Horizontal" => false,
                    _ => true,
                };
            }
            "Spacing" => spacing = value.parse::<f64>().ok(),
            "Background" => background = parse_color(&value).ok(),
            _ => {}
        }
    }

    let orientation_code = quote! { __element.set_vertical(#orientation_vertical)?; };

    let spacing_code = if let Some(s) = spacing {
        quote! { __element.set_spacing(#s)?; }
    } else {
        quote! {}
    };

    let background_code = if let Some(bg) = background {
        quote! { __element.set_background(#bg)?; }
    } else {
        quote! {}
    };

    Ok(quote! {
        (|| -> winrt_xaml::error::Result<winrt_xaml::xaml_native::XamlUIElement> {
            let __element = winrt_xaml::xaml_native::XamlStackPanel::new()?;
            #orientation_code
            #spacing_code
            #background_code
            Ok(__element.as_uielement())
        })()
    })
}

/// Generate code for a Grid element.
pub fn generate_grid_code(_element: &BytesStart) -> Result<TokenStream, String> {
    Ok(quote! {
        (|| -> winrt_xaml::error::Result<winrt_xaml::xaml_native::XamlUIElement> {
            let __element = winrt_xaml::xaml_native::XamlGrid::new()?;
            Ok(__element.as_uielement())
        })()
    })
}

/// Generate code for a ScrollViewer element.
pub fn generate_scrollviewer_code(_element: &BytesStart) -> Result<TokenStream, String> {
    Ok(quote! {
        (|| -> winrt_xaml::error::Result<winrt_xaml::xaml_native::XamlUIElement> {
            let __element = winrt_xaml::xaml_native::XamlScrollViewer::new()?;
            Ok(__element.as_uielement())
        })()
    })
}
