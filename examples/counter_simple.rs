//! Simple counter example demonstrating WinRT button and text controls.
//!
//! This example shows:
//! - Button creation and click handling
//! - TextBlock for display
//! - Stack panel layout
//! - Shared state management

use winrt_xaml::error::Result;
use winrt_xaml::xaml_native::*;
use windows::core::w;
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::System::Com::{CoInitializeEx, CoUninitialize, COINIT_APARTMENTTHREADED};
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Arc;
use std::ptr;

fn create_host_window() -> Result<HWND> {
    unsafe {
        let class_name = w!("WinRT_CounterSimple");
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
            w!("Counter Example (Simple)"),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT, CW_USEDEFAULT, 400, 350,
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
    println!("╔═══════════════════════════════════════════════╗");
    println!("║         Counter Example (Simple)            ║");
    println!("╚═══════════════════════════════════════════════╝\n");

    println!("Creating counter application...");

    // Initialize COM
    unsafe { CoInitializeEx(None, COINIT_APARTMENTTHREADED).ok(); }

    // Initialize XAML framework
    let _manager = XamlManager::new()?;

    // Create host window
    let host_hwnd = create_host_window()?;

    // Create XAML source
    let mut xaml_source = XamlSource::new()?;
    let island_hwnd = xaml_source.attach_to_window(host_hwnd)?;

    // Shared counter state
    let counter = Arc::new(AtomicI32::new(0));

    // Create main layout panel
    let main_panel = XamlStackPanel::new()?;
    main_panel.set_vertical(true)?;
    main_panel.set_spacing(22.0)?;
    main_panel.set_background(0xFF1C1C1C)?; // Dark theme
    main_panel.set_padding(32.0, 32.0, 32.0, 32.0)?;
    main_panel.set_corner_radius(12.0)?;

    // Title with modern styling
    let title = XamlTextBlock::new()?;
    title.set_text("⚡ Simple Counter")?;
    title.set_font_size(30.0)?;
    title.set_font_weight(700)?; // Bold
    title.set_foreground(0xFFFFFFFF)?; // White
    title.set_margin(0.0, 0.0, 0.0, 12.0)?;
    main_panel.add_child(&title.as_uielement())?;

    // Display label with modern styling
    let label = Arc::new(XamlTextBlock::new()?);
    label.set_text("Count: 0")?;
    label.set_font_size(44.0)?;
    label.set_font_weight(600)?; // SemiBold
    label.set_foreground(0xFF00FF9F)?; // Bright green
    label.set_margin(0.0, 18.0, 0.0, 25.0)?;
    main_panel.add_child(&label.as_uielement())?;

    // Button row
    let button_row = XamlStackPanel::new()?;
    button_row.set_vertical(false)?;
    button_row.set_spacing(10.0)?;

    // Increment button with styling
    let inc_button = XamlButton::new()?;
    inc_button.set_content("➕ Increment")?;
    inc_button.set_size(140.0, 56.0)?;
    inc_button.set_background(0xFF107C10)?; // Green
    inc_button.set_foreground(0xFFFFFFFF)?;
    inc_button.set_corner_radius(10.0)?;
    inc_button.set_padding(16.0, 12.0, 16.0, 12.0)?;

    let counter_clone = counter.clone();
    let label_clone = Arc::clone(&label);
    inc_button.on_click(move || {
        let new_value = counter_clone.fetch_add(1, Ordering::SeqCst) + 1;
        let text = format!("Count: {}", new_value);
        let _ = label_clone.set_text(&text);
        println!("Counter incremented to: {}", new_value);
    })?;

    button_row.add_child(&inc_button.as_uielement())?;

    // Decrement button with styling
    let dec_button = XamlButton::new()?;
    dec_button.set_content("➖ Decrement")?;
    dec_button.set_size(140.0, 56.0)?;
    dec_button.set_background(0xFFE74856)?; // Red
    dec_button.set_foreground(0xFFFFFFFF)?;
    dec_button.set_corner_radius(10.0)?;
    dec_button.set_padding(16.0, 12.0, 16.0, 12.0)?;

    let counter_clone = counter.clone();
    let label_clone = Arc::clone(&label);
    dec_button.on_click(move || {
        let new_value = counter_clone.fetch_sub(1, Ordering::SeqCst) - 1;
        let text = format!("Count: {}", new_value);
        let _ = label_clone.set_text(&text);
        println!("Counter decremented to: {}", new_value);
    })?;

    button_row.add_child(&dec_button.as_uielement())?;

    // Create reset button
    let reset_button = XamlButton::new()?;
    reset_button.set_content("Reset")?;
    reset_button.set_size(120.0, 50.0)?;

    let counter_clone = counter.clone();
    let label_clone = Arc::clone(&label);
    reset_button.on_click(move || {
        counter_clone.store(0, Ordering::SeqCst);
        let _ = label_clone.set_text("Count: 0");
        println!("Counter reset to 0");
    })?;

    button_row.add_child(&reset_button.as_uielement())?;

    main_panel.add_child(&button_row.as_uielement())?;

    // Set content
    xaml_source.set_content_element(&main_panel.as_uielement())?;

    // Show and size the island
    unsafe {
        use windows::Win32::UI::WindowsAndMessaging::*;
        use windows::Win32::Foundation::RECT;

        let _ = ShowWindow(island_hwnd, SW_SHOW);
        let mut rect = RECT::default();
        let _ = GetClientRect(host_hwnd, &mut rect);
        let _ = SetWindowPos(island_hwnd, None, 0, 0,
            rect.right - rect.left, rect.bottom - rect.top,
            SWP_NOZORDER | SWP_NOACTIVATE);
    }

    println!("✅ Counter application started!");
    println!("Starting application...\n");

    // Run message loop
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
