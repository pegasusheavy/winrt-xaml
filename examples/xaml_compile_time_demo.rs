//! Compile-time XAML parsing demo.
//!
//! This example demonstrates the `xaml!` macro which parses XAML at compile time
//! and generates Rust code to create WinRT controls. This provides:
//!
//! - **Compile-time validation**: XAML errors are caught during compilation
//! - **Zero runtime overhead**: No parsing at runtime
//! - **Type safety**: Generated code is fully typed
//! - **IDE support**: Full autocomplete and error checking
//!
//! Run with: cargo run --example xaml_compile_time_demo

#![allow(unused)]

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use winrt_xaml::error::Result;
use winrt_xaml::xaml; // The compile-time macro!
use winrt_xaml::xaml_native::*;
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::System::Com::{CoInitializeEx, CoUninitialize, COINIT_APARTMENTTHREADED};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::WindowsAndMessaging::*;

fn main() -> Result<()> {
    unsafe {
        CoInitializeEx(None, COINIT_APARTMENTTHREADED).ok();
    }

    println!("\n╔═══════════════════════════════════════════════════════════════════════════╗");
    println!("║      🚀 COMPILE-TIME XAML DEMO 🚀                                      ║");
    println!("╚═══════════════════════════════════════════════════════════════════════════╝\n");

    println!("This example uses the xaml! macro to parse XAML at COMPILE TIME!");
    println!("All XAML is validated when you run 'cargo build'.\n");

    // Initialize XAML
    let _manager = XamlManager::new()?;

    // Create host window
    let hwnd = create_window("Compile-Time XAML Demo", 500, 700)?;

    // Create XAML source
    let mut xaml_source = XamlSource::new()?;
    xaml_source.attach_to_window(hwnd)?;

    // Create UI using compile-time XAML
    create_ui(&xaml_source, hwnd)?;

    unsafe {
        ShowWindow(hwnd, SW_SHOW);
    }

    // Message loop
    unsafe {
        let mut msg = MSG::default();
        while GetMessageW(&mut msg, None, 0, 0).into() {
            let _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }

    unsafe {
        CoUninitialize();
    }

    Ok(())
}

fn create_ui(xaml_source: &XamlSource, hwnd: HWND) -> Result<()> {
    let main_panel = XamlStackPanel::new()?;
    main_panel.set_vertical(true)?;
    main_panel.set_spacing(15.0)?;
    main_panel.set_background(0xFF1A1A1A)?;
    main_panel.set_padding(20.0, 20.0, 20.0, 20.0)?;

    // ============================================================
    // COMPILE-TIME XAML EXAMPLES
    // ============================================================

    // Example 1: Button created at compile time
    println!("📝 Example 1: Compile-time Button");
    let button1 = xaml! {
        r##"<Button Content="Compile-Time Button"
                  Width="400"
                  Height="60"
                  Background="#FF0078D4"
                  Foreground="#FFFFFFFF"
                  CornerRadius="10" />"##
    }?;
    main_panel.add_child(&button1)?;
    println!("   ✅ Created at compile time!\n");

    // Example 2: TextBlock created at compile time
    println!("📝 Example 2: Compile-time TextBlock");
    let label = xaml! {
        r##"<TextBlock Text="This UI was generated at compile time!"
                     FontSize="18"
                     FontWeight="700"
                     Foreground="#FF00D7FF" />"##
    }?;
    main_panel.add_child(&label)?;
    println!("   ✅ Created at compile time!\n");

    // Example 3: TextBox created at compile time
    println!("📝 Example 3: Compile-time TextBox");
    let textbox = xaml! {
        r##"<TextBox PlaceholderText="Enter text... (compile-time!)"
                   Width="400"
                   Height="56"
                   Background="#FF2D2D2D"
                   Foreground="#FFFFFFFF" />"##
    }?;
    main_panel.add_child(&textbox)?;
    println!("   ✅ Created at compile time!\n");

    // Example 4: Styled button
    println!("📝 Example 4: Styled Button");
    let styled_button = xaml! {
        r##"<Button Content="Green Success Button"
                  Width="400"
                  Height="60"
                  Background="#FF28A745"
                  Foreground="#FFFFFFFF"
                  CornerRadius="12" />"##
    }?;
    main_panel.add_child(&styled_button)?;
    println!("   ✅ Created at compile time!\n");

    // Example 5: Another styled button
    println!("📝 Example 5: Warning Button");
    let warning_button = xaml! {
        r##"<Button Content="Warning Button"
                  Width="400"
                  Height="60"
                  Background="#FFFFC107"
                  Foreground="#FF000000"
                  CornerRadius="8" />"##
    }?;
    main_panel.add_child(&warning_button)?;
    println!("   ✅ Created at compile time!\n");

    // Description
    let desc = xaml! {
        r##"<TextBlock Text="All controls above were created from XAML at compile time!"
                     FontSize="14"
                     Foreground="#FFCCCCCC" />"##
    }?;
    main_panel.add_child(&desc)?;

    let desc2 = xaml! {
        r##"<TextBlock Text="No runtime parsing overhead! ⚡"
                     FontSize="14"
                     Foreground="#FF00FF00" />"##
    }?;
    main_panel.add_child(&desc2)?;

    // Exit button
    let exit_btn = XamlButton::new()?;
    exit_btn.set_content("Exit Demo")?;
    exit_btn.set_size(400.0, 50.0)?;
    exit_btn.set_background(0xFFDC3545)?;
    exit_btn.set_foreground(0xFFFFFFFF)?;
    exit_btn.set_corner_radius(8.0)?;

    let running = Arc::new(AtomicBool::new(true));
    let running_clone = running.clone();
    exit_btn.on_click(move || {
        println!("\n👋 Exiting Compile-Time XAML Demo");
        running_clone.store(false, Ordering::Relaxed);
        unsafe {
            PostQuitMessage(0);
        }
    })?;

    main_panel.add_child(&exit_btn.as_uielement())?;

    // Set content
    xaml_source.set_content_element(&main_panel.as_uielement())?;

    // Show and size the island
    unsafe {
        if let Some(island_hwnd) = xaml_source.island_hwnd() {
            let _ = ShowWindow(island_hwnd, SW_SHOW);
            let mut rect = windows::Win32::Foundation::RECT::default();
            let _ = GetClientRect(hwnd, &mut rect);
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
    }

    println!("✨ UI loaded successfully!");
    println!("💡 All controls were generated at COMPILE TIME!\n");

    Ok(())
}

fn create_window(title: &str, width: i32, height: i32) -> Result<HWND> {
    unsafe {
        let class_name = windows::core::w!("XamlCompileTimeDemoClass");

        let wc = WNDCLASSW {
            lpfnWndProc: Some(window_proc),
            hInstance: GetModuleHandleW(None).unwrap().into(),
            lpszClassName: class_name,
            hCursor: LoadCursorW(None, IDC_ARROW).unwrap_or_default(),
            ..Default::default()
        };

        RegisterClassW(&wc);

        let title_wide: Vec<u16> = title.encode_utf16().chain(Some(0)).collect();

        let hwnd = CreateWindowExW(
            WINDOW_EX_STYLE::default(),
            class_name,
            windows::core::PCWSTR(title_wide.as_ptr()),
            WS_OVERLAPPEDWINDOW,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            width,
            height,
            None,
            None,
            GetModuleHandleW(None).unwrap(),
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
