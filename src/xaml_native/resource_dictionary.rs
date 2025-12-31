//! Resource Dictionary for storing shared resources

use super::ffi::{self, XamlResourceDictionaryHandle, XamlUIElementHandle};
use crate::error::{Error, Result};
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

/// A WinRT ResourceDictionary for storing shared resources like colors, fonts, and styles.
pub struct XamlResourceDictionary {
    handle: XamlResourceDictionaryHandle,
}

impl XamlResourceDictionary {
    /// Create a new ResourceDictionary
    pub fn new() -> Result<Self> {
        let handle = unsafe { ffi::xaml_resource_dictionary_create() };
        if handle.0.is_null() {
            return Err(Error::control_creation("Failed to create ResourceDictionary"));
        }
        Ok(Self { handle })
    }

    /// Insert a color resource with a key
    ///
    /// # Arguments
    /// * `key` - The resource key
    /// * `color` - ARGB color value (e.g., 0xFF0078D4 for blue)
    pub fn insert_color(&self, key: impl AsRef<str>, color: u32) -> Result<()> {
        let key_wide: Vec<u16> = OsStr::new(key.as_ref())
            .encode_wide()
            .chain(Some(0))
            .collect();

        let result = unsafe {
            ffi::xaml_resource_dictionary_insert_color(
                self.handle,
                key_wide.as_ptr(),
                color,
            )
        };

        if result != 0 {
            return Err(Error::invalid_operation("Failed to insert color resource"));
        }

        Ok(())
    }

    /// Insert a numeric resource with a key
    pub fn insert_double(&self, key: impl AsRef<str>, value: f64) -> Result<()> {
        let key_wide: Vec<u16> = OsStr::new(key.as_ref())
            .encode_wide()
            .chain(Some(0))
            .collect();

        let result = unsafe {
            ffi::xaml_resource_dictionary_insert_double(
                self.handle,
                key_wide.as_ptr(),
                value,
            )
        };

        if result != 0 {
            return Err(Error::invalid_operation("Failed to insert double resource"));
        }

        Ok(())
    }

    /// Insert a string resource with a key
    pub fn insert_string(&self, key: impl AsRef<str>, value: impl AsRef<str>) -> Result<()> {
        let key_wide: Vec<u16> = OsStr::new(key.as_ref())
            .encode_wide()
            .chain(Some(0))
            .collect();

        let value_wide: Vec<u16> = OsStr::new(value.as_ref())
            .encode_wide()
            .chain(Some(0))
            .collect();

        let result = unsafe {
            ffi::xaml_resource_dictionary_insert_string(
                self.handle,
                key_wide.as_ptr(),
                value_wide.as_ptr(),
            )
        };

        if result != 0 {
            return Err(Error::invalid_operation("Failed to insert string resource"));
        }

        Ok(())
    }

    /// Check if a key exists in the dictionary
    pub fn has_key(&self, key: impl AsRef<str>) -> bool {
        let key_wide: Vec<u16> = OsStr::new(key.as_ref())
            .encode_wide()
            .chain(Some(0))
            .collect();

        unsafe { ffi::xaml_resource_dictionary_has_key(self.handle, key_wide.as_ptr()) != 0 }
    }

    /// Get a color resource by key
    pub fn get_color(&self, key: impl AsRef<str>) -> Option<u32> {
        if !self.has_key(key.as_ref()) {
            return None;
        }

        let key_wide: Vec<u16> = OsStr::new(key.as_ref())
            .encode_wide()
            .chain(Some(0))
            .collect();

        let color = unsafe {
            ffi::xaml_resource_dictionary_get_color(self.handle, key_wide.as_ptr())
        };

        Some(color)
    }

    /// Get a numeric resource by key
    pub fn get_double(&self, key: impl AsRef<str>) -> Option<f64> {
        if !self.has_key(key.as_ref()) {
            return None;
        }

        let key_wide: Vec<u16> = OsStr::new(key.as_ref())
            .encode_wide()
            .chain(Some(0))
            .collect();

        let value = unsafe {
            ffi::xaml_resource_dictionary_get_double(self.handle, key_wide.as_ptr())
        };

        Some(value)
    }

    /// Remove a resource by key
    pub fn remove(&self, key: impl AsRef<str>) -> Result<()> {
        let key_wide: Vec<u16> = OsStr::new(key.as_ref())
            .encode_wide()
            .chain(Some(0))
            .collect();

        let result = unsafe {
            ffi::xaml_resource_dictionary_remove(self.handle, key_wide.as_ptr())
        };

        if result != 0 {
            return Err(Error::invalid_operation("Failed to remove resource"));
        }

        Ok(())
    }

    /// Clear all resources
    pub fn clear(&self) {
        unsafe {
            ffi::xaml_resource_dictionary_clear(self.handle);
        }
    }

    /// Apply this resource dictionary to a UI element
    pub(crate) fn apply_to_element(&self, element: XamlUIElementHandle) -> Result<()> {
        let result = unsafe {
            ffi::xaml_uielement_set_resources(element, self.handle)
        };

        if result != 0 {
            return Err(Error::invalid_operation("Failed to apply resources to element"));
        }

        Ok(())
    }

    /// Get the raw handle
    pub(crate) fn handle(&self) -> XamlResourceDictionaryHandle {
        self.handle
    }
}

impl Default for XamlResourceDictionary {
    fn default() -> Self {
        Self::new().expect("Failed to create default ResourceDictionary")
    }
}

impl Drop for XamlResourceDictionary {
    fn drop(&mut self) {
        if !self.handle.0.is_null() {
            unsafe {
                ffi::xaml_resource_dictionary_destroy(self.handle);
            }
        }
    }
}

// ResourceDictionary is thread-safe
unsafe impl Send for XamlResourceDictionary {}
unsafe impl Sync for XamlResourceDictionary {}
