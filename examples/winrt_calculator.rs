//! WinRT Calculator Example
//!
//! A calculator UI demonstrating:
//! - Grid layout for button arrangement
//! - TextBox for display
//! - Multiple buttons in a structured layout

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
        let class_name = w!("WinRT_Calculator");
        let wc = WNDCLASSW {
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(window_proc),
            hInstance: GetModuleHandleW(None)?.into(),
            lpszClassName: class_name,
            hCursor: LoadCursorW(None, IDC_ARROW)?,
            ..Default::default()
        };
        let _ = RegisterClassW(&wc);

        CreateWindowExW(
            WINDOW_EX_STYLE(0),
            class_name,
            w!("WinRT Calculator"),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT, CW_USEDEFAULT, 400, 600,
            None, None,
            GetModuleHandleW(None)?,
            Some(ptr::null()),
        ).map_err(|e| winrt_xaml::error::Error::window_creation(format!("{:?}", e)))
    }
}

unsafe extern "system" fn window_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg {
        WM_SIZE => {
            if let Ok(child) = GetWindow(hwnd, GW_CHILD) {
                if !child.0.is_null() {
                    let width = (lparam.0 & 0xFFFF) as i32;
                    let height = ((lparam.0 >> 16) & 0xFFFF) as i32;
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
    println!("║      WinRT Calculator - UI Demo             ║");
    println!("╚═══════════════════════════════════════════════╝\n");

    unsafe { CoInitializeEx(None, COINIT_APARTMENTTHREADED).ok(); }

    let _manager = XamlManager::new()?;
    let host_hwnd = create_host_window()?;
    let mut xaml_source = XamlSource::new()?;
    let island_hwnd = xaml_source.attach_to_window(host_hwnd)?;

    // Build calculator UI
    let main_panel = XamlStackPanel::new()?;
    main_panel.set_vertical(true)?;
    main_panel.set_spacing(10.0)?;

    // Title
    let title = XamlTextBlock::new()?;
    title.set_text("Calculator")?;
    title.set_font_size(24.0)?;
    main_panel.add_child(&title.as_uielement())?;

    // Display
    let display = XamlTextBox::new()?;
    display.set_text("0")?;
    display.set_size(350.0, 60.0)?;
    main_panel.add_child(&display.as_uielement())?;

    // Button rows
    let numbers = [
        ["7", "8", "9", "÷"],
        ["4", "5", "6", "×"],
        ["1", "2", "3", "-"],
        ["0", ".", "=", "+"],
    ];

    for row in &numbers {
        let row_panel = XamlStackPanel::new()?;
        row_panel.set_vertical(false)?;
        row_panel.set_spacing(5.0)?;

        for &label in row {
            let button = XamlButton::new()?;
            button.set_content(label)?;
            button.set_size(80.0, 60.0)?;
            row_panel.add_child(&button.as_uielement())?;
        }

        main_panel.add_child(&row_panel.as_uielement())?;
    }

    // Clear button
    let clear_btn = XamlButton::new()?;
    clear_btn.set_content("Clear (C)")?;
    clear_btn.set_size(350.0, 50.0)?;
    main_panel.add_child(&clear_btn.as_uielement())?;

    // Info
    let info = XamlTextBlock::new()?;
    info.set_text("📝 UI Demo - Event handling coming soon!")?;
    info.set_font_size(12.0)?;
    main_panel.add_child(&info.as_uielement())?;

    xaml_source.set_content_element(&main_panel.as_uielement())?;

    // Size island
    unsafe {
        let mut rect = windows::Win32::Foundation::RECT::default();
        let _ = GetClientRect(host_hwnd, &mut rect);
        let _ = SetWindowPos(island_hwnd, None, 0, 0,
            rect.right - rect.left, rect.bottom - rect.top,
            SWP_NOZORDER | SWP_NOACTIVATE);
    }

    println!("✅ Calculator UI ready!");
    println!("📊 Demonstrating:");
    println!("   • StackPanel with nested panels");
    println!("   • TextBox for display");
    println!("   • Grid-like button layout");
    println!("   • Fluent Design button styling");
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

