//! Button control implementation using Win32.

use crate::controls::{Control, UIElement};
use crate::error::{Error, Result};
use crate::events::{ClickEventArgs, EventHandler};
use parking_lot::RwLock;
use std::sync::Arc;
use windows::core::{w, PCWSTR};
use windows::Win32::{
    Foundation::*,
    System::LibraryLoader::GetModuleHandleW,
    UI::WindowsAndMessaging::*,
};

/// A button control.
#[derive(Clone, Debug)]
pub struct Button {
    element: UIElement,
    inner: Arc<ButtonInner>,
}

#[derive(Debug)]
struct ButtonInner {
    content: RwLock<String>,
    click: EventHandler<ClickEventArgs>,
}

impl Control for Button {
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

impl Button {
    /// Create a new button.
    pub fn new() -> Result<Self> {
        let inner = Arc::new(ButtonInner {
            content: RwLock::new(String::new()),
            click: EventHandler::new(),
        });

        Ok(Button {
            element: UIElement::empty(),
            inner,
        })
    }

    /// Create the Win32 button control.
    pub fn create(&self, parent: HWND) -> Result<()> {
        unsafe {
            let hinstance = GetModuleHandleW(None)?;
            let content = self.inner.content.read().clone();
            let content_wide: Vec<u16> = content.encode_utf16().chain(Some(0)).collect();

            // Get position and size from the element
            let (x, y) = self.element.position();
            let (width, height) = self.element.size();

            println!("Creating button at ({}, {}) size {}x{} with text: '{}'", x, y, width, height, content);

            let hwnd = CreateWindowExW(
                WINDOW_EX_STYLE(0),
                w!("BUTTON"),
                PCWSTR(content_wide.as_ptr()),
                WS_CHILD | WS_VISIBLE | WS_TABSTOP | WINDOW_STYLE(BS_PUSHBUTTON as u32),
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
                return Err(Error::control_creation("Failed to create button"));
            }

            println!("Button created successfully with HWND: {:?}", hwnd);
            self.element.set_hwnd(hwnd);

            Ok(())
        }
    }

    /// Get the button content text.
    pub fn content(&self) -> String {
        self.inner.content.read().clone()
    }

    /// Set the button content text.
    pub fn set_content(&self, content: impl Into<String>) -> Result<()> {
        let content = content.into();
        *self.inner.content.write() = content.clone();

        let hwnd = self.element.hwnd();
        if !hwnd.0.is_null() {
            unsafe {
                let content_wide: Vec<u16> = content.encode_utf16().chain(Some(0)).collect();
                SetWindowTextW(hwnd, PCWSTR(content_wide.as_ptr()))?;
            }
        }

        Ok(())
    }

    /// Set the button content text (fluent API).
    pub fn with_content(self, content: impl Into<String>) -> Result<Self> {
        self.set_content(content)?;
        Ok(self)
    }

    /// Subscribe to the click event.
    pub fn click(&self) -> &EventHandler<ClickEventArgs> {
        &self.inner.click
    }

    /// Trigger the click event (internal use).
    pub(crate) fn trigger_click(&self) {
        let args = ClickEventArgs::new();
        self.inner.click.invoke(&args);
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

    /// Get the underlying UI element.
    pub fn element(&self) -> &UIElement {
        &self.element
    }

    /// Get the HWND of this button.
    pub fn hwnd(&self) -> HWND {
        self.element.hwnd()
    }

    /// Set the position of the button.
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

    /// Set the size of the button.
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

    /// Set whether the button is enabled.
    pub fn set_enabled(&self, enabled: bool) -> Result<()> {
        self.element.set_enabled(enabled);
        // TODO: Call EnableWindow when available in windows crate
        Ok(())
    }

    /// Check if the button is enabled.
    pub fn is_enabled(&self) -> bool {
        self.element.is_enabled()
    }
}

impl Default for Button {
    fn default() -> Self {
        Self::new().expect("Failed to create button")
    }
}

impl From<Button> for UIElement {
    fn from(button: Button) -> Self {
        button.element.clone()
    }
}
