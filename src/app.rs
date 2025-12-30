//! Application management and message loop.

use crate::error::{Error, Result};
use once_cell::sync::Lazy;
use parking_lot::RwLock;
use std::sync::Arc;
use windows::Win32::{
    Foundation::*,
    UI::WindowsAndMessaging::*,
};

/// Global application instance
static APP_INSTANCE: Lazy<RwLock<Option<Arc<ApplicationInner>>>> = Lazy::new(|| RwLock::new(None));

/// The main application instance.
///
/// This manages the application lifecycle and message loop.
#[derive(Clone)]
pub struct Application {
    inner: Arc<ApplicationInner>,
}

struct ApplicationInner {
    exit_code: RwLock<i32>,
    should_exit: RwLock<bool>,
}

impl Application {
    /// Create a new application instance.
    pub fn new() -> Result<Self> {
        let mut app = APP_INSTANCE.write();

        if app.is_some() {
            return Err(Error::application("Application already created"));
        }

        let inner = Arc::new(ApplicationInner {
            exit_code: RwLock::new(0),
            should_exit: RwLock::new(false),
        });

        *app = Some(inner.clone());

        Ok(Application { inner })
    }

    /// Get the current application instance if one exists.
    pub fn current() -> Option<Self> {
        APP_INSTANCE
            .read()
            .as_ref()
            .map(|inner| Application {
                inner: inner.clone(),
            })
    }

    /// Run the application message loop.
    ///
    /// This will block until the application exits.
    pub fn run(&self) -> Result<()> {
        unsafe {
            let mut msg = MSG::default();

            while GetMessageW(&mut msg, HWND(std::ptr::null_mut()), 0, 0).as_bool() {
                if *self.inner.should_exit.read() {
                    break;
                }

                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }

            Ok(())
        }
    }

    /// Exit the application with the specified exit code.
    pub fn exit_with_code(&self, code: i32) {
        *self.inner.exit_code.write() = code;
        *self.inner.should_exit.write() = true;

        unsafe {
            PostQuitMessage(code);
        }
    }

    /// Exit the application with exit code 0.
    pub fn exit(&self) {
        self.exit_with_code(0);
    }

    /// Get the exit code.
    pub fn exit_code(&self) -> i32 {
        *self.inner.exit_code.read()
    }

    /// Check if the application should exit.
    pub fn should_exit(&self) -> bool {
        *self.inner.should_exit.read()
    }
}

impl Default for Application {
    fn default() -> Self {
        Self::new().expect("Failed to create application")
    }
}
