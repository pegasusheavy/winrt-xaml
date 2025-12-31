//! Counter example demonstrating state management with WinRT.

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

        CreateWindowExW(
            WINDOW_EX_STYLE(0),
            class_name,
            w!("Counter Application (Full)"),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT, CW_USEDEFAULT, 500, 400,
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
    println!("║          Counter Application (Full)         ║");
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
    main_panel.set_spacing(25.0)?;
    main_panel.set_background(0xFF1A1A1A)?; // Dark theme
    main_panel.set_padding(35.0, 35.0, 35.0, 35.0)?;
    main_panel.set_corner_radius(14.0)?;

    // Title with modern styling
    let title = XamlTextBlock::new()?;
    title.set_text("🔢 Counter")?;
    title.set_font_size(32.0)?;
    title.set_font_weight(700)?; // Bold
    title.set_foreground(0xFFFFFFFF)?; // White
    title.set_margin(0.0, 0.0, 0.0, 15.0)?;
    main_panel.add_child(&title.as_uielement())?;

    // Counter display with modern styling
    let display = Arc::new(XamlTextBlock::new()?);
    display.set_text("Count: 0")?;
    display.set_font_size(48.0)?;
    display.set_font_weight(600)?; // SemiBold
    display.set_foreground(0xFF00D4FF)?; // Bright cyan
    display.set_margin(0.0, 20.0, 0.0, 30.0)?;
    main_panel.add_child(&display.as_uielement())?;

    // Increment/Decrement row
    let inc_dec_row = XamlStackPanel::new()?;
    inc_dec_row.set_vertical(false)?;
    inc_dec_row.set_spacing(10.0)?;

    // Decrement button with styling
    let dec_button = XamlButton::new()?;
    dec_button.set_content("➖")?;
    dec_button.set_size(110.0, 58.0)?;
    dec_button.set_background(0xFFE74856)?; // Red
    dec_button.set_foreground(0xFFFFFFFF)?;
    dec_button.set_corner_radius(10.0)?;
    dec_button.set_padding(14.0, 10.0, 14.0, 10.0)?;

    let counter_clone = counter.clone();
    let display_clone = Arc::clone(&display);
    dec_button.on_click(move || {
        let new_value = counter_clone.fetch_sub(1, Ordering::SeqCst) - 1;
        let _ = display_clone.set_text(&format!("Count: {}", new_value));
        println!("Decremented to: {}", new_value);
    })?;

    inc_dec_row.add_child(&dec_button.as_uielement())?;

    // Increment button with styling
    let inc_button = XamlButton::new()?;
    inc_button.set_content("➕")?;
    inc_button.set_size(110.0, 58.0)?;
    inc_button.set_background(0xFF107C10)?; // Green
    inc_button.set_foreground(0xFFFFFFFF)?;
    inc_button.set_corner_radius(10.0)?;
    inc_button.set_padding(14.0, 10.0, 14.0, 10.0)?;

    let counter_clone = counter.clone();
    let display_clone = Arc::clone(&display);
    inc_button.on_click(move || {
        let new_value = counter_clone.fetch_add(1, Ordering::SeqCst) + 1;
        let _ = display_clone.set_text(&format!("Count: {}", new_value));
        println!("Incremented to: {}", new_value);
    })?;

    inc_dec_row.add_child(&inc_button.as_uielement())?;

    main_panel.add_child(&inc_dec_row.as_uielement())?;

    // Reset/Double row
    let reset_double_row = XamlStackPanel::new()?;
    reset_double_row.set_vertical(false)?;
    reset_double_row.set_spacing(10.0)?;

    // Reset button with styling
    let reset_button = XamlButton::new()?;
    reset_button.set_content("🔄 Reset")?;
    reset_button.set_size(110.0, 58.0)?;
    reset_button.set_background(0xFF0078D4)?; // Blue
    reset_button.set_foreground(0xFFFFFFFF)?;
    reset_button.set_corner_radius(10.0)?;
    reset_button.set_padding(14.0, 10.0, 14.0, 10.0)?;

    let counter_clone = counter.clone();
    let display_clone = Arc::clone(&display);
    reset_button.on_click(move || {
        counter_clone.store(0, Ordering::SeqCst);
        let _ = display_clone.set_text("Count: 0");
        println!("Reset to 0");
    })?;

    reset_double_row.add_child(&reset_button.as_uielement())?;

    // Double button with styling
    let double_button = XamlButton::new()?;
    double_button.set_content("✖️ Double")?;
    double_button.set_size(110.0, 58.0)?;
    double_button.set_background(0xFFFF8C00)?; // Orange
    double_button.set_foreground(0xFFFFFFFF)?;
    double_button.set_corner_radius(10.0)?;
    double_button.set_padding(14.0, 10.0, 14.0, 10.0)?;

    let counter_clone = counter.clone();
    let display_clone = Arc::clone(&display);
    double_button.on_click(move || {
        let current = counter_clone.load(Ordering::SeqCst);
        let new_value = current * 2;
        counter_clone.store(new_value, Ordering::SeqCst);
        let _ = display_clone.set_text(&format!("Count: {}", new_value));
        println!("Doubled to: {}", new_value);
    })?;

    reset_double_row.add_child(&double_button.as_uielement())?;

    main_panel.add_child(&reset_double_row.as_uielement())?;

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

    println!("✅ Counter application started!\n");

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
