//! Safe Rust wrappers for XAML Islands via C++ helper DLL.
//!
//! This module provides a safe API for creating and managing WinRT XAML controls
//! through a C++ bridge that handles the complex COM interop.

pub mod ffi;

use crate::error::{Error, Result};
use windows::Win32::Foundation::HWND;

fn to_wide_string(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(Some(0)).collect()
}

// ===== XAML Manager =====

/// Manages the Windows XAML framework.
/// Must be kept alive for the duration of XAML usage.
pub struct XamlManager {
    handle: ffi::XamlManagerHandle,
}

impl XamlManager {
    /// Initialize the XAML framework for the current thread.
    pub fn new() -> Result<Self> {
        let handle = unsafe { ffi::xaml_initialize() };
        if handle.0.is_null() {
            return Err(Error::initialization("Failed to initialize XAML Manager".to_string()));
        }
        Ok(XamlManager { handle })
    }
}

impl Drop for XamlManager {
    fn drop(&mut self) {
        unsafe {
            ffi::xaml_uninitialize(self.handle);
        }
    }
}

unsafe impl Send for XamlManager {}
unsafe impl Sync for XamlManager {}

// ===== XAML Source (DesktopWindowXamlSource) =====

/// Represents a DesktopWindowXamlSource for hosting XAML content.
pub struct XamlSource {
    handle: ffi::XamlSourceHandle,
    island_hwnd: Option<HWND>,
}

impl XamlSource {
    /// Create a new XAML source.
    pub fn new() -> Result<Self> {
        let handle = unsafe { ffi::xaml_source_create() };
        if handle.0.is_null() {
            return Err(Error::control_creation("Failed to create XAML Source".to_string()));
        }
        Ok(XamlSource {
            handle,
            island_hwnd: None,
        })
    }

    /// Attach the XAML source to a parent Win32 window.
    pub fn attach_to_window(&mut self, parent_hwnd: HWND) -> Result<HWND> {
        let island_hwnd = unsafe { ffi::xaml_source_attach_to_window(self.handle, parent_hwnd) };
        if island_hwnd.0.is_null() {
            return Err(Error::window_creation("Failed to attach XAML Source".to_string()));
        }
        self.island_hwnd = Some(island_hwnd);
        Ok(island_hwnd)
    }

    /// Set the content to a Button.
    pub fn set_content(&self, button: &XamlButton) -> Result<()> {
        let result = unsafe { ffi::xaml_source_set_content(self.handle, button.handle) };
        if result != 0 {
            return Err(Error::control_creation("Failed to set content".to_string()));
        }
        Ok(())
    }

    /// Set the content to any UIElement.
    pub fn set_content_element(&self, element: &XamlUIElement) -> Result<()> {
        let result = unsafe { ffi::xaml_source_set_content_generic(self.handle, element.handle) };
        if result != 0 {
            return Err(Error::control_creation("Failed to set content element".to_string()));
        }
        Ok(())
    }

    /// Get the XAML island HWND (if attached).
    pub fn island_hwnd(&self) -> Option<HWND> {
        self.island_hwnd
    }
}

impl Drop for XamlSource {
    fn drop(&mut self) {
        unsafe {
            ffi::xaml_source_destroy(self.handle);
        }
    }
}

unsafe impl Send for XamlSource {}
unsafe impl Sync for XamlSource {}

// ===== UIElement (Base Type) =====

/// Represents a generic WinRT UIElement.
pub struct XamlUIElement {
    handle: ffi::XamlUIElementHandle,
}

impl XamlUIElement {
    /// Create from a raw handle (internal use).
    pub(crate) fn from_handle(handle: ffi::XamlUIElementHandle) -> Self {
        XamlUIElement { handle }
    }

    /// Get the raw handle.
    pub fn handle(&self) -> ffi::XamlUIElementHandle {
        self.handle
    }
}

unsafe impl Send for XamlUIElement {}
unsafe impl Sync for XamlUIElement {}

// ===== Button =====

/// WinRT Button control.
pub struct XamlButton {
    handle: ffi::XamlButtonHandle,
}

impl XamlButton {
    /// Create a new WinRT Button.
    pub fn new() -> Result<Self> {
        let handle = unsafe { ffi::xaml_button_create() };
        if handle.0.is_null() {
            return Err(Error::control_creation("Failed to create Button".to_string()));
        }
        Ok(XamlButton { handle })
    }

