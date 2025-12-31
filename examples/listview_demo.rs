//! ListView Demo - Demonstrates list management and selection
//!
//! This example showcases the XamlListView control with:
//! - Adding and removing items
//! - Selection modes (Single, Multiple)
//! - Selection changed events
//! - Item management
//!
//! Run with: `cargo run --example listview_demo --features xaml-islands`

use std::sync::{Arc, Mutex};
use winrt_xaml::prelude::*;
use windows::core::w;
use windows::Win32::Foundation::*;
use windows::Win32::System::Com::*;
use windows::Win32::System::LibraryLoader::*;
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::Win32::Graphics::Gdi::*;

fn main() -> Result<()> {
    println!("📋 ListView Demo\n");

    unsafe {
        let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED);
        let _xaml_manager = XamlManager::new()?;

        let instance = GetModuleHandleW(None)?;
        let window_class = w!("ListViewDemo");

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
            w!("ListView Demo - Task Manager"),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            600,
            500,
            None,
            None,
            instance,
            None,
        ).expect("Failed to create window");

        let xaml_source = XamlSource::new()?;
        let island_hwnd = xaml_source.island_hwnd().expect("Failed to get island HWND");

        // Create main grid
        let main_grid = XamlGrid::new()?;
        main_grid.set_background(0xFF2D2D30)?; // Dark background
        main_grid.set_padding(20.0, 20.0, 20.0, 20.0)?;

        // Add row definitions
        main_grid.add_row_auto()?; // Title
        main_grid.add_row_auto()?; // Input section
        main_grid.add_row_star(1.0)?; // ListView
        main_grid.add_row_auto()?; // Status
        main_grid.add_row_auto()?; // Controls

        let mut current_row = 0;

        // ===== Title =====
        let title = XamlTextBlock::new()?;
        title.set_text("📋 Task Manager")?;
        title.set_font_size(32.0)?;
        title.set_font_weight(700)?;
        title.set_foreground(0xFFFFFFFF)?;
        title.as_uielement().set_grid_row(current_row)?;
        main_grid.add_child(&title.as_uielement())?;
        current_row += 1;

        // ===== Input Section =====
        let input_panel = XamlStackPanel::new()?;
        input_panel.set_orientation(Orientation::Horizontal)?;
        input_panel.set_spacing(10.0)?;
        input_panel.as_uielement().set_grid_row(current_row)?;

        let input_label = XamlTextBlock::new()?;
        input_label.set_text("New Task:")?;
        input_label.set_foreground(0xFFFFFFFF)?;
        input_label.set_font_size(16.0)?;
        input_panel.add_child(&input_label.as_uielement())?;

        let input_box = Arc::new(XamlTextBox::new()?);
        // Note: Width and placeholder text require additional FFI implementation
        input_panel.add_child(&input_box.as_uielement())?;

        main_grid.add_child(&input_panel.as_uielement())?;
        current_row += 1;

        // ===== ListView =====
        let listview = Arc::new(XamlListView::new()?);
        listview.set_selection_mode(ListViewSelectionMode::Single)?;

        // Add some initial items
        listview.add_item("✅ Complete project documentation")?;
        listview.add_item("🔧 Fix ListView implementation")?;
        listview.add_item("📝 Write unit tests")?;
        listview.add_item("🚀 Deploy to production")?;
        listview.add_item("📊 Review performance metrics")?;

        listview.as_uielement().set_grid_row(current_row)?;
        main_grid.add_child(&listview.as_uielement())?;
        current_row += 1;

        // ===== Status Text =====
        let status_text = Arc::new(XamlTextBlock::new()?);
        status_text.set_text(&format!("Total tasks: {} | Selected: None", listview.item_count()))?;
        status_text.set_foreground(0xFF00D4FF)?;
        status_text.set_font_size(14.0)?;
        status_text.as_uielement().set_grid_row(current_row)?;
        main_grid.add_child(&status_text.as_uielement())?;
        current_row += 1;

        // ===== Control Buttons =====
        let button_panel = XamlStackPanel::new()?;
        button_panel.set_orientation(Orientation::Horizontal)?;
        button_panel.set_spacing(10.0)?;
        button_panel.as_uielement().set_grid_row(current_row)?;

        // Add button
        let add_button = XamlButton::new()?;
        add_button.set_content("➕ Add Task")?;
        add_button.set_background(0xFF0078D4)?;
        add_button.set_foreground(0xFFFFFFFF)?;
        add_button.set_padding(15.0, 8.0, 15.0, 8.0)?;

        let listview_clone = listview.clone();
        let input_clone = input_box.clone();
        let status_clone = status_text.clone();
        add_button.on_click(move || {
            if let Ok(text) = input_clone.get_text() {
                if !text.trim().is_empty() {
                    let _ = listview_clone.add_item(&text);
                    let _ = input_clone.set_text("");
                    let count = listview_clone.item_count();
                    let _ = status_clone.set_text(&format!("Total tasks: {} | Task added!", count));
                    println!("Added task: {}", text);
                }
            }
        });
        button_panel.add_child(&add_button.as_uielement())?;

        // Remove button
        let remove_button = XamlButton::new()?;
        remove_button.set_content("🗑️ Remove Selected")?;
        remove_button.set_background(0xFFE81123)?;
        remove_button.set_foreground(0xFFFFFFFF)?;
        remove_button.set_padding(15.0, 8.0, 15.0, 8.0)?;

        let listview_clone = listview.clone();
        let status_clone = status_text.clone();
        remove_button.on_click(move || {
            let index = listview_clone.selected_index();
            if index >= 0 {
                if let Ok(item) = listview_clone.get_item(index) {
                    let _ = listview_clone.remove_item(index);
                    let count = listview_clone.item_count();
                    let _ = status_clone.set_text(&format!("Total tasks: {} | Removed: {}", count, item));
                    println!("Removed task: {}", item);
                }
            } else {
                let _ = status_clone.set_text("Please select a task to remove");
            }
        });
        button_panel.add_child(&remove_button.as_uielement())?;

        // Clear button
        let clear_button = XamlButton::new()?;
        clear_button.set_content("🧹 Clear All")?;
        clear_button.set_background(0xFF6B6B6B)?;
        clear_button.set_foreground(0xFFFFFFFF)?;
        clear_button.set_padding(15.0, 8.0, 15.0, 8.0)?;

        let listview_clone = listview.clone();
        let status_clone = status_text.clone();
        clear_button.on_click(move || {
            let _ = listview_clone.clear();
            let _ = status_clone.set_text("Total tasks: 0 | All tasks cleared");
            println!("Cleared all tasks");
        });
        button_panel.add_child(&clear_button.as_uielement())?;

        main_grid.add_child(&button_panel.as_uielement())?;

        // Selection changed event
        let status_clone = status_text.clone();
        let listview_clone = listview.clone();
        listview.on_selection_changed(move |index| {
            if index >= 0 {
                if let Ok(item) = listview_clone.get_item(index) {
                    let count = listview_clone.item_count();
                    let _ = status_clone.set_text(&format!("Total tasks: {} | Selected: {}", count, item));
                    println!("Selected: {} (index {})", item, index);
                }
            } else {
                let count = listview_clone.item_count();
                let _ = status_clone.set_text(&format!("Total tasks: {} | Selected: None", count));
            }
        })?;

        xaml_source.set_content_element(&main_grid.as_uielement())?;

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

        println!("✅ ListView Demo running!");
        println!("   • Type a task name and click 'Add Task'");
        println!("   • Select a task to see details");
        println!("   • Click 'Remove Selected' to delete a task");
        println!("   • Click 'Clear All' to remove all tasks\n");

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
