//! TextBlock control implementation using Win32 STATIC control.

use crate::controls::{Control, UIElement};
use crate::error::{Error, Result};
use parking_lot::RwLock;
use std::sync::Arc;
use windows::core::{w, PCWSTR};
use windows::Win32::{
    Foundation::*,
    System::LibraryLoader::GetModuleHandleW,
    UI::WindowsAndMessaging::*,
};

// Static control styles
const SS_LEFT: u32 = 0x00000000;
const SS_CENTER: u32 = 0x00000001;
const SS_RIGHT: u32 = 0x00000002;

/// Text alignment options.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextAlignment {
    /// Left-aligned text.
    Left,
    /// Center-aligned text.
    Center,
    /// Right-aligned text.
    Right,
}

/// A text display control.
#[derive(Clone, Debug)]
pub struct TextBlock {
    element: UIElement,
    inner: Arc<TextBlockInner>,
}

#[derive(Debug)]
struct TextBlockInner {
    text: RwLock<String>,
    font_size: RwLock<f64>,
    alignment: RwLock<TextAlignment>,
}

impl Control for TextBlock {
    fn create_control(&self, parent: HWND) -> Result<()> {
        self.create(parent)
    }

    fn as_element(&self) -> &UIElement {
        &self.element
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl TextBlock {
    /// Create a new text block.
    pub fn new() -> Result<Self> {
        let inner = Arc::new(TextBlockInner {
            text: RwLock::new(String::new()),
            font_size: RwLock::new(14.0),
            alignment: RwLock::new(TextAlignment::Left),
        });

        Ok(TextBlock {
            element: UIElement::empty(),
            inner,
        })
    }

    /// Create the Win32 static control.
    pub(crate) fn create(&self, parent: HWND) -> Result<()> {
        unsafe {
            let hinstance = GetModuleHandleW(None)?;
            let text = self.inner.text.read().clone();
            let text_wide: Vec<u16> = text.encode_utf16().chain(Some(0)).collect();

            let alignment = match *self.inner.alignment.read() {
                TextAlignment::Left => SS_LEFT,
                TextAlignment::Center => SS_CENTER,
                TextAlignment::Right => SS_RIGHT,
            };

            // Get position and size from the element
            let (x, y) = self.element.position();
            let (width, height) = self.element.size();

            let hwnd = CreateWindowExW(
                WINDOW_EX_STYLE(0),
                w!("STATIC"),
                PCWSTR(text_wide.as_ptr()),
                WS_CHILD | WS_VISIBLE | WINDOW_STYLE(alignment),
                x,
                y,
                width,
                height,
                parent,
                HMENU(std::ptr::null_mut()),
                HINSTANCE(hinstance.0),
                None,
            )?;

            if hwnd.0.is_null() {
                return Err(Error::control_creation("Failed to create text block"));
            }

            self.element.set_hwnd(hwnd);

            Ok(())
        }
    }

    /// Get the text content.
    pub fn text(&self) -> String {
        self.inner.text.read().clone()
    }

    /// Set the text content.
    pub fn set_text(&self, text: impl Into<String>) -> Result<()> {
        let text = text.into();
        *self.inner.text.write() = text.clone();

        let hwnd = self.element.hwnd();
        if !hwnd.0.is_null() {
            unsafe {
                let text_wide: Vec<u16> = text.encode_utf16().chain(Some(0)).collect();
                SetWindowTextW(hwnd, PCWSTR(text_wide.as_ptr()))?;
            }
        }

        Ok(())
    }

    /// Set the text content (fluent API).
    pub fn with_text(self, text: impl Into<String>) -> Result<Self> {
        self.set_text(text)?;
        Ok(self)
    }

    /// Get the font size.
    pub fn font_size(&self) -> f64 {
        *self.inner.font_size.read()
    }

    /// Set the font size.
    pub fn set_font_size(&self, size: f64) {
        *self.inner.font_size.write() = size;
        // TODO: Update font in Win32 control
    }

    /// Set the font size (fluent API).
    pub fn with_font_size(self, size: f64) -> Self {
        self.set_font_size(size);
        self
    }

    /// Get the text alignment.
    pub fn alignment(&self) -> TextAlignment {
        *self.inner.alignment.read()
    }

    /// Set the text alignment.
    pub fn set_alignment(&self, alignment: TextAlignment) {
        *self.inner.alignment.write() = alignment;
        // TODO: Recreate control with new style
    }

    /// Get the underlying UI element.
    pub fn element(&self) -> &UIElement {
        &self.element
    }

    /// Get the HWND of this text block.
    pub fn hwnd(&self) -> HWND {
        self.element.hwnd()
    }

    /// Set the X position (fluent API).
    pub fn with_x(self, x: i32) -> Self {
        self.element.set_x(x);
        self
    }

    /// Set the Y position (fluent API).
    pub fn with_y(self, y: i32) -> Self {
        self.element.set_y(y);
        self
    }

    /// Set the width (fluent API).
    pub fn with_width(self, width: i32) -> Self {
        self.element.set_width(width);
        self
    }

    /// Set the height (fluent API).
    pub fn with_height(self, height: i32) -> Self {
        self.element.set_height(height);
        self
    }

    /// Set the position of the text block.
    pub fn set_position(&self, x: i32, y: i32) -> Result<()> {
        self.element.set_x(x);
        self.element.set_y(y);

        let hwnd = self.hwnd();
        if !hwnd.0.is_null() {
            unsafe {
                SetWindowPos(
                    hwnd,
                    HWND(std::ptr::null_mut()),
                    x,
                    y,
                    0,
                    0,
                    SWP_NOSIZE | SWP_NOZORDER,
                )?;
            }
        }

        Ok(())
    }

    /// Set the size of the text block.
    pub fn set_size(&self, width: i32, height: i32) -> Result<()> {
        self.element.set_width(width);
        self.element.set_height(height);

        let hwnd = self.hwnd();
        if !hwnd.0.is_null() {
            unsafe {
                SetWindowPos(
                    hwnd,
                    HWND(std::ptr::null_mut()),
                    0,
                    0,
                    width,
                    height,
                    SWP_NOMOVE | SWP_NOZORDER,
                )?;
            }
        }

        Ok(())
    }
}

impl Default for TextBlock {
    fn default() -> Self {
        Self::new().expect("Failed to create text block")
    }
}

impl From<TextBlock> for UIElement {
    fn from(block: TextBlock) -> Self {
        block.element.clone()
    }
}