    /// Set the button content (text).
    pub fn set_content(&self, content: &str) -> Result<()> {
        let wide_content = to_wide_string(content);
        let result = unsafe { ffi::xaml_button_set_content(self.handle, wide_content.as_ptr()) };
        if result != 0 {
            return Err(Error::control_creation("Failed to set button content".to_string()));
        }
        Ok(())
    }

    /// Set the button size (width and height).
    pub fn set_size(&self, width: f64, height: f64) -> Result<()> {
        let result = unsafe { ffi::xaml_button_set_size(self.handle, width, height) };
        if result != 0 {
            return Err(Error::control_creation("Failed to set button size".to_string()));
        }
        Ok(())
    }

    /// Convert to UIElement for use in layout containers.
    pub fn as_uielement(&self) -> XamlUIElement {
        let handle = unsafe { ffi::xaml_button_as_uielement(self.handle) };
        XamlUIElement::from_handle(handle)
    }
}

impl Drop for XamlButton {
    fn drop(&mut self) {
        unsafe {
            ffi::xaml_button_destroy(self.handle);
        }
    }
}

unsafe impl Send for XamlButton {}
unsafe impl Sync for XamlButton {}

// ===== TextBlock =====

/// WinRT TextBlock control.
pub struct XamlTextBlock {
    handle: ffi::XamlTextBlockHandle,
}

impl XamlTextBlock {
    /// Create a new WinRT TextBlock.
    pub fn new() -> Result<Self> {
        let handle = unsafe { ffi::xaml_textblock_create() };
        if handle.0.is_null() {
            return Err(Error::control_creation("Failed to create TextBlock".to_string()));
        }
        Ok(XamlTextBlock { handle })
    }

    /// Set the text content.
    pub fn set_text(&self, text: &str) -> Result<()> {
        let wide_text = to_wide_string(text);
        let result = unsafe { ffi::xaml_textblock_set_text(self.handle, wide_text.as_ptr()) };
        if result != 0 {
            return Err(Error::control_creation("Failed to set text".to_string()));
        }
        Ok(())
    }

    /// Set the font size.
    pub fn set_font_size(&self, size: f64) -> Result<()> {
        let result = unsafe { ffi::xaml_textblock_set_font_size(self.handle, size) };
        if result != 0 {
            return Err(Error::control_creation("Failed to set font size".to_string()));
        }
        Ok(())
    }

    /// Convert to UIElement for use in layout containers.
    pub fn as_uielement(&self) -> XamlUIElement {
        let handle = unsafe { ffi::xaml_textblock_as_uielement(self.handle) };
        XamlUIElement::from_handle(handle)
    }
}

impl Drop for XamlTextBlock {
    fn drop(&mut self) {
        unsafe {
            ffi::xaml_textblock_destroy(self.handle);
        }
    }
}

unsafe impl Send for XamlTextBlock {}
unsafe impl Sync for XamlTextBlock {}

// ===== TextBox =====

/// WinRT TextBox control.
pub struct XamlTextBox {
    handle: ffi::XamlTextBoxHandle,
}

impl XamlTextBox {
    /// Create a new WinRT TextBox.
    pub fn new() -> Result<Self> {
        let handle = unsafe { ffi::xaml_textbox_create() };
        if handle.0.is_null() {
            return Err(Error::control_creation("Failed to create TextBox".to_string()));
        }
        Ok(XamlTextBox { handle })
    }

    /// Set the text content.
    pub fn set_text(&self, text: &str) -> Result<()> {
        let wide_text = to_wide_string(text);
        let result = unsafe { ffi::xaml_textbox_set_text(self.handle, wide_text.as_ptr()) };
        if result != 0 {
            return Err(Error::control_creation("Failed to set text".to_string()));
        }
        Ok(())
    }

    /// Set the placeholder text.
    pub fn set_placeholder(&self, placeholder: &str) -> Result<()> {
        let wide_placeholder = to_wide_string(placeholder);
        let result = unsafe { ffi::xaml_textbox_set_placeholder(self.handle, wide_placeholder.as_ptr()) };
        if result != 0 {
            return Err(Error::control_creation("Failed to set placeholder".to_string()));
        }
        Ok(())
    }

