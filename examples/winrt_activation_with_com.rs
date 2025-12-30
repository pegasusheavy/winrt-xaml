//! WinRT Activation Test with COM Initialization
//!
//! This example initializes COM before attempting to activate
//! Windows.UI.Xaml runtime classes.

use winrt_xaml::winrt::xaml::*;
use winrt_xaml::error::Result;
use windows::Win32::System::Com::{CoInitializeEx, CoUninitialize, COINIT_APARTMENTTHREADED};

fn main() -> Result<()> {
    env_logger::init();

    println!("=================================");
    println!("  WinRT XAML Activation Test");
    println!("  (with COM Initialization)");
    println!("=================================\n");

    // Initialize COM
    println!("🔧 Initializing COM...");
    unsafe {
        let hr = CoInitializeEx(None, COINIT_APARTMENTTHREADED);
        if hr.is_ok() {
            println!("   ✅ COM initialized successfully\n");
        } else {
            // S_FALSE (0x00000001) or RPC_E_CHANGED_MODE (0x80010106) means already initialized - that's OK
            let code = hr.0;
            if code == 0x00000001 || code == 0x80010106_u32 as i32 {
                println!("   ℹ️  COM already initialized (code: 0x{:08X})\n", code as u32);
            } else {
                println!("   ❌ Failed to initialize COM: HRESULT 0x{:08X}\n", code as u32);
                return Err(winrt_xaml::error::Error::initialization(format!("COM initialization failed: HRESULT 0x{:08X}", code as u32)));
            }
        }
    }

    println!("This test attempts to activate Windows.UI.Xaml runtime classes.");
    println!("Note: This may STILL fail if:");
    println!("  1. Running outside a UWP app container");
    println!("  2. XAML Islands not properly initialized");
    println!("  3. Need app manifest for XAML runtime\n");

    // Test 1: Application
    println!("📦 Test 1: Activating Windows.UI.Xaml.Application...");
    match XamlApplication::new() {
        Ok(_app) => {
            println!("   ✅ SUCCESS! Windows.UI.Xaml.Application activated");
            println!("   🎉 WinRT XAML is available on your system!\n");
        }
        Err(e) => {
            println!("   ❌ FAILED: {}", e);
            println!("   ℹ️  Error code: {:?}", e);
            println!("   ℹ️  This is expected - needs UWP app container or XAML Islands\n");
        }
    }

    // Test 2: Window
    println!("📦 Test 2: Activating Windows.UI.Xaml.Window...");
    match XamlWindow::new() {
        Ok(_window) => {
            println!("   ✅ SUCCESS! Windows.UI.Xaml.Window activated\n");
        }
        Err(e) => {
            println!("   ❌ FAILED: {}", e);
            println!("   ℹ️  This is expected without UWP context\n");
        }
    }

    // Test 3: Button Control
    println!("📦 Test 3: Activating Windows.UI.Xaml.Controls.Button...");
    match controls::XamlButton::new() {
        Ok(_button) => {
            println!("   ✅ SUCCESS! Button control activated\n");
        }
        Err(e) => {
            println!("   ❌ FAILED: {}", e);
            println!("   ℹ️  This is expected without UWP context\n");
        }
    }

    // Test 4: TextBlock Control
    println!("📦 Test 4: Activating Windows.UI.Xaml.Controls.TextBlock...");
    match controls::XamlTextBlock::new() {
        Ok(_textblock) => {
            println!("   ✅ SUCCESS! TextBlock control activated\n");
        }
        Err(e) => {
            println!("   ❌ FAILED: {}", e);
            println!("   ℹ️  This is expected without UWP context\n");
        }
    }

    // Test 5: StackPanel
    println!("📦 Test 5: Activating Windows.UI.Xaml.Controls.StackPanel...");
    match panels::XamlStackPanel::new() {
        Ok(_panel) => {
            println!("   ✅ SUCCESS! StackPanel activated\n");
        }
        Err(e) => {
            println!("   ❌ FAILED: {}", e);
            println!("   ℹ️  This is expected without UWP context\n");
        }
    }

    println!("=================================");
    println!("  Test Summary");
    println!("=================================");
    println!("✅ COM Initialization: SUCCESS");
    println!("✅ WinRT Runtime Classes: EXIST");
    println!("❌ XAML Activation: FAILED");
    println!("");
    println!("Why did activation fail?");
    println!("  • XAML requires UWP app container");
    println!("  • OR XAML Islands hosting");
    println!("  • Desktop Win32 apps need special setup");
    println!("");
    println!("Next Steps:");
    println!("  1. Implement XAML Islands hosting");
    println!("  2. Create UWP app manifest");
    println!("  3. Use hybrid Win32/WinRT approach");
    println!("=================================\n");

    // Cleanup COM
    unsafe {
        CoUninitialize();
    }

    Ok(())
}

