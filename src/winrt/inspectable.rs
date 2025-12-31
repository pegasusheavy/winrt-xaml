//! IInspectable interface - base interface for all WinRT objects.

use windows::core::{IInspectable as CoreIInspectable, Interface};

/// IInspectable is the base interface for all WinRT objects.
///
/// It extends IUnknown with methods to get runtime type information.
#[repr(transparent)]
pub struct IInspectable {
    inner: CoreIInspectable,
}

impl IInspectable {
    /// Cast to a specific WinRT interface.
    pub fn cast<T: Interface>(&self) -> windows::core::Result<T> {
        self.inner.cast()
    }

    /// Get the inner Core IInspectable.
    pub fn inner(&self) -> &CoreIInspectable {
        &self.inner
    }

    /// Clone the inner IInspectable.
    pub fn to_inner(&self) -> CoreIInspectable {
        self.inner.clone()
    }
}

impl From<CoreIInspectable> for IInspectable {
    fn from(inner: CoreIInspectable) -> Self {
        IInspectable { inner }
    }
}

impl Clone for IInspectable {
    fn clone(&self) -> Self {
        IInspectable {
            inner: self.inner.clone(),
        }
    }
}

unsafe impl Send for IInspectable {}
unsafe impl Sync for IInspectable {}

