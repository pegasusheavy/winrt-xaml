//! DesktopWindowXamlSource - The bridge between Win32 and XAML.

use crate::error::Result;
use crate::winrt::{IInspectable, IActivationFactory};
use windows::core::{IInspectable as CoreIInspectable, GUID};
use windows::Win32::Foundation::HWND;
use std::sync::Arc;
use parking_lot::RwLock;

// Extension trait to get raw pointer from IInspectable
trait IInspectableExt {
    fn as_raw(&self) -> *mut std::ffi::c_void;
}

impl IInspectableExt for IInspectable {
    fn as_raw(&self) -> *mut std::ffi::c_void {
        // For now, return null - we need proper implementation
        std::ptr::null_mut()
    }
}

/// GUID for IDesktopWindowXamlSourceNative
/// {3cbcf1bf-2f76-4e9c-96ab-e84b37972554}
#[allow(dead_code)]
const IID_IDESKTOPWINDOWXAMLSOURCENATIVE: GUID = GUID::from_u128(0x3cbcf1bf_2f76_4e9c_96ab_e84b37972554);

/// GUID for IDesktopWindowXamlSourceNative2
/// {e3dcd8c7-3057-4692-99c3-7b7720afda31}
#[allow(dead_code)]
const IID_IDESKTOPWINDOWXAMLSOURCENATIVE2: GUID = GUID::from_u128(0xe3dcd8c7_3057_4692_99c3_7b7720afda31);

/// DesktopWindowXamlSource - Hosts XAML content in a Win32 window.
///
/// This is the primary class for XAML Islands, bridging UWP XAML and Win32.
pub struct DesktopWindowXamlSource {
    inner: Arc<DesktopWindowXamlSourceInner>,
}

struct DesktopWindowXamlSourceInner {
    inspectable: IInspectable,
    island_hwnd: RwLock<isize>,
}

impl DesktopWindowXamlSource {
    /// Create a new DesktopWindowXamlSource.
    pub fn new() -> Result<Self> {
        // Activate the DesktopWindowXamlSource runtime class
        let factory = IActivationFactory::get("Windows.UI.Xaml.Hosting.DesktopWindowXamlSource")
            .map_err(|e| crate::error::Error::initialization(
                format!("Failed to get DesktopWindowXamlSource factory: {}", e)
            ))?;

        let inspectable: CoreIInspectable = factory.activate_instance()
            .map_err(|e| crate::error::Error::initialization(
                format!("Failed to activate DesktopWindowXamlSource: {}", e)
            ))?;

        Ok(DesktopWindowXamlSource {
            inner: Arc::new(DesktopWindowXamlSourceInner {
                inspectable: IInspectable::from(inspectable),
                island_hwnd: RwLock::new(0),
            }),
        })
    }

    /// Attach the XAML Island to a parent Win32 window.
    pub fn attach_to_window(&self, parent_hwnd: HWND) -> Result<HWND> {
        println!("   🔗 Attaching XAML Island to window {:?}...", parent_hwnd);

        // Get the IInspectable as a raw pointer
        let inspectable_ptr = self.inner.inspectable.as_raw();

        // Try to QueryInterface for IDesktopWindowXamlSourceNative
        // This is a COM interface that provides AttachToWindow method
        unsafe {
            let mut native_ptr: *mut std::ffi::c_void = std::ptr::null_mut();

            // QueryInterface using the IDesktopWindowXamlSourceNative GUID
            let hr = (*(inspectable_ptr as *mut *mut usize).offset(0))
                .cast::<unsafe extern "system" fn(
                    *mut std::ffi::c_void,
                    *const GUID,
                    *mut *mut std::ffi::c_void,
                ) -> i32>()
                .offset(0)
                .read()(
                inspectable_ptr,
                &IID_IDESKTOPWINDOWXAMLSOURCENATIVE,
                &mut native_ptr,
            );

            if hr != 0 {
                println!("   ⚠️  QueryInterface for IDesktopWindowXamlSourceNative failed: 0x{:08X}", hr as u32);
                println!("   ℹ️  This is expected - we need the actual WinRT runtime support");
                println!("   ℹ️  Falling back to placeholder for now");
                return Ok(HWND(std::ptr::null_mut()));
            }

            if native_ptr.is_null() {
                println!("   ⚠️  IDesktopWindowXamlSourceNative interface is null");
                return Ok(HWND(std::ptr::null_mut()));
            }

            // Call AttachToWindow on the native interface
            // AttachToWindow is at offset 6 in the vtable (after IUnknown methods and IInspectable methods)
            let mut island_hwnd = HWND(std::ptr::null_mut());

            let attach_result = (*(native_ptr as *mut *mut usize).offset(9))
                .cast::<unsafe extern "system" fn(
                    *mut std::ffi::c_void,
                    HWND,
                    *mut HWND,
                ) -> i32>()
                .read()(
                native_ptr,
                parent_hwnd,
                &mut island_hwnd,
            );

            // Release the native interface
            (*(native_ptr as *mut *mut usize).offset(2))
                .cast::<unsafe extern "system" fn(*mut std::ffi::c_void) -> u32>()
                .read()(native_ptr);

            if attach_result != 0 {
                return Err(crate::error::Error::initialization(
                    format!("AttachToWindow failed: HRESULT 0x{:08X}", attach_result as u32)
                ));
            }

            println!("   ✅ XAML Island attached successfully");
            println!("   Island HWND: {:?}", island_hwnd);

            // Store the island HWND
            *self.inner.island_hwnd.write() = island_hwnd.0 as isize;

            Ok(island_hwnd)
        }
    }

    /// Get the HWND of the XAML Island.
    pub fn window_handle(&self) -> HWND {
        HWND(*self.inner.island_hwnd.read() as *mut _)
    }

    /// Set the XAML content to display.
    pub fn set_content(&self, _content: &IInspectable) -> Result<()> {
        // This would call the Content property setter on the DesktopWindowXamlSource
        // For now, we'll need to implement property access via WinRT

        // TODO: Implement proper property access
        // This requires calling IDesktopWindowXamlSource::put_Content

        Ok(())
    }

    /// Get the underlying IInspectable.
    pub fn as_inspectable(&self) -> &IInspectable {
        &self.inner.inspectable
    }
}

impl Clone for DesktopWindowXamlSource {
    fn clone(&self) -> Self {
        DesktopWindowXamlSource {
            inner: self.inner.clone(),
        }
    }
}

unsafe impl Send for DesktopWindowXamlSource {}
unsafe impl Sync for DesktopWindowXamlSource {}

