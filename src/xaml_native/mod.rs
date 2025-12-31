//! Safe Rust wrappers for XAML Islands via C++ helper DLL.
//!
//! This module provides a safe API for creating and managing WinRT XAML controls
//! through a C++ bridge that handles the complex COM interop.

pub mod ffi;
mod resource_dictionary;
mod animation;

pub use resource_dictionary::*;
pub use animation::*;

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

    /// Set the Grid.Row attached property.
    pub fn set_grid_row(&self, row: i32) -> Result<()> {
        let result = unsafe { ffi::xaml_grid_set_child_row(self.handle, row) };
        if result != 0 {
            return Err(Error::invalid_operation("Failed to set grid row".to_string()));
        }
        Ok(())
    }

    /// Set the Grid.Column attached property.
    pub fn set_grid_column(&self, column: i32) -> Result<()> {
        let result = unsafe { ffi::xaml_grid_set_child_column(self.handle, column) };
        if result != 0 {
            return Err(Error::invalid_operation("Failed to set grid column".to_string()));
        }
        Ok(())
    }

    /// Set the Grid.RowSpan attached property.
    pub fn set_grid_row_span(&self, row_span: i32) -> Result<()> {
        let result = unsafe { ffi::xaml_grid_set_child_row_span(self.handle, row_span) };
        if result != 0 {
            return Err(Error::invalid_operation("Failed to set grid row span".to_string()));
        }
        Ok(())
    }

    /// Set the Grid.ColumnSpan attached property.
    pub fn set_grid_column_span(&self, column_span: i32) -> Result<()> {
        let result = unsafe { ffi::xaml_grid_set_child_column_span(self.handle, column_span) };
        if result != 0 {
            return Err(Error::invalid_operation("Failed to set grid column span".to_string()));
        }
        Ok(())
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

    /// Register a click event handler.
    /// The callback will be called when the button is clicked.
    /// Note: The callback must be 'static and will be leaked to ensure it remains valid.
    pub fn on_click<F>(&self, callback: F) -> Result<()>
    where
        F: Fn() + Send + 'static,
    {
        // Box the callback and convert to a raw pointer
        let boxed_callback = Box::new(callback);
        let user_data = Box::into_raw(boxed_callback) as *mut std::ffi::c_void;

        // Define the C callback that will be called by C++
        extern "C" fn trampoline<F>(user_data: *mut std::ffi::c_void)
        where
            F: Fn(),
        {
            unsafe {
                let callback = &*(user_data as *const F);
                callback();
            }
        }

        // Register the click handler
        let result = unsafe {
            ffi::xaml_button_register_click(
                self.handle,
                trampoline::<F>,
                user_data,
            )
        };

        if result != 0 {
            // If registration failed, clean up the leaked box
            unsafe {
                let _ = Box::from_raw(user_data as *mut F);
            }
            return Err(Error::control_creation("Failed to register click handler".to_string()));
        }

        // Note: The callback is intentionally leaked and will remain valid for the button's lifetime
        // In a production app, you'd want to store the pointer and clean it up in Drop
        Ok(())
    }

    /// Set the button background color (ARGB format: 0xAARRGGBB).
    pub fn set_background(&self, color: u32) -> Result<()> {
        let result = unsafe { ffi::xaml_button_set_background(self.handle, color) };
        if result != 0 {
            return Err(Error::control_creation("Failed to set background".to_string()));
        }
        Ok(())
    }

    /// Set the button foreground (text) color (ARGB format: 0xAARRGGBB).
    pub fn set_foreground(&self, color: u32) -> Result<()> {
        let result = unsafe { ffi::xaml_button_set_foreground(self.handle, color) };
        if result != 0 {
            return Err(Error::control_creation("Failed to set foreground".to_string()));
        }
        Ok(())
    }

    /// Set the button corner radius for rounded corners.
    pub fn set_corner_radius(&self, radius: f64) -> Result<()> {
        let result = unsafe { ffi::xaml_button_set_corner_radius(self.handle, radius) };
        if result != 0 {
            return Err(Error::control_creation("Failed to set corner radius".to_string()));
        }
        Ok(())
    }

    /// Set the button padding (inner spacing).
    pub fn set_padding(&self, left: f64, top: f64, right: f64, bottom: f64) -> Result<()> {
        let result = unsafe { ffi::xaml_button_set_padding(self.handle, left, top, right, bottom) };
        if result != 0 {
            return Err(Error::control_creation("Failed to set padding".to_string()));
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

    /// Set the text foreground color (ARGB format: 0xAARRGGBB).
    pub fn set_foreground(&self, color: u32) -> Result<()> {
        let result = unsafe { ffi::xaml_textblock_set_foreground(self.handle, color) };
        if result != 0 {
            return Err(Error::control_creation("Failed to set foreground".to_string()));
        }
        Ok(())
    }

    /// Set the font weight (400=Normal, 600=SemiBold, 700=Bold).
    pub fn set_font_weight(&self, weight: i32) -> Result<()> {
        let result = unsafe { ffi::xaml_textblock_set_font_weight(self.handle, weight) };
        if result != 0 {
            return Err(Error::control_creation("Failed to set font weight".to_string()));
        }
        Ok(())
    }

    /// Set the margin (outer spacing).
    pub fn set_margin(&self, left: f64, top: f64, right: f64, bottom: f64) -> Result<()> {
        let result = unsafe { ffi::xaml_textblock_set_margin(self.handle, left, top, right, bottom) };
        if result != 0 {
            return Err(Error::control_creation("Failed to set margin".to_string()));
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

    /// Get the current text content.
    pub fn get_text(&self) -> Result<String> {
        const BUFFER_SIZE: i32 = 1024;
        let mut buffer: Vec<u16> = vec![0; BUFFER_SIZE as usize];

        let result = unsafe {
            ffi::xaml_textbox_get_text(self.handle, buffer.as_mut_ptr(), BUFFER_SIZE)
        };

        if result < 0 {
            return Err(Error::control_creation("Failed to get text".to_string()));
        }

        // Convert the wide string to Rust String
        let len = result as usize;
        let text = String::from_utf16_lossy(&buffer[..len]);
        Ok(text)
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

    /// Set the textbox background color (ARGB format: 0xAARRGGBB).
    pub fn set_background(&self, color: u32) -> Result<()> {
        let result = unsafe { ffi::xaml_textbox_set_background(self.handle, color) };
        if result != 0 {
            return Err(Error::control_creation("Failed to set background".to_string()));
        }
        Ok(())
    }

    /// Set the textbox foreground (text) color (ARGB format: 0xAARRGGBB).
    pub fn set_foreground(&self, color: u32) -> Result<()> {
        let result = unsafe { ffi::xaml_textbox_set_foreground(self.handle, color) };
        if result != 0 {
            return Err(Error::control_creation("Failed to set foreground".to_string()));
        }
        Ok(())
    }

    /// Set the textbox corner radius for rounded corners.
    pub fn set_corner_radius(&self, radius: f64) -> Result<()> {
        let result = unsafe { ffi::xaml_textbox_set_corner_radius(self.handle, radius) };
        if result != 0 {
            return Err(Error::control_creation("Failed to set corner radius".to_string()));
        }
        Ok(())
    }

    /// Set the textbox padding (inner spacing).
    pub fn set_padding(&self, left: f64, top: f64, right: f64, bottom: f64) -> Result<()> {
        let result = unsafe { ffi::xaml_textbox_set_padding(self.handle, left, top, right, bottom) };
        if result != 0 {
            return Err(Error::control_creation("Failed to set padding".to_string()));
        }
        Ok(())
    }

    /// Convert to UIElement for use in layout containers.
    pub fn as_uielement(&self) -> XamlUIElement {
        let handle = unsafe { ffi::xaml_textbox_as_uielement(self.handle) };
        XamlUIElement::from_handle(handle)
    }

    /// Register a callback for when the text changes.
    pub fn on_text_changed<F>(&self, callback: F) -> Result<()>
    where
        F: Fn() + Send + Sync + 'static,
    {
        let boxed = Box::new(callback);
        let ptr = Box::into_raw(boxed);

        extern "C" fn trampoline<F>(user_data: *mut std::ffi::c_void)
        where
            F: Fn() + Send + Sync + 'static,
        {
            unsafe {
                let callback = &*(user_data as *const F);
                callback();
            }
        }

        unsafe {
            ffi::xaml_textbox_on_text_changed(
                self.handle,
                std::mem::transmute(trampoline::<F> as *const ())
            );
        }

        std::mem::forget(ptr);
        Ok(())
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

    /// Set the orientation using the Orientation enum.
    pub fn set_orientation(&self, orientation: crate::layout::Orientation) -> Result<()> {
        use crate::layout::Orientation;
        self.set_vertical(orientation == Orientation::Vertical)
    }

    /// Set the spacing between children.
    pub fn set_spacing(&self, spacing: f64) -> Result<()> {
        let result = unsafe { ffi::xaml_stackpanel_set_spacing(self.handle, spacing) };
        if result != 0 {
            return Err(Error::control_creation("Failed to set spacing".to_string()));
        }
        Ok(())
    }

    /// Set the panel background color (ARGB format: 0xAARRGGBB).
    pub fn set_background(&self, color: u32) -> Result<()> {
        let result = unsafe { ffi::xaml_stackpanel_set_background(self.handle, color) };
        if result != 0 {
            return Err(Error::control_creation("Failed to set background".to_string()));
        }
        Ok(())
    }

    /// Set the panel padding (inner spacing).
    pub fn set_padding(&self, left: f64, top: f64, right: f64, bottom: f64) -> Result<()> {
        let result = unsafe { ffi::xaml_stackpanel_set_padding(self.handle, left, top, right, bottom) };
        if result != 0 {
            return Err(Error::control_creation("Failed to set padding".to_string()));
        }
        Ok(())
    }

    /// Set the panel corner radius for rounded corners.
    pub fn set_corner_radius(&self, radius: f64) -> Result<()> {
        let result = unsafe { ffi::xaml_stackpanel_set_corner_radius(self.handle, radius) };
        if result != 0 {
            return Err(Error::control_creation("Failed to set corner radius".to_string()));
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

    /// Set the grid background color (ARGB format: 0xAARRGGBB).
    pub fn set_background(&self, color: u32) -> Result<()> {
        let result = unsafe { ffi::xaml_grid_set_background(self.handle, color) };
        if result != 0 {
            return Err(Error::control_creation("Failed to set background".to_string()));
        }
        Ok(())
    }

    /// Set the grid padding (inner spacing).
    pub fn set_padding(&self, left: f64, top: f64, right: f64, bottom: f64) -> Result<()> {
        let result = unsafe { ffi::xaml_grid_set_padding(self.handle, left, top, right, bottom) };
        if result != 0 {
            return Err(Error::control_creation("Failed to set padding".to_string()));
        }
        Ok(())
    }

    /// Set the grid corner radius for rounded corners.
    pub fn set_corner_radius(&self, radius: f64) -> Result<()> {
        let result = unsafe { ffi::xaml_grid_set_corner_radius(self.handle, radius) };
        if result != 0 {
            return Err(Error::control_creation("Failed to set corner radius".to_string()));
        }
        Ok(())
    }

    /// Add a row definition with pixel height.
    pub fn add_row_pixels(&self, height: f64) -> Result<()> {
        let result = unsafe { ffi::xaml_grid_add_row_definition(self.handle, height, 0, 0) };
        if result != 0 {
            return Err(Error::control_creation("Failed to add row definition".to_string()));
        }
        Ok(())
    }

    /// Add a row definition with Auto sizing.
    pub fn add_row_auto(&self) -> Result<()> {
        let result = unsafe { ffi::xaml_grid_add_row_definition(self.handle, 0.0, 1, 0) };
        if result != 0 {
            return Err(Error::control_creation("Failed to add row definition".to_string()));
        }
        Ok(())
    }

    /// Add a row definition with Star sizing (proportional).
    pub fn add_row_star(&self, proportion: f64) -> Result<()> {
        let result = unsafe { ffi::xaml_grid_add_row_definition(self.handle, proportion, 0, 1) };
        if result != 0 {
            return Err(Error::control_creation("Failed to add row definition".to_string()));
        }
        Ok(())
    }

    /// Add a column definition with pixel width.
    pub fn add_column_pixels(&self, width: f64) -> Result<()> {
        let result = unsafe { ffi::xaml_grid_add_column_definition(self.handle, width, 0, 0) };
        if result != 0 {
            return Err(Error::control_creation("Failed to add column definition".to_string()));
        }
        Ok(())
    }

    /// Add a column definition with Auto sizing.
    pub fn add_column_auto(&self) -> Result<()> {
        let result = unsafe { ffi::xaml_grid_add_column_definition(self.handle, 0.0, 1, 0) };
        if result != 0 {
            return Err(Error::control_creation("Failed to add column definition".to_string()));
        }
        Ok(())
    }

    /// Add a column definition with Star sizing (proportional).
    pub fn add_column_star(&self, proportion: f64) -> Result<()> {
        let result = unsafe { ffi::xaml_grid_add_column_definition(self.handle, proportion, 0, 1) };
        if result != 0 {
            return Err(Error::control_creation("Failed to add column definition".to_string()));
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

// ===== ScrollViewer =====

/// Represents a WinRT ScrollViewer control that provides scrollable content.
pub struct XamlScrollViewer {
    handle: ffi::XamlScrollViewerHandle,
}

/// Scroll mode for ScrollViewer
pub enum ScrollMode {
    Disabled = 0,
    Enabled = 1,
    Auto = 2,
}

/// Scroll bar visibility for ScrollViewer
pub enum ScrollBarVisibility {
    Disabled = 0,
    Auto = 1,
    Hidden = 2,
    Visible = 3,
}

impl XamlScrollViewer {
    /// Create a new WinRT ScrollViewer.
    pub fn new() -> Result<Self> {
        let handle = unsafe { ffi::xaml_scrollviewer_create() };
        if handle.0.is_null() {
            return Err(Error::control_creation("Failed to create ScrollViewer".to_string()));
        }
        Ok(XamlScrollViewer { handle })
    }

    /// Set the content of the ScrollViewer.
    pub fn set_content(&self, content: &XamlUIElement) -> Result<()> {
        let result = unsafe { ffi::xaml_scrollviewer_set_content(self.handle, content.handle) };
        if result != 0 {
            return Err(Error::control_creation("Failed to set content".to_string()));
        }
        Ok(())
    }

    /// Set the horizontal scroll mode.
    pub fn set_horizontal_scroll_mode(&self, mode: ScrollMode) -> Result<()> {
        let result = unsafe { ffi::xaml_scrollviewer_set_horizontal_scroll_mode(self.handle, mode as i32) };
        if result != 0 {
            return Err(Error::control_creation("Failed to set horizontal scroll mode".to_string()));
        }
        Ok(())
    }

    /// Set the vertical scroll mode.
    pub fn set_vertical_scroll_mode(&self, mode: ScrollMode) -> Result<()> {
        let result = unsafe { ffi::xaml_scrollviewer_set_vertical_scroll_mode(self.handle, mode as i32) };
        if result != 0 {
            return Err(Error::control_creation("Failed to set vertical scroll mode".to_string()));
        }
        Ok(())
    }

    /// Set the horizontal scroll bar visibility.
    pub fn set_horizontal_scrollbar_visibility(&self, visibility: ScrollBarVisibility) -> Result<()> {
        let result = unsafe { ffi::xaml_scrollviewer_set_horizontal_scroll_bar_visibility(self.handle, visibility as i32) };
        if result != 0 {
            return Err(Error::control_creation("Failed to set horizontal scrollbar visibility".to_string()));
        }
        Ok(())
    }

    /// Set the vertical scroll bar visibility.
    pub fn set_vertical_scrollbar_visibility(&self, visibility: ScrollBarVisibility) -> Result<()> {
        let result = unsafe { ffi::xaml_scrollviewer_set_vertical_scroll_bar_visibility(self.handle, visibility as i32) };
        if result != 0 {
            return Err(Error::control_creation("Failed to set vertical scrollbar visibility".to_string()));
        }
        Ok(())
    }

    /// Convert to a UIElement for use as content in other containers.
    pub fn as_uielement(&self) -> XamlUIElement {
        let handle = unsafe { ffi::xaml_scrollviewer_as_uielement(self.handle) };
        XamlUIElement { handle }
    }
}

impl Drop for XamlScrollViewer {
    fn drop(&mut self) {
        unsafe {
            ffi::xaml_scrollviewer_destroy(self.handle);
        }
    }
}

unsafe impl Send for XamlScrollViewer {}
unsafe impl Sync for XamlScrollViewer {}

// ===== CheckBox =====

/// A WinRT CheckBox control.
pub struct XamlCheckBox {
    handle: ffi::XamlCheckBoxHandle,
}

impl XamlCheckBox {
    /// Create a new CheckBox.
    pub fn new() -> Result<Self> {
        let handle = unsafe { ffi::xaml_checkbox_create() };
        if handle.0.is_null() {
            return Err(Error::control_creation("Failed to create CheckBox".to_string()));
        }
        Ok(XamlCheckBox { handle })
    }

    /// Set the content/label of the checkbox.
    pub fn set_content(&self, content: &str) -> Result<()> {
        let wide_content = to_wide_string(content);
        let result = unsafe { ffi::xaml_checkbox_set_content(self.handle, wide_content.as_ptr()) };
        if result != 0 {
            return Err(Error::control_creation("Failed to set checkbox content".to_string()));
        }
        Ok(())
    }

    /// Set whether the checkbox is checked.
    pub fn set_is_checked(&self, is_checked: bool) -> Result<()> {
        let result = unsafe { ffi::xaml_checkbox_set_is_checked(self.handle, is_checked) };
        if result != 0 {
            return Err(Error::control_creation("Failed to set checkbox state".to_string()));
        }
        Ok(())
    }

    /// Get whether the checkbox is checked.
    pub fn is_checked(&self) -> bool {
        unsafe { ffi::xaml_checkbox_get_is_checked(self.handle) }
    }

    /// Convert to a UIElement for use as content in other containers.
    pub fn as_uielement(&self) -> XamlUIElement {
        let handle = unsafe { ffi::xaml_checkbox_as_uielement(self.handle) };
        XamlUIElement { handle }
    }
}

unsafe impl Send for XamlCheckBox {}
unsafe impl Sync for XamlCheckBox {}

// ===== ComboBox =====

/// A WinRT ComboBox control.
pub struct XamlComboBox {
    handle: ffi::XamlComboBoxHandle,
}

impl XamlComboBox {
    /// Create a new ComboBox.
    pub fn new() -> Result<Self> {
        let handle = unsafe { ffi::xaml_combobox_create() };
        if handle.0.is_null() {
            return Err(Error::control_creation("Failed to create ComboBox".to_string()));
        }
        Ok(XamlComboBox { handle })
    }

    /// Add an item to the combobox.
    pub fn add_item(&self, item: &str) -> Result<()> {
        let wide_item = to_wide_string(item);
        let result = unsafe { ffi::xaml_combobox_add_item(self.handle, wide_item.as_ptr()) };
        if result != 0 {
            return Err(Error::control_creation("Failed to add combobox item".to_string()));
        }
        Ok(())
    }

    /// Set the selected index.
    pub fn set_selected_index(&self, index: i32) -> Result<()> {
        let result = unsafe { ffi::xaml_combobox_set_selected_index(self.handle, index) };
        if result != 0 {
            return Err(Error::control_creation("Failed to set selected index".to_string()));
        }
        Ok(())
    }

    /// Get the selected index.
    pub fn get_selected_index(&self) -> i32 {
        unsafe { ffi::xaml_combobox_get_selected_index(self.handle) }
    }

    /// Convert to a UIElement for use as content in other containers.
    pub fn as_uielement(&self) -> XamlUIElement {
        let handle = unsafe { ffi::xaml_combobox_as_uielement(self.handle) };
        XamlUIElement { handle }
    }
}

unsafe impl Send for XamlComboBox {}
unsafe impl Sync for XamlComboBox {}

// ===== Slider =====

/// A WinRT Slider control.
pub struct XamlSlider {
    handle: ffi::XamlSliderHandle,
}

impl XamlSlider {
    /// Create a new Slider.
    pub fn new() -> Result<Self> {
        let handle = unsafe { ffi::xaml_slider_create() };
        if handle.0.is_null() {
            return Err(Error::control_creation("Failed to create Slider".to_string()));
        }
        Ok(XamlSlider { handle })
    }

    /// Set the minimum value.
    pub fn set_minimum(&self, minimum: f64) -> Result<()> {
        let result = unsafe { ffi::xaml_slider_set_minimum(self.handle, minimum) };
        if result != 0 {
            return Err(Error::control_creation("Failed to set slider minimum".to_string()));
        }
        Ok(())
    }

    /// Set the maximum value.
    pub fn set_maximum(&self, maximum: f64) -> Result<()> {
        let result = unsafe { ffi::xaml_slider_set_maximum(self.handle, maximum) };
        if result != 0 {
            return Err(Error::control_creation("Failed to set slider maximum".to_string()));
        }
        Ok(())
    }

    /// Set the current value.
    pub fn set_value(&self, value: f64) -> Result<()> {
        let result = unsafe { ffi::xaml_slider_set_value(self.handle, value) };
        if result != 0 {
            return Err(Error::control_creation("Failed to set slider value".to_string()));
        }
        Ok(())
    }

    /// Get the current value.
    pub fn get_value(&self) -> f64 {
        unsafe { ffi::xaml_slider_get_value(self.handle) }
    }

    /// Convert to a UIElement for use as content in other containers.
    pub fn as_uielement(&self) -> XamlUIElement {
        let handle = unsafe { ffi::xaml_slider_as_uielement(self.handle) };
        XamlUIElement { handle }
    }
}

unsafe impl Send for XamlSlider {}
unsafe impl Sync for XamlSlider {}

// ===== ProgressBar =====

/// A WinRT ProgressBar control.
pub struct XamlProgressBar {
    handle: ffi::XamlProgressBarHandle,
}

impl XamlProgressBar {
    /// Create a new ProgressBar.
    pub fn new() -> Result<Self> {
        let handle = unsafe { ffi::xaml_progressbar_create() };
        if handle.0.is_null() {
            return Err(Error::control_creation("Failed to create ProgressBar".to_string()));
        }
        Ok(XamlProgressBar { handle })
    }

    /// Set the minimum value.
    pub fn set_minimum(&self, minimum: f64) -> Result<()> {
        let result = unsafe { ffi::xaml_progressbar_set_minimum(self.handle, minimum) };
        if result != 0 {
            return Err(Error::control_creation("Failed to set progressbar minimum".to_string()));
        }
        Ok(())
    }

    /// Set the maximum value.
    pub fn set_maximum(&self, maximum: f64) -> Result<()> {
        let result = unsafe { ffi::xaml_progressbar_set_maximum(self.handle, maximum) };
        if result != 0 {
            return Err(Error::control_creation("Failed to set progressbar maximum".to_string()));
        }
        Ok(())
    }

    /// Set the current value.
    pub fn set_value(&self, value: f64) -> Result<()> {
        let result = unsafe { ffi::xaml_progressbar_set_value(self.handle, value) };
        if result != 0 {
            return Err(Error::control_creation("Failed to set progressbar value".to_string()));
        }
        Ok(())
    }

    /// Set whether the progress bar is indeterminate (animated).
    pub fn set_is_indeterminate(&self, is_indeterminate: bool) -> Result<()> {
        let result = unsafe { ffi::xaml_progressbar_set_is_indeterminate(self.handle, is_indeterminate) };
        if result != 0 {
            return Err(Error::control_creation("Failed to set progressbar indeterminate state".to_string()));
        }
        Ok(())
    }

    /// Convert to a UIElement for use as content in other containers.
    pub fn as_uielement(&self) -> XamlUIElement {
        let handle = unsafe { ffi::xaml_progressbar_as_uielement(self.handle) };
        XamlUIElement { handle }
    }
}

unsafe impl Send for XamlProgressBar {}
unsafe impl Sync for XamlProgressBar {}

// ===== RadioButton =====

/// A WinRT RadioButton control for mutually exclusive selections.
pub struct XamlRadioButton {
    handle: ffi::XamlRadioButtonHandle,
}

impl XamlRadioButton {
    /// Create a new RadioButton.
    pub fn new() -> Result<Self> {
        let handle = unsafe { ffi::xaml_radiobutton_create() };
        if handle.0.is_null() {
            return Err(Error::control_creation("Failed to create RadioButton".to_string()));
        }
        Ok(XamlRadioButton { handle })
    }

    /// Set the content (label) of the radio button.
    pub fn set_content(&self, content: impl AsRef<str>) -> Result<()> {
        let content_wide = to_wide_string(content.as_ref());
        let result = unsafe { ffi::xaml_radiobutton_set_content(self.handle, content_wide.as_ptr()) };
        if result != 0 {
            return Err(Error::control_creation("Failed to set radiobutton content".to_string()));
        }
        Ok(())
    }

    /// Set whether the radio button is checked.
    pub fn set_is_checked(&self, is_checked: bool) -> Result<()> {
        let result = unsafe { ffi::xaml_radiobutton_set_is_checked(self.handle, if is_checked { 1 } else { 0 }) };
        if result != 0 {
            return Err(Error::control_creation("Failed to set radiobutton checked state".to_string()));
        }
        Ok(())
    }

    /// Get whether the radio button is checked.
    pub fn is_checked(&self) -> bool {
        let result = unsafe { ffi::xaml_radiobutton_get_is_checked(self.handle) };
        result != 0
    }

    /// Set the group name for mutual exclusivity.
    /// Radio buttons with the same group name are mutually exclusive.
    pub fn set_group_name(&self, group_name: impl AsRef<str>) -> Result<()> {
        let group_wide = to_wide_string(group_name.as_ref());
        let result = unsafe { ffi::xaml_radiobutton_set_group_name(self.handle, group_wide.as_ptr()) };
        if result != 0 {
            return Err(Error::control_creation("Failed to set radiobutton group name".to_string()));
        }
        Ok(())
    }

    /// Register a callback for when the radio button is checked.
    pub fn on_checked<F>(&self, callback: F) -> Result<()>
    where
        F: Fn() + Send + Sync + 'static,
    {
        let boxed = Box::new(callback);
        let ptr = Box::into_raw(boxed);

        extern "C" fn trampoline<F>(user_data: *mut std::ffi::c_void)
        where
            F: Fn() + Send + Sync + 'static,
        {
            unsafe {
                let callback = &*(user_data as *const F);
                callback();
            }
        }

        unsafe {
            ffi::xaml_radiobutton_on_checked(
                self.handle,
                std::mem::transmute(trampoline::<F> as *const ())
            );
        }

        // Note: This leaks the callback. In production, you'd want proper cleanup.
        std::mem::forget(ptr);
        Ok(())
    }

    /// Register a callback for when the radio button is unchecked.
    pub fn on_unchecked<F>(&self, callback: F) -> Result<()>
    where
        F: Fn() + Send + Sync + 'static,
    {
        let boxed = Box::new(callback);
        let ptr = Box::into_raw(boxed);

        extern "C" fn trampoline<F>(user_data: *mut std::ffi::c_void)
        where
            F: Fn() + Send + Sync + 'static,
        {
            unsafe {
                let callback = &*(user_data as *const F);
                callback();
            }
        }

        unsafe {
            ffi::xaml_radiobutton_on_unchecked(
                self.handle,
                std::mem::transmute(trampoline::<F> as *const ())
            );
        }

        std::mem::forget(ptr);
        Ok(())
    }

    /// Convert to a UIElement for use as content in other containers.
    pub fn as_uielement(&self) -> XamlUIElement {
        let handle = unsafe { ffi::xaml_radiobutton_as_uielement(self.handle) };
        XamlUIElement { handle }
    }
}

impl Drop for XamlRadioButton {
    fn drop(&mut self) {
        unsafe {
            ffi::xaml_radiobutton_destroy(self.handle);
        }
    }
}

unsafe impl Send for XamlRadioButton {}
unsafe impl Sync for XamlRadioButton {}

// ===== Image =====

/// Stretch modes for Image control.
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageStretch {
    /// No stretching.
    None = 0,
    /// Fill the available space.
    Fill = 1,
    /// Uniform scaling to fit.
    Uniform = 2,
    /// Uniform scaling to fill.
    UniformToFill = 3,
}

/// A WinRT Image control for displaying images.
pub struct XamlImage {
    handle: ffi::XamlImageHandle,
}

impl XamlImage {
    /// Create a new Image.
    pub fn new() -> Result<Self> {
        let handle = unsafe { ffi::xaml_image_create() };
        if handle.0.is_null() {
            return Err(Error::control_creation("Failed to create Image".to_string()));
        }
        Ok(XamlImage { handle })
    }

    /// Set the image source from a URI.
    ///
    /// # Examples
    /// ```no_run
    /// # use winrt_xaml::xaml_native::XamlImage;
    /// let image = XamlImage::new()?;
    /// image.set_source("ms-appx:///Assets/logo.png")?;
    /// image.set_source("https://example.com/image.jpg")?;
    /// # Ok::<(), winrt_xaml::Error>(())
    /// ```
    pub fn set_source(&self, uri: impl AsRef<str>) -> Result<()> {
        let uri_wide = to_wide_string(uri.as_ref());
        let result = unsafe { ffi::xaml_image_set_source(self.handle, uri_wide.as_ptr()) };
        if result != 0 {
            return Err(Error::control_creation("Failed to set image source".to_string()));
        }
        Ok(())
    }

    /// Set the stretch mode for the image.
    pub fn set_stretch(&self, stretch: ImageStretch) -> Result<()> {
        let result = unsafe { ffi::xaml_image_set_stretch(self.handle, stretch as i32) };
        if result != 0 {
            return Err(Error::control_creation("Failed to set image stretch".to_string()));
        }
        Ok(())
    }

    /// Set the size of the image.
    pub fn set_size(&self, width: f64, height: f64) -> Result<()> {
        let result = unsafe { ffi::xaml_image_set_size(self.handle, width, height) };
        if result != 0 {
            return Err(Error::control_creation("Failed to set image size".to_string()));
        }
        Ok(())
    }

    /// Convert to a UIElement for use as content in other containers.
    pub fn as_uielement(&self) -> XamlUIElement {
        let handle = unsafe { ffi::xaml_image_as_uielement(self.handle) };
        XamlUIElement { handle }
    }
}

impl Drop for XamlImage {
    fn drop(&mut self) {
        unsafe {
            ffi::xaml_image_destroy(self.handle);
        }
    }
}

unsafe impl Send for XamlImage {}
unsafe impl Sync for XamlImage {}

