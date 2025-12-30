//! WinRT Functional Calculator Example
//!
//! A working calculator with actual button functionality demonstrating:
//! - Grid layout for button arrangement
//! - TextBox for display
//! - Click event handlers
//! - State management across button clicks

use winrt_xaml::error::Result;
use winrt_xaml::xaml_native::*;
use windows::core::w;
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::System::Com::{CoInitializeEx, CoUninitialize, COINIT_APARTMENTTHREADED};
use std::ptr;
use std::sync::{Arc, Mutex};

fn create_host_window() -> Result<HWND> {
    unsafe {
        let class_name = w!("WinRT_Calculator_Functional");
        let wc = WNDCLASSW {
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(window_proc),
            hInstance: GetModuleHandleW(None)?.into(),
            lpszClassName: class_name,
            hCursor: LoadCursorW(None, IDC_ARROW)?,
            ..Default::default()
        };
        let _ = RegisterClassW(&wc);

        CreateWindowExW(
            WINDOW_EX_STYLE(0),
            class_name,
            w!("WinRT Calculator (Functional)"),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT, CW_USEDEFAULT, 400, 650,
            None, None,
            GetModuleHandleW(None)?,
            Some(ptr::null()),
        ).map_err(|e| winrt_xaml::error::Error::window_creation(format!("{:?}", e)))
    }
}

