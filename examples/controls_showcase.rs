//! Comprehensive showcase of all WinRT XAML controls.
//!
//! This example demonstrates:
//! - CheckBox control
//! - ComboBox control
//! - Slider control
//! - ProgressBar control
//! - All controls styled with modern Fluent Design
//!
//! Run with: `cargo run --example controls_showcase`

use std::sync::Arc;
use winrt_xaml::prelude::*;
use windows::core::w;
use windows::Win32::Foundation::*;
use windows::Win32::System::Com::*;
use windows::Win32::System::LibraryLoader::*;
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::Win32::Graphics::Gdi::*;

fn main() -> Result<()> {
    println!("🎨 WinRT XAML Controls Showcase");
    println!("================================\n");

    unsafe {
        // Initialize COM
        let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED);

        // Initialize XAML
        let _xaml_manager = XamlManager::new()?;

        // Create Win32 window
        let instance = GetModuleHandleW(None)?;
        let window_class = w!("WinRTControlsShowcase");

        let wc = WNDCLASSW {
            lpfnWndProc: Some(window_proc),
            hInstance: instance.into(),
            lpszClassName: window_class,
            hCursor: LoadCursorW(None, IDC_ARROW).ok().unwrap_or_default(),
            hbrBackground: HBRUSH((COLOR_WINDOW.0 + 1) as *mut _),
            ..Default::default()
        };

        RegisterClassW(&wc);

        let host_hwnd = CreateWindowExW(
            WINDOW_EX_STYLE::default(),
            window_class,
            w!("WinRT XAML Controls Showcase"),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            900,
            750,
            None,
            None,
            instance,
            None,
        ).expect("Failed to create window");

        // Create XAML source and attach to window
        let mut xaml_source = XamlSource::new()?;
        let island_hwnd = xaml_source.attach_to_window(host_hwnd)?;

        // Create UI
        create_ui(&mut xaml_source)?;

        // Size and show the XAML island
        let mut rect = RECT::default();
        GetClientRect(host_hwnd, &mut rect)?;
        SetWindowPos(
            island_hwnd,
            None,
            0,
            0,
            rect.right - rect.left,
            rect.bottom - rect.top,
            SWP_SHOWWINDOW,
        )?;

        ShowWindow(host_hwnd, SW_SHOW);
        ShowWindow(island_hwnd, SW_SHOW);

        // Message loop
        let mut msg = MSG::default();
        while GetMessageW(&mut msg, None, 0, 0).into() {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }

        CoUninitialize();
    }

    Ok(())
}

fn create_ui(xaml_source: &mut XamlSource) -> Result<()> {
    // Create a ScrollViewer to hold all content
    let scroll_viewer = XamlScrollViewer::new()?;
    scroll_viewer.set_vertical_scroll_mode(ScrollMode::Auto)?;
    scroll_viewer.set_horizontal_scroll_mode(ScrollMode::Disabled)?;
    scroll_viewer.set_vertical_scrollbar_visibility(ScrollBarVisibility::Auto)?;

    // Main container
    let main_panel = XamlStackPanel::new()?;
    main_panel.set_orientation(Orientation::Vertical)?;
    main_panel.set_spacing(30.0)?;

    // Title
    let title = XamlTextBlock::new()?;
    title.set_text("🎨 WinRT XAML Controls Showcase")?;
    title.set_font_size(32.0)?;
    title.set_font_weight(700)?;
    main_panel.add_child(&title.as_uielement())?;

    // ===== CheckBox Section =====
    create_checkbox_section(&main_panel)?;

    // ===== ComboBox Section =====
    create_combobox_section(&main_panel)?;

    // ===== Slider Section =====
    create_slider_section(&main_panel)?;

    // ===== ProgressBar Section =====
    create_progressbar_section(&main_panel)?;

    // Set content
    scroll_viewer.set_content(&main_panel.as_uielement())?;
    xaml_source.set_content_element(&scroll_viewer.as_uielement())?;

    Ok(())
}

fn create_checkbox_section(parent: &XamlStackPanel) -> Result<()> {
    // Section header
    let header = XamlTextBlock::new()?;
    header.set_text("✅ CheckBox Controls")?;
    header.set_font_size(24.0)?;
    header.set_font_weight(600)?;
    parent.add_child(&header.as_uielement())?;

    // Container for checkboxes
    let checkbox_panel = XamlStackPanel::new()?;
    checkbox_panel.set_orientation(Orientation::Vertical)?;
    checkbox_panel.set_spacing(12.0)?;

    // Create checkboxes
    let checkbox1 = XamlCheckBox::new()?;
    checkbox1.set_content("Enable dark mode")?;
    checkbox1.set_is_checked(true)?;
    checkbox_panel.add_child(&checkbox1.as_uielement())?;

    let checkbox2 = XamlCheckBox::new()?;
    checkbox2.set_content("Enable notifications")?;
    checkbox2.set_is_checked(false)?;
    checkbox_panel.add_child(&checkbox2.as_uielement())?;

    let checkbox3 = XamlCheckBox::new()?;
    checkbox3.set_content("Auto-save changes")?;
    checkbox3.set_is_checked(true)?;
    checkbox_panel.add_child(&checkbox3.as_uielement())?;

    parent.add_child(&checkbox_panel.as_uielement())?;
    Ok(())
}

