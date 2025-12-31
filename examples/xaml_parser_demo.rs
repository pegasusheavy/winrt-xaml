//! XAML Parser Demo
//!
//! This example demonstrates parsing XAML markup to create UI elements.
//! It shows how to define UI layouts in XAML strings and load them dynamically.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use windows::core::w;
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::System::Com::{CoInitializeEx, CoUninitialize, COINIT_APARTMENTTHREADED};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::WindowsAndMessaging::*;
use winrt_xaml::prelude::*;
use winrt_xaml::xaml::XamlReader;
use winrt_xaml::xaml_native::*;

fn main() -> Result<()> {
    println!("🎨 XAML Parser Demo");
    println!("==================\n");

    // Initialize COM
    unsafe {
        CoInitializeEx(None, COINIT_APARTMENTTHREADED).ok();
    }

    // Initialize XAML
    let _manager = XamlManager::new()?;
    println!("✅ XAML Manager initialized");

    // Create the host window
    let host_hwnd = create_host_window()?;
    println!("✅ Host window created");

    // Create XAML source
    let mut xaml_source = XamlSource::new()?;
    let island_hwnd = xaml_source.attach_to_window(host_hwnd)?;
    println!("✅ XAML Island attached\n");

    // Example 1: Simple button from XAML
    println!("📝 Example 1: Parsing a simple button");
    let button_xaml = r#"
        <Button Content="Click Me!" Width="200" Height="50" />
    "#;

    match XamlReader::parse(button_xaml) {
        Ok(_element) => {
            println!("   ✅ Successfully parsed button from XAML");
            // In a real app, you would use the element here
        }
        Err(e) => println!("   ❌ Error: {}", e),
    }

    // Example 2: TextBlock with properties
    println!("\n📝 Example 2: Parsing a TextBlock");
    let textblock_xaml = r#"
        <TextBlock Text="Hello from XAML!" FontSize="24" Width="300" Height="40" />
    "#;

    match XamlReader::parse(textblock_xaml) {
        Ok(_) => println!("   ✅ Successfully parsed TextBlock from XAML"),
        Err(e) => println!("   ❌ Error: {}", e),
    }

    // Example 3: StackPanel with orientation
    println!("\n📝 Example 3: Parsing a StackPanel");
    let panel_xaml = r#"
        <StackPanel Orientation="Vertical" Spacing="10" />
    "#;

    match XamlReader::parse(panel_xaml) {
        Ok(_) => println!("   ✅ Successfully parsed StackPanel from XAML"),
        Err(e) => println!("   ❌ Error: {}", e),
    }

    // Example 4: Build a complete UI from XAML
    println!("\n📝 Example 4: Building a complete UI from XAML");
    println!("   Building calculator UI...\n");

    // Build the UI using WinRT XAML directly (since nested parsing isn't implemented yet)
    let main_panel = XamlStackPanel::new()?;
    main_panel.set_vertical(true)?;
    main_panel.set_spacing(20.0)?;
    main_panel.set_background(0xFF1A1A1A)?;
    main_panel.set_padding(30.0, 30.0, 30.0, 30.0)?;

    // Title
    let title = XamlTextBlock::new()?;
    title.set_text("XAML Parser Demo")?;
    title.set_font_size(32.0)?;
    title.set_font_weight(700)?;
    title.set_foreground(0xFF00D7FF)?;
    title.set_margin(0.0, 0.0, 0.0, 20.0)?;
    main_panel.add_child(&title.as_uielement())?;

    // Description
    let desc = XamlTextBlock::new()?;
    desc.set_text("This demo shows XAML parsing capabilities.\nClick buttons below to test different XAML snippets.")?;
    desc.set_font_size(14.0)?;
    desc.set_foreground(0xFFCCCCCC)?;
    desc.set_margin(0.0, 0.0, 0.0, 20.0)?;
    main_panel.add_child(&desc.as_uielement())?;

    // Example buttons
    let examples = vec![
        (
            "Parse Button",
            r#"<Button Content="Hello" Width="200" Height="50" />"#,
        ),
        (
            "Parse TextBlock",
            r#"<TextBlock Text="Hello XAML" FontSize="24" />"#,
        ),
        (
            "Parse TextBox",
            r#"<TextBox PlaceholderText="Enter text..." Width="300" Height="40" />"#,
        ),
        (
            "Parse CheckBox",
            r#"<CheckBox Content="I agree" IsChecked="true" />"#,
        ),
        (
            "Parse StackPanel",
            r#"<StackPanel Orientation="Horizontal" Spacing="15" />"#,
        ),
    ];

    let running = Arc::new(AtomicBool::new(true));

    for (label, xaml) in examples {
        let button = XamlButton::new()?;
        button.set_content(label)?;
        button.set_size(400.0, 50.0)?;
        button.set_background(0xFF0078D4)?;
        button.set_foreground(0xFFFFFFFF)?;
        button.set_corner_radius(8.0)?;
        button.set_padding(20.0, 10.0, 20.0, 10.0)?;

        let xaml_str = xaml.to_string();
        let running_clone = running.clone();
        button.on_click(move || {
            if running_clone.load(Ordering::Relaxed) {
                println!("\n🔍 Parsing XAML:");
                println!("   {}", xaml_str.trim());
                match XamlReader::parse(&xaml_str) {
                    Ok(_) => println!("   ✅ Parse successful!"),
                    Err(e) => println!("   ❌ Parse failed: {}", e),
                }
            }
        })?;

        main_panel.add_child(&button.as_uielement())?;
    }

    // Status text
    let status = XamlTextBlock::new()?;
    status.set_text("Click any button above to parse XAML")?;
    status.set_font_size(12.0)?;
    status.set_foreground(0xFF888888)?;
    status.set_margin(0.0, 20.0, 0.0, 0.0)?;
    main_panel.add_child(&status.as_uielement())?;

    // Exit button
    let exit_btn = XamlButton::new()?;
    exit_btn.set_content("Exit Demo")?;
    exit_btn.set_size(400.0, 50.0)?;
    exit_btn.set_background(0xFFDC3545)?;
    exit_btn.set_foreground(0xFFFFFFFF)?;
    exit_btn.set_corner_radius(8.0)?;

    let running_clone = running.clone();
    exit_btn.on_click(move || {
        println!("\n👋 Exiting XAML Parser Demo");
        running_clone.store(false, Ordering::Relaxed);
        unsafe {
            PostQuitMessage(0);
        }
    })?;
    main_panel.add_child(&exit_btn.as_uielement())?;

    // Set content
    xaml_source.set_content_element(&main_panel.as_uielement())?;

    // Show windows
    unsafe {
        ShowWindow(island_hwnd, SW_SHOW);
        ShowWindow(host_hwnd, SW_SHOW);
    }

    println!("\n✨ UI loaded successfully!");
    println!("💡 Click buttons to test XAML parsing\n");

    // Message loop
    let mut msg = MSG::default();
    unsafe {
        while GetMessageW(&mut msg, HWND(std::ptr::null_mut()), 0, 0).as_bool() {
            let _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }

    // Cleanup
    unsafe {
        CoUninitialize();
    }

    Ok(())
}

fn create_host_window() -> Result<HWND> {
    unsafe {
        let hinstance = GetModuleHandleW(None)?;

        let class_name = w!("XamlParserDemoWindow");

        let wc = WNDCLASSW {
            lpfnWndProc: Some(window_proc),
            hInstance: hinstance.into(),
            lpszClassName: class_name,
            style: CS_HREDRAW | CS_VREDRAW,
            hCursor: LoadCursorW(None, IDC_ARROW)?,
            ..Default::default()
        };

        RegisterClassW(&wc);

        let hwnd = CreateWindowExW(
            WINDOW_EX_STYLE::default(),
            class_name,
            w!("XAML Parser Demo - WinRT XAML"),
            WS_OVERLAPPEDWINDOW,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            500,
            900,
            None,
            None,
            hinstance,
            None,
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
        WM_DESTROY => {
            PostQuitMessage(0);
            LRESULT(0)
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}
