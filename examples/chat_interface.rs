//! Chat interface example using WinRT.

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
        let class_name = w!("WinRT_ChatInterface");
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
            w!("Chat Interface"),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT, CW_USEDEFAULT, 500, 700,
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
    println!("║            Chat Interface                    ║");
    println!("╚═══════════════════════════════════════════════╝\n");

    println!("Creating chat interface...");

    // Initialize COM
    unsafe { CoInitializeEx(None, COINIT_APARTMENTTHREADED).ok(); }

    // Initialize XAML framework
    let _manager = XamlManager::new()?;

    // Create host window
    let host_hwnd = create_host_window()?;

    // Create XAML source
    let mut xaml_source = XamlSource::new()?;
    let island_hwnd = xaml_source.attach_to_window(host_hwnd)?;

    // Message storage
    let messages = Arc::new(Mutex::new(Vec::<String>::new()));

    // Create main layout panel with modern styling
    let main_panel = XamlStackPanel::new()?;
    main_panel.set_vertical(true)?;
    main_panel.set_spacing(20.0)?;
    main_panel.set_background(0xFF202020)?; // Dark gray background
    main_panel.set_padding(30.0, 30.0, 30.0, 30.0)?;
    main_panel.set_corner_radius(12.0)?; // Rounded corners

    // Title with modern styling
    let title = XamlTextBlock::new()?;
    title.set_text("💬 Chat Room")?;
    title.set_font_size(32.0)?;
    title.set_font_weight(700)?; // Bold
    title.set_foreground(0xFFFFFFFF)?; // White text
    title.set_margin(0.0, 0.0, 0.0, 10.0)?;
    main_panel.add_child(&title.as_uielement())?;

    // Messages display with stylish container
    let messages_display = Arc::new(XamlTextBlock::new()?);
    messages_display.set_text("No messages yet\n\nType a message below and click Send!")?;
    messages_display.set_font_size(15.0)?;
    messages_display.set_foreground(0xFFE0E0E0)?; // Light gray text
    messages_display.set_margin(0.0, 10.0, 0.0, 10.0)?;
    main_panel.add_child(&messages_display.as_uielement())?;

    // Spacer
    let spacer = XamlTextBlock::new()?;
    spacer.set_text("")?;
    main_panel.add_child(&spacer.as_uielement())?;

    // Message input label with modern styling
    let input_label = XamlTextBlock::new()?;
    input_label.set_text("📝 Your message:")?;
    input_label.set_font_size(16.0)?;
    input_label.set_font_weight(600)?; // SemiBold
    input_label.set_foreground(0xFFFFFFFF)?; // White text
    input_label.set_margin(0.0, 10.0, 0.0, 5.0)?;
    main_panel.add_child(&input_label.as_uielement())?;

    // Message input with modern styling
    let message_input = Arc::new(XamlTextBox::new()?);
    message_input.set_placeholder("Type a message...")?;
    message_input.set_size(450.0, 60.0)?; // Much larger height to prevent clipping
    message_input.set_background(0xFF2D2D2D)?; // Slightly lighter gray
    message_input.set_foreground(0xFFFFFFFF)?; // White text
    message_input.set_corner_radius(8.0)?; // Rounded corners
    message_input.set_padding(12.0, 5.0, 12.0, 5.0)?; // Minimal vertical padding
    main_panel.add_child(&message_input.as_uielement())?;

    // Button row
    let button_row = XamlStackPanel::new()?;
    button_row.set_vertical(false)?;
    button_row.set_spacing(15.0)?;
    button_row.set_padding(0.0, 15.0, 0.0, 0.0)?;

    // Send button with vibrant blue styling
    let send_button = XamlButton::new()?;
    send_button.set_content("📤 Send")?;
    send_button.set_size(170.0, 48.0)?;
    send_button.set_background(0xFF0078D4)?; // Microsoft blue
    send_button.set_foreground(0xFFFFFFFF)?; // White text
    send_button.set_corner_radius(8.0)?; // Rounded corners
    send_button.set_padding(16.0, 12.0, 16.0, 12.0)?;

    let messages_clone = Arc::clone(&messages);
    let input_clone = Arc::clone(&message_input);
    let display_clone = Arc::clone(&messages_display);
    send_button.on_click(move || {
        // Get the actual text from the input field
        match input_clone.get_text() {
            Ok(text) => {
                // Only send if there's actual text
                let trimmed = text.trim();
                if !trimmed.is_empty() {
                    let mut msgs = messages_clone.lock().unwrap();
                    let new_message = format!("You: {}", trimmed);
                    msgs.push(new_message.clone());

                    // Show last 8 messages
                    let recent: Vec<_> = msgs.iter().rev().take(8).rev().cloned().collect();
                    let display_text = if recent.is_empty() {
                        "No messages yet".to_string()
                    } else {
                        recent.join("\n")
                    };
                    let _ = display_clone.set_text(&display_text);

                    // Clear the input field
                    let _ = input_clone.set_text("");

                    println!("✓ Sent: {}", new_message);
                }
            }
            Err(e) => {
                println!("✗ Error getting text: {}", e);
            }
        }
    })?;

    button_row.add_child(&send_button.as_uielement())?;

    // Clear button with stylish red accent
    let clear_button = XamlButton::new()?;
    clear_button.set_content("🗑️ Clear")?;
    clear_button.set_size(170.0, 48.0)?;
    clear_button.set_background(0xFF8B0000)?; // Dark red
    clear_button.set_foreground(0xFFFFFFFF)?; // White text
    clear_button.set_corner_radius(8.0)?; // Rounded corners
    clear_button.set_padding(16.0, 12.0, 16.0, 12.0)?;

    let messages_clone = Arc::clone(&messages);
    let display_clone = Arc::clone(&messages_display);
    clear_button.on_click(move || {
        messages_clone.lock().unwrap().clear();
        let _ = display_clone.set_text("No messages yet\n\nType a message below and click Send!");
        println!("✓ Chat cleared");
    })?;

    button_row.add_child(&clear_button.as_uielement())?;

    main_panel.add_child(&button_row.as_uielement())?;

    // Info
    let info = XamlTextBlock::new()?;
    info.set_text("📝 Simplified chat demo - no networking")?;
    info.set_font_size(12.0)?;
    main_panel.add_child(&info.as_uielement())?;

    // Set content
    xaml_source.set_content_element(&main_panel.as_uielement())?;

    // Show and size the island
    unsafe {
        let _ = ShowWindow(island_hwnd, SW_SHOW);
        let mut rect = windows::Win32::Foundation::RECT::default();
        let _ = GetClientRect(host_hwnd, &mut rect);
        let _ = SetWindowPos(island_hwnd, None, 0, 0,
            rect.right - rect.left, rect.bottom - rect.top,
            SWP_NOZORDER | SWP_NOACTIVATE);
    }

    println!("✅ Chat interface started!");
    println!("📊 Features:");
    println!("   • Send messages");
    println!("   • View message history");
    println!("   • Clear chat");
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
