//! Counter example demonstrating state management.

use winrt_xaml::prelude::*;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Arc;

fn main() -> Result<()> {
    env_logger::init();

    println!("Creating counter application...");

    let app = Application::new()?;

    let window = Window::builder()
        .title("Counter Example")
        .size(500, 350)
        .build()?;

    // Shared counter state
    let counter = Arc::new(AtomicI32::new(0));

    // Title
    let title = TextBlock::new()?
        .with_text("Counter Application")?;
    title.set_position(150, 30);
    title.set_size(200, 30);

    // Counter display
    let display = TextBlock::new()?
        .with_text("Count: 0")?;
    display.set_position(180, 100);
    display.set_size(140, 40);

    // Increment button
    let inc_button = Button::new()?
        .with_content("+")?;
    inc_button.set_position(120, 180);
    inc_button.set_size(60, 40);

    let counter_clone = counter.clone();
    let display_clone = display.clone();
    inc_button.click().subscribe(move |_| {
        let new_value = counter_clone.fetch_add(1, Ordering::SeqCst) + 1;
        let _ = display_clone.set_text(&format!("Count: {}", new_value));
        println!("Incremented to: {}", new_value);
    });

    // Decrement button
    let dec_button = Button::new()?
        .with_content("-")?;
    dec_button.set_position(200, 180);
    dec_button.set_size(60, 40);

    let counter_clone = counter.clone();
    let display_clone = display.clone();
    dec_button.click().subscribe(move |_| {
        let new_value = counter_clone.fetch_sub(1, Ordering::SeqCst) - 1;
        let _ = display_clone.set_text(&format!("Count: {}", new_value));
        println!("Decremented to: {}", new_value);
    });

    // Reset button
    let reset_button = Button::new()?
        .with_content("Reset")?;
    reset_button.set_position(280, 180);
    reset_button.set_size(80, 40);

    let counter_clone = counter.clone();
    let display_clone = display.clone();
    reset_button.click().subscribe(move |_| {
        counter_clone.store(0, Ordering::SeqCst);
        let _ = display_clone.set_text("Count: 0");
        println!("Counter reset");
    });

    // Double button
    let double_button = Button::new()?
        .with_content("×2")?;
    double_button.set_position(150, 240);
    double_button.set_size(80, 40);

    let counter_clone = counter.clone();
    let display_clone = display.clone();
    double_button.click().subscribe(move |_| {
        let current = counter_clone.load(Ordering::SeqCst);
        let new_value = current * 2;
        counter_clone.store(new_value, Ordering::SeqCst);
        let _ = display_clone.set_text(&format!("Count: {}", new_value));
        println!("Doubled to: {}", new_value);
    });

    // Half button
    let half_button = Button::new()?
        .with_content("÷2")?;
    half_button.set_position(250, 240);
    half_button.set_size(80, 40);

    let counter_clone = counter.clone();
    let display_clone = display.clone();
    half_button.click().subscribe(move |_| {
        let current = counter_clone.load(Ordering::SeqCst);
        let new_value = current / 2;
        counter_clone.store(new_value, Ordering::SeqCst);
        let _ = display_clone.set_text(&format!("Count: {}", new_value));
        println!("Halved to: {}", new_value);
    });

    window.set_content(inc_button)?;

    // Show the window
    window.show()?;

    println!("Starting application...");
    app.run()
}
