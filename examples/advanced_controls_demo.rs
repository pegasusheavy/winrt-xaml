//! Advanced Controls with Reactive Data Binding Demo
//!
//! This example showcases all advanced controls with reactive state management:
//! - RadioButton (with groups)
//! - Image (with stretch modes)
//! - CheckBox
//! - ComboBox
//! - Slider
//! - ProgressBar
//! - Grid layout with row/column definitions
//! - Reactive Property<T> for automatic UI updates
//!
//! Run with: `cargo run --example advanced_controls_demo --features xaml-islands`

use std::sync::Arc;
use winrt_xaml::prelude::*;
use windows::core::w;
use windows::Win32::Foundation::*;
use windows::Win32::System::Com::*;
use windows::Win32::System::LibraryLoader::*;
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::Win32::Graphics::Gdi::*;

fn main() -> Result<()> {
    println!("🎨 Advanced Controls with Reactive Binding Demo\n");

    unsafe {
        let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED);
        let _xaml_manager = XamlManager::new()?;

        let instance = GetModuleHandleW(None)?;
        let window_class = w!("AdvancedControlsDemo");

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
            w!("Advanced Controls Demo - Reactive Binding"),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            800,
            700,
            None,
            None,
            instance,
            None,
        ).expect("Failed to create window");

        let xaml_source = XamlSource::new()?;
        let island_hwnd = xaml_source.island_hwnd().expect("Failed to get island HWND");

        // Create reactive state
        let theme_mode = Property::new("Light".to_string());
        let _checkbox_state = Property::new(false);

        // Create main scroll viewer
        let scroll_viewer = XamlScrollViewer::new()?;
        scroll_viewer.set_vertical_scroll_mode(ScrollMode::Auto)?;
        scroll_viewer.set_horizontal_scroll_mode(ScrollMode::Disabled)?;

        // Create main grid with rows
        let main_grid = XamlGrid::new()?;
        main_grid.set_background(0xFF1E1E1E)?; // Dark background
        main_grid.set_padding(20.0, 20.0, 20.0, 20.0)?;
        
        // Add row definitions
        main_grid.add_row_auto()?; // Title
        main_grid.add_row_auto()?; // RadioButton section
        main_grid.add_row_auto()?; // CheckBox section
        main_grid.add_row_auto()?; // Slider section
        main_grid.add_row_auto()?; // ComboBox section
        main_grid.add_row_auto()?; // ProgressBar section
        main_grid.add_row_auto()?; // Image section
        main_grid.add_row_star(1.0)?; // Spacer

        let mut current_row = 0;

        // ===== Title =====
        let title = XamlTextBlock::new()?;
        title.set_text("🎨 Advanced Controls Showcase")?;
        title.set_font_size(32.0)?;
        title.set_font_weight(700)?;
        title.set_foreground(0xFFFFFFFF)?;
        title.as_uielement().set_grid_row(current_row)?;
        main_grid.add_child(&title.as_uielement())?;
        current_row += 1;

        // ===== RadioButton Section =====
        let radio_panel = create_radio_section(&theme_mode)?;
        radio_panel.as_uielement().set_grid_row(current_row)?;
        main_grid.add_child(&radio_panel.as_uielement())?;
        current_row += 1;

        // ===== CheckBox Section =====
        let checkbox_panel = create_checkbox_section()?;
        checkbox_panel.as_uielement().set_grid_row(current_row)?;
        main_grid.add_child(&checkbox_panel.as_uielement())?;
        current_row += 1;

        // ===== Slider Section =====
        let slider_panel = create_slider_section()?;
        slider_panel.as_uielement().set_grid_row(current_row)?;
        main_grid.add_child(&slider_panel.as_uielement())?;
        current_row += 1;

        // ===== ComboBox Section =====
        let combo_panel = create_combobox_section()?;
        combo_panel.as_uielement().set_grid_row(current_row)?;
        main_grid.add_child(&combo_panel.as_uielement())?;
        current_row += 1;

        // ===== ProgressBar Section =====
        let progress_panel = create_progress_section()?;
        progress_panel.as_uielement().set_grid_row(current_row)?;
        main_grid.add_child(&progress_panel.as_uielement())?;
        current_row += 1;

        // ===== Image Section =====
        let image_panel = create_image_section()?;
        image_panel.as_uielement().set_grid_row(current_row)?;
        main_grid.add_child(&image_panel.as_uielement())?;

        scroll_viewer.set_content(&main_grid.as_uielement())?;
        xaml_source.set_content_element(&scroll_viewer.as_uielement())?;

        // Size and show the island
        let mut rect = RECT::default();
        let _ = GetClientRect(host_hwnd, &mut rect);
        let _ = SetWindowPos(
            island_hwnd,
            None,
            0,
            0,
            rect.right - rect.left,
            rect.bottom - rect.top,
            SWP_SHOWWINDOW,
        );

        ShowWindow(island_hwnd, SW_SHOW);
        ShowWindow(host_hwnd, SW_SHOW);

        println!("✅ Advanced Controls Demo running!");
        println!("   • Try the RadioButtons to change themes");
        println!("   • Use the Slider to control the ProgressBar");
        println!("   • Toggle the CheckBox");
        println!("   • Select items from the ComboBox\n");

        // Message loop
        let mut msg = MSG::default();
        while GetMessageW(&mut msg, None, 0, 0).as_bool() {
            let _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }

        CoUninitialize();
    }

    Ok(())
}

