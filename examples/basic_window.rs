//! Basic window example demonstrating WinRT-XAML.
//!
//! This example shows how to create a simple window with basic controls using pure WinRT.

use winrt_xaml::error::Result;
use winrt_xaml::xaml_native::*;
use windows::core::w;
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::System::Com::{CoInitializeEx, CoUninitialize, COINIT_APARTMENTTHREADED};
use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};
use std::ptr;

fn create_host_window() -> Result<HWND> {
    unsafe {
        let class_name = w!("WinRT_BasicWindow");
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
            w!("Basic WinRT-XAML Window"),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT, CW_USEDEFAULT, 600, 400,
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
    println!("║      Basic WinRT-XAML Window Example        ║");
    println!("╚═══════════════════════════════════════════════╝\n");

    // Initialize COM for WinRT
    unsafe { CoInitializeEx(None, COINIT_APARTMENTTHREADED).ok(); }

    // Initialize XAML framework
    let _manager = XamlManager::new()?;

    // Create host window
    let host_hwnd = create_host_window()?;

    // Create XAML source and attach to window
    let mut xaml_source = XamlSource::new()?;
    let island_hwnd = xaml_source.attach_to_window(host_hwnd)?;

    // Create main layout panel
    let main_panel = XamlStackPanel::new()?;
    main_panel.set_vertical(true)?;
    main_panel.set_spacing(20.0)?;

    // Title
    let title = XamlTextBlock::new()?;
    title.set_text("Welcome to WinRT-XAML!")?;
    title.set_font_size(28.0)?;
    main_panel.add_child(&title.as_uielement())?;

    // Description
    let description = XamlTextBlock::new()?;
    description.set_text("This is a basic window with interactive buttons.")?;
    description.set_font_size(14.0)?;
    main_panel.add_child(&description.as_uielement())?;

    // Click counter button
    let click_button = XamlButton::new()?;
    click_button.set_content("Click Me!")?;
    click_button.set_size(200.0, 50.0)?;

    let click_count = Arc::new(AtomicUsize::new(0));
    let count_clone = click_count.clone();
    let status_text = Arc::new(XamlTextBlock::new()?);
    status_text.set_text("Not clicked yet")?;
    status_text.set_font_size(16.0)?;

    let status_clone = Arc::clone(&status_text);
    click_button.on_click(move || {
        let count = count_clone.fetch_add(1, Ordering::SeqCst) + 1;
        let message = format!("Button clicked {} time{}!", count, if count == 1 { "" } else { "s" });
        let _ = status_clone.set_text(&message);
        println!("{}", message);
    })?;

    main_panel.add_child(&click_button.as_uielement())?;
    main_panel.add_child(&status_text.as_uielement())?;

    // Exit button
    let exit_button = XamlButton::new()?;
    exit_button.set_content("Exit Application")?;
    exit_button.set_size(200.0, 50.0)?;

    exit_button.on_click(|| {
        println!("Exiting application...");
        std::process::exit(0);
    })?;

    main_panel.add_child(&exit_button.as_uielement())?;

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

    println!("✅ Application started!");
    println!("📊 Features:");
    println!("   • Interactive click counter");
    println!("   • Real-time status updates");
    println!("   • Fluent Design styling");
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
