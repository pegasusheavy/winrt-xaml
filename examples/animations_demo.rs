//! Demonstration of WinRT animations and storyboards.
//!
//! This example shows how to use the animation system to create smooth transitions
//! and visual effects.

use winrt_xaml::prelude::*;
use winrt_xaml::xaml_native::{
    XamlButton, XamlDoubleAnimation, XamlManager, XamlSource, XamlStackPanel, XamlStoryboard,
    XamlTextBlock,
};
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, DefWindowProcW, DispatchMessageW, GetMessageW, LoadCursorW, PostQuitMessage,
    RegisterClassW, ShowWindow, TranslateMessage, CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT,
    IDC_ARROW, MSG, SW_SHOW, WINDOW_EX_STYLE, WM_DESTROY, WM_SIZE, WNDCLASSW, WS_OVERLAPPEDWINDOW,
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
        let class_name = w!("WinRTAnimationsDemo");

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
            w!("WinRT Animations Demo"),
            WS_OVERLAPPEDWINDOW,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            600,
            500,
            None,
            None,
            h_instance,
            None,
        )?;

        // Initialize XAML
        let _manager = XamlManager::new()?;
        let source = XamlSource::new()?;
        let island_hwnd = source.attach_to_window(hwnd)?;

        // Create UI
        let root = XamlStackPanel::new()?;
        root.set_orientation(Orientation::Vertical)?;
        root.set_spacing(20.0)?;
        root.as_uielement().set_padding(30.0, 30.0, 30.0, 30.0)?;
        root.as_uielement().set_background(0xFF1E1E1E)?; // Dark background

        // Title
        let title = XamlTextBlock::new()?;
        title.set_text("🎬 WinRT Animation System")?;
        title.set_font_size(32.0)?;
        title.set_font_weight(700)?;
        title.as_uielement().set_foreground(0xFFFFFFFF)?;
        root.add_child(&title)?;

        // Description
        let desc = XamlTextBlock::new()?;
        desc.set_text("Click buttons to see smooth animations in action")?;
        desc.set_font_size(16.0)?;
        desc.as_uielement().set_foreground(0xFFAAAAAA)?;
        root.add_child(&desc)?;

        // Animated button 1 - Fade animation
        let fade_button = XamlButton::new()?;
        fade_button.set_content("Fade Animation")?;
        fade_button.set_size(200.0, 50.0)?;
        fade_button.as_uielement().set_background(0xFF0078D4)?;
        fade_button.as_uielement().set_foreground(0xFFFFFFFF)?;
        fade_button.as_uielement().set_corner_radius(5.0)?;
        fade_button.as_uielement().set_padding(15.0, 10.0, 15.0, 10.0)?;

        // Create fade animation
        let fade_animation = XamlDoubleAnimation::builder()
            .from(1.0)
            .to(0.0)
            .duration_ms(500)
            .build()?;

        fade_animation.set_target_property(fade_button.as_uielement().handle(), "Opacity")?;

        let fade_storyboard = XamlStoryboard::new()?;
        fade_storyboard.add_animation(&fade_animation)?;
        fade_storyboard.set_target(fade_button.as_uielement().handle())?;

        fade_button.on_click({
            move || {
                let _ = fade_storyboard.begin();
            }
        })?;

        root.add_child(&fade_button)?;

        // Animated button 2 - Scale animation (simulated with width)
        let scale_button = XamlButton::new()?;
        scale_button.set_content("Scale Animation")?;
        scale_button.set_size(200.0, 50.0)?;
        scale_button.as_uielement().set_background(0xFF107C10)?;
        scale_button.as_uielement().set_foreground(0xFFFFFFFF)?;
        scale_button.as_uielement().set_corner_radius(5.0)?;
        scale_button.as_uielement().set_padding(15.0, 10.0, 15.0, 10.0)?;

        // Create scale animation (animating width)
        let scale_animation = XamlDoubleAnimation::builder()
            .from(200.0)
            .to(300.0)
            .duration_ms(300)
            .build()?;

        scale_animation.set_target_property(scale_button.as_uielement().handle(), "Width")?;

        let scale_storyboard = XamlStoryboard::new()?;
        scale_storyboard.add_animation(&scale_animation)?;
        scale_storyboard.set_target(scale_button.as_uielement().handle())?;

        scale_button.on_click({
            move || {
                let _ = scale_storyboard.begin();
            }
        })?;

        root.add_child(&scale_button)?;

        // Control buttons
        let controls = XamlStackPanel::new()?;
        controls.set_orientation(Orientation::Horizontal)?;
        controls.set_spacing(10.0)?;

        let pause_btn = XamlButton::new()?;
        pause_btn.set_content("Pause All")?;
        pause_btn.set_size(100.0, 40.0)?;
        pause_btn.as_uielement().set_background(0xFFF7630C)?;
        pause_btn.as_uielement().set_foreground(0xFFFFFFFF)?;
        pause_btn.as_uielement().set_corner_radius(5.0)?;

        let resume_btn = XamlButton::new()?;
        resume_btn.set_content("Resume")?;
        resume_btn.set_size(100.0, 40.0)?;
        resume_btn.as_uielement().set_background(0xFF8764B8)?;
        resume_btn.as_uielement().set_foreground(0xFFFFFFFF)?;
        resume_btn.as_uielement().set_corner_radius(5.0)?;

        let stop_btn = XamlButton::new()?;
        stop_btn.set_content("Stop All")?;
        stop_btn.set_size(100.0, 40.0)?;
        stop_btn.as_uielement().set_background(0xFFE81123)?;
        stop_btn.as_uielement().set_foreground(0xFFFFFFFF)?;
        stop_btn.as_uielement().set_corner_radius(5.0)?;

        // Wire up control buttons
        let fade_sb_clone = XamlStoryboard::new()?;
        let scale_sb_clone = XamlStoryboard::new()?;

        pause_btn.on_click({
            let fade_sb = fade_sb_clone;
            let scale_sb = scale_sb_clone;
            move || {
                let _ = fade_sb.pause();
                let _ = scale_sb.pause();
            }
        })?;

        let fade_sb_clone2 = XamlStoryboard::new()?;
        let scale_sb_clone2 = XamlStoryboard::new()?;

        resume_btn.on_click({
            let fade_sb = fade_sb_clone2;
            let scale_sb = scale_sb_clone2;
            move || {
                let _ = fade_sb.resume();
                let _ = scale_sb.resume();
            }
        })?;

        let fade_sb_clone3 = XamlStoryboard::new()?;
        let scale_sb_clone3 = XamlStoryboard::new()?;

        stop_btn.on_click({
            let fade_sb = fade_sb_clone3;
            let scale_sb = scale_sb_clone3;
            move || {
                let _ = fade_sb.stop();
                let _ = scale_sb.stop();
            }
        })?;

        controls.add_child(&pause_btn)?;
        controls.add_child(&resume_btn)?;
        controls.add_child(&stop_btn)?;

        root.add_child(&controls)?;

        // Info text
        let info = XamlTextBlock::new()?;
        info.set_text(
            "💡 Animations use WinRT Storyboard and DoubleAnimation\n\
             Smooth 60 FPS animations powered by Windows composition engine"
        )?;
        info.set_font_size(14.0)?;
        info.as_uielement().set_foreground(0xFF888888)?;
        info.as_uielement().set_margin(0.0, 20.0, 0.0, 0.0)?;
        root.add_child(&info)?;

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
        WM_SIZE => {
            // Resize XAML island to match window
            LRESULT(0)
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}
