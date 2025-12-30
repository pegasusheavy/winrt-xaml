//! WinRT Activation Test
//!
//! This example attempts to activate Windows.UI.Xaml runtime classes
//! to test if WinRT XAML is available on the system.

use winrt_xaml::winrt::xaml::*;
use winrt_xaml::error::Result;

fn main() -> Result<()> {
    env_logger::init();

    println!("=================================");
    println!("  WinRT XAML Activation Test");
    println!("=================================\n");

    println!("This test attempts to activate Windows.UI.Xaml runtime classes.");
    println!("Note: This may fail if:");
    println!("  1. Running outside a UWP app container");
    println!("  2. XAML Islands not properly initialized");
    println!("  3. Windows 10/11 XAML runtime not available\n");

    // Test 1: Application
    println!("📦 Test 1: Activating Windows.UI.Xaml.Application...");
    match XamlApplication::new() {
        Ok(app) => {
            println!("   ✅ SUCCESS! Windows.UI.Xaml.Application activated");
            println!("   ℹ️  This means WinRT XAML is available on your system!\n");

            // Try to get current instance
            println!("📦 Test 1b: Getting Application.Current...");
            match XamlApplication::current() {
                Ok(_) => println!("   ✅ Application.Current works\n"),
                Err(e) => println!("   ⚠️  Application.Current failed: {}\n", e),
            }
        }
        Err(e) => {
            println!("   ❌ FAILED: {}", e);
            println!("   ℹ️  This is expected if not running in UWP context\n");
        }
    }

    // Test 2: Window
    println!("📦 Test 2: Activating Windows.UI.Xaml.Window...");
    match XamlWindow::new() {
        Ok(window) => {
            println!("   ✅ SUCCESS! Windows.UI.Xaml.Window activated");

            // Try to set title
            println!("   📝 Setting window title...");
            match window.set_title("WinRT Test Window") {
                Ok(_) => println!("   ✅ Title set successfully"),
                Err(e) => println!("   ⚠️  Failed to set title: {}", e),
            }

            // Try to activate
            println!("   🪟 Activating window...");
            match window.activate() {
                Ok(_) => println!("   ✅ Window activated\n"),
                Err(e) => println!("   ⚠️  Failed to activate: {}\n", e),
            }
        }
        Err(e) => {
            println!("   ❌ FAILED: {}", e);
            println!("   ℹ️  This is expected if not running in UWP context\n");
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
            println!("   ℹ️  This is expected if not running in UWP context\n");
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
            println!("   ℹ️  This is expected if not running in UWP context\n");
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
            println!("   ℹ️  This is expected if not running in UWP context\n");
        }
    }

    // Test 6: Grid
    println!("📦 Test 6: Activating Windows.UI.Xaml.Controls.Grid...");
    match panels::XamlGrid::new() {
        Ok(_grid) => {
            println!("   ✅ SUCCESS! Grid activated\n");
        }
        Err(e) => {
            println!("   ❌ FAILED: {}", e);
            println!("   ℹ️  This is expected if not running in UWP context\n");
        }
    }

    println!("=================================");
    println!("  Test Summary");
    println!("=================================");
    println!("If all tests FAILED:");
    println!("  • You're likely not in a UWP app container");
    println!("  • XAML Islands not initialized");
    println!("  • This is EXPECTED for desktop Win32 apps");
    println!("");
    println!("If any tests SUCCEEDED:");
    println!("  • WinRT XAML is available!");
    println!("  • You can use native XAML controls");
    println!("  • The library has true WinRT support");
    println!("=================================\n");

    Ok(())
}