    /// Set the text box size (width and height).
    pub fn set_size(&self, width: f64, height: f64) -> Result<()> {
        let result = unsafe { ffi::xaml_textbox_set_size(self.handle, width, height) };
        if result != 0 {
            return Err(Error::control_creation("Failed to set size".to_string()));
        }
        Ok(())
    }

    /// Convert to UIElement for use in layout containers.
    pub fn as_uielement(&self) -> XamlUIElement {
        let handle = unsafe { ffi::xaml_textbox_as_uielement(self.handle) };
        XamlUIElement::from_handle(handle)
    }
}

impl Drop for XamlTextBox {
    fn drop(&mut self) {
        unsafe {
            ffi::xaml_textbox_destroy(self.handle);
        }
    }
}

unsafe impl Send for XamlTextBox {}
unsafe impl Sync for XamlTextBox {}

// ===== StackPanel =====

/// WinRT StackPanel layout container.
pub struct XamlStackPanel {
    handle: ffi::XamlStackPanelHandle,
}

impl XamlStackPanel {
    /// Create a new WinRT StackPanel.
    pub fn new() -> Result<Self> {
        let handle = unsafe { ffi::xaml_stackpanel_create() };
        if handle.0.is_null() {
            return Err(Error::control_creation("Failed to create StackPanel".to_string()));
        }
        Ok(XamlStackPanel { handle })
    }

    /// Add a child element to the panel.
    pub fn add_child(&self, child: &XamlUIElement) -> Result<()> {
        let result = unsafe { ffi::xaml_stackpanel_add_child(self.handle, child.handle) };
        if result != 0 {
            return Err(Error::control_creation("Failed to add child".to_string()));
        }
        Ok(())
    }

    /// Set the orientation (true = vertical, false = horizontal).
    pub fn set_vertical(&self, vertical: bool) -> Result<()> {
        let result = unsafe { ffi::xaml_stackpanel_set_orientation(self.handle, if vertical { 1 } else { 0 }) };
        if result != 0 {
            return Err(Error::control_creation("Failed to set orientation".to_string()));
        }
        Ok(())
    }

    /// Set the spacing between children.
    pub fn set_spacing(&self, spacing: f64) -> Result<()> {
        let result = unsafe { ffi::xaml_stackpanel_set_spacing(self.handle, spacing) };
        if result != 0 {
            return Err(Error::control_creation("Failed to set spacing".to_string()));
        }
        Ok(())
    }

    /// Convert to UIElement for use in layout containers.
    pub fn as_uielement(&self) -> XamlUIElement {
        let handle = unsafe { ffi::xaml_stackpanel_as_uielement(self.handle) };
        XamlUIElement::from_handle(handle)
    }
}

impl Drop for XamlStackPanel {
    fn drop(&mut self) {
        unsafe {
            ffi::xaml_stackpanel_destroy(self.handle);
        }
    }
}

unsafe impl Send for XamlStackPanel {}
unsafe impl Sync for XamlStackPanel {}

// ===== Grid =====

/// WinRT Grid layout container.
pub struct XamlGrid {
    handle: ffi::XamlGridHandle,
}

impl XamlGrid {
    /// Create a new WinRT Grid.
    pub fn new() -> Result<Self> {
        let handle = unsafe { ffi::xaml_grid_create() };
        if handle.0.is_null() {
            return Err(Error::control_creation("Failed to create Grid".to_string()));
        }
        Ok(XamlGrid { handle })
    }

    /// Add a child element to the grid.
    pub fn add_child(&self, child: &XamlUIElement) -> Result<()> {
        let result = unsafe { ffi::xaml_grid_add_child(self.handle, child.handle) };
        if result != 0 {
            return Err(Error::control_creation("Failed to add child".to_string()));
        }
        Ok(())
    }

    /// Convert to UIElement for use in layout containers.
    pub fn as_uielement(&self) -> XamlUIElement {
        let handle = unsafe { ffi::xaml_grid_as_uielement(self.handle) };
        XamlUIElement::from_handle(handle)
    }
}

impl Drop for XamlGrid {
    fn drop(&mut self) {
        unsafe {
            ffi::xaml_grid_destroy(self.handle);
        }
    }
}

unsafe impl Send for XamlGrid {}
unsafe impl Sync for XamlGrid {}
