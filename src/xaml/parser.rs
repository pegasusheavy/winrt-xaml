//! XAML parser implementation.
//!
//! This module provides XAML parsing functionality for creating WinRT XAML UI elements
//! from XAML markup strings. The parser creates actual XamlButton, XamlTextBlock, etc.
//! controls that can be used directly in your UI.

use crate::xaml_native::*;
use crate::error::{Error, Result};
use quick_xml::events::{Event, BytesStart};
use quick_xml::Reader;
use std::str;

/// XAML reader for parsing XAML markup into WinRT XAML controls.
pub struct XamlReader;

impl XamlReader {
    /// Load XAML from a string and return the root XamlUIElement.
    pub fn load(xaml: &str) -> Result<XamlUIElement> {
        Self::parse(xaml)
    }

    /// Parse XAML into a WinRT XAML UI element.
    pub fn parse(xaml: &str) -> Result<XamlUIElement> {
        let mut reader = Reader::from_str(xaml);
        reader.config_mut().trim_text(true);

        let mut buf = Vec::new();

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(e)) | Ok(Event::Empty(e)) => {
                    let name_bytes = e.name();
                    let element_name = str::from_utf8(name_bytes.as_ref())
                        .map_err(|e| Error::xaml_parse(format!("Invalid UTF-8: {}", e)))?;
                    
                    return Self::parse_element(&e, element_name);
                }
                Ok(Event::Eof) => break,
                Err(e) => return Err(Error::xaml_parse(format!("XML parse error: {}", e))),
                _ => (),
            }
            buf.clear();
        }

        Err(Error::xaml_parse("No root element found"))
    }

    fn parse_element(element: &BytesStart, element_name: &str) -> Result<XamlUIElement> {
        match element_name {
            "Button" => Self::parse_button(element),
            "TextBlock" => Self::parse_textblock(element),
            "TextBox" => Self::parse_textbox(element),
            "StackPanel" => Self::parse_stackpanel(element),
            "Grid" => Self::parse_grid(element),
            "ScrollViewer" => Self::parse_scrollviewer(element),
            _ => Err(Error::xaml_parse(format!("Unknown element: {}", element_name))),
        }
    }

    fn parse_button(element: &BytesStart) -> Result<XamlUIElement> {
        let button = XamlButton::new()?;
        
        let mut width: Option<f64> = None;
        let mut height: Option<f64> = None;

        for attr in element.attributes() {
            let attr = attr.map_err(|e| Error::xaml_parse(format!("Attribute error: {}", e)))?;
            let key = str::from_utf8(attr.key.as_ref())
                .map_err(|e| Error::xaml_parse(format!("Invalid UTF-8: {}", e)))?;
            let value = attr.unescape_value()
                .map_err(|e| Error::xaml_parse(format!("Invalid attribute value: {}", e)))?;

            match key {
                "Content" => button.set_content(value.as_ref())?,
                "Width" => {
                    width = value.parse::<f64>().ok();
                }
                "Height" => {
                    height = value.parse::<f64>().ok();
                }
                "Background" => {
                    if let Ok(color) = Self::parse_color(value.as_ref()) {
                        button.set_background(color)?;
                    }
                }
                "Foreground" => {
                    if let Ok(color) = Self::parse_color(value.as_ref()) {
                        button.set_foreground(color)?;
                    }
                }
                "CornerRadius" => {
                    if let Ok(radius) = value.parse::<f64>() {
                        button.set_corner_radius(radius)?;
                    }
                }
                "x:Name" | "Name" => {
                    // Store name for later reference (future feature)
                }
                _ => {
                    // Ignore unknown attributes
                }
            }
        }
        
        // Set size if both width and height were specified
        if let (Some(w), Some(h)) = (width, height) {
            button.set_size(w, h)?;
        }

        Ok(button.as_uielement())
    }

    fn parse_textblock(element: &BytesStart) -> Result<XamlUIElement> {
        let textblock = XamlTextBlock::new()?;

        for attr in element.attributes() {
            let attr = attr.map_err(|e| Error::xaml_parse(format!("Attribute error: {}", e)))?;
            let key = str::from_utf8(attr.key.as_ref())
                .map_err(|e| Error::xaml_parse(format!("Invalid UTF-8: {}", e)))?;
            let value = attr.unescape_value()
                .map_err(|e| Error::xaml_parse(format!("Invalid attribute value: {}", e)))?;

            match key {
                "Text" => textblock.set_text(value.as_ref())?,
                "FontSize" => {
                    if let Ok(size) = value.parse::<f64>() {
                        textblock.set_font_size(size)?;
                    }
                }
                "FontWeight" => {
                    if let Ok(weight) = value.parse::<i32>() {
                        textblock.set_font_weight(weight)?;
                    }
                }
                "Foreground" => {
                    if let Ok(color) = Self::parse_color(value.as_ref()) {
                        textblock.set_foreground(color)?;
                    }
                }
                _ => {}
            }
        }

        Ok(textblock.as_uielement())
    }

    fn parse_textbox(element: &BytesStart) -> Result<XamlUIElement> {
        let textbox = XamlTextBox::new()?;
        
        let mut width: Option<f64> = None;
        let mut height: Option<f64> = None;

        for attr in element.attributes() {
            let attr = attr.map_err(|e| Error::xaml_parse(format!("Attribute error: {}", e)))?;
            let key = str::from_utf8(attr.key.as_ref())
                .map_err(|e| Error::xaml_parse(format!("Invalid UTF-8: {}", e)))?;
            let value = attr.unescape_value()
                .map_err(|e| Error::xaml_parse(format!("Invalid attribute value: {}", e)))?;

            match key {
                "Text" => textbox.set_text(value.as_ref())?,
                "PlaceholderText" => textbox.set_placeholder(value.as_ref())?,
                "Width" => {
                    width = value.parse::<f64>().ok();
                }
                "Height" => {
                    height = value.parse::<f64>().ok();
                }
                "Background" => {
                    if let Ok(color) = Self::parse_color(value.as_ref()) {
                        textbox.set_background(color)?;
                    }
                }
                "Foreground" => {
                    if let Ok(color) = Self::parse_color(value.as_ref()) {
                        textbox.set_foreground(color)?;
                    }
                }
                _ => {}
            }
        }
        
        // Set size if both width and height were specified
        if let (Some(w), Some(h)) = (width, height) {
            textbox.set_size(w, h)?;
        }

        Ok(textbox.as_uielement())
    }

    fn parse_stackpanel(element: &BytesStart) -> Result<XamlUIElement> {
        let panel = XamlStackPanel::new()?;

        for attr in element.attributes() {
            let attr = attr.map_err(|e| Error::xaml_parse(format!("Attribute error: {}", e)))?;
            let key = str::from_utf8(attr.key.as_ref())
                .map_err(|e| Error::xaml_parse(format!("Invalid UTF-8: {}", e)))?;
            let value = attr.unescape_value()
                .map_err(|e| Error::xaml_parse(format!("Invalid attribute value: {}", e)))?;

            match key {
                "Orientation" => {
                    let is_vertical = match value.as_ref() {
                        "Horizontal" => false,
                        "Vertical" | _ => true,
                    };
                    panel.set_vertical(is_vertical)?;
                }
                "Spacing" => {
                    if let Ok(spacing) = value.parse::<f64>() {
                        panel.set_spacing(spacing)?;
                    }
                }
                "Background" => {
                    if let Ok(color) = Self::parse_color(value.as_ref()) {
                        panel.set_background(color)?;
                    }
                }
                _ => {}
            }
        }

        Ok(panel.as_uielement())
    }

    fn parse_grid(_element: &BytesStart) -> Result<XamlUIElement> {
        let grid = XamlGrid::new()?;
        Ok(grid.as_uielement())
    }

    fn parse_scrollviewer(_element: &BytesStart) -> Result<XamlUIElement> {
        let scrollviewer = XamlScrollViewer::new()?;
        Ok(scrollviewer.as_uielement())
    }

    /// Parse a color string in format "#AARRGGBB" or "#RRGGBB"
    fn parse_color(color_str: &str) -> Result<u32> {
        if !color_str.starts_with('#') {
            return Err(Error::xaml_parse(format!("Invalid color format: {}", color_str)));
        }

        let hex = &color_str[1..];
        let value = u32::from_str_radix(hex, 16)
            .map_err(|e| Error::xaml_parse(format!("Invalid hex color: {}", e)))?;

        // If 6 digits (RGB), add full alpha
        if hex.len() == 6 {
            Ok(0xFF000000 | value)
        } else if hex.len() == 8 {
            Ok(value)
        } else {
            Err(Error::xaml_parse(format!("Invalid color length: {}", color_str)))
        }
    }
}
