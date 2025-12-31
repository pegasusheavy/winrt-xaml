//! XAML parser implementation.
//!
//! This module provides basic XAML parsing functionality for creating UI elements
//! from XAML markup strings.

use crate::controls::{Button, TextBlock, TextBox, UIElement, CheckBox};
use crate::layout::{StackPanel, Grid, Border, Orientation};
use crate::error::{Error, Result};
use quick_xml::events::{Event, BytesStart};
use quick_xml::Reader;
use std::str;

/// XAML reader for parsing XAML markup.
pub struct XamlReader;

impl XamlReader {
    /// Load XAML from a string and return the root UIElement.
    pub fn load(xaml: &str) -> Result<UIElement> {
        Self::parse(xaml)
    }

    /// Parse XAML into a UI element tree.
    pub fn parse(xaml: &str) -> Result<UIElement> {
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

    fn parse_element(element: &BytesStart, element_name: &str) -> Result<UIElement> {
        match element_name {
            "Button" => Self::parse_button(element),
            "TextBlock" => Self::parse_textblock(element),
            "TextBox" => Self::parse_textbox(element),
            "CheckBox" => Self::parse_checkbox(element),
            "StackPanel" => Self::parse_stackpanel(element),
            "Grid" => Self::parse_grid(element),
            "Border" => Self::parse_border(element),
            _ => Err(Error::xaml_parse(format!("Unknown element: {}", element_name))),
        }
    }

    fn parse_button(element: &BytesStart) -> Result<UIElement> {
        let button = Button::new()?;

        for attr in element.attributes() {
            let attr = attr.map_err(|e| Error::xaml_parse(format!("Attribute error: {}", e)))?;
            let key = str::from_utf8(attr.key.as_ref())
                .map_err(|e| Error::xaml_parse(format!("Invalid UTF-8: {}", e)))?;
            let value = attr.unescape_value()
                .map_err(|e| Error::xaml_parse(format!("Invalid attribute value: {}", e)))?;

            match key {
                "Content" => button.set_content(value.as_ref())?,
                "Width" => {
                    if let Ok(width) = value.parse::<i32>() {
                        button.element().set_width(width);
                    }
                }
                "Height" => {
                    if let Ok(height) = value.parse::<i32>() {
                        button.element().set_height(height);
                    }
                }
                "x:Name" | "Name" => {
                    // Store name for later reference (future feature)
                }
                _ => {
                    // Ignore unknown attributes for now
                }
            }
        }

        Ok(button.into())
    }

    fn parse_textblock(element: &BytesStart) -> Result<UIElement> {
        let textblock = TextBlock::new()?;

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
                        textblock.set_font_size(size);
                    }
                }
                "Width" => {
                    if let Ok(width) = value.parse::<i32>() {
                        textblock.element().set_width(width);
                    }
                }
                "Height" => {
                    if let Ok(height) = value.parse::<i32>() {
                        textblock.element().set_height(height);
                    }
                }
                _ => {}
            }
        }

        Ok(textblock.into())
    }

    fn parse_textbox(element: &BytesStart) -> Result<UIElement> {
        let textbox = TextBox::new()?;

        for attr in element.attributes() {
            let attr = attr.map_err(|e| Error::xaml_parse(format!("Attribute error: {}", e)))?;
            let key = str::from_utf8(attr.key.as_ref())
                .map_err(|e| Error::xaml_parse(format!("Invalid UTF-8: {}", e)))?;
            let value = attr.unescape_value()
                .map_err(|e| Error::xaml_parse(format!("Invalid attribute value: {}", e)))?;

            match key {
                "Text" => textbox.set_text(value.as_ref())?,
                "PlaceholderText" => textbox.set_placeholder(value.as_ref()),
                "Width" => {
                    if let Ok(width) = value.parse::<i32>() {
                        textbox.element().set_width(width);
                    }
                }
                "Height" => {
                    if let Ok(height) = value.parse::<i32>() {
                        textbox.element().set_height(height);
                    }
                }
                _ => {}
            }
        }

        Ok(textbox.into())
    }

    fn parse_checkbox(element: &BytesStart) -> Result<UIElement> {
        let checkbox = CheckBox::new()?;

        for attr in element.attributes() {
            let attr = attr.map_err(|e| Error::xaml_parse(format!("Attribute error: {}", e)))?;
            let key = str::from_utf8(attr.key.as_ref())
                .map_err(|e| Error::xaml_parse(format!("Invalid UTF-8: {}", e)))?;
            let value = attr.unescape_value()
                .map_err(|e| Error::xaml_parse(format!("Invalid attribute value: {}", e)))?;

            match key {
                "Content" => checkbox.set_content(value.as_ref())?,
                "IsChecked" => {
                    if let Ok(checked) = value.parse::<bool>() {
                        checkbox.set_checked(checked);
                    }
                }
                _ => {}
            }
        }

        Ok(checkbox.into())
    }

    fn parse_stackpanel(element: &BytesStart) -> Result<UIElement> {
        let panel = StackPanel::new()?;

        for attr in element.attributes() {
            let attr = attr.map_err(|e| Error::xaml_parse(format!("Attribute error: {}", e)))?;
            let key = str::from_utf8(attr.key.as_ref())
                .map_err(|e| Error::xaml_parse(format!("Invalid UTF-8: {}", e)))?;
            let value = attr.unescape_value()
                .map_err(|e| Error::xaml_parse(format!("Invalid attribute value: {}", e)))?;

            match key {
                "Orientation" => {
                    let orientation = match value.as_ref() {
                        "Horizontal" => Orientation::Horizontal,
                        "Vertical" => Orientation::Vertical,
                        _ => Orientation::Vertical,
                    };
                    panel.set_orientation(orientation);
                }
                "Spacing" => {
                    if let Ok(spacing) = value.parse::<i32>() {
                        panel.set_spacing(spacing);
                    }
                }
                _ => {}
            }
        }

        Ok(panel.into())
    }

    fn parse_grid(_element: &BytesStart) -> Result<UIElement> {
        let grid = Grid::new()?;
        Ok(grid.into())
    }

    fn parse_border(_element: &BytesStart) -> Result<UIElement> {
        let border = Border::new()?;
        Ok(border.into())
    }
}
