//! Settings panel demonstrating configuration UI with WinRT.

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
        let class_name = w!("WinRT_SettingsPanel");
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
            w!("Application Settings"),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT, CW_USEDEFAULT, 500, 600,
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
    println!("║            Settings Panel                    ║");
    println!("╚═══════════════════════════════════════════════╝\n");

    println!("Creating settings panel...");

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
    main_panel.set_spacing(20.0)?;

    // Title
    let title = XamlTextBlock::new()?;
    title.set_text("Application Settings")?;
    title.set_font_size(28.0)?;
    main_panel.add_child(&title.as_uielement())?;

    // General Settings section
    let general_header = XamlTextBlock::new()?;
    general_header.set_text("General Settings")?;
    general_header.set_font_size(18.0)?;
    main_panel.add_child(&general_header.as_uielement())?;

    // Username setting
    let username_label = XamlTextBlock::new()?;
    username_label.set_text("Username:")?;
    username_label.set_font_size(14.0)?;
    main_panel.add_child(&username_label.as_uielement())?;

    let username_input = XamlTextBox::new()?;
    username_input.set_placeholder("Enter username")?;
    username_input.set_size(300.0, 32.0)?;
    main_panel.add_child(&username_input.as_uielement())?;

    // Theme setting
    let theme_label = XamlTextBlock::new()?;
    theme_label.set_text("Theme:")?;
    theme_label.set_font_size(14.0)?;
    main_panel.add_child(&theme_label.as_uielement())?;

    let theme_row = XamlStackPanel::new()?;
    theme_row.set_vertical(false)?;
    theme_row.set_spacing(10.0)?;

    let light_button = XamlButton::new()?;
    light_button.set_content("Light")?;
    light_button.set_size(100.0, 40.0)?;
    light_button.on_click(|| println!("✓ Light theme selected"))?;
    theme_row.add_child(&light_button.as_uielement())?;

    let dark_button = XamlButton::new()?;
    dark_button.set_content("Dark")?;
    dark_button.set_size(100.0, 40.0)?;
    dark_button.on_click(|| println!("✓ Dark theme selected"))?;
    theme_row.add_child(&dark_button.as_uielement())?;

    main_panel.add_child(&theme_row.as_uielement())?;

    // Save button
    let status = Arc::new(XamlTextBlock::new()?);
    status.set_text("Make changes and click Save")?;
    status.set_font_size(12.0)?;

    let save_button = XamlButton::new()?;
    save_button.set_content("Save Settings")?;
    save_button.set_size(200.0, 50.0)?;

    let status_clone = Arc::clone(&status);
    save_button.on_click(move || {
        let _ = status_clone.set_text("✓ Settings saved successfully!");
        println!("✓ Settings saved");
    })?;

    main_panel.add_child(&save_button.as_uielement())?;
    main_panel.add_child(&status.as_uielement())?;

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

    println!("✅ Settings panel started!");
    println!("📊 Features:");
    println!("   • Username configuration");
    println!("   • Theme selection");
    println!("   • Save settings");
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
