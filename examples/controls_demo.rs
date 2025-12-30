//! Controls demonstration showing various WinRT UI controls.

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
        let class_name = w!("WinRT_ControlsDemo");
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
            w!("WinRT Controls Demo"),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT, CW_USEDEFAULT, 600, 700,
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
    println!("║          WinRT Controls Demo                ║");
    println!("╚═══════════════════════════════════════════════╝\n");

    println!("Creating controls demo...");

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
    title.set_text("UI Controls Demonstration")?;
    title.set_font_size(28.0)?;
    main_panel.add_child(&title.as_uielement())?;

    // Description
    let desc = XamlTextBlock::new()?;
    desc.set_text("Showcasing available WinRT XAML controls:")?;
    desc.set_font_size(14.0)?;
    main_panel.add_child(&desc.as_uielement())?;

    // Button demo
    let button_label = XamlTextBlock::new()?;
    button_label.set_text("Button Control:")?;
    button_label.set_font_size(16.0)?;
    main_panel.add_child(&button_label.as_uielement())?;

    let button = XamlButton::new()?;
    button.set_content("Click Me!")?;
    button.set_size(200.0, 40.0)?;

    let status = Arc::new(XamlTextBlock::new()?);
    status.set_text("Not clicked yet")?;
    status.set_font_size(12.0)?;

    let status_clone = Arc::clone(&status);
    button.on_click(move || {
        let _ = status_clone.set_text("Button clicked! ✓");
        println!("Button clicked!");
    })?;

    main_panel.add_child(&button.as_uielement())?;
    main_panel.add_child(&status.as_uielement())?;

    // TextBox demo
    let textbox_label = XamlTextBlock::new()?;
    textbox_label.set_text("TextBox Control:")?;
    textbox_label.set_font_size(16.0)?;
    main_panel.add_child(&textbox_label.as_uielement())?;

    let textbox = XamlTextBox::new()?;
    textbox.set_placeholder("Enter text here...")?;
    textbox.set_size(300.0, 32.0)?;
    main_panel.add_child(&textbox.as_uielement())?;

    // TextBlock demo
    let textblock_label = XamlTextBlock::new()?;
    textblock_label.set_text("TextBlock Control (various sizes):")?;
    textblock_label.set_font_size(16.0)?;
    main_panel.add_child(&textblock_label.as_uielement())?;

    let small_text = XamlTextBlock::new()?;
    small_text.set_text("Small text (12pt)")?;
    small_text.set_font_size(12.0)?;
    main_panel.add_child(&small_text.as_uielement())?;

    let medium_text = XamlTextBlock::new()?;
    medium_text.set_text("Medium text (16pt)")?;
    medium_text.set_font_size(16.0)?;
    main_panel.add_child(&medium_text.as_uielement())?;

    let large_text = XamlTextBlock::new()?;
    large_text.set_text("Large text (24pt)")?;
    large_text.set_font_size(24.0)?;
    main_panel.add_child(&large_text.as_uielement())?;

    // Layout demo
    let layout_label = XamlTextBlock::new()?;
    layout_label.set_text("StackPanel Layout (horizontal):")?;
    layout_label.set_font_size(16.0)?;
    main_panel.add_child(&layout_label.as_uielement())?;

    let h_panel = XamlStackPanel::new()?;
    h_panel.set_vertical(false)?;
    h_panel.set_spacing(10.0)?;

    for i in 1..=3 {
        let btn = XamlButton::new()?;
        btn.set_content(&format!("Btn {}", i))?;
        btn.set_size(80.0, 40.0)?;
        let idx = i;
        btn.on_click(move || println!("Button {} clicked", idx))?;
        h_panel.add_child(&btn.as_uielement())?;
    }

    main_panel.add_child(&h_panel.as_uielement())?;

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

    println!("✅ Controls demo started!");
    println!("📊 Showcasing:");
    println!("   • XamlButton with click events");
    println!("   • XamlTextBox with placeholder");
    println!("   • XamlTextBlock with various sizes");
    println!("   • XamlStackPanel layouts (vertical & horizontal)\n");

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
