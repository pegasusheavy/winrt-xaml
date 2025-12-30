//! Window management using Win32 APIs.

use crate::controls::UIElement;
use crate::error::{Error, Result};
use once_cell::sync::Lazy;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use windows::core::*;
use windows::Win32::{
    Foundation::*,
    Graphics::Gdi::*,
    System::LibraryLoader::GetModuleHandleW,
    UI::WindowsAndMessaging::*,
};

const WINDOW_CLASS_NAME: PCWSTR = w!("WinRTXamlWindow");

/// Global window storage
static WINDOWS: Lazy<RwLock<HashMap<isize, Arc<WindowInner>>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

/// Register the window class (call once)
fn register_window_class() -> Result<()> {
    unsafe {
        let hinstance = GetModuleHandleW(None)?;

        let wc = WNDCLASSW {
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(window_proc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: hinstance.into(),
            hIcon: LoadIconW(HINSTANCE(std::ptr::null_mut()), IDI_APPLICATION)?,
            hCursor: LoadCursorW(HINSTANCE(std::ptr::null_mut()), IDC_ARROW)?,
            hbrBackground: HBRUSH(((COLOR_WINDOW.0 + 1) as isize) as *mut core::ffi::c_void),
            lpszMenuName: PCWSTR::null(),
            lpszClassName: WINDOW_CLASS_NAME,
        };

        let atom = RegisterClassW(&wc);
        if atom == 0 {
            return Err(Error::window_creation("Failed to register window class"));
        }

        Ok(())
    }
}

/// Window procedure for handling messages
unsafe extern "system" fn window_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_DESTROY => {
            // Clean up window from storage
            WINDOWS.write().remove(&(hwnd.0 as isize));
            PostQuitMessage(0);
            LRESULT(0)
        }
        WM_CLOSE => {
            let _ = DestroyWindow(hwnd);
            LRESULT(0)
        }
        WM_SIZE => {
            // Handle resize
            if let Some(window) = WINDOWS.read().get(&(hwnd.0 as isize)) {
                let width = lparam.0 & 0xFFFF;
                let height = (lparam.0 >> 16) & 0xFFFF;
                *window.size.write() = (width as i32, height as i32);

                // Relayout content if needed
                if let Some(content) = window.content.read().as_ref() {
                    // TODO: Trigger layout update on content
                    let _ = content;
                }
            }
            LRESULT(0)
        }
        WM_PAINT => {
            let mut ps = PAINTSTRUCT::default();
            unsafe {
                let _hdc = BeginPaint(hwnd, &mut ps);
                // Custom painting can go here
                let _ = EndPaint(hwnd, &ps);
            }
            LRESULT(0)
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}

/// A window that can host UI content.
#[derive(Clone)]
pub struct Window {
    inner: Arc<WindowInner>,
}

struct WindowInner {
    hwnd: RwLock<isize>,
    title: RwLock<String>,
    size: RwLock<(i32, i32)>,
    position: RwLock<(i32, i32)>,
    content: RwLock<Option<UIElement>>,
    is_visible: RwLock<bool>,
}

impl Window {
    /// Create a new window.
    pub fn new() -> Result<Self> {
        // Ensure window class is registered
        static REGISTER: Lazy<Result<()>> = Lazy::new(register_window_class);
        REGISTER.as_ref().map_err(|e| Error::window_creation(format!("{}", e)))?;

        let inner = Arc::new(WindowInner {
            hwnd: RwLock::new(0),
            title: RwLock::new("Window".to_string()),
            size: RwLock::new((800, 600)),
            position: RwLock::new((CW_USEDEFAULT, CW_USEDEFAULT)),
            content: RwLock::new(None),
            is_visible: RwLock::new(false),
        });

        Ok(Window { inner })
    }

    /// Create a window builder for fluent API.
    pub fn builder() -> WindowBuilder {
        WindowBuilder::new()
    }

    /// Create the actual Win32 window.
    fn create_window(&self) -> Result<()> {
        unsafe {
            let hinstance = GetModuleHandleW(None)?;
            let title = self.inner.title.read();
            let title_wide: Vec<u16> = title.encode_utf16().chain(Some(0)).collect();
            let (width, height) = *self.inner.size.read();
            let (x, y) = *self.inner.position.read();

            let hwnd = CreateWindowExW(
                WINDOW_EX_STYLE(0),
                WINDOW_CLASS_NAME,
                PCWSTR(title_wide.as_ptr()),
                WS_OVERLAPPEDWINDOW,
                x,
                y,
                width,
                height,
                HWND(std::ptr::null_mut()),
                HMENU(std::ptr::null_mut()),
                HINSTANCE(hinstance.0),
                None,
            )?;

            if hwnd.0.is_null() {
                return Err(Error::window_creation("CreateWindowExW failed"));
            }

            *self.inner.hwnd.write() = hwnd.0 as isize;

            // Store window in global map
            WINDOWS.write().insert(hwnd.0 as isize, self.inner.clone());

            Ok(())
        }
    }

    /// Get the window handle.
    pub fn hwnd(&self) -> HWND {
        HWND(*self.inner.hwnd.read() as *mut core::ffi::c_void)
    }

    /// Set the window title.
    pub fn set_title(&self, title: impl AsRef<str>) -> Result<()> {
        let title = title.as_ref();
        *self.inner.title.write() = title.to_string();

        let hwnd = self.hwnd();
        if !hwnd.0.is_null() {
            unsafe {
                let title_wide: Vec<u16> = title.encode_utf16().chain(Some(0)).collect();
                SetWindowTextW(hwnd, PCWSTR(title_wide.as_ptr()))?;
            }
        }

        Ok(())
    }

    /// Get the window title.
    pub fn title(&self) -> String {
        self.inner.title.read().clone()
    }

    /// Set the window size.
    pub fn set_size(&self, width: i32, height: i32) -> Result<()> {
        *self.inner.size.write() = (width, height);

        let hwnd = self.hwnd();
        if !hwnd.0.is_null() {
            unsafe {
                SetWindowPos(
                    hwnd,
                    HWND(std::ptr::null_mut()),
                    0,
                    0,
                    width,
                    height,
                    SWP_NOMOVE | SWP_NOZORDER,
                )?;
            }
        }

        Ok(())
    }

    /// Get the window size.
    pub fn size(&self) -> (i32, i32) {
        *self.inner.size.read()
    }

    /// Set the window position.
    pub fn set_position(&self, x: i32, y: i32) -> Result<()> {
        *self.inner.position.write() = (x, y);

        let hwnd = self.hwnd();
        if !hwnd.0.is_null() {
            unsafe {
                SetWindowPos(hwnd, HWND(std::ptr::null_mut()), x, y, 0, 0, SWP_NOSIZE | SWP_NOZORDER)?;
            }
        }

        Ok(())
    }

    /// Get the window position.
    pub fn position(&self) -> (i32, i32) {
        *self.inner.position.read()
    }

    /// Show the window.
    pub fn show(&self) -> Result<()> {
        let hwnd = self.hwnd();
        if hwnd.0.is_null() {
            self.create_window()?;
        }

        unsafe {
            ShowWindow(self.hwnd(), SW_SHOW);
            let _ = UpdateWindow(self.hwnd());
        }

        *self.inner.is_visible.write() = true;
        Ok(())
    }

    /// Hide the window.
    pub fn hide(&self) -> Result<()> {
        let hwnd = self.hwnd();
        if !hwnd.0.is_null() {
            unsafe {
                ShowWindow(hwnd, SW_HIDE);
            }
        }

        *self.inner.is_visible.write() = false;
        Ok(())
    }

    /// Check if the window is visible.
    pub fn is_visible(&self) -> bool {
        *self.inner.is_visible.read()
    }

    /// Close the window.
    pub fn close(&self) -> Result<()> {
        let hwnd = self.hwnd();
        if !hwnd.0.is_null() {
            unsafe {
                DestroyWindow(hwnd)?;
            }
            *self.inner.hwnd.write() = 0;
        }
        Ok(())
    }

    /// Center the window on the screen.
    pub fn center(&self) -> Result<()> {
        unsafe {
            let (width, height) = self.size();
            let screen_width = GetSystemMetrics(SM_CXSCREEN);
            let screen_height = GetSystemMetrics(SM_CYSCREEN);

            let x = (screen_width - width) / 2;
            let y = (screen_height - height) / 2;

            self.set_position(x, y)?;
        }

        Ok(())
    }

    /// Set the window content.
    pub fn set_content(&self, content: impl Into<UIElement>) -> Result<()> {
        let content = content.into();
        *self.inner.content.write() = Some(content.clone());

        // If window is created, create child controls
        let hwnd = self.hwnd();
        if !hwnd.0.is_null() {
            // TODO: Create child window for content
            // This will be implemented when we have UIElement properly working
        }

        Ok(())
    }

    /// Get the window content.
    pub fn content(&self) -> Option<UIElement> {
        self.inner.content.read().clone()
    }

    /// Maximize the window.
    pub fn maximize(&self) -> Result<()> {
        let hwnd = self.hwnd();
        if !hwnd.0.is_null() {
            unsafe {
                ShowWindow(hwnd, SW_MAXIMIZE);
            }
        }
        Ok(())
    }

    /// Minimize the window.
    pub fn minimize(&self) -> Result<()> {
        let hwnd = self.hwnd();
        if !hwnd.0.is_null() {
            unsafe {
                ShowWindow(hwnd, SW_MINIMIZE);
            }
        }
        Ok(())
    }

    /// Restore the window.
    pub fn restore(&self) -> Result<()> {
        let hwnd = self.hwnd();
        if !hwnd.0.is_null() {
            unsafe {
                ShowWindow(hwnd, SW_RESTORE);
            }
        }
        Ok(())
    }
}

impl Default for Window {
    fn default() -> Self {
        Self::new().expect("Failed to create window")
    }
}

/// Builder for creating windows with a fluent API.
pub struct WindowBuilder {
    title: String,
    width: i32,
    height: i32,
    x: i32,
    y: i32,
}

impl WindowBuilder {
    /// Create a new window builder.
    pub fn new() -> Self {
        Self {
            title: "Window".to_string(),
            width: 800,
            height: 600,
            x: CW_USEDEFAULT,
            y: CW_USEDEFAULT,
        }
    }

    /// Set the window title.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// Set the window size.
    pub fn size(mut self, width: i32, height: i32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    /// Set the window position.
    pub fn position(mut self, x: i32, y: i32) -> Self {
        self.x = x;
        self.y = y;
        self
    }

    /// Build the window.
    pub fn build(self) -> Result<Window> {
        let window = Window::new()?;
        *window.inner.title.write() = self.title;
        *window.inner.size.write() = (self.width, self.height);
        *window.inner.position.write() = (self.x, self.y);
        Ok(window)
    }
}

impl Default for WindowBuilder {
    fn default() -> Self {
        Self::new()
    }
}