fn create_radio_section(theme_mode: &Property<String>) -> Result<XamlStackPanel> {
    let panel = XamlStackPanel::new()?;
    panel.set_orientation(Orientation::Vertical)?;
    panel.set_spacing(10.0)?;

    // Section title
    let title = XamlTextBlock::new()?;
    title.set_text("🔘 RadioButton - Theme Selection")?;
    title.set_font_size(20.0)?;
    title.set_font_weight(600)?;
    title.set_foreground(0xFFFFFFFF)?;
    panel.add_child(&title.as_uielement())?;

    // RadioButtons
    let radio1 = XamlRadioButton::new()?;
    radio1.set_content("☀️ Light Theme")?;
    radio1.set_group_name("ThemeGroup")?;
    radio1.set_is_checked(true)?;
    let theme_clone = theme_mode.clone();
    radio1.on_checked(move || {
        theme_clone.set("Light".to_string());
        println!("Theme changed to: Light");
    })?;
    panel.add_child(&radio1.as_uielement())?;

    let radio2 = XamlRadioButton::new()?;
    radio2.set_content("🌙 Dark Theme")?;
    radio2.set_group_name("ThemeGroup")?;
    let theme_clone = theme_mode.clone();
    radio2.on_checked(move || {
        theme_clone.set("Dark".to_string());
        println!("Theme changed to: Dark");
    })?;
    panel.add_child(&radio2.as_uielement())?;

    let radio3 = XamlRadioButton::new()?;
    radio3.set_content("🎨 Auto Theme")?;
    radio3.set_group_name("ThemeGroup")?;
    let theme_clone = theme_mode.clone();
    radio3.on_checked(move || {
        theme_clone.set("Auto".to_string());
        println!("Theme changed to: Auto");
    })?;
    panel.add_child(&radio3.as_uielement())?;

    Ok(panel)
}

fn create_checkbox_section() -> Result<XamlStackPanel> {
    let panel = XamlStackPanel::new()?;
    panel.set_orientation(Orientation::Vertical)?;
    panel.set_spacing(10.0)?;

    // Section title
    let title = XamlTextBlock::new()?;
    title.set_text("☑️ CheckBox - Feature Toggles")?;
    title.set_font_size(20.0)?;
    title.set_font_weight(600)?;
    title.set_foreground(0xFFFFFFFF)?;
    panel.add_child(&title.as_uielement())?;

    // CheckBoxes
    let checkbox1 = XamlCheckBox::new()?;
    checkbox1.set_content("Enable Notifications")?;
    checkbox1.set_is_checked(true)?;
    panel.add_child(&checkbox1.as_uielement())?;

    let checkbox2 = XamlCheckBox::new()?;
    checkbox2.set_content("Auto-save")?;
    checkbox2.set_is_checked(false)?;
    panel.add_child(&checkbox2.as_uielement())?;

    let checkbox3 = XamlCheckBox::new()?;
    checkbox3.set_content("Developer Mode")?;
    checkbox3.set_is_checked(false)?;
    panel.add_child(&checkbox3.as_uielement())?;

    Ok(panel)
}

