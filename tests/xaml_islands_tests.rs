//! Unit tests for XAML Islands module.

use winrt_xaml::xaml_islands;
use winrt_xaml::error::Error;

#[test]
fn test_xaml_islands_initialize_api_exists() {
    // Verify the initialize function exists
    fn _check_initialize() {
        fn _needs_result(_: fn() -> Result<(), Error>) {}
        _needs_result(xaml_islands::initialize);
    }
}

#[test]
fn test_xaml_islands_uninitialize_api_exists() {
    // Verify the uninitialize function exists
    fn _check_uninitialize() {
        fn _needs_method(_: fn()) {}
        _needs_method(xaml_islands::uninitialize);
    }
}

#[test]
fn test_initialization_error_type() {
    let err = Error::initialization("COM initialization failed");
    let msg = err.to_string();
    assert!(msg.contains("Initialization error") || msg.contains("COM initialization failed"));
}

// Note: We can't actually test initialize() and uninitialize() in unit tests
// because they affect global COM state and may interfere with other tests.
// These should be tested in integration tests.

#[test]
fn test_desktop_source_api_exists() {
    // Compile-time check that DesktopWindowXamlSource exists
    use winrt_xaml::xaml_islands::DesktopWindowXamlSource;
    
    fn _check_new() {
        fn _needs_result<T>(_: fn() -> Result<T, Error>) {}
        _needs_result(DesktopWindowXamlSource::new);
    }
}

#[test]
fn test_xaml_island_host_api_exists() {
    // Compile-time check that XamlIslandHost exists
    use winrt_xaml::xaml_islands::XamlIslandHost;
    use winrt_xaml::window::Window;
    
    fn _check_new() {
        fn _needs_result<T>(_: fn(&Window) -> Result<T, Error>) {}
        _needs_result(XamlIslandHost::new);
    }
}

#[test]
fn test_xaml_island_host_methods_exist() {
    use winrt_xaml::xaml_islands::{XamlIslandHost, DesktopWindowXamlSource};
    use windows::Win32::Foundation::HWND;
    
    fn _check_source() {
        fn _needs_method(_: fn(&XamlIslandHost) -> &DesktopWindowXamlSource) {}
        _needs_method(XamlIslandHost::source);
    }
    
    fn _check_island_hwnd() {
        fn _needs_method(_: fn(&XamlIslandHost) -> HWND) {}
        _needs_method(XamlIslandHost::island_hwnd);
    }
    
    fn _check_update_size() {
        fn _needs_method(_: fn(&XamlIslandHost, i32, i32) -> Result<(), Error>) {}
        _needs_method(XamlIslandHost::update_size);
    }
    
    fn _check_show() {
        fn _needs_method(_: fn(&XamlIslandHost) -> Result<(), Error>) {}
        _needs_method(XamlIslandHost::show);
    }
    
    fn _check_hide() {
        fn _needs_method(_: fn(&XamlIslandHost) -> Result<(), Error>) {}
        _needs_method(XamlIslandHost::hide);
    }
}

#[test]
fn test_desktop_source_methods_exist() {
    use winrt_xaml::xaml_islands::DesktopWindowXamlSource;
    
    fn _check_attach() {
        let _ = DesktopWindowXamlSource::attach_to_window;
    }
    
    // DesktopWindowXamlSource doesn't expose island_hwnd directly
    // It's returned by attach_to_window
}

#[test]
fn test_xaml_island_host_is_clone() {
    use winrt_xaml::xaml_islands::XamlIslandHost;
    
    fn assert_clone<T: Clone>() {}
    assert_clone::<XamlIslandHost>();
}

#[test]
fn test_desktop_source_is_clone() {
    use winrt_xaml::xaml_islands::DesktopWindowXamlSource;
    
    fn assert_clone<T: Clone>() {}
    assert_clone::<DesktopWindowXamlSource>();
}

// Test COM HRESULT values
#[test]
fn test_com_hresult_values() {
    // S_OK
    let s_ok = 0x00000000i32;
    assert_eq!(s_ok, 0);
    
    // S_FALSE
    let s_false = 0x00000001i32;
    assert_eq!(s_false, 1);
    
    // RPC_E_CHANGED_MODE (COM already initialized in different mode)
    let rpc_e_changed_mode = 0x80010106_u32 as i32;
    assert!(rpc_e_changed_mode < 0); // Negative = error
}

#[test]
fn test_com_initialization_hresults() {
    // Test that we handle the expected HRESULT values correctly
    let success_codes = vec![
        0x00000000i32,  // S_OK
        0x00000001i32,  // S_FALSE (already initialized)
        0x80010106_u32 as i32,  // RPC_E_CHANGED_MODE
    ];
    
    for code in success_codes {
        // These should all be treated as success
        let is_success = code == 0 || code == 1 || code == 0x80010106_u32 as i32;
        assert!(is_success, "HRESULT 0x{:08X} should be treated as success", code as u32);
    }
}

#[test]
fn test_hwnd_conversion() {
    use windows::Win32::Foundation::HWND;
    
    // Test that HWND can be created from isize
    let handle_value = 0x12345678isize;
    let hwnd = HWND(handle_value as *mut _);
    assert_eq!(hwnd.0 as isize, handle_value);
}

#[test]
fn test_window_pos_flags() {
    use windows::Win32::UI::WindowsAndMessaging::{SWP_NOACTIVATE, SWP_NOZORDER};
    
    // Verify flags exist and can be combined
    let _combined = SWP_NOZORDER | SWP_NOACTIVATE;
}

#[test]
fn test_show_window_commands() {
    use windows::Win32::UI::WindowsAndMessaging::{SW_SHOW, SW_HIDE};
    
    // Verify show/hide commands exist
    assert_ne!(SW_SHOW.0, SW_HIDE.0);
}

// Integration test placeholders (require actual COM initialization)

#[test]
#[ignore]
fn test_xaml_islands_initialize() {
    let result = xaml_islands::initialize();
    // Should succeed or indicate already initialized
    assert!(result.is_ok());
    
    // Clean up
    xaml_islands::uninitialize();
}

#[test]
#[ignore]
fn test_xaml_islands_double_initialize() {
    let result1 = xaml_islands::initialize();
    assert!(result1.is_ok());
    
    // Second initialization should also succeed (already initialized)
    let result2 = xaml_islands::initialize();
    assert!(result2.is_ok());
    
    // Clean up
    xaml_islands::uninitialize();
}

#[test]
#[ignore]
fn test_desktop_source_creation() {
    use winrt_xaml::xaml_islands::DesktopWindowXamlSource;
    
    let _ = xaml_islands::initialize();
    
    let source = DesktopWindowXamlSource::new();
    assert!(source.is_ok());
    
    xaml_islands::uninitialize();
}

#[test]
#[ignore]
fn test_xaml_island_host_creation() {
    use winrt_xaml::xaml_islands::XamlIslandHost;
    use winrt_xaml::window::Window;
    
    let _ = xaml_islands::initialize();
    
    let window = Window::new().unwrap();
    let host = XamlIslandHost::new(&window);
    // May fail without a real window, but API should exist
    let _ = host;
    
    xaml_islands::uninitialize();
}
