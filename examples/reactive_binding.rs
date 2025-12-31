//! Reactive data binding example.
//!
//! This example demonstrates the reactive state management system in winrt-xaml.
//! It shows how to use Property<T>, ObservableCollection<T>, and Computed<T>
//! for automatic UI updates without manual event handling.
//!
//! Run with: `cargo run --example reactive_binding`

use std::sync::Arc;
use winrt_xaml::prelude::*;
use windows::core::w;
use windows::Win32::Foundation::*;
use windows::Win32::System::Com::*;
use windows::Win32::System::LibraryLoader::*;
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::Win32::Graphics::Gdi::*;

fn main() -> Result<()> {
    println!("🔄 WinRT XAML Reactive Binding Example");
    println!("======================================\n");

    unsafe {
        // Initialize COM
        let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED);

        // Initialize XAML
        let _xaml_manager = XamlManager::new()?;

        // Create Win32 window
        let instance = GetModuleHandleW(None)?;
        let window_class = w!("WinRTReactiveBinding");

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
            w!("Reactive Data Binding - WinRT XAML"),
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

        // Create XAML source and attach to window
        let mut xaml_source = XamlSource::new()?;
        let island_hwnd = xaml_source.attach_to_window(host_hwnd)?;

        // Create UI with reactive bindings
        create_reactive_ui(&mut xaml_source)?;

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

fn create_reactive_ui(xaml_source: &mut XamlSource) -> Result<()> {
    // ===== Reactive State =====

    // Simple reactive property
    let count = Property::new(0);

    // Reactive properties for calculator
    let num1 = Property::new(0);
    let num2 = Property::new(0);

    // Computed property (automatically updates when num1 or num2 changes)
    let sum = Computed::from_properties2(&num1, &num2, |a, b| a + b);

    // Observable collection for todo list
    let todos = ObservableCollection::new();

    // ===== UI Layout =====

    let scroll_viewer = XamlScrollViewer::new()?;
    scroll_viewer.set_vertical_scroll_mode(ScrollMode::Auto)?;
    scroll_viewer.set_horizontal_scroll_mode(ScrollMode::Disabled)?;

    let main_panel = XamlStackPanel::new()?;
    main_panel.set_orientation(Orientation::Vertical)?;
    main_panel.set_spacing(30.0)?;

    // Title
    let title = XamlTextBlock::new()?;
    title.set_text("🔄 Reactive Data Binding Examples")?;
    title.set_font_size(28.0)?;
    title.set_font_weight(700)?;
    main_panel.add_child(&title.as_uielement())?;

    // ===== Example 1: Simple Counter with Property<T> =====
    create_counter_example(&main_panel, &count)?;

    // ===== Example 2: Computed Properties =====
    create_calculator_example(&main_panel, &num1, &num2, &sum)?;

    // ===== Example 3: Observable Collection =====
    create_todo_example(&main_panel, &todos)?;

    // Set content
    scroll_viewer.set_content(&main_panel.as_uielement())?;
    xaml_source.set_content_element(&scroll_viewer.as_uielement())?;

    Ok(())
}

fn create_counter_example(parent: &XamlStackPanel, count: &Property<i32>) -> Result<()> {
    let section = XamlStackPanel::new()?;
    section.set_orientation(Orientation::Vertical)?;
    section.set_spacing(10.0)?;

    // Section title
    let header = XamlTextBlock::new()?;
    header.set_text("1️⃣ Simple Counter (Property<T>)")?;
    header.set_font_size(20.0)?;
    header.set_font_weight(600)?;
    section.add_child(&header.as_uielement())?;

    // Description
    let desc = XamlTextBlock::new()?;
    desc.set_text("Property<T> automatically notifies subscribers when the value changes.")?;
    desc.set_font_size(14.0)?;
    section.add_child(&desc.as_uielement())?;

    // Counter display (automatically updates when count changes)
    let counter_text = Arc::new(XamlTextBlock::new()?);
    counter_text.set_font_size(24.0)?;
    counter_text.set_font_weight(700)?;

    // Bind counter_text to count property
    count.subscribe({
        let counter_text = counter_text.clone();
        move |value| {
            let _ = counter_text.set_text(&format!("Count: {}", value));
        }
    });

    section.add_child(&counter_text.as_uielement())?;

    // Buttons
    let buttons = XamlStackPanel::new()?;
    buttons.set_orientation(Orientation::Horizontal)?;
    buttons.set_spacing(10.0)?;

    let increment_btn = XamlButton::new()?;
    increment_btn.set_content("➕ Increment")?;
    increment_btn.on_click({
        let count = count.clone();
        move || {
            count.update(|c| *c += 1);)
        }
    })?;
    buttons.add_child(&increment_btn.as_uielement())?;

    let decrement_btn = XamlButton::new()?;
    decrement_btn.set_content("➖ Decrement")?;
    decrement_btn.on_click({
        let count = count.clone();
        move || {
            count.update(|c| *c -= 1);)
        }
    })?;
    buttons.add_child(&decrement_btn.as_uielement())?;

    let reset_btn = XamlButton::new()?;
    reset_btn.set_content("🔄 Reset")?;
    reset_btn.on_click({
        let count = count.clone();
        move || {
            count.set(0);)
        }
    })?;
    buttons.add_child(&reset_btn.as_uielement())?;

    section.add_child(&buttons.as_uielement())?;
    parent.add_child(&section.as_uielement())?;

    Ok(())
}

