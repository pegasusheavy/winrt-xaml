//! Form demonstration with input validation using WinRT.

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
        let class_name = w!("WinRT_FormDemo");
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
            w!("User Registration Form"),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT, CW_USEDEFAULT, 500, 550,
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
    println!("║          User Registration Form              ║");
    println!("╚═══════════════════════════════════════════════╝\n");

    println!("Creating form demo...");

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
    main_panel.set_spacing(15.0)?;

    // Title
    let title = XamlTextBlock::new()?;
    title.set_text("User Registration Form")?;
    title.set_font_size(28.0)?;
    main_panel.add_child(&title.as_uielement())?;

    // Name field
    let name_label = XamlTextBlock::new()?;
    name_label.set_text("Name:")?;
    name_label.set_font_size(14.0)?;
    main_panel.add_child(&name_label.as_uielement())?;

    let name_input = XamlTextBox::new()?;
    name_input.set_placeholder("Enter your name")?;
    name_input.set_size(350.0, 32.0)?;
    main_panel.add_child(&name_input.as_uielement())?;

    // Email field
    let email_label = XamlTextBlock::new()?;
    email_label.set_text("Email:")?;
    email_label.set_font_size(14.0)?;
    main_panel.add_child(&email_label.as_uielement())?;

    let email_input = XamlTextBox::new()?;
    email_input.set_placeholder("Enter your email")?;
    email_input.set_size(350.0, 32.0)?;
    main_panel.add_child(&email_input.as_uielement())?;

    // Age field
    let age_label = XamlTextBlock::new()?;
    age_label.set_text("Age:")?;
    age_label.set_font_size(14.0)?;
    main_panel.add_child(&age_label.as_uielement())?;

    let age_input = XamlTextBox::new()?;
    age_input.set_placeholder("Enter your age")?;
    age_input.set_size(350.0, 32.0)?;
    main_panel.add_child(&age_input.as_uielement())?;

    // Submit button
    let submit_button = XamlButton::new()?;
    submit_button.set_content("Submit Form")?;
    submit_button.set_size(200.0, 50.0)?;

    submit_button.on_click(|| {
        println!("✓ Form submitted!");
        println!("  (In a real app, would validate and process form data)");
    })?;

    main_panel.add_child(&submit_button.as_uielement())?;

    // Status text
    let status = XamlTextBlock::new()?;
    status.set_text("Fill out the form and click Submit")?;
    status.set_font_size(12.0)?;
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

    println!("✅ Form demo started!");
    println!("📊 Demonstrating:");
    println!("   • Multiple TextBox inputs");
    println!("   • Form layout with labels");
    println!("   • Submit button");
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
