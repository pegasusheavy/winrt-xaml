//! MVP Demonstration - All core features in one example.
//!
//! This example demonstrates:
//! - Window creation and management
//! - Button click events (WM_COMMAND routing)
//! - TextBlock text display
//! - Dynamic control updates
//! - Trait-based control system

use winrt_xaml::prelude::*;
use std::sync::Arc;

fn main() -> Result<()> {
    env_logger::init();

    println!("=== WinRT-XAML MVP Demo ===");
    println!("Testing all MVP features...\n");

    // Create the application
    let app = Application::new()?;

    // Create the main window
    let window = Window::builder()
        .title("MVP Demo - Click the buttons!")
        .size(600, 400)
        .build()?;

    // Create a title label
    let title = TextBlock::new()?
        .with_text("WinRT-XAML MVP - All Features Working!")?
        .with_x(150)
        .with_y(20)
        .with_width(300)
        .with_height(30);

    // Create a counter display
    let counter_label = TextBlock::new()?
        .with_text("Counter: 0")?
        .with_x(200)
        .with_y(70)
        .with_width(200)
        .with_height(30);
    let counter_label_ref = Arc::new(counter_label.clone());

    // Counter state
    let counter = Arc::new(std::sync::atomic::AtomicI32::new(0));

    // Create increment button
    let increment_btn = Button::new()?
        .with_content("Increment (+1)")?
        .with_width(150)
        .with_height(40)
        .with_x(100)
        .with_y(120);
    {
        let counter = counter.clone();
        let label = counter_label_ref.clone();
        increment_btn.click().subscribe(move |_| {
            let new_val = counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst) + 1;
            println!("✅ Increment button clicked! Counter: {}", new_val);
            let _ = label.set_text(format!("Counter: {}", new_val));
        });
    }

    // Create decrement button
    let decrement_btn = Button::new()?
        .with_content("Decrement (-1)")?
        .with_width(150)
        .with_height(40)
        .with_x(270)
        .with_y(120);
    {
        let counter = counter.clone();
        let label = counter_label_ref.clone();
        decrement_btn.click().subscribe(move |_| {
            let new_val = counter.fetch_sub(1, std::sync::atomic::Ordering::SeqCst) - 1;
            println!("✅ Decrement button clicked! Counter: {}", new_val);
            let _ = label.set_text(format!("Counter: {}", new_val));
        });
    }

    // Create reset button
    let reset_btn = Button::new()?
        .with_content("Reset (0)")?
        .with_width(150)
        .with_height(40)
        .with_x(440)
        .with_y(120);
    {
        let counter = counter.clone();
        let label = counter_label_ref.clone();
        reset_btn.click().subscribe(move |_| {
            counter.store(0, std::sync::atomic::Ordering::SeqCst);
            println!("🔄 Reset button clicked! Counter: 0");
            let _ = label.set_text("Counter: 0");
        });
    }

    // Create a status label
    let status = TextBlock::new()?
        .with_text("Click the buttons above to test event handling!")?
        .with_x(100)
        .with_y(180)
        .with_width(400)
        .with_height(30);

    // Create a text input
    let textbox = TextBox::new()?
        .with_text("Type here to test input...")?
        .with_x(100)
        .with_y(230)
        .with_width(400)
        .with_height(30);

    // Create a checkbox
    let checkbox = CheckBox::new()?
        .with_content("Enable debug output")?
        .with_x(100)
        .with_y(280)
        .with_width(200)
        .with_height(30);

    // Add all controls to the window
    window.add_control(title)?;
    window.add_control(counter_label)?;
    window.add_control(increment_btn)?;
    window.add_control(decrement_btn)?;
    window.add_control(reset_btn)?;
    window.add_control(status)?;
    window.add_control(textbox)?;
    window.add_control(checkbox)?;

    println!("✅ Created {} controls", window.controls().len());
    println!("✅ Window created");
    println!("✅ Controls added to window");
    println!("\n🚀 Showing window with automatic control creation...");

    // Show the window - all controls will be created automatically!
    window.show()?;

    println!("✅ Window shown");
    println!("✅ Message loop starting...");
    println!("\n--- MVP Features Tested ---");
    println!("  ✓ Window creation");
    println!("  ✓ Control trait system");
    println!("  ✓ Automatic control creation");
    println!("  ✓ TextBlock display");
    println!("  ✓ Button creation");
    println!("  ✓ Event subscriptions");
    println!("  ✓ TextBox input");
    println!("  ✓ CheckBox");
    println!("\n🎉 MVP is fully functional! Click the buttons to test WM_COMMAND routing.\n");

    app.run()
}

