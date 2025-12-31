//! Comprehensive WinRT Controls Demo
//!
//! This example demonstrates all available WinRT XAML controls:
//! - Button
//! - TextBlock
//! - TextBox
//! - StackPanel (layout)
//! - Grid (layout)

use winrt_xaml::error::Result;
use winrt_xaml::xaml_native::*;
use windows::core::w;
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::System::Com::{CoInitializeEx, CoUninitialize, COINIT_APARTMENTTHREADED};
use std::ptr;

// Helper to create a Win32 host window
fn create_host_window(title: &str) -> Result<HWND> {
    unsafe {
        let class_name = w!("WinRT_Controls_Demo");

        let wc = WNDCLASSW {
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(window_proc),
            hInstance: GetModuleHandleW(None)?.into(),
            lpszClassName: class_name,
            hCursor: LoadCursorW(None, IDC_ARROW)?,
            ..Default::default()
        };

        if RegisterClassW(&wc) == 0 {
            let last_error = windows::Win32::Foundation::GetLastError();
            if last_error.0 != 1410 { // ERROR_CLASS_ALREADY_EXISTS
                return Err(winrt_xaml::error::Error::window_creation(
                    format!("Failed to register window class: {:?}", last_error)
                ));
            }
        }

        let title_wide: Vec<u16> = title.encode_utf16().chain(Some(0)).collect();
        let hwnd = CreateWindowExW(
            WINDOW_EX_STYLE(0),
            class_name,
            windows::core::PCWSTR(title_wide.as_ptr()),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            900,
            700,
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
    println!("\n╔═══════════════════════════════════════════════════════════╗");
    println!("║         WinRT XAML Controls Demo - Full Showcase         ║");
    println!("╚═══════════════════════════════════════════════════════════╝\n");

    // Initialize COM
    unsafe {
        let hr = CoInitializeEx(None, COINIT_APARTMENTTHREADED);
        if hr.is_err() {
            return Err(winrt_xaml::error::Error::initialization(
                format!("Failed to initialize COM: {:?}", hr)
            ));
        }
    }

    // Initialize XAML
    println!("🔧 Initializing XAML framework...");
    let _manager = XamlManager::new()?;
    println!("   ✅ XAML framework initialized\n");

    // Create host window
    println!("🪟 Creating host window...");
    let host_hwnd = create_host_window("WinRT Controls Demo")?;
    println!("   ✅ Host window created\n");

    // Create XAML source
    println!("🌉 Creating XAML Islands source...");
    let mut xaml_source = XamlSource::new()?;
    let island_hwnd = xaml_source.attach_to_window(host_hwnd)?;
    println!("   ✅ XAML Island attached\n");

    // Build UI hierarchy
    println!("🎨 Building UI with multiple controls...");

    // Create main StackPanel
    let main_panel = XamlStackPanel::new()?;
    main_panel.set_vertical(true)?;
    main_panel.set_spacing(20.0)?;
    main_panel.set_background(0xFF1E1E1E)?; // Dark theme
    main_panel.set_padding(35.0, 35.0, 35.0, 35.0)?;
    main_panel.set_corner_radius(14.0)?;

    // Title with modern styling
    let title = XamlTextBlock::new()?;
    title.set_text("🎨 WinRT XAML Showcase")?;
    title.set_font_size(32.0)?;
    title.set_font_weight(700)?; // Bold
    title.set_foreground(0xFFFFFFFF)?; // White
    title.set_margin(0.0, 0.0, 0.0, 10.0)?;
    main_panel.add_child(&title.as_uielement())?;

    // Description with modern styling
    let description = XamlTextBlock::new()?;
    description.set_text("Real Windows.UI.Xaml controls through XAML Islands")?;
    description.set_font_size(16.0)?;
    description.set_foreground(0xFFB0B0B0)?; // Light gray
    description.set_margin(0.0, 0.0, 0.0, 25.0)?;
    main_panel.add_child(&description.as_uielement())?;

    // Button demo section
    let button_label = XamlTextBlock::new()?;
    button_label.set_text("📱 Button Control")?;
    button_label.set_font_size(18.0)?;
    button_label.set_font_weight(600)?; // SemiBold
    button_label.set_foreground(0xFF00D4FF)?; // Cyan
    button_label.set_margin(0.0, 10.0, 0.0, 8.0)?;
    main_panel.add_child(&button_label.as_uielement())?;

    let button = XamlButton::new()?;
    button.set_content("🎉 Click Me!")?;
    button.set_size(420.0, 56.0)?;
    button.set_background(0xFF0078D4)?; // Microsoft blue
    button.set_foreground(0xFFFFFFFF)?;
    button.set_corner_radius(8.0)?;
    button.set_padding(16.0, 12.0, 16.0, 12.0)?;
    main_panel.add_child(&button.as_uielement())?;

    // TextBox demo section
    let textbox_label = XamlTextBlock::new()?;
    textbox_label.set_text("📝 TextBox Control")?;
    textbox_label.set_font_size(18.0)?;
    textbox_label.set_font_weight(600)?; // SemiBold
    textbox_label.set_foreground(0xFF00D4FF)?; // Cyan
    textbox_label.set_margin(0.0, 15.0, 0.0, 8.0)?;
    main_panel.add_child(&textbox_label.as_uielement())?;

    let textbox = XamlTextBox::new()?;
    textbox.set_placeholder("Type something here...")?;
    textbox.set_size(420.0, 56.0)?; // Much larger to prevent clipping
    textbox.set_background(0xFF2D2D2D)?; // Dark input
    textbox.set_foreground(0xFFFFFFFF)?; // White text
    textbox.set_corner_radius(8.0)?;
    textbox.set_padding(12.0, 5.0, 12.0, 5.0)?; // Minimal vertical padding
    main_panel.add_child(&textbox.as_uielement())?;

    // Info text with modern styling
    let info = XamlTextBlock::new()?;
    info.set_text("✨ Native WinRT with Fluent Design")?;
    info.set_font_size(15.0)?;
    info.set_foreground(0xFF90EE90)?; // Light green
    info.set_margin(0.0, 20.0, 0.0, 0.0)?;
    main_panel.add_child(&info.as_uielement())?;

    // Set the main panel as content
    xaml_source.set_content_element(&main_panel.as_uielement())?;
    println!("   ✅ UI hierarchy built\n");

    // Show and size the island window
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

    println!("═══════════════════════════════════════════════════════════");
    println!("✅ SUCCESS! WinRT Controls Demo Running!");
    println!("═══════════════════════════════════════════════════════════");
    println!("\n📊 What you're seeing:");
    println!("   • TextBlock controls with custom font sizes");
    println!("   • Button with Fluent Design hover effects");
    println!("   • TextBox with placeholder text");
    println!("   • StackPanel layout with spacing");
    println!("   • All rendered through XAML Islands");
    println!("\n💡 Try interacting with the controls!");
    println!("🎬 Close the window to exit.\n");

    let result = run_message_loop();

    // Cleanup
    unsafe {
        CoUninitialize();
    }

    result
}

