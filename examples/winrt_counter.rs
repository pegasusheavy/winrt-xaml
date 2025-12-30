//! Simple WinRT Counter Example
//!
//! A basic counter application demonstrating:
//! - Button clicks
//! - TextBlock updates
//! - Stack Panel layout

use winrt_xaml::error::Result;
use winrt_xaml::xaml_native::*;
use windows::core::w;
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::System::Com::{CoInitializeEx, CoUninitialize, COINIT_APARTMENTTHREADED};
use std::ptr;

fn create_host_window() -> Result<HWND> {
    unsafe {
        let class_name = w!("WinRT_Counter");

        let wc = WNDCLASSW {
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(window_proc),
            hInstance: GetModuleHandleW(None)?.into(),
            lpszClassName: class_name,
            hCursor: LoadCursorW(None, IDC_ARROW)?,
            ..Default::default()
        };

        let _ = RegisterClassW(&wc);

        let hwnd = CreateWindowExW(
            WINDOW_EX_STYLE(0),
            class_name,
            w!("WinRT Counter"),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            500,
            400,
            None,
            None,
            GetModuleHandleW(None)?,
            Some(ptr::null()),
        )?;

        Ok(hwnd)
    }
}

unsafe extern "system" fn window_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_SIZE => {
            let width = (lparam.0 & 0xFFFF) as i32;
            let height = ((lparam.0 >> 16) & 0xFFFF) as i32;
            if let Ok(child) = GetWindow(hwnd, GW_CHILD) {
                if !child.0.is_null() {
                    let _ = SetWindowPos(child, None, 0, 0, width, height, SWP_NOZORDER | SWP_NOACTIVATE);
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

fn main() -> Result<()> {
    println!("\n╔═══════════════════════════════════════════════╗");
    println!("║      WinRT Counter - Simple Demo            ║");
    println!("╚═══════════════════════════════════════════════╝\n");

    unsafe {
        let hr = CoInitializeEx(None, COINIT_APARTMENTTHREADED);
        if hr.is_err() {
            return Err(winrt_xaml::error::Error::initialization(format!("COM init failed: {:?}", hr)));
        }
    }

    let _manager = XamlManager::new()?;
    let host_hwnd = create_host_window()?;
    let mut xaml_source = XamlSource::new()?;
    let island_hwnd = xaml_source.attach_to_window(host_hwnd)?;

    // Build UI with modern styling
    let panel = XamlStackPanel::new()?;
    panel.set_vertical(true)?;
    panel.set_spacing(25.0)?;
    panel.set_background(0xFF1A1A1A)?; // Very dark gray
    panel.set_padding(40.0, 40.0, 40.0, 40.0)?;
    panel.set_corner_radius(16.0)?;

    let title = XamlTextBlock::new()?;
    title.set_text("⚡ Counter")?;
    title.set_font_size(36.0)?;
    title.set_font_weight(700)?; // Bold
    title.set_foreground(0xFFFFFFFF)?; // White
    title.set_margin(0.0, 0.0, 0.0, 15.0)?;
    panel.add_child(&title.as_uielement())?;

    let counter_display = XamlTextBlock::new()?;
    counter_display.set_text("Count: 0")?;
    counter_display.set_font_size(56.0)?;
    counter_display.set_font_weight(600)?; // SemiBold
    counter_display.set_foreground(0xFF00D4FF)?; // Bright cyan
    counter_display.set_margin(0.0, 20.0, 0.0, 30.0)?;
    panel.add_child(&counter_display.as_uielement())?;

    let increment_btn = XamlButton::new()?;
    increment_btn.set_content("➕ Increment")?;
    increment_btn.set_size(220.0, 64.0)?;
    increment_btn.set_background(0xFF107C10)?; // Green
    increment_btn.set_foreground(0xFFFFFFFF)?;
    increment_btn.set_corner_radius(10.0)?;
    increment_btn.set_padding(16.0, 12.0, 16.0, 12.0)?;
    panel.add_child(&increment_btn.as_uielement())?;

    let decrement_btn = XamlButton::new()?;
    decrement_btn.set_content("➖ Decrement")?;
    decrement_btn.set_size(220.0, 64.0)?;
    decrement_btn.set_background(0xFFE74856)?; // Red
    decrement_btn.set_foreground(0xFFFFFFFF)?;
    decrement_btn.set_corner_radius(10.0)?;
    decrement_btn.set_padding(16.0, 12.0, 16.0, 12.0)?;
    panel.add_child(&decrement_btn.as_uielement())?;

    let reset_btn = XamlButton::new()?;
    reset_btn.set_content("🔄 Reset")?;
    reset_btn.set_size(220.0, 64.0)?;
    reset_btn.set_background(0xFF0078D4)?; // Blue
    reset_btn.set_foreground(0xFFFFFFFF)?;
    reset_btn.set_corner_radius(10.0)?;
    reset_btn.set_padding(16.0, 12.0, 16.0, 12.0)?;
    panel.add_child(&reset_btn.as_uielement())?;

    xaml_source.set_content_element(&panel.as_uielement())?;

    // Show and size island
    unsafe {
        let _ = ShowWindow(island_hwnd, SW_SHOW);
        let mut rect = windows::Win32::Foundation::RECT::default();
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

    println!("✅ Counter app ready!");
    println!("📝 Note: Event handling will be added in a future update");
    println!("🎬 Close window to exit\n");

    // Message loop
    unsafe {
        let mut msg = MSG::default();
        while GetMessageW(&mut msg, None, 0, 0).as_bool() {
            let _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
        CoUninitialize();
    }

    Ok(())
}

