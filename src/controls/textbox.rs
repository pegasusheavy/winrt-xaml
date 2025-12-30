//! TextBox control implementation using Win32 EDIT control.

use crate::controls::{Control, UIElement};
use crate::error::{Error, Result};
use crate::events::{EventHandler, TextChangedEventArgs};
use parking_lot::RwLock;
use std::sync::Arc;
use windows::core::{w, PCWSTR};
use windows::Win32::{
    Foundation::*,
    System::LibraryLoader::GetModuleHandleW,
    UI::WindowsAndMessaging::*,
};

// Edit control styles
const ES_LEFT: u32 = 0x0000;
const ES_AUTOHSCROLL: u32 = 0x0080;

/// A text input control.
#[derive(Clone, Debug)]
pub struct TextBox {
    element: UIElement,
    inner: Arc<TextBoxInner>,
}

#[derive(Debug)]
struct TextBoxInner {
    text: RwLock<String>,
    placeholder: RwLock<String>,
    max_length: RwLock<Option<u32>>,
    text_changed: EventHandler<TextChangedEventArgs>,
}

impl Control for TextBox {
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

impl TextBox {
    /// Create a new text box.
    pub fn new() -> Result<Self> {
        let inner = Arc::new(TextBoxInner {
            text: RwLock::new(String::new()),
            placeholder: RwLock::new(String::new()),
            max_length: RwLock::new(None),
            text_changed: EventHandler::new(),
        });

        Ok(TextBox {
            element: UIElement::empty(),
            inner,
        })
    }

    /// Create the Win32 edit control.
    pub(crate) fn create(&self, parent: HWND) -> Result<()> {
        unsafe {
            let hinstance = GetModuleHandleW(None)?;
            let text = self.inner.text.read().clone();
            let text_wide: Vec<u16> = text.encode_utf16().chain(Some(0)).collect();

            let hwnd = CreateWindowExW(
                WINDOW_EX_STYLE(0),
                w!("EDIT"),
                PCWSTR(text_wide.as_ptr()),
                WS_CHILD | WS_VISIBLE | WS_BORDER | WS_TABSTOP | WINDOW_STYLE(ES_LEFT | ES_AUTOHSCROLL),
                0,
                0,
                200,
                25,
                parent,
                HMENU(std::ptr::null_mut()),
                HINSTANCE(hinstance.0),
                None,
            )?;

            if hwnd.0.is_null() {
                return Err(Error::control_creation("Failed to create text box"));
            }

            self.element.set_hwnd(hwnd);
            self.element.set_width(200);
            self.element.set_height(25);

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

    /// Get the placeholder text.
    pub fn placeholder(&self) -> String {
        self.inner.placeholder.read().clone()
    }

    /// Set the placeholder text.
    pub fn set_placeholder(&self, placeholder: impl Into<String>) {
        *self.inner.placeholder.write() = placeholder.into();
    }

    /// Set the placeholder text (fluent API).
    pub fn with_placeholder(self, placeholder: impl Into<String>) -> Self {
        self.set_placeholder(placeholder);
        self
    }

    /// Get the maximum text length.
    pub fn max_length(&self) -> Option<u32> {
        *self.inner.max_length.read()
    }

    /// Set the maximum text length.
    pub fn set_max_length(&self, max_length: Option<u32>) {
        *self.inner.max_length.write() = max_length;
    }

    /// Set the maximum text length (fluent API).
    pub fn with_max_length(self, max_length: Option<u32>) -> Self {
        self.set_max_length(max_length);
        self
    }

    /// Subscribe to the text changed event.
    pub fn text_changed(&self) -> &EventHandler<TextChangedEventArgs> {
        &self.inner.text_changed
    }

    /// Get the underlying UI element.
    pub fn element(&self) -> &UIElement {
        &self.element
    }

    /// Get the HWND of this text box.
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

    /// Set the position of the text box.
    pub fn set_position(&self, x: i32, y: i32) -> Result<()> {
        self.element.set_x(x);
        self.element.set_y(y);

        let hwnd = self.hwnd();
        if !hwnd.0.is_null() {
            unsafe {
                SetWindowPos(hwnd, HWND(std::ptr::null_mut()), x, y, 0, 0, SWP_NOSIZE | SWP_NOZORDER)?;
            }
        }

        Ok(())
    }

    /// Set the size of the text box.
    pub fn set_size(&self, width: i32, height: i32) -> Result<()> {
        self.element.set_width(width);
        self.element.set_height(height);

        let hwnd = self.hwnd();
        if !hwnd.0.is_null() {
            unsafe {
                SetWindowPos(hwnd, HWND(std::ptr::null_mut()), 0, 0, width, height, SWP_NOMOVE | SWP_NOZORDER)?;
            }
        }

        Ok(())
    }
}

impl Default for TextBox {
    fn default() -> Self {
        Self::new().expect("Failed to create text box")
    }
}

impl From<TextBox> for UIElement {
    fn from(textbox: TextBox) -> Self {
        textbox.element.clone()
    }
}

