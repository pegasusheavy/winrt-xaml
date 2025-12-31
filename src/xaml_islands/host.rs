//! XAML Island host window integration.

use super::DesktopWindowXamlSource;
use crate::error::Result;
use crate::window::Window;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{SetWindowPos, ShowWindow, SWP_NOACTIVATE, SWP_NOZORDER, SW_SHOW, SW_HIDE};

/// Helper for hosting XAML Islands in a Win32 window.
#[derive(Clone)]
pub struct XamlIslandHost {
    source: DesktopWindowXamlSource,
    #[allow(dead_code)]
    parent_hwnd: isize,
    island_hwnd: isize,
}

impl XamlIslandHost {
    /// Create a new XAML Island host for the given window.
    pub fn new(window: &Window) -> Result<Self> {
        let parent_hwnd = window.hwnd();

        // Create the DesktopWindowXamlSource
        let source = DesktopWindowXamlSource::new()?;

        // Attach to the parent window
        let island_hwnd = source.attach_to_window(parent_hwnd)?;

        println!("✅ XAML Island attached to window");
        println!("   Parent HWND: {:?}", parent_hwnd);
        println!("   Island HWND: {:?}", island_hwnd);

        Ok(XamlIslandHost {
            source,
            parent_hwnd: parent_hwnd.0 as isize,
            island_hwnd: island_hwnd.0 as isize,
        })
    }

    /// Get the DesktopWindowXamlSource.
    pub fn source(&self) -> &DesktopWindowXamlSource {
        &self.source
    }

    /// Get the island HWND.
    pub fn island_hwnd(&self) -> HWND {
        HWND(self.island_hwnd as *mut _)
    }

    /// Update the island size to match the parent window.
    pub fn update_size(&self, width: i32, height: i32) -> Result<()> {
        let hwnd = self.island_hwnd();
        unsafe {
            SetWindowPos(
                hwnd,
                HWND(std::ptr::null_mut()),
                0,
                0,
                width,
                height,
                SWP_NOZORDER | SWP_NOACTIVATE,
            )?;
        }
        Ok(())
    }

    /// Show the XAML Island.
    pub fn show(&self) -> Result<()> {
        let hwnd = self.island_hwnd();
        unsafe {
            let _ = ShowWindow(hwnd, SW_SHOW);
        }
        Ok(())
    }

    /// Hide the XAML Island.
    pub fn hide(&self) -> Result<()> {
        let hwnd = self.island_hwnd();
        unsafe {
            let _ = ShowWindow(hwnd, SW_HIDE);
        }
        Ok(())
    }
}

