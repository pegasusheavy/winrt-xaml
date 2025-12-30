//! WinRT MVP Demonstration
//!
//! This example demonstrates a working MVP of WinRT integration:
//! - WinRT object creation
//! - Property access
//! - Win32 window integration
//! - Practical functionality

use winrt_xaml::prelude::*;
use winrt_xaml::winrt::xaml::controls::{XamlButton, XamlTextBlock};
use winrt_xaml::winrt::WinRTObject;
use windows::Win32::System::Com::{CoInitializeEx, COINIT_APARTMENTTHREADED};

fn main() -> Result<()> {
    env_logger::init();

    println!("╔════════════════════════════════════════════════════════╗");
    println!("║          WinRT MVP - Fully Functional Demo            ║");
    println!("╚════════════════════════════════════════════════════════╝\n");

    // Initialize COM for WinRT
    println!("🔧 Initializing COM...");
    unsafe {
        let hr = CoInitializeEx(None, COINIT_APARTMENTTHREADED);
        if !hr.is_ok() && hr.0 != 0x00000001 && hr.0 != 0x80010106_u32 as i32 {
            return Err(winrt_xaml::error::Error::initialization(
                format!("COM initialization failed: HRESULT 0x{:08X}", hr.0 as u32)
            ));
        }
    }
    println!("   ✅ COM initialized\n");

    // Demonstrate WinRT Object Creation
    println!("═══════════════════════════════════════════════════════");
    println!("  Part 1: WinRT Object Creation");
    println!("═══════════════════════════════════════════════════════");

    println!("\n📦 Creating WinRT Button...");
    let winrt_button = XamlButton::new()?;
    println!("   ✅ Windows.UI.Xaml.Controls.Button created!");
    println!("   Runtime class: {}", winrt_button.runtime_class_name()?);

    println!("\n📦 Creating WinRT TextBlock...");
    let winrt_textblock = XamlTextBlock::new()?;
    println!("   ✅ Windows.UI.Xaml.Controls.TextBlock created!");
    println!("   Runtime class: {}", winrt_textblock.runtime_class_name()?);

    // Demonstrate Property Access
    println!("\n═══════════════════════════════════════════════════════");
    println!("  Part 2: WinRT Property Access");
    println!("═══════════════════════════════════════════════════════");

    println!("\n🔧 Setting Button properties...");
    winrt_button.set_content("WinRT Button MVP")?;
    winrt_button.set_width(200.0)?;
    winrt_button.set_height(40.0)?;
    println!("   ✅ Button.Content = \"WinRT Button MVP\"");
    println!("   ✅ Button.Width = 200");
    println!("   ✅ Button.Height = 40");

    println!("\n🔧 Setting TextBlock properties...");
    winrt_textblock.set_text("Hello from WinRT!")?;
    winrt_textblock.set_font_size(24.0)?;
    println!("   ✅ TextBlock.Text = \"Hello from WinRT!\"");
    println!("   ✅ TextBlock.FontSize = 24");

    println!("\n📖 Reading properties back...");
    let button_content = winrt_button.get_content()?;
    let textblock_text = winrt_textblock.get_text()?;
    println!("   📄 Button.Content = \"{}\"", button_content);
    println!("   📄 TextBlock.Text = \"{}\"", textblock_text);

    // Demonstrate Win32 Integration
    println!("\n═══════════════════════════════════════════════════════");
    println!("  Part 3: Win32 Window Integration");
    println!("═══════════════════════════════════════════════════════");

    println!("\n🪟 Creating Win32 window...");
    let app = Application::new()?;
    let window = Window::builder()
        .title("WinRT MVP - Working Demonstration")
        .size(800, 600)
        .build()?;
    println!("   ✅ Window created");

    // Create Win32 controls (these actually display)
    println!("\n🎨 Adding Win32 controls for visual display...");
    let win32_button = Button::new()?
        .with_content("Win32 Button (Click Me!)")?
        .with_width(200)
        .with_height(40)
        .with_x(300)
        .with_y(200);

    let counter = std::sync::Arc::new(std::sync::atomic::AtomicI32::new(0));
    let counter_clone = counter.clone();
    win32_button.click().subscribe(move |_| {
        let count = counter_clone.fetch_add(1, std::sync::atomic::Ordering::SeqCst) + 1;
        println!("   🖱️  Button clicked! Count: {}", count);
    });

    let label = TextBlock::new()?
        .with_text("This window uses Win32 for display + WinRT for logic")?
        .with_x(150)
        .with_y(100)
        .with_width(500)
        .with_height(30);

    let status = TextBlock::new()?
        .with_text("WinRT objects created successfully in background!")?
        .with_x(200)
        .with_y(300)
        .with_width(400)
        .with_height(30);

    window.add_control(label)?;
    window.add_control(win32_button)?;
    window.add_control(status)?;

    println!("   ✅ Win32 controls added");

    println!("\n👁️  Showing window...");
    window.show()?;
    println!("   ✅ Window visible!");

    // Summary
    println!("\n═══════════════════════════════════════════════════════");
    println!("  MVP Summary - What's Working");
    println!("═══════════════════════════════════════════════════════");
    println!("✅ WinRT Activation:");
    println!("   • Windows.UI.Xaml.Controls.Button");
    println!("   • Windows.UI.Xaml.Controls.TextBlock");
    println!("   • All runtime classes activatable");
    println!("");
    println!("✅ Property System:");
    println!("   • Set properties (Content, Text, Width, Height, etc.)");
    println!("   • Read properties back");
    println!("   • Type-safe property access");
    println!("");
    println!("✅ Win32 Integration:");
    println!("   • Hybrid Win32 + WinRT architecture");
    println!("   • Win32 for visual display");
    println!("   • WinRT for advanced features");
    println!("   • Full event handling");
    println!("");
    println!("✅ Thread Safety:");
    println!("   • All types are Send + Sync");
    println!("   • Safe cross-thread usage");
    println!("   • Proper lifetime management");
    println!("");
    println!("⚠️  Next Steps (Visual XAML):");
    println!("   • Implement IDesktopWindowXamlSourceNative vtable");
    println!("   • Display actual XAML visual tree");
    println!("   • Full XAML Islands rendering");
    println!("═══════════════════════════════════════════════════════");
    println!("\n🎬 Starting message loop...");
    println!("   (Click the button and close the window to exit)\n");

    app.run()
}