fn create_combobox_section(parent: &XamlStackPanel) -> Result<()> {
    // Section header
    let header = XamlTextBlock::new()?;
    header.set_text("📋 ComboBox Controls")?;
    header.set_font_size(24.0)?;
    header.set_font_weight(600)?;
    parent.add_child(&header.as_uielement())?;

    // Container for comboboxes
    let combobox_panel = XamlStackPanel::new()?;
    combobox_panel.set_orientation(Orientation::Vertical)?;
    combobox_panel.set_spacing(15.0)?;

    // Theme selector
    let theme_label = XamlTextBlock::new()?;
    theme_label.set_text("Select Theme:")?;
    theme_label.set_font_size(16.0)?;
    combobox_panel.add_child(&theme_label.as_uielement())?;

    let theme_combo = XamlComboBox::new()?;
    theme_combo.add_item("Dark Theme")?;
    theme_combo.add_item("Light Theme")?;
    theme_combo.add_item("High Contrast")?;
    theme_combo.add_item("System Default")?;
    theme_combo.set_selected_index(0)?;
    combobox_panel.add_child(&theme_combo.as_uielement())?;

    // Language selector
    let lang_label = XamlTextBlock::new()?;
    lang_label.set_text("Select Language:")?;
    lang_label.set_font_size(16.0)?;
    combobox_panel.add_child(&lang_label.as_uielement())?;

    let lang_combo = XamlComboBox::new()?;
    lang_combo.add_item("English")?;
    lang_combo.add_item("Spanish")?;
    lang_combo.add_item("French")?;
    lang_combo.add_item("German")?;
    lang_combo.add_item("Japanese")?;
    lang_combo.add_item("Chinese")?;
    lang_combo.set_selected_index(0)?;
    combobox_panel.add_child(&lang_combo.as_uielement())?;

    parent.add_child(&combobox_panel.as_uielement())?;
    Ok(())
}

fn create_slider_section(parent: &XamlStackPanel) -> Result<()> {
    // Section header
    let header = XamlTextBlock::new()?;
    header.set_text("🎚️ Slider Controls")?;
    header.set_font_size(24.0)?;
    header.set_font_weight(600)?;
    parent.add_child(&header.as_uielement())?;

    // Container for sliders
    let slider_panel = XamlStackPanel::new()?;
    slider_panel.set_orientation(Orientation::Vertical)?;
    slider_panel.set_spacing(20.0)?;

    // Volume slider
    let volume_label = XamlTextBlock::new()?;
    volume_label.set_text("Volume: 75%")?;
    volume_label.set_font_size(16.0)?;
    slider_panel.add_child(&volume_label.as_uielement())?;

    let volume_slider = XamlSlider::new()?;
    volume_slider.set_minimum(0.0)?;
    volume_slider.set_maximum(100.0)?;
    volume_slider.set_value(75.0)?;
    slider_panel.add_child(&volume_slider.as_uielement())?;

    // Brightness slider
    let brightness_label = XamlTextBlock::new()?;
    brightness_label.set_text("Brightness: 50%")?;
    brightness_label.set_font_size(16.0)?;
    slider_panel.add_child(&brightness_label.as_uielement())?;

    let brightness_slider = XamlSlider::new()?;
    brightness_slider.set_minimum(0.0)?;
    brightness_slider.set_maximum(100.0)?;
    brightness_slider.set_value(50.0)?;
    slider_panel.add_child(&brightness_slider.as_uielement())?;

    parent.add_child(&slider_panel.as_uielement())?;
    Ok(())
}

fn create_progressbar_section(parent: &XamlStackPanel) -> Result<()> {
    // Section header
    let header = XamlTextBlock::new()?;
    header.set_text("📊 ProgressBar Controls")?;
    header.set_font_size(24.0)?;
    header.set_font_weight(600)?;
    parent.add_child(&header.as_uielement())?;

    // Container for progress bars
    let progress_panel = XamlStackPanel::new()?;
    progress_panel.set_orientation(Orientation::Vertical)?;
    progress_panel.set_spacing(20.0)?;

    // Determinate progress bar
    let progress_label1 = XamlTextBlock::new()?;
    progress_label1.set_text("Download Progress: 65%")?;
    progress_label1.set_font_size(16.0)?;
    progress_panel.add_child(&progress_label1.as_uielement())?;

    let progress1 = XamlProgressBar::new()?;
    progress1.set_minimum(0.0)?;
    progress1.set_maximum(100.0)?;
    progress1.set_value(65.0)?;
    progress_panel.add_child(&progress1.as_uielement())?;

    // Another determinate progress bar
    let progress_label2 = XamlTextBlock::new()?;
    progress_label2.set_text("Installation Progress: 30%")?;
    progress_label2.set_font_size(16.0)?;
    progress_panel.add_child(&progress_label2.as_uielement())?;

    let progress2 = XamlProgressBar::new()?;
    progress2.set_minimum(0.0)?;
    progress2.set_maximum(100.0)?;
    progress2.set_value(30.0)?;
    progress_panel.add_child(&progress2.as_uielement())?;

    // Indeterminate progress bar
    let progress_label3 = XamlTextBlock::new()?;
    progress_label3.set_text("Loading... (Indeterminate)")?;
    progress_label3.set_font_size(16.0)?;
    progress_panel.add_child(&progress_label3.as_uielement())?;

    let progress3 = XamlProgressBar::new()?;
    progress3.set_is_indeterminate(true)?;
    progress_panel.add_child(&progress3.as_uielement())?;

    parent.add_child(&progress_panel.as_uielement())?;
    Ok(())
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
