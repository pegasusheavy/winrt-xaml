//! WindowsXamlManager - Initializes the XAML framework for the current thread.
//!
//! Required before creating any Windows.UI.Xaml.UIElement objects.

use crate::error::Result;
use crate::winrt::{IInspectable, IActivationFactory};
use windows::core::IInspectable as CoreIInspectable;

/// WindowsXamlManager - Manages XAML framework initialization.
pub struct WindowsXamlManager {
    inspectable: IInspectable,
}

impl WindowsXamlManager {
    /// Initialize the XAML framework for the current thread.
    ///
    /// This must be called before creating any XAML UI elements.
    /// Returns a WindowsXamlManager that keeps the framework alive.
    pub fn initialize_for_current_thread() -> Result<Self> {
        println!("🔧 Initializing WindowsXamlManager...");

        // Try to activate Windows.UI.Xaml.Hosting.WindowsXamlManager
        let factory = IActivationFactory::get("Windows.UI.Xaml.Hosting.WindowsXamlManager")
            .map_err(|e| crate::error::Error::initialization(
                format!("Failed to get WindowsXamlManager factory: {}", e)
            ))?;

        let inspectable: CoreIInspectable = factory.activate_instance()
            .map_err(|e| crate::error::Error::initialization(
                format!("Failed to activate WindowsXamlManager: {}", e)
            ))?;

        println!("   ✅ WindowsXamlManager initialized successfully");

        Ok(WindowsXamlManager {
            inspectable: IInspectable::from(inspectable),
        })
    }

    /// Get the underlying IInspectable.
    pub fn as_inspectable(&self) -> &IInspectable {
        &self.inspectable
    }
}

unsafe impl Send for WindowsXamlManager {}
unsafe impl Sync for WindowsXamlManager {}


