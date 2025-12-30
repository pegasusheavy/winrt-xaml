//! Pure WinRT Button Example
//!
//! This example demonstrates a REAL WinRT XAML button using XAML Islands.
//! The button is a true WinRT control, not a Win32 control.

use winrt_xaml::xaml_native::*;
use winrt_xaml::error::Result;
use windows::core::w;
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, RECT, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::System::Com::{CoInitializeEx, CoUninitialize, COINIT_APARTMENTTHREADED};
use std::ptr;

// Minimal Win32 window for hosting XAML (not for UI)
fn create_host_window() -> Result<HWND> {
    unsafe {
        let class_name = w!("WinRT_XAML_Host");

        let wc = WNDCLASSW {
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(host_window_proc),
            hInstance: GetModuleHandleW(None)?.into(),
            lpszClassName: class_name,
            hCursor: LoadCursorW(None, IDC_ARROW)?,
            ..Default::default()
        };

        if RegisterClassW(&wc) == 0 {
            return Err(winrt_xaml::error::Error::window_creation("Failed to register window class".to_string()));
        }

        let hwnd = CreateWindowExW(
            WINDOW_EX_STYLE(0),
            class_name,
            w!("Pure WinRT XAML Button - REAL WinRT!"),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            800,
            600,
            None,
            None,
            GetModuleHandleW(None)?,
            Some(ptr::null()),
        )?;

        Ok(hwnd)
    }
}

unsafe extern "system" fn host_window_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_SIZE => {
            // Resize XAML island when window resizes
            let width = (lparam.0 & 0xFFFF) as i32;
            let height = ((lparam.0 >> 16) & 0xFFFF) as i32;

            // Find the XAML island child window
            if let Ok(child) = GetWindow(hwnd, GW_CHILD) {
                if !child.0.is_null() {
                let _ = SetWindowPos(
                    child,
                    None,
                    0,
                    0,
                    width,
                    height,
                    SWP_NOZORDER | SWP_NOACTIVATE,
                );
                }
            }
            LRESULT(0)
        }
        WM_DESTROY => {
            PostQuitMessage(0);
            LRESULT(0)
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}

fn run_message_loop() -> Result<()> {
    unsafe {
        let mut msg = MSG::default();
        while GetMessageW(&mut msg, None, 0, 0).as_bool() {
            let _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    println!("╔═══════════════════════════════════════════════════════════╗");
    println!("║         Pure WinRT XAML Button - REAL WinRT!             ║");
    println!("╚═══════════════════════════════════════════════════════════╝\n");

    // Step 0: Initialize COM
    println!("🔧 Step 0: Initializing COM for WinRT...");
    unsafe {
        let hr = CoInitializeEx(None, COINIT_APARTMENTTHREADED);
        if hr.is_err() {
            return Err(winrt_xaml::error::Error::initialization(format!("Failed to initialize COM: {:?}", hr)));
        }
    }
    println!("   ✅ COM initialized\n");

    // Step 1: Initialize XAML framework
    println!("🔧 Step 1: Initializing Windows XAML Manager...");
    let _manager = XamlManager::new()?;
    println!("   ✅ XAML framework initialized\n");

    // Step 2: Create hosting window (minimal Win32, no UI)
    println!("🪟 Step 2: Creating host window (Win32 for hosting only)...");
    let host_hwnd = create_host_window()?;
    println!("   ✅ Host window created: {:?}\n", host_hwnd);

    // Step 3: Create XAML source (the bridge to WinRT)
    println!("🌉 Step 3: Creating DesktopWindowXamlSource...");
    let mut xaml_source = XamlSource::new()?;
    println!("   ✅ XAML source created\n");

    // Step 4: Attach XAML to host window
    println!("🔗 Step 4: Attaching XAML Island to host window...");
    let island_hwnd = xaml_source.attach_to_window(host_hwnd)?;
    println!("   ✅ XAML Island attached");
    println!("   Island HWND: {:?}\n", island_hwnd);

    // Step 5: Create WinRT Button
    println!("🔘 Step 5: Creating WinRT XAML Button...");
    let button = XamlButton::new()?;
    println!("   ✅ WinRT Button created\n");

    // Step 6: Configure button
    println!("⚙️  Step 6: Configuring button properties...");
    button.set_content("🎉 Click Me! I'm a Real WinRT Button!")?;
    button.set_size(300.0, 60.0)?;
    println!("   ✅ Button configured\n");

    // Step 7: Display the button
    println!("👁️  Step 7: Setting button as XAML content...");
    xaml_source.set_content(&button)?;
    println!("   ✅ Button is now visible!\n");

    // Make island window visible and sized
    unsafe {
        let _ = ShowWindow(island_hwnd, SW_SHOW);
        let mut rect = RECT::default();
        let _ = GetClientRect(host_hwnd, &mut rect);
        let _ = SetWindowPos(
            island_hwnd,
            None,
            0,
            0,
            rect.right - rect.left,
            rect.bottom - rect.top,
            SWP_NOZORDER | SWP_NOACTIVATE,
        );
    }

    println!("═══════════════════════════════════════════════════════════");
    println!("✅ SUCCESS! Pure WinRT XAML Button is now rendering!");
    println!("═══════════════════════════════════════════════════════════");
    println!("\n📊 What you're seeing:");
    println!("   • REAL WinRT XAML Button (not Win32)");
    println!("   • Fluent Design styling");
    println!("   • Modern Windows 11 appearance");
    println!("   • True XAML visual tree");
    println!("\n💡 This is NOT a Win32 button dressed up.");
    println!("   This is the ACTUAL Windows.UI.Xaml.Controls.Button!");
    println!("\n🎬 Close the window to exit.\n");

    let result = run_message_loop();

    // Cleanup COM
    unsafe {
        CoUninitialize();
    }

    result
}

