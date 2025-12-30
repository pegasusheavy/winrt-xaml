//! XAML Islands - Host UWP XAML in Win32 windows.
//!
//! This module provides infrastructure for hosting Windows.UI.Xaml content
//! in traditional Win32 desktop applications using XAML Islands.

pub mod desktop_source;
pub mod host;

pub use desktop_source::DesktopWindowXamlSource;
pub use host::XamlIslandHost;

use crate::error::Result;
use windows::Win32::System::Com::{CoInitializeEx, COINIT_APARTMENTTHREADED};

/// Initialize XAML Islands for the current thread.
///
/// This must be called before using any XAML Islands functionality.
/// It initializes COM in apartment-threaded mode.
pub fn initialize() -> Result<()> {
    unsafe {
        let hr = CoInitializeEx(None, COINIT_APARTMENTTHREADED);
        if hr.is_ok() || hr.0 == 0x00000001 || hr.0 == 0x80010106_u32 as i32 {
            // Success or already initialized
            Ok(())
        } else {
            Err(crate::error::Error::initialization(
                format!("Failed to initialize COM for XAML Islands: HRESULT 0x{:08X}", hr.0 as u32)
            ))
        }
    }
}

/// Uninitialize XAML Islands for the current thread.
pub fn uninitialize() {
    unsafe {
        windows::Win32::System::Com::CoUninitialize();
    }
}