fn create_calculator_example(
    parent: &XamlStackPanel,
    num1: &Property<i32>,
    num2: &Property<i32>,
    sum: &Computed<i32>,
) -> Result<()> {
    let section = XamlStackPanel::new()?;
    section.set_orientation(Orientation::Vertical)?;
    section.set_spacing(10.0)?;

    // Section title
    let header = XamlTextBlock::new()?;
    header.set_text("2️⃣ Computed Properties (Computed<T>)")?;
    header.set_font_size(20.0)?;
    header.set_font_weight(600)?;
    section.add_child(&header.as_uielement())?;

    // Description
    let desc = XamlTextBlock::new()?;
    desc.set_text("Computed<T> automatically recalculates when dependencies change.")?;
    desc.set_font_size(14.0)?;
    section.add_child(&desc.as_uielement())?;

    // Display current values
    let values_text = Arc::new(XamlTextBlock::new()?);
    values_text.set_font_size(18.0)?;

    // Subscribe to both properties to update display
    let update_display = {
        let values_text = values_text.clone();
        let num1 = num1.clone();
        let num2 = num2.clone();
        let sum = sum.clone();
        Arc::new(move || {
            let _ = values_text.set_text(&format!(
                "{} + {} = {}",
                num1.get(),
                num2.get(),
                sum.get()
            ));
        })
    };

    num1.subscribe({
        let update = update_display.clone();
        move |_| update()
    });

    num2.subscribe({
        let update = update_display.clone();
        move |_| update()
    });

    // Initial display
    update_display();

    section.add_child(&values_text.as_uielement())?;

    // Controls for num1
    let num1_panel = XamlStackPanel::new()?;
    num1_panel.set_orientation(Orientation::Horizontal)?;
    num1_panel.set_spacing(10.0)?;

    let num1_label = XamlTextBlock::new()?;
    num1_label.set_text("Number 1:")?;
    num1_panel.add_child(&num1_label.as_uielement())?;

    let num1_minus = XamlButton::new()?;
    num1_minus.set_content("-")?;
    num1_minus.on_click({
        let num1 = num1.clone();
        move || {
            num1.update(|n| *n -= 1);)
        }
    })?;
    num1_panel.add_child(&num1_minus.as_uielement())?;

    let num1_plus = XamlButton::new()?;
    num1_plus.set_content("+")?;
    num1_plus.on_click({
        let num1 = num1.clone();
        move || {
            num1.update(|n| *n += 1);)
        }
    })?;
    num1_panel.add_child(&num1_plus.as_uielement())?;

    section.add_child(&num1_panel.as_uielement())?;

    // Controls for num2
    let num2_panel = XamlStackPanel::new()?;
    num2_panel.set_orientation(Orientation::Horizontal)?;
    num2_panel.set_spacing(10.0)?;

    let num2_label = XamlTextBlock::new()?;
    num2_label.set_text("Number 2:")?;
    num2_panel.add_child(&num2_label.as_uielement())?;

    let num2_minus = XamlButton::new()?;
    num2_minus.set_content("-")?;
    num2_minus.on_click({
        let num2 = num2.clone();
        move || {
            num2.update(|n| *n -= 1);)
        }
    })?;
    num2_panel.add_child(&num2_minus.as_uielement())?;

    let num2_plus = XamlButton::new()?;
    num2_plus.set_content("+")?;
    num2_plus.on_click({
        let num2 = num2.clone();
        move || {
            num2.update(|n| *n += 1);)
        }
    })?;
    num2_panel.add_child(&num2_plus.as_uielement())?;

    section.add_child(&num2_panel.as_uielement())?;
    parent.add_child(&section.as_uielement())?;

    Ok(())
}

