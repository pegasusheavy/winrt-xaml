//! CheckBox control implementation using Win32 BUTTON with BS_CHECKBOX.

use crate::controls::{Control, UIElement};
use crate::error::{Error, Result};
use crate::events::{CheckedEventArgs, EventHandler};
use parking_lot::RwLock;
use std::sync::Arc;
use windows::core::{w, PCWSTR};
use windows::Win32::{
    Foundation::*,
    System::LibraryLoader::GetModuleHandleW,
    UI::WindowsAndMessaging::*,
};

// Button state constants
const BST_UNCHECKED: usize = 0x0000;
const BST_CHECKED: usize = 0x0001;

/// A checkbox control.
#[derive(Clone, Debug)]
pub struct CheckBox {
    element: UIElement,
    inner: Arc<CheckBoxInner>,
}

#[derive(Debug)]
struct CheckBoxInner {
    content: RwLock<String>,
    is_checked: RwLock<bool>,
    checked: EventHandler<CheckedEventArgs>,
}

impl Control for CheckBox {
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

impl CheckBox {
    /// Create a new checkbox.
    pub fn new() -> Result<Self> {
        let inner = Arc::new(CheckBoxInner {
            content: RwLock::new(String::new()),
            is_checked: RwLock::new(false),
            checked: EventHandler::new(),
        });

        Ok(CheckBox {
            element: UIElement::empty(),
            inner,
        })
    }

    /// Create the Win32 checkbox control.
    pub(crate) fn create(&self, parent: HWND) -> Result<()> {
        unsafe {
            let hinstance = GetModuleHandleW(None)?;
            let content = self.inner.content.read().clone();
            let content_wide: Vec<u16> = content.encode_utf16().chain(Some(0)).collect();

            let hwnd = CreateWindowExW(
                WINDOW_EX_STYLE(0),
                w!("BUTTON"),
                PCWSTR(content_wide.as_ptr()),
                WS_CHILD | WS_VISIBLE | WS_TABSTOP | WINDOW_STYLE(BS_CHECKBOX as u32),
                0,
                0,
                150,
                25,
                parent,
                HMENU(std::ptr::null_mut()),
                HINSTANCE(hinstance.0),
                None,
            )?;

            if hwnd.0.is_null() {
                return Err(Error::control_creation("Failed to create checkbox"));
            }

            self.element.set_hwnd(hwnd);
            self.element.set_width(150);
            self.element.set_height(25);

            Ok(())
        }
    }

    /// Get the checkbox content text.
    pub fn content(&self) -> String {
        self.inner.content.read().clone()
    }

    /// Set the checkbox content text.
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

    /// Set the checkbox content text (fluent API).
    pub fn with_content(self, content: impl Into<String>) -> Result<Self> {
        self.set_content(content)?;
        Ok(self)
    }

    /// Check if the checkbox is checked.
    pub fn is_checked(&self) -> bool {
        *self.inner.is_checked.read()
    }

    /// Set the checked state.
    pub fn set_checked(&self, checked: bool) {
        *self.inner.is_checked.write() = checked;

        let hwnd = self.element.hwnd();
        if !hwnd.0.is_null() {
            unsafe {
                SendMessageW(
                    hwnd,
                    BM_SETCHECK,
                    WPARAM(if checked { BST_CHECKED } else { BST_UNCHECKED }),
                    LPARAM(0),
                );
            }
        }
    }

    /// Subscribe to the checked event.
    pub fn checked(&self) -> &EventHandler<CheckedEventArgs> {
        &self.inner.checked
    }

    /// Get the underlying UI element.
    pub fn element(&self) -> &UIElement {
        &self.element
    }

    /// Get the HWND of this checkbox.
    pub fn hwnd(&self) -> HWND {
        self.element.hwnd()
    }

    /// Set the position of the checkbox.
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

    /// Set the size of the checkbox.
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

impl Default for CheckBox {
    fn default() -> Self {
        Self::new().expect("Failed to create checkbox")
    }
}

impl From<CheckBox> for UIElement {
    fn from(checkbox: CheckBox) -> Self {
        checkbox.element.clone()
    }
}






