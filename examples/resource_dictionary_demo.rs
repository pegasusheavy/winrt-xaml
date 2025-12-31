//! Demonstration of WinRT Resource Dictionaries for shared styling.
//!
//! This example shows how to use ResourceDictionary to define reusable
//! colors, fonts, and other resources.

use winrt_xaml::prelude::*;
use winrt_xaml::xaml_native::{
    XamlButton, XamlManager, XamlResourceDictionary, XamlSource, XamlStackPanel, XamlTextBlock,
};
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, DefWindowProcW, DispatchMessageW, GetMessageW, LoadCursorW, PostQuitMessage,
    RegisterClassW, ShowWindow, TranslateMessage, CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT,
    IDC_ARROW, MSG, SW_SHOW, WINDOW_EX_STYLE, WM_DESTROY, WNDCLASSW, WS_OVERLAPPEDWINDOW,
};
use windows::{core::*, w};

fn main() -> Result<()> {
    unsafe {
        // Initialize COM
        windows::Win32::System::Com::CoInitializeEx(
            None,
            windows::Win32::System::Com::COINIT_APARTMENTTHREADED,
        )?;

        // Register window class
        let h_instance = GetModuleHandleW(None)?;
        let class_name = w!("WinRTResourceDictionaryDemo");

        let wc = WNDCLASSW {
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(window_proc),
            hInstance: h_instance.into(),
            lpszClassName: class_name,
            hCursor: LoadCursorW(None, IDC_ARROW)?,
            ..Default::default()
        };

        RegisterClassW(&wc);

        // Create window
        let hwnd = CreateWindowExW(
            WINDOW_EX_STYLE::default(),
            class_name,
            w!("WinRT Resource Dictionary Demo"),
            WS_OVERLAPPEDWINDOW,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            700,
            600,
            None,
            None,
            h_instance,
            None,
        )?;

        // Initialize XAML
        let _manager = XamlManager::new()?;
        let source = XamlSource::new()?;
        let island_hwnd = source.attach_to_window(hwnd)?;

        // Create a Resource Dictionary with theme colors
        let theme = XamlResourceDictionary::new()?;
        theme.insert_color("PrimaryColor", 0xFF0078D4)?;  // Blue
        theme.insert_color("SecondaryColor", 0xFF107C10)?; // Green
        theme.insert_color("AccentColor", 0xFFF7630C)?;    // Orange
        theme.insert_color("DangerColor", 0xFFE81123)?;    // Red
        theme.insert_color("BackgroundColor", 0xFF1E1E1E)?; // Dark gray
        theme.insert_color("TextColor", 0xFFFFFFFF)?;      // White
        theme.insert_double("StandardSpacing", 20.0)?;
        theme.insert_double("LargeSpacing", 40.0)?;
        theme.insert_double("ButtonHeight", 50.0)?;
        theme.insert_string("AppName", "Resource Dictionary Demo")?;

        // Create UI
        let root = XamlStackPanel::new()?;
        root.set_orientation(Orientation::Vertical)?;
        root.set_spacing(theme.get_double("StandardSpacing").unwrap_or(20.0))?;
        root.as_uielement().set_padding(30.0, 30.0, 30.0, 30.0)?;

        // Apply theme to root
        if let Some(bg_color) = theme.get_color("BackgroundColor") {
            root.as_uielement().set_background(bg_color)?;
        }

        // Title
        let title = XamlTextBlock::new()?;
        title.set_text("📚 Resource Dictionary System")?;
        title.set_font_size(32.0)?;
        title.set_font_weight(700)?;
        if let Some(text_color) = theme.get_color("TextColor") {
            title.as_uielement().set_foreground(text_color)?;
        }
        root.add_child(&title)?;

        // Description
        let desc = XamlTextBlock::new()?;
        desc.set_text("Centralized theme management with reusable resources")?;
        desc.set_font_size(16.0)?;
        title.as_uielement().set_foreground(0xFFAAAAAA)?;
        root.add_child(&desc)?;

        // Theme info
        let info = XamlTextBlock::new()?;
        info.set_text(format!(
            "Theme Resources:\n\
             • Primary Color: #{:06X}\n\
             • Secondary Color: #{:06X}\n\
             • Accent Color: #{:06X}\n\
             • Standard Spacing: {}px\n\
             • Button Height: {}px",
            theme.get_color("PrimaryColor").unwrap_or(0) & 0xFFFFFF,
            theme.get_color("SecondaryColor").unwrap_or(0) & 0xFFFFFF,
            theme.get_color("AccentColor").unwrap_or(0) & 0xFFFFFF,
            theme.get_double("StandardSpacing").unwrap_or(0.0),
            theme.get_double("ButtonHeight").unwrap_or(0.0),
        ))?;
        info.set_font_size(14.0)?;
        info.as_uielement().set_foreground(0xFFCCCCCC)?;
        info.as_uielement().set_margin(0.0, 10.0, 0.0, 20.0)?;
        root.add_child(&info)?;

        // Buttons using theme colors
        let button_height = theme.get_double("ButtonHeight").unwrap_or(50.0);

        let primary_btn = XamlButton::new()?;
        primary_btn.set_content("Primary Action")?;
        primary_btn.set_size(250.0, button_height)?;
        if let Some(color) = theme.get_color("PrimaryColor") {
            primary_btn.as_uielement().set_background(color)?;
        }
        if let Some(color) = theme.get_color("TextColor") {
            primary_btn.as_uielement().set_foreground(color)?;
        }
        primary_btn.as_uielement().set_corner_radius(5.0)?;
        primary_btn.as_uielement().set_padding(15.0, 10.0, 15.0, 10.0)?;
        root.add_child(&primary_btn)?;

        let secondary_btn = XamlButton::new()?;
        secondary_btn.set_content("Secondary Action")?;
        secondary_btn.set_size(250.0, button_height)?;
        if let Some(color) = theme.get_color("SecondaryColor") {
            secondary_btn.as_uielement().set_background(color)?;
        }
        if let Some(color) = theme.get_color("TextColor") {
            secondary_btn.as_uielement().set_foreground(color)?;
        }
        secondary_btn.as_uielement().set_corner_radius(5.0)?;
        secondary_btn.as_uielement().set_padding(15.0, 10.0, 15.0, 10.0)?;
        root.add_child(&secondary_btn)?;

        let accent_btn = XamlButton::new()?;
        accent_btn.set_content("Accent Action")?;
        accent_btn.set_size(250.0, button_height)?;
        if let Some(color) = theme.get_color("AccentColor") {
            accent_btn.as_uielement().set_background(color)?;
        }
        if let Some(color) = theme.get_color("TextColor") {
            accent_btn.as_uielement().set_foreground(color)?;
        }
        accent_btn.as_uielement().set_corner_radius(5.0)?;
        accent_btn.as_uielement().set_padding(15.0, 10.0, 15.0, 10.0)?;
        root.add_child(&accent_btn)?;

        let danger_btn = XamlButton::new()?;
        danger_btn.set_content("Danger Action")?;
        danger_btn.set_size(250.0, button_height)?;
        if let Some(color) = theme.get_color("DangerColor") {
            danger_btn.as_uielement().set_background(color)?;
        }
        if let Some(color) = theme.get_color("TextColor") {
            danger_btn.as_uielement().set_foreground(color)?;
        }
        danger_btn.as_uielement().set_corner_radius(5.0)?;
        danger_btn.as_uielement().set_padding(15.0, 10.0, 15.0, 10.0)?;
        root.add_child(&danger_btn)?;

        // Benefits section
        let benefits = XamlTextBlock::new()?;
        benefits.set_text(
            "✨ Benefits of Resource Dictionaries:\n\
             • Centralized theme management\n\
             • Easy to update colors/styles globally\n\
             • Type-safe resource access\n\
             • Supports colors, numbers, and strings\n\
             • Can be applied to any UI element"
        )?;
        benefits.set_font_size(14.0)?;
        benefits.as_uielement().set_foreground(0xFF888888)?;
        benefits.as_uielement().set_margin(0.0, 30.0, 0.0, 0.0)?;
        root.add_child(&benefits)?;

        // Set content
        source.set_content(&root)?;

        // Size the island to fill the window
        use windows::Win32::UI::WindowsAndMessaging::{GetClientRect, SetWindowPos, SWP_NOZORDER};
        let mut rect = windows::Win32::Foundation::RECT::default();
        GetClientRect(hwnd, &mut rect)?;
        SetWindowPos(
            island_hwnd,
            None,
            0,
            0,
            rect.right - rect.left,
            rect.bottom - rect.top,
            SWP_NOZORDER,
        )?;

        // Show windows
        ShowWindow(hwnd, SW_SHOW);
        ShowWindow(island_hwnd, SW_SHOW);

        // Message loop
        let mut msg = MSG::default();
        while GetMessageW(&mut msg, None, 0, 0).as_bool() {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }

        windows::Win32::System::Com::CoUninitialize();
        Ok(())
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
