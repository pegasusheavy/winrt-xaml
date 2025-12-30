//! ComboBox control implementation using Win32 COMBOBOX.

use crate::controls::UIElement;
use crate::error::{Error, Result};
use crate::events::{EventHandler, SelectionChangedEventArgs};
use parking_lot::RwLock;
use std::sync::Arc;
use windows::core::{w, PCWSTR};
use windows::Win32::{
    Foundation::*,
    System::LibraryLoader::GetModuleHandleW,
    UI::WindowsAndMessaging::*,
};

/// A combobox (dropdown) control.
#[derive(Clone)]
pub struct ComboBox {
    element: UIElement,
    inner: Arc<ComboBoxInner>,
}

struct ComboBoxInner {
    items: RwLock<Vec<String>>,
    selected_index: RwLock<i32>,
    selection_changed: EventHandler<SelectionChangedEventArgs>,
}

impl ComboBox {
    /// Create a new combobox.
    pub fn new() -> Result<Self> {
        let inner = Arc::new(ComboBoxInner {
            items: RwLock::new(Vec::new()),
            selected_index: RwLock::new(-1),
            selection_changed: EventHandler::new(),
        });

        Ok(ComboBox {
            element: UIElement::empty(),
            inner,
        })
    }

    /// Create the Win32 combobox control.
    pub(crate) fn create(&self, parent: HWND) -> Result<()> {
        unsafe {
            let hinstance = GetModuleHandleW(None)?;

            let hwnd = CreateWindowExW(
                WINDOW_EX_STYLE(0),
                w!("COMBOBOX"),
                PCWSTR::null(),
                WS_CHILD | WS_VISIBLE | WS_TABSTOP | WINDOW_STYLE(CBS_DROPDOWNLIST as u32 | CBS_HASSTRINGS as u32),
                0,
                0,
                200,
                200,
                parent,
                HMENU(std::ptr::null_mut()),
                HINSTANCE(hinstance.0),
                None,
            )?;

            if hwnd.0.is_null() {
                return Err(Error::control_creation("Failed to create combobox"));
            }

            self.element.set_hwnd(hwnd);
            self.element.set_width(200);
            self.element.set_height(25);

            Ok(())
        }
    }

    /// Get the items in the combobox.
    pub fn items(&self) -> Vec<String> {
        self.inner.items.read().clone()
    }

    /// Add an item to the combobox.
    pub fn add_item(&self, item: impl Into<String>) -> Result<()> {
        let item = item.into();
        self.inner.items.write().push(item.clone());

        let hwnd = self.element.hwnd();
        if !hwnd.0.is_null() {
            unsafe {
                let item_wide: Vec<u16> = item.encode_utf16().chain(Some(0)).collect();
                SendMessageW(hwnd, CB_ADDSTRING, WPARAM(0), LPARAM(item_wide.as_ptr() as isize));
            }
        }

        Ok(())
    }

    /// Clear all items from the combobox.
    pub fn clear_items(&self) {
        self.inner.items.write().clear();

        let hwnd = self.element.hwnd();
        if !hwnd.0.is_null() {
            unsafe {
                SendMessageW(hwnd, CB_RESETCONTENT, WPARAM(0), LPARAM(0));
            }
        }
    }

    /// Get the selected index.
    pub fn selected_index(&self) -> i32 {
        *self.inner.selected_index.read()
    }

    /// Set the selected index.
    pub fn set_selected_index(&self, index: i32) {
        *self.inner.selected_index.write() = index;

        let hwnd = self.element.hwnd();
        if !hwnd.0.is_null() {
            unsafe {
                SendMessageW(hwnd, CB_SETCURSEL, WPARAM(index as usize), LPARAM(0));
            }
        }
    }

    /// Set the selected index (fluent API).
    pub fn with_selected_index(self, index: i32) -> Self {
        self.set_selected_index(index);
        self
    }

    /// Subscribe to the selection changed event.
    pub fn selection_changed(&self) -> &EventHandler<SelectionChangedEventArgs> {
        &self.inner.selection_changed
    }

    /// Get the underlying UI element.
    pub fn element(&self) -> &UIElement {
        &self.element
    }

    /// Get the HWND of this combobox.
    pub fn hwnd(&self) -> HWND {
        self.element.hwnd()
    }

    /// Set the position of the combobox.
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

    /// Set the size of the combobox.
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

impl Default for ComboBox {
    fn default() -> Self {
        Self::new().expect("Failed to create combobox")
    }
}

impl From<ComboBox> for UIElement {
    fn from(combobox: ComboBox) -> Self {
        combobox.element.clone()
    }
}