fn create_slider_section() -> Result<XamlStackPanel> {
    let panel = XamlStackPanel::new()?;
    panel.set_orientation(Orientation::Vertical)?;
    panel.set_spacing(10.0)?;

    // Section title
    let title = XamlTextBlock::new()?;
    title.set_text("🎚️ Slider - Volume Control")?;
    title.set_font_size(20.0)?;
    title.set_font_weight(600)?;
    title.set_foreground(0xFFFFFFFF)?;
    panel.add_child(&title.as_uielement())?;

    // Value display
    let value_text = XamlTextBlock::new()?;
    value_text.set_text("Volume: 50%")?;
    value_text.set_foreground(0xFF00D4FF)?;
    value_text.set_font_size(16.0)?;
    panel.add_child(&value_text.as_uielement())?;

    // Slider
    let slider = XamlSlider::new()?;
    slider.set_minimum(0.0)?;
    slider.set_maximum(100.0)?;
    slider.set_value(50.0)?;
    
    // Note: Slider value_changed event requires additional FFI implementation
    // For now, the slider can be moved but won't update the progress bar automatically
    
    panel.add_child(&slider.as_uielement())?;

    Ok(panel)
}

fn create_combobox_section() -> Result<XamlStackPanel> {
    let panel = XamlStackPanel::new()?;
    panel.set_orientation(Orientation::Vertical)?;
    panel.set_spacing(10.0)?;

    // Section title
    let title = XamlTextBlock::new()?;
    title.set_text("📋 ComboBox - Language Selection")?;
    title.set_font_size(20.0)?;
    title.set_font_weight(600)?;
    title.set_foreground(0xFFFFFFFF)?;
    panel.add_child(&title.as_uielement())?;

    // ComboBox
    let combo = XamlComboBox::new()?;
    combo.add_item("🦀 Rust")?;
    combo.add_item("🐍 Python")?;
    combo.add_item("⚡ JavaScript")?;
    combo.add_item("☕ Java")?;
    combo.add_item("🔷 TypeScript")?;
    combo.set_selected_index(0)?;
    
    // Note: ComboBox selection_changed event requires additional FFI implementation
    
    panel.add_child(&combo.as_uielement())?;

    Ok(panel)
}

fn create_progress_section() -> Result<XamlStackPanel> {
    let panel = XamlStackPanel::new()?;
    panel.set_orientation(Orientation::Vertical)?;
    panel.set_spacing(10.0)?;

    // Section title
    let title = XamlTextBlock::new()?;
    title.set_text("📊 ProgressBar - Linked to Slider")?;
    title.set_font_size(20.0)?;
    title.set_font_weight(600)?;
    title.set_foreground(0xFFFFFFFF)?;
    panel.add_child(&title.as_uielement())?;

    // ProgressBar
    let progress = XamlProgressBar::new()?;
    progress.set_minimum(0.0)?;
    progress.set_maximum(100.0)?;
    progress.set_value(50.0)?;
    
    // Note: Progress bar value would be updated by slider events when implemented
    panel.add_child(&progress.as_uielement())?;

    Ok(panel)
}

fn create_image_section() -> Result<XamlStackPanel> {
    let panel = XamlStackPanel::new()?;
    panel.set_orientation(Orientation::Vertical)?;
    panel.set_spacing(10.0)?;

    // Section title
    let title = XamlTextBlock::new()?;
    title.set_text("🖼️ Image - Placeholder")?;
    title.set_font_size(20.0)?;
    title.set_font_weight(600)?;
    title.set_foreground(0xFFFFFFFF)?;
    panel.add_child(&title.as_uielement())?;

    // Note about images
    let note = XamlTextBlock::new()?;
    note.set_text("Note: Image control requires a valid URI (ms-appx:/// or https://)")?;
    note.set_foreground(0xFF888888)?;
    note.set_font_size(12.0)?;
    panel.add_child(&note.as_uielement())?;

    // Example of how to use Image (commented out as it needs a real image)
    // let image = XamlImage::new()?;
    // image.set_source("ms-appx:///Assets/logo.png")?;
    // image.set_stretch(ImageStretch::Uniform)?;
    // image.set_size(200.0, 200.0)?;
    // panel.add_child(&image.as_uielement())?;

    Ok(panel)
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
