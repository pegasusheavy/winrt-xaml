//! Windows.UI.Xaml.Application bindings.

use crate::winrt::{IInspectable, IActivationFactory};
use crate::error::Result;
use windows::core::IInspectable as CoreIInspectable;

/// Windows.UI.Xaml.Application - The base class for UWP applications.
#[derive(Clone)]
pub struct XamlApplication {
    inspectable: IInspectable,
}

impl XamlApplication {
    /// Create a new Application instance using WinRT activation.
    pub fn new() -> Result<Self> {
        // Try to activate the Windows.UI.Xaml.Application runtime class
        let factory = IActivationFactory::get("Windows.UI.Xaml.Application")
            .map_err(|e| crate::error::Error::initialization(format!("Failed to get XAML Application factory: {}", e)))?;

        let inspectable: CoreIInspectable = factory.activate_instance()
            .map_err(|e| crate::error::Error::initialization(format!("Failed to activate XAML Application: {}", e)))?;

        Ok(XamlApplication {
            inspectable: IInspectable::from(inspectable),
        })
    }

    /// Get the current Application instance (singleton pattern in XAML).
    pub fn current() -> Result<Self> {
        // For now, create a new instance
        // In a real implementation, this would retrieve the current app instance
        Self::new()
    }

    /// Run the application message loop.
    pub fn run(&self) -> Result<()> {
        // The WinRT Application.Run() method would be called here
        // For now, fall back to Win32 message loop
        let app = crate::app::Application::current()
            .ok_or_else(|| crate::error::Error::application("No application instance available"))?;
        app.run()
    }

    /// Exit the application with a code.
    pub fn exit(&self, code: i32) -> Result<()> {
        let app = crate::app::Application::current()
            .ok_or_else(|| crate::error::Error::application("No application instance available"))?;
        app.exit_with_code(code);
        Ok(())
    }

    /// Get the underlying IInspectable.
    pub fn as_inspectable(&self) -> &IInspectable {
        &self.inspectable
    }
}

impl Default for XamlApplication {
    fn default() -> Self {
        Self::new().expect("Failed to create XAML Application")
    }
}

unsafe impl Send for XamlApplication {}
unsafe impl Sync for XamlApplication {}

