//! Simple reactive data binding example.
//!
//! This example demonstrates Property<T> for automatic UI updates.
//!
//! Run with: `cargo run --example reactive_binding_simple`

use std::sync::Arc;
use winrt_xaml::prelude::*;
use windows::core::w;
use windows::Win32::Foundation::*;
use windows::Win32::System::Com::*;
use windows::Win32::System::LibraryLoader::*;
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::Win32::Graphics::Gdi::*;

fn main() -> Result<()> {
    println!("🔄 Simple Reactive Binding Example\n");

    unsafe {
        let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED);
        let _xaml_manager = XamlManager::new()?;

        let instance = GetModuleHandleW(None)?;
        let window_class = w!("ReactiveSimple");

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
            w!("Reactive Binding - Counter"),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            400,
            300,
            None,
            None,
            instance,
            None,
        ).expect("Failed to create window");

        let mut xaml_source = XamlSource::new()?;
        let island_hwnd = xaml_source.attach_to_window(host_hwnd)?;

        create_ui(&mut xaml_source)?;

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
    // Create reactive property
    let count = Property::new(0);

    let panel = XamlStackPanel::new()?;
    panel.set_orientation(Orientation::Vertical)?;
    panel.set_spacing(20.0)?;

    // Title
    let title = XamlTextBlock::new()?;
    title.set_text("Reactive Counter")?;
    title.set_font_size(24.0)?;
    title.set_font_weight(700)?;
    panel.add_child(&title.as_uielement())?;

    // Counter display (automatically updates!)
    let counter_text = Arc::new(XamlTextBlock::new()?);
    counter_text.set_font_size(32.0)?;
    counter_text.set_font_weight(700)?;

    // Bind text to count property
    count.subscribe({
        let counter_text = counter_text.clone();
        move |value| {
            let _ = counter_text.set_text(&format!("Count: {}", value));
        }
    });

    panel.add_child(&counter_text.as_uielement())?;

    // Buttons
    let buttons = XamlStackPanel::new()?;
    buttons.set_orientation(Orientation::Horizontal)?;
    buttons.set_spacing(10.0)?;

    let increment = XamlButton::new()?;
    increment.set_content("➕ Increment")?;
    increment.on_click({
        let count = count.clone();
        move || {
            count.update(|c| *c += 1);
        }
    })?;
    buttons.add_child(&increment.as_uielement())?;

    let decrement = XamlButton::new()?;
    decrement.set_content("➖ Decrement")?;
    decrement.on_click({
        let count = count.clone();
        move || {
            count.update(|c| *c -= 1);
        }
    })?;
    buttons.add_child(&decrement.as_uielement())?;

    let reset = XamlButton::new()?;
    reset.set_content("🔄 Reset")?;
    reset.on_click({
        let count = count.clone();
        move || {
            count.set(0);
        }
    })?;
    buttons.add_child(&reset.as_uielement())?;

    panel.add_child(&buttons.as_uielement())?;

    xaml_source.set_content_element(&panel.as_uielement())?;
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