fn create_todo_example(parent: &XamlStackPanel, todos: &ObservableCollection<String>) -> Result<()> {
    let section = XamlStackPanel::new()?;
    section.set_orientation(Orientation::Vertical)?;
    section.set_spacing(10.0)?;

    // Section title
    let header = XamlTextBlock::new()?;
    header.set_text("3️⃣ Observable Collection (ObservableCollection<T>)")?;
    header.set_font_size(20.0)?;
    header.set_font_weight(600)?;
    section.add_child(&header.as_uielement())?;

    // Description
    let desc = XamlTextBlock::new()?;
    desc.set_text("ObservableCollection<T> notifies subscribers of add/remove/clear operations.")?;
    desc.set_font_size(14.0)?;
    section.add_child(&desc.as_uielement())?;

    // Todo count (automatically updates)
    let count_text = Arc::new(XamlTextBlock::new()?);
    count_text.set_font_size(16.0)?;

    todos.subscribe({
        let count_text = count_text.clone();
        let todos = todos.clone();
        move |_change| {
            let _ = count_text.set_text(&format!("Total todos: {}", todos.len()));
        }
    });

    // Initial count
    count_text.set_text(&format!("Total todos: {}", todos.len()))?;
    section.add_child(&count_text.as_uielement())?;

    // Todo list display
    let todo_list = Arc::new(XamlStackPanel::new()?);
    todo_list.set_orientation(Orientation::Vertical)?;
    todo_list.set_spacing(5.0)?;

    // Subscribe to collection changes to update UI
    todos.subscribe({
        let todo_list = todo_list.clone();
        let todos = todos.clone();
        move |change| {
            use CollectionChange::*;
            match change {
                Added { .. } | Removed { .. } | Cleared | Reset { .. } => {
                    // Rebuild the entire list (in a real app, you'd be more selective)
                    // For now, this demonstrates the concept
                    let _ = rebuild_todo_list(&todo_list, &todos);
                }
                _ => {}
            }
        }
    });

    section.add_child(&todo_list.as_uielement())?;

    // Add todo buttons
    let buttons = XamlStackPanel::new()?;
    buttons.set_orientation(Orientation::Horizontal)?;
    buttons.set_spacing(10.0)?;

    let add_btn1 = XamlButton::new()?;
    add_btn1.set_content("➕ Add 'Buy milk'")?;
    add_btn1.on_click({
        let todos = todos.clone();
        move || {
            todos.push("Buy milk".to_string());
            Ok(())
        }
    })?;
    buttons.add_child(&add_btn1.as_uielement())?;

    let add_btn2 = XamlButton::new()?;
    add_btn2.set_content("➕ Add 'Write code'")?;
    add_btn2.on_click({
        let todos = todos.clone();
        move || {
            todos.push("Write code".to_string());
            Ok(())
        }
    })?;
    buttons.add_child(&add_btn2.as_uielement())?;

    let clear_btn = XamlButton::new()?;
    clear_btn.set_content("🗑️ Clear All")?;
    clear_btn.on_click({
        let todos = todos.clone();
        move || {
            todos.clear();)
        }
    })?;
    buttons.add_child(&clear_btn.as_uielement())?;

    section.add_child(&buttons.as_uielement())?;
    parent.add_child(&section.as_uielement())?;

    Ok(())
}

fn rebuild_todo_list(list: &XamlStackPanel, todos: &ObservableCollection<String>) -> Result<()> {
    // Note: In a real implementation, you'd want to track child elements and only
    // add/remove what changed. This is a simplified demonstration.
    // For now, we just show the count since we can't easily remove children yet.
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


