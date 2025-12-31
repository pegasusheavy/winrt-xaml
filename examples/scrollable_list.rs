//! Scrollable List Example
//!
//! Demonstrates:
//! - ScrollViewer control for scrollable content
//! - Vertical scrolling with many items
//! - Modern styling with dark theme
//! - StackPanel inside ScrollViewer

use winrt_xaml::error::Result;
use winrt_xaml::xaml_native::*;
use windows::core::*;
use windows::Win32::Foundation::*;
use windows::Win32::System::Com::*;
use windows::Win32::System::LibraryLoader::*;
use windows::Win32::UI::WindowsAndMessaging::*;

// Host window constants
const WINDOW_WIDTH: i32 = 500;
const WINDOW_HEIGHT: i32 = 600;

fn main() -> Result<()> {
    unsafe {
        // Initialize COM
        CoInitializeEx(None, COINIT_APARTMENTTHREADED).ok()?;
    }

    // Initialize XAML
    let _xaml_manager = XamlManager::new()?;

    // Create host window
    let hwnd = create_host_window("Scrollable List Demo", WINDOW_WIDTH, WINDOW_HEIGHT)?;

    // Create XAML source and attach to window
    let mut xaml_source = XamlSource::new()?;
    let island_hwnd = xaml_source.attach_to_window(hwnd)?;

    // Show the island window
    unsafe {
        ShowWindow(island_hwnd, SW_SHOW);
    }

    // Create the UI
    create_scrollable_list(&xaml_source)?;

    // Show window
    unsafe {
        ShowWindow(hwnd, SW_SHOW);
    }

    println!("\n╔═══════════════════════════════════════════════════════════════╗");
    println!("║         📜 SCROLLABLE LIST EXAMPLE 📜                      ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");
    println!("");
    println!("✨ FEATURES:");
    println!("   • ScrollViewer with vertical scrolling");
    println!("   • 30 styled items in a list");
    println!("   • Automatic scrollbar when content overflows");
    println!("   • Modern dark theme design");
    println!("");
    println!("🖱️  USE:");
    println!("   • Scroll with mouse wheel or drag scrollbar");
    println!("   • Content is taller than window");
    println!("");
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

fn create_scrollable_list(xaml_source: &XamlSource) -> Result<()> {
    // Create main container panel
    let container = XamlStackPanel::new()?;
    container.set_vertical(true)?;
    container.set_spacing(0.0)?;
    container.set_background(0xFF1A1A1A)?; // Dark theme
    container.set_padding(0.0, 0.0, 0.0, 0.0)?;

    // Create title (fixed at top, not scrolled)
    let title_panel = XamlStackPanel::new()?;
    title_panel.set_vertical(true)?;
    title_panel.set_background(0xFF0078D4)?; // Blue header
    title_panel.set_padding(25.0, 20.0, 25.0, 20.0)?;

    let title = XamlTextBlock::new()?;
    title.set_text("📜 Scrollable Item List")?;
    title.set_font_size(28.0)?;
    title.set_font_weight(700)?; // Bold
    title.set_foreground(0xFFFFFFFF)?; // White
    title_panel.add_child(&title.as_uielement())?;

    let subtitle = XamlTextBlock::new()?;
    subtitle.set_text("Scroll down to see all 30 items")?;
    subtitle.set_font_size(14.0)?;
    subtitle.set_foreground(0xFFE0E0E0)?; // Light gray
    subtitle.set_margin(0.0, 5.0, 0.0, 0.0)?;
    title_panel.add_child(&subtitle.as_uielement())?;

    container.add_child(&title_panel.as_uielement())?;

    // Create ScrollViewer for the list content
    let scroll_viewer = XamlScrollViewer::new()?;
    scroll_viewer.set_vertical_scroll_mode(ScrollMode::Enabled)?;
    scroll_viewer.set_horizontal_scroll_mode(ScrollMode::Disabled)?;
    scroll_viewer.set_vertical_scrollbar_visibility(ScrollBarVisibility::Auto)?;

    // Create content panel inside ScrollViewer
    let content_panel = XamlStackPanel::new()?;
    content_panel.set_vertical(true)?;
    content_panel.set_spacing(8.0)?;
    content_panel.set_padding(20.0, 20.0, 20.0, 20.0)?;
    content_panel.set_background(0xFF1A1A1A)?; // Dark background

    // Add 30 items to demonstrate scrolling
    let colors = vec![
        (0xFF00D4FF, "Cyan"),
        (0xFF00FF9F, "Green"),
        (0xFFFF8C00, "Orange"),
        (0xFFE74856, "Red"),
        (0xFF0078D4, "Blue"),
        (0xFFB4009E, "Purple"),
    ];

    for i in 1..=30 {
        let item_panel = XamlStackPanel::new()?;
        item_panel.set_vertical(false)?;
        item_panel.set_spacing(15.0)?;
        item_panel.set_background(0xFF2D2D2D)?; // Dark item background
        item_panel.set_padding(18.0, 15.0, 18.0, 15.0)?;
        item_panel.set_corner_radius(10.0)?;

        // Item number badge
        let color_index = (i - 1) % colors.len();
        let (badge_color, _color_name) = colors[color_index];

        let badge = XamlTextBlock::new()?;
        badge.set_text(&format!("{}", i))?;
        badge.set_font_size(20.0)?;
        badge.set_font_weight(700)?;
        badge.set_foreground(badge_color)?;
        badge.set_margin(0.0, 0.0, 10.0, 0.0)?;
        item_panel.add_child(&badge.as_uielement())?;

        // Item text
        let item_text = XamlTextBlock::new()?;
        item_text.set_text(&format!("List Item #{} - This is a scrollable item", i))?;
        item_text.set_font_size(16.0)?;
        item_text.set_foreground(0xFFFFFFFF)?; // White
        item_panel.add_child(&item_text.as_uielement())?;

        content_panel.add_child(&item_panel.as_uielement())?;
    }

    // Set the content panel as the ScrollViewer's content
    scroll_viewer.set_content(&content_panel.as_uielement())?;

    // Add ScrollViewer to main container
    container.add_child(&scroll_viewer.as_uielement())?;

    // Set the container as the XAML source content
    xaml_source.set_content_element(&container.as_uielement())?;

    Ok(())
}

fn create_host_window(title: &str, width: i32, height: i32) -> Result<HWND> {
    unsafe {
        let class_name = w!("WinRTXamlHostWindow");
        let h_instance = HINSTANCE(GetModuleHandleW(None).unwrap_or_default().0);

        let wc = WNDCLASSW {
            lpfnWndProc: Some(window_proc),
            hInstance: h_instance,
            lpszClassName: class_name,
            hCursor: LoadCursorW(None, IDC_ARROW).unwrap_or_default(),
            ..Default::default()
        };

        RegisterClassW(&wc);

        let hwnd = CreateWindowExW(
            WINDOW_EX_STYLE::default(),
            class_name,
            &HSTRING::from(title),
            WS_OVERLAPPEDWINDOW,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            width,
            height,
            None,
            None,
            h_instance,
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
