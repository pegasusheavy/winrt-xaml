//! Color picker demonstrating interactive color selection with WinRT.

use winrt_xaml::error::Result;
use winrt_xaml::xaml_native::*;
use windows::core::w;
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::System::Com::{CoInitializeEx, CoUninitialize, COINIT_APARTMENTTHREADED};
use std::sync::Arc;
use std::ptr;

fn create_host_window() -> Result<HWND> {
    unsafe {
        let class_name = w!("WinRT_ColorPicker");
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
            w!("Color Picker"),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT, CW_USEDEFAULT, 450, 650,
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
    println!("║             Color Picker                     ║");
    println!("╚═══════════════════════════════════════════════╝\n");

    println!("Creating color picker...");

    // Initialize COM
    unsafe { CoInitializeEx(None, COINIT_APARTMENTTHREADED).ok(); }

    // Initialize XAML framework
    let _manager = XamlManager::new()?;

    // Create host window
    let host_hwnd = create_host_window()?;

    // Create XAML source
    let mut xaml_source = XamlSource::new()?;
    let island_hwnd = xaml_source.attach_to_window(host_hwnd)?;

    // Create main layout panel
    let main_panel = XamlStackPanel::new()?;
    main_panel.set_vertical(true)?;
    main_panel.set_spacing(22.0)?;
    main_panel.set_background(0xFF1A1A1A)?; // Dark theme
    main_panel.set_padding(35.0, 35.0, 35.0, 35.0)?;
    main_panel.set_corner_radius(14.0)?;

    // Title with modern styling
    let title = XamlTextBlock::new()?;
    title.set_text("🎨 Color Picker")?;
    title.set_font_size(32.0)?;
    title.set_font_weight(700)?; // Bold
    title.set_foreground(0xFFFFFFFF)?; // White
    title.set_margin(0.0, 0.0, 0.0, 12.0)?;
    main_panel.add_child(&title.as_uielement())?;

    // Description with styling
    let desc = XamlTextBlock::new()?;
    desc.set_text("Select a color:")?;
    desc.set_font_size(18.0)?;
    desc.set_foreground(0xFFAAAAAA)?; // Gray
    desc.set_margin(0.0, 0.0, 0.0, 15.0)?;
    main_panel.add_child(&desc.as_uielement())?;

    // Color display
    let color_display = Arc::new(XamlTextBlock::new()?);
    color_display.set_text("No color selected")?;
    color_display.set_font_size(20.0)?;
    color_display.set_font_weight(600)?; // SemiBold
    color_display.set_foreground(0xFF00D4FF)?; // Cyan
    color_display.set_margin(0.0, 10.0, 0.0, 15.0)?;
    main_panel.add_child(&color_display.as_uielement())?;

    // Color buttons
    let colors = vec![
        ("Red", "#FF0000"),
        ("Green", "#00FF00"),
        ("Blue", "#0000FF"),
        ("Yellow", "#FFFF00"),
        ("Purple", "#800080"),
        ("Orange", "#FFA500"),
    ];

    for (name, hex) in colors {
        let color_row = XamlStackPanel::new()?;
        color_row.set_vertical(false)?;
        color_row.set_spacing(10.0)?;

        let color_button = XamlButton::new()?;
        color_button.set_content(name)?;
        color_button.set_size(160.0, 50.0)?;
        color_button.set_background(0xFF0078D4)?; // Blue
        color_button.set_foreground(0xFFFFFFFF)?;
        color_button.set_corner_radius(10.0)?;
        color_button.set_padding(16.0, 10.0, 16.0, 10.0)?;

        let display_clone = Arc::clone(&color_display);
        let color_name = name.to_string();
        let color_hex = hex.to_string();
        color_button.on_click(move || {
            let message = format!("Selected: {} ({})", color_name, color_hex);
            let _ = display_clone.set_text(&message);
            println!("✓ Color selected: {} ({})", color_name, color_hex);
        })?;

        color_row.add_child(&color_button.as_uielement())?;

        let hex_label = XamlTextBlock::new()?;
        hex_label.set_text(hex)?;
        hex_label.set_font_size(16.0)?;
        hex_label.set_foreground(0xFFFFFFFF)?; // White
        color_row.add_child(&hex_label.as_uielement())?;

        main_panel.add_child(&color_row.as_uielement())?;
    }

    // Reset button
    let reset_button = XamlButton::new()?;
    reset_button.set_content("🔄 Reset Selection")?;
    reset_button.set_size(220.0, 52.0)?;
    reset_button.set_background(0xFFE74856)?; // Red
    reset_button.set_foreground(0xFFFFFFFF)?;
    reset_button.set_corner_radius(10.0)?;
    reset_button.set_padding(16.0, 10.0, 16.0, 10.0)?;

    let display_clone = Arc::clone(&color_display);
    reset_button.on_click(move || {
        let _ = display_clone.set_text("No color selected");
        println!("✓ Selection reset");
    })?;

    main_panel.add_child(&reset_button.as_uielement())?;

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

    println!("✅ Color picker started!");
    println!("📊 Features:");
    println!("   • 6 predefined colors");
    println!("   • Real-time color display");
    println!("   • Reset selection");
    println!("🎬 Close window to exit\n");

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
