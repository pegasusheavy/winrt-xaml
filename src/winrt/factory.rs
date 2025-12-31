//! IActivationFactory interface for creating WinRT objects.

use super::inspectable::IInspectable;
use windows::core::{IInspectable as CoreIInspectable, Interface, HSTRING};
use windows::Win32::System::WinRT::RoGetActivationFactory;

/// IActivationFactory is used to create instances of WinRT runtime classes.
#[repr(transparent)]
pub struct IActivationFactory {
    inner: CoreIInspectable,
}

impl IActivationFactory {
    /// Activate a WinRT runtime class by name.
    ///
    /// # Arguments
    /// * `class_name` - The fully-qualified runtime class name (e.g., "Windows.UI.Xaml.Application")
    ///
    /// # Returns
    /// An IActivationFactory that can be used to create instances of the class.
    pub fn get(class_name: &str) -> windows::core::Result<Self> {
        unsafe {
            let name = HSTRING::from(class_name);
            let factory: CoreIInspectable = RoGetActivationFactory(&name)?;

            Ok(IActivationFactory { inner: factory })
        }
    }

    /// Activate an instance of the runtime class (parameterless constructor).
    pub fn activate_instance<T: Interface>(&self) -> windows::core::Result<T> {
        self.inner.cast()
    }

    /// Cast to a specific factory interface.
    pub fn cast<T: Interface>(&self) -> windows::core::Result<T> {
        self.inner.cast()
    }

    /// Get the inner IInspectable.
    pub fn as_inspectable(&self) -> IInspectable {
        IInspectable::from(self.inner.clone())
    }
}

impl Clone for IActivationFactory {
    fn clone(&self) -> Self {
        IActivationFactory {
            inner: self.inner.clone(),
        }
    }
}

impl From<CoreIInspectable> for IActivationFactory {
    fn from(inner: CoreIInspectable) -> Self {
        IActivationFactory { inner }
    }
}

unsafe impl Send for IActivationFactory {}
unsafe impl Sync for IActivationFactory {}

/// Helper function to activate a WinRT runtime class.
///
/// # Example
/// ```ignore
/// let app: IApplication = activate("Windows.UI.Xaml.Application")?;
/// ```
pub fn activate<T: Interface>(class_name: &str) -> windows::core::Result<T> {
    let factory = IActivationFactory::get(class_name)?;
    factory.activate_instance()
}