unsafe extern "system" fn window_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg {
        WM_SIZE => {
            if let Ok(child) = GetWindow(hwnd, GW_CHILD) {
                if !child.0.is_null() {
                    let width = (lparam.0 & 0xFFFF) as i32;
                    let height = ((lparam.0 >> 16) & 0xFFFF) as i32;
                    let _ = SetWindowPos(child, None, 0, 0, width, height, SWP_NOZORDER | SWP_NOACTIVATE);
                }
            }
            LRESULT(0)
        }
        WM_DESTROY => {
            PostQuitMessage(0);
            LRESULT(0)
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}

// Calculator state
struct CalculatorState {
    display: String,
    current_value: f64,
    operation: Option<char>,
    new_number: bool,
}

impl CalculatorState {
    fn new() -> Self {
        CalculatorState {
            display: "0".to_string(),
            current_value: 0.0,
            operation: None,
            new_number: true,
        }
    }

    fn append_digit(&mut self, digit: &str) {
        if self.new_number {
            self.display = digit.to_string();
            self.new_number = false;
        } else {
            if self.display == "0" {
                self.display = digit.to_string();
            } else {
                self.display.push_str(digit);
            }
        }
    }

    fn set_operation(&mut self, op: char) {
        if let Ok(val) = self.display.parse::<f64>() {
            if let Some(current_op) = self.operation {
                self.current_value = match current_op {
                    '+' => self.current_value + val,
                    '-' => self.current_value - val,
                    '×' => self.current_value * val,
                    '÷' => if val != 0.0 { self.current_value / val } else { 0.0 },
                    _ => val,
                };
                self.display = self.current_value.to_string();
            } else {
                self.current_value = val;
            }
        }
        self.operation = Some(op);
        self.new_number = true;
    }

    fn calculate(&mut self) {
        if let Ok(val) = self.display.parse::<f64>() {
            if let Some(op) = self.operation {
                self.current_value = match op {
                    '+' => self.current_value + val,
                    '-' => self.current_value - val,
                    '×' => self.current_value * val,
                    '÷' => if val != 0.0 { self.current_value / val } else { 0.0 },
                    _ => val,
                };
                self.display = self.current_value.to_string();
                self.operation = None;
                self.new_number = true;
            }
        }
    }

    fn clear(&mut self) {
        self.display = "0".to_string();
        self.current_value = 0.0;
        self.operation = None;
        self.new_number = true;
    }
}

fn main() -> Result<()> {
    println!("\n╔═══════════════════════════════════════════════╗");
    println!("║   WinRT Functional Calculator - WORKING!    ║");
    println!("╚═══════════════════════════════════════════════╝\n");

    unsafe { CoInitializeEx(None, COINIT_APARTMENTTHREADED).ok(); }

    let _manager = XamlManager::new()?;
    let host_hwnd = create_host_window()?;
    let mut xaml_source = XamlSource::new()?;
    let island_hwnd = xaml_source.attach_to_window(host_hwnd)?;

    // Shared calculator state
    let state = Arc::new(Mutex::new(CalculatorState::new()));

    // Build calculator UI
    let main_panel = XamlStackPanel::new()?;
    main_panel.set_vertical(true)?;
    main_panel.set_spacing(15.0)?;
    main_panel.set_background(0xFF1E1E1E)?; // Dark theme
    main_panel.set_padding(25.0, 25.0, 25.0, 25.0)?;
    main_panel.set_corner_radius(12.0)?;

    // Title with modern styling
    let title = XamlTextBlock::new()?;
    title.set_text("🧮 Calculator")?;
    title.set_font_size(28.0)?;
    title.set_font_weight(700)?; // Bold
    title.set_foreground(0xFFFFFFFF)?; // White
    title.set_margin(0.0, 0.0, 0.0, 10.0)?;
    main_panel.add_child(&title.as_uielement())?;

    // Display with modern styling
    let display = Arc::new(XamlTextBox::new()?);
    display.set_text("0")?;
    display.set_size(350.0, 70.0)?;
    display.set_background(0xFF2D2D2D)?; // Darker gray
    display.set_foreground(0xFF00D4AA)?; // Cyan for display
    display.set_corner_radius(8.0)?;
    display.set_padding(15.0, 12.0, 15.0, 12.0)?;
    main_panel.add_child(&display.as_uielement())?;

    // Button rows
    let buttons = [
        ["7", "8", "9", "÷"],
        ["4", "5", "6", "×"],
        ["1", "2", "3", "-"],
        ["0", ".", "=", "+"],
    ];

    for row in &buttons {
        let row_panel = XamlStackPanel::new()?;
        row_panel.set_vertical(false)?;
        row_panel.set_spacing(5.0)?;

        for &label in row {
            let button = XamlButton::new()?;
            button.set_content(label)?;
            button.set_size(80.0, 64.0)?;
            
            // Style buttons based on type
            match label {
                "÷" | "×" | "-" | "+" => {
                    // Operator buttons - orange accent
                    button.set_background(0xFFFF8C00)?; // Dark orange
                    button.set_foreground(0xFFFFFFFF)?;
                }
                "=" => {
                    // Equals button - bright blue
                    button.set_background(0xFF0078D4)?; // Microsoft blue
                    button.set_foreground(0xFFFFFFFF)?;
                }
                _ => {
                    // Number buttons - dark gray
                    button.set_background(0xFF3A3A3A)?;
                    button.set_foreground(0xFFFFFFFF)?;
                }
            }
            button.set_corner_radius(8.0)?;
            button.set_padding(10.0, 8.0, 10.0, 8.0)?;

            // Clone Arc references for the closure
            let state_clone = Arc::clone(&state);
            let display_clone = Arc::clone(&display);
            let label_str = label.to_string();

            // Register click handler
            button.on_click(move || {
                let mut state = state_clone.lock().unwrap();

                match label_str.as_str() {
                    "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => {
                        state.append_digit(&label_str);
                    }
                    "." => {
                        if !state.display.contains('.') {
                            state.append_digit(".");
                        }
                    }
                    "+" | "-" | "×" | "÷" => {
                        state.set_operation(label_str.chars().next().unwrap());
                    }
                    "=" => {
                        state.calculate();
                    }
                    _ => {}
                }

                // Update display
                let _ = display_clone.set_text(&state.display);
            })?;

            row_panel.add_child(&button.as_uielement())?;
        }

        main_panel.add_child(&row_panel.as_uielement())?;
    }

    // Clear button
    let clear_btn = XamlButton::new()?;
    clear_btn.set_content("Clear (C)")?;
    clear_btn.set_size(350.0, 50.0)?;

    let state_clone = Arc::clone(&state);
    let display_clone = Arc::clone(&display);
    clear_btn.on_click(move || {
        let mut state = state_clone.lock().unwrap();
        state.clear();
        let _ = display_clone.set_text(&state.display);
    })?;

    main_panel.add_child(&clear_btn.as_uielement())?;

    // Info
    let info = XamlTextBlock::new()?;
    info.set_text("✨ Fully functional! Click the buttons!")?;
    info.set_font_size(12.0)?;
    main_panel.add_child(&info.as_uielement())?;

    xaml_source.set_content_element(&main_panel.as_uielement())?;

    // Show and size the island
    unsafe {
        let _ = ShowWindow(island_hwnd, SW_SHOW);
        let mut rect = windows::Win32::Foundation::RECT::default();
        let _ = GetClientRect(host_hwnd, &mut rect);
        let _ = SetWindowPos(island_hwnd, None, 0, 0,
            rect.right - rect.left, rect.bottom - rect.top,
            SWP_NOZORDER | SWP_NOACTIVATE);
    }

    println!("✅ Functional calculator ready!");
    println!("📊 Features:");
    println!("   • Working button click handlers");
    println!("   • Basic arithmetic operations");
    println!("   • Real-time display updates");
    println!("   • Fluent Design button styling");
    println!("🎬 Try it out! Close window to exit\n");

    // Message loop
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

