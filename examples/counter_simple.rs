//! Simple counter example demonstrating Win32 button and text controls.
//!
//! This example shows:
//! - Button creation and click handling
//! - TextBlock for display
//! - Manual layout positioning
//! - Shared state management

use winrt_xaml::prelude::*;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Arc;

fn main() -> Result<()> {
    env_logger::init();

    println!("Creating counter application...");

    let app = Application::new()?;

    let window = Window::builder()
        .title("Counter Example")
        .size(400, 300)
        .build()?;

    // Shared counter state
    let counter = Arc::new(AtomicI32::new(0));

    // Create display label
    let label = TextBlock::new()?
        .with_text("Count: 0")?;
    label.set_position(150, 80);
    label.set_size(100, 30);

    // Create increment button
    let inc_button = Button::new()?
        .with_content("Increment")?;
    inc_button.set_position(100, 150);
    inc_button.set_size(80, 40);

    let counter_clone = counter.clone();
    let label_clone = label.clone();
    inc_button.click().subscribe(move |_| {
        let new_value = counter_clone.fetch_add(1, Ordering::SeqCst) + 1;
        let text = format!("Count: {}", new_value);
        let _ = label_clone.set_text(&text);
        println!("Counter incremented to: {}", new_value);
    });

    // Create decrement button
    let dec_button = Button::new()?
        .with_content("Decrement")?;
    dec_button.set_position(220, 150);
    dec_button.set_size(80, 40);

    let counter_clone = counter.clone();
    let label_clone = label.clone();
    dec_button.click().subscribe(move |_| {
        let new_value = counter_clone.fetch_sub(1, Ordering::SeqCst) - 1;
        let text = format!("Count: {}", new_value);
        let _ = label_clone.set_text(&text);
        println!("Counter decremented to: {}", new_value);
    });

    // Create reset button
    let reset_button = Button::new()?
        .with_content("Reset")?;
    reset_button.set_position(160, 200);
    reset_button.set_size(80, 40);

    let counter_clone = counter.clone();
    let label_clone = label.clone();
    reset_button.click().subscribe(move |_| {
        counter_clone.store(0, Ordering::SeqCst);
        let _ = label_clone.set_text("Count: 0");
        println!("Counter reset to 0");
    });

    // Set the first button as window content (others are children)
    window.set_content(inc_button)?;

    println!("Starting application...");
    app.run()
}

