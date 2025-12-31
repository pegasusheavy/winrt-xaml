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
    println!("║                  ⚡ COMPILE-TIME XAML DEMO ⚡                          ║");
    println!("╚═══════════════════════════════════════════════════════════════════════════╝\n");

    println!("🎯 This demo showcases the xaml! macro:");
    println!("   • XAML parsed at COMPILE TIME (not runtime!)");
    println!("   • All syntax validated during 'cargo build'");
    println!("   • Zero runtime parsing overhead");
    println!("   • Full type safety and IDE support\n");
    
    println!("🔧 Creating UI from compile-time XAML...\n");

    // Initialize XAML
    let _manager = XamlManager::new()?;

    // Create host window
    let hwnd = create_window("⚡ Compile-Time XAML Demo - WinRT", 550, 900)?;

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
    main_panel.set_spacing(12.0)?;
    main_panel.set_background(0xFF0D1117)?;
    main_panel.set_padding(25.0, 25.0, 25.0, 25.0)?;

    // ============================================================
    // HEADER - COMPILE-TIME XAML DEMO
    // ============================================================

    let title = xaml! {
        r##"<TextBlock Text="⚡ Compile-Time XAML Demo ⚡"
                     FontSize="32"
                     FontWeight="700"
                     Foreground="#FF58A6FF" />"##
    }?;
    main_panel.add_child(&title)?;

    let subtitle = xaml! {
        r##"<TextBlock Text="All controls below were created from XAML at compile time!"
                     FontSize="16"
                     Foreground="#FF8B949E" />"##
    }?;
    main_panel.add_child(&subtitle)?;

    // Spacer
    let spacer1 = xaml! {
        r##"<TextBlock Text=" "
                     FontSize="8" />"##
    }?;
    main_panel.add_child(&spacer1)?;

    // ============================================================
    // SECTION 1: INTERACTIVE BUTTONS
    // ============================================================

    let section1_title = xaml! {
        r##"<TextBlock Text="🎯 Interactive Buttons (Compile-Time)"
                     FontSize="20"
                     FontWeight="700"
                     Foreground="#FF7EE787" />"##
    }?;
    main_panel.add_child(&section1_title)?;

    // Primary action button
    let primary_btn = xaml! {
        r##"<Button Content="Primary Action"
                  Width="450"
                  Height="50"
                  Background="#FF238636"
                  Foreground="#FFFFFFFF"
                  CornerRadius="6" />"##
    }?;
    main_panel.add_child(&primary_btn)?;
    println!("✅ Created Primary Button from compile-time XAML");

    // Secondary button
    let secondary_btn = xaml! {
        r##"<Button Content="Secondary Action"
                  Width="450"
                  Height="50"
                  Background="#FF21262D"
                  Foreground="#FFC9D1D9"
                  CornerRadius="6" />"##
    }?;
    main_panel.add_child(&secondary_btn)?;
    println!("✅ Created Secondary Button from compile-time XAML");

    // Danger button
    let danger_btn = xaml! {
        r##"<Button Content="Danger Zone"
                  Width="450"
                  Height="50"
                  Background="#FFDA3633"
                  Foreground="#FFFFFFFF"
                  CornerRadius="6" />"##
    }?;
    main_panel.add_child(&danger_btn)?;
    println!("✅ Created Danger Button from compile-time XAML");

    // Spacer
    main_panel.add_child(&spacer1)?;

    // ============================================================
    // SECTION 2: TEXT INPUT
    // ============================================================

    let section2_title = xaml! {
        r##"<TextBlock Text="📝 Text Input (Compile-Time)"
                     FontSize="20"
                     FontWeight="700"
                     Foreground="#FFF0883E" />"##
    }?;
    main_panel.add_child(&section2_title)?;

    let username_input = xaml! {
        r##"<TextBox PlaceholderText="Username..."
                   Width="450"
                   Height="40"
                   Background="#FF0D1117"
                   Foreground="#FFC9D1D9" />"##
    }?;
    main_panel.add_child(&username_input)?;
    println!("✅ Created Username Input from compile-time XAML");

    let password_input = xaml! {
        r##"<TextBox PlaceholderText="Password..."
                   Width="450"
                   Height="40"
                   Background="#FF0D1117"
                   Foreground="#FFC9D1D9" />"##
    }?;
    main_panel.add_child(&password_input)?;
    println!("✅ Created Password Input from compile-time XAML");

    // Spacer
    main_panel.add_child(&spacer1)?;

    // ============================================================
    // SECTION 3: STATUS INDICATORS
    // ============================================================

    let section3_title = xaml! {
        r##"<TextBlock Text="📊 Status Indicators (Compile-Time)"
                     FontSize="20"
                     FontWeight="700"
                     Foreground="#FFD29922" />"##
    }?;
    main_panel.add_child(&section3_title)?;

    let success_msg = xaml! {
        r##"<TextBlock Text="✅ Compile-time validation passed!"
                     FontSize="16"
                     Foreground="#FF7EE787" />"##
    }?;
    main_panel.add_child(&success_msg)?;

    let info_msg = xaml! {
        r##"<TextBlock Text="ℹ️ Zero runtime parsing overhead"
                     FontSize="16"
                     Foreground="#FF58A6FF" />"##
    }?;
    main_panel.add_child(&info_msg)?;

    let perf_msg = xaml! {
        r##"<TextBlock Text="⚡ Maximum performance achieved!"
                     FontSize="16"
                     Foreground="#FFFFA657" />"##
    }?;
    main_panel.add_child(&perf_msg)?;

    // Spacer
    main_panel.add_child(&spacer1)?;

    // ============================================================
    // FOOTER INFO
    // ============================================================

    let footer_info = xaml! {
        r##"<TextBlock Text="All XAML above was validated at cargo build time"
                     FontSize="14"
                     Foreground="#FF8B949E" />"##
    }?;
    main_panel.add_child(&footer_info)?;

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

    println!("\n╔═══════════════════════════════════════════════════════════════════════════╗");
    println!("║                    ✨ UI LOADED SUCCESSFULLY! ✨                       ║");
    println!("╚═══════════════════════════════════════════════════════════════════════════╝\n");

    println!("🎉 All controls were created from XAML at COMPILE TIME!");
    println!("⚡ Zero runtime parsing - maximum performance!");
    println!("🔒 Type-safe and validated at build time\n");

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
