//! Todo application demonstrating list management with WinRT.

use winrt_xaml::error::Result;
use winrt_xaml::xaml_native::*;
use windows::core::w;
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::System::Com::{CoInitializeEx, CoUninitialize, COINIT_APARTMENTTHREADED};
use std::sync::{Arc, Mutex};
use std::ptr;

fn create_host_window() -> Result<HWND> {
    unsafe {
        let class_name = w!("WinRT_TodoApp");
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
            w!("Todo Application"),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT, CW_USEDEFAULT, 450, 500,
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
    println!("║              Todo Application                ║");
    println!("╚═══════════════════════════════════════════════╝\n");

    println!("Creating todo app...");

    // Initialize COM
    unsafe { CoInitializeEx(None, COINIT_APARTMENTTHREADED).ok(); }

    // Initialize XAML framework
    let _manager = XamlManager::new()?;

    // Create host window
    let host_hwnd = create_host_window()?;

    // Create XAML source
    let mut xaml_source = XamlSource::new()?;
    let island_hwnd = xaml_source.attach_to_window(host_hwnd)?;

    // Todo state
    let todo_count = Arc::new(Mutex::new(0));

    // Create main layout panel
    let main_panel = XamlStackPanel::new()?;
    main_panel.set_vertical(true)?;
    main_panel.set_spacing(15.0)?;

    // Title
    let title = XamlTextBlock::new()?;
    title.set_text("Todo List")?;
    title.set_font_size(28.0)?;
    main_panel.add_child(&title.as_uielement())?;

    // Input field
    let input = Arc::new(XamlTextBox::new()?);
    input.set_placeholder("Enter a new todo...")?;
    input.set_size(350.0, 32.0)?;
    main_panel.add_child(&input.as_uielement())?;

    // Add button
    let add_button = XamlButton::new()?;
    add_button.set_content("Add Todo")?;
    add_button.set_size(150.0, 40.0)?;

    let count_clone = Arc::clone(&todo_count);
    let status_text = Arc::new(XamlTextBlock::new()?);
    status_text.set_text("No todos yet")?;
    status_text.set_font_size(14.0)?;

    let status_clone = Arc::clone(&status_text);
    add_button.on_click(move || {
        let mut count = count_clone.lock().unwrap();
        *count += 1;
        let message = format!("{} todo{} added", *count, if *count == 1 { "" } else { "s" });
        let _ = status_clone.set_text(&message);
        println!("✓ Todo added ({})", *count);
    })?;

    main_panel.add_child(&add_button.as_uielement())?;

    // Status text
    main_panel.add_child(&status_text.as_uielement())?;

    // Clear button
    let clear_button = XamlButton::new()?;
    clear_button.set_content("Clear All")?;
    clear_button.set_size(150.0, 40.0)?;

    let count_clone = Arc::clone(&todo_count);
    let status_clone = Arc::clone(&status_text);
    clear_button.on_click(move || {
        let mut count = count_clone.lock().unwrap();
        *count = 0;
        let _ = status_clone.set_text("All todos cleared");
        println!("✓ All todos cleared");
    })?;

    main_panel.add_child(&clear_button.as_uielement())?;

    // Info
    let info = XamlTextBlock::new()?;
    info.set_text("📝 Simplified todo demo - demonstrates button interactions")?;
    info.set_font_size(12.0)?;
    main_panel.add_child(&info.as_uielement())?;

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

    println!("✅ Todo app started!");
    println!("📊 Features:");
    println!("   • Add todo items");
    println!("   • Track todo count");
    println!("   • Clear all todos");
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
