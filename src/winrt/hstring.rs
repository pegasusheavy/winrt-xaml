//! WinRT HSTRING implementation.
//!
//! HSTRING is the WinRT string type, which is an immutable, reference-counted
//! UTF-16 string.

use windows::core::HSTRING as WindowsHString;
use std::fmt;

/// A WinRT string (HSTRING wrapper).
#[derive(Clone)]
pub struct HString {
    inner: WindowsHString,
}

impl HString {
    /// Create a new HSTRING from a Rust string.
    pub fn from(s: impl AsRef<str>) -> Self {
        HString {
            inner: WindowsHString::from(s.as_ref()),
        }
    }

    /// Create an empty HSTRING.
    pub fn new() -> Self {
        HString {
            inner: WindowsHString::new(),
        }
    }

    /// Get the raw HSTRING pointer.
    pub fn as_ptr(&self) -> *const u16 {
        self.inner.as_ptr()
    }

    /// Get the length of the string.
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Check if the string is empty.
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Convert to a Rust String.
    pub fn to_string_lossy(&self) -> String {
        self.inner.to_string_lossy()
    }

    /// Get the inner windows HSTRING.
    pub fn inner(&self) -> &WindowsHString {
        &self.inner
    }
}

impl Default for HString {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for HString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HString(\"{}\")", self.to_string_lossy())
    }
}

impl fmt::Display for HString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string_lossy())
    }
}

impl From<&str> for HString {
    fn from(s: &str) -> Self {
        HString::from(s)
    }
}

impl From<String> for HString {
    fn from(s: String) -> Self {
        HString::from(s.as_str())
    }
}


