//! FFI bindings to the C++ XAML Islands helper DLL.

use std::ffi::c_void;
use windows::Win32::Foundation::HWND;
use crate::error::Result;

// Opaque handle types from C++
#[repr(C)]
#[derive(Clone, Copy)]
pub struct XamlManagerHandle(*mut c_void);
unsafe impl Send for XamlManagerHandle {}
unsafe impl Sync for XamlManagerHandle {}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct XamlSourceHandle(*mut c_void);
unsafe impl Send for XamlSourceHandle {}
unsafe impl Sync for XamlSourceHandle {}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct XamlButtonHandle(*mut c_void);
unsafe impl Send for XamlButtonHandle {}
unsafe impl Sync for XamlButtonHandle {}

// Raw FFI functions
#[link(name = "xaml_islands_helper", kind = "dylib")]
extern "C" {
    fn xaml_initialize() -> XamlManagerHandle;
    fn xaml_uninitialize(manager: XamlManagerHandle);

    fn xaml_source_create() -> XamlSourceHandle;
    fn xaml_source_destroy(source: XamlSourceHandle);
    fn xaml_source_attach_to_window(source: XamlSourceHandle, parent_hwnd: HWND) -> HWND;
    fn xaml_source_set_size(source: XamlSourceHandle, width: i32, height: i32) -> i32;
    fn xaml_source_set_content(source: XamlSourceHandle, button: XamlButtonHandle) -> i32;

    fn xaml_button_create() -> XamlButtonHandle;
    fn xaml_button_destroy(button: XamlButtonHandle);
    fn xaml_button_set_content(button: XamlButtonHandle, content: *const u16) -> i32;
    fn xaml_button_set_size(button: XamlButtonHandle, width: f64, height: f64) -> i32;

    fn xaml_get_last_error() -> *const u16;
}

// Safe Rust wrappers

/// XAML framework manager.
/// Must be kept alive for the duration of XAML usage.
pub struct XamlManager {
    handle: XamlManagerHandle,
}

impl XamlManager {
    /// Initialize XAML framework for the current thread.
    pub fn new() -> Result<Self> {
        let handle = unsafe { xaml_initialize() };
        if handle.0.is_null() {
            return Err(crate::error::Error::initialization(
                Self::get_last_error().unwrap_or_else(|| "Failed to initialize XAML".to_string())
            ));
        }
        Ok(XamlManager { handle })
    }

    fn get_last_error() -> Option<String> {
        unsafe {
            let ptr = xaml_get_last_error();
            if ptr.is_null() {
                return None;
            }

            // Find length
            let mut len = 0;
            while *ptr.add(len) != 0 {
                len += 1;
            }

            let slice = std::slice::from_raw_parts(ptr, len);
            Some(String::from_utf16_lossy(slice))
        }
    }
}

impl Drop for XamlManager {
    fn drop(&mut self) {
        unsafe {
            xaml_uninitialize(self.handle);
        }
    }
}

/// DesktopWindowXamlSource - hosts XAML content.
pub struct XamlSource {
    handle: XamlSourceHandle,
    island_hwnd: Option<HWND>,
}

impl XamlSource {
    /// Create a new XAML source.
    pub fn new() -> Result<Self> {
        let handle = unsafe { xaml_source_create() };
        if handle.0.is_null() {
            return Err(crate::error::Error::initialization(
                XamlManager::get_last_error().unwrap_or_else(|| "Failed to create XAML source".to_string())
            ));
        }
        Ok(XamlSource {
            handle,
            island_hwnd: None,
        })
    }

    /// Attach this XAML source to a parent window.
    pub fn attach_to_window(&mut self, parent_hwnd: HWND) -> Result<HWND> {
        let island_hwnd = unsafe {
            xaml_source_attach_to_window(self.handle, parent_hwnd)
        };

        if island_hwnd.0.is_null() {
            return Err(crate::error::Error::initialization(
                XamlManager::get_last_error().unwrap_or_else(|| "Failed to attach XAML to window".to_string())
            ));
        }

        self.island_hwnd = Some(island_hwnd);
        Ok(island_hwnd)
    }

    /// Get the island HWND.
    pub fn island_hwnd(&self) -> Option<HWND> {
        self.island_hwnd
    }

    /// Set the content to display.
    pub fn set_content(&self, button: &XamlButton) -> Result<()> {
        let result = unsafe {
            xaml_source_set_content(self.handle, button.handle)
        };

        if result != 0 {
            return Err(crate::error::Error::control_creation(
                XamlManager::get_last_error().unwrap_or_else(|| "Failed to set XAML content".to_string())
            ));
        }

        Ok(())
    }

    /// Set the size of the XAML island.
    pub fn set_size(&self, width: i32, height: i32) -> Result<()> {
        unsafe {
            xaml_source_set_size(self.handle, width, height);
        }
        Ok(())
    }
}

impl Drop for XamlSource {
    fn drop(&mut self) {
        unsafe {
            xaml_source_destroy(self.handle);
        }
    }
}

/// WinRT XAML Button.
pub struct XamlButton {
    handle: XamlButtonHandle,
}

impl XamlButton {
    /// Create a new WinRT button.
    pub fn new() -> Result<Self> {
        let handle = unsafe { xaml_button_create() };
        if handle.0.is_null() {
            return Err(crate::error::Error::control_creation(
                XamlManager::get_last_error().unwrap_or_else(|| "Failed to create XAML button".to_string())
            ));
        }
        Ok(XamlButton { handle })
    }

    /// Set the button content (text).
    pub fn set_content(&self, content: &str) -> Result<()> {
        let wide: Vec<u16> = content.encode_utf16().chain(std::iter::once(0)).collect();
        let result = unsafe {
            xaml_button_set_content(self.handle, wide.as_ptr())
        };

        if result != 0 {
            return Err(crate::error::Error::control_creation(
                XamlManager::get_last_error().unwrap_or_else(|| "Failed to set button content".to_string())
            ));
        }

        Ok(())
    }

    /// Set the button size.
    pub fn set_size(&self, width: f64, height: f64) -> Result<()> {
        let result = unsafe {
            xaml_button_set_size(self.handle, width, height)
        };

        if result != 0 {
            return Err(crate::error::Error::control_creation(
                XamlManager::get_last_error().unwrap_or_else(|| "Failed to set button size".to_string())
            ));
        }

        Ok(())
    }
}

impl Drop for XamlButton {
    fn drop(&mut self) {
        unsafe {
            xaml_button_destroy(self.handle);
        }
    }
}

// Convenience functions
pub fn initialize_xaml() -> Result<XamlManager> {
    XamlManager::new()
}

pub fn create_xaml_source() -> Result<XamlSource> {
    XamlSource::new()
}

pub fn create_xaml_button() -> Result<XamlButton> {
    XamlButton::new()
}

