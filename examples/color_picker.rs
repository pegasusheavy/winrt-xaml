//! Color picker example.

use winrt_xaml::prelude::*;
use std::sync::Arc;
use parking_lot::RwLock;

fn main() -> Result<()> {
    env_logger::init();

    let app = Application::new()?;

    let window = Window::builder()
        .title("Color Picker")
        .size(450, 500)
        .build()?;

    let color_state = Arc::new(RwLock::new(ColorState { r: 128, g: 128, b: 128 }));

    // Title
    let title = TextBlock::new()?
        .with_text("RGB Color Picker")?;
    title.set_position(140, 20);
    title.set_size(170, 30);

    // Red label
    let red_label = TextBlock::new()?
        .with_text("Red: 128")?;
    red_label.set_position(50, 80);
    red_label.set_size(100, 25);

    // Red slider
    let red_slider = Slider::new()?
        .with_minimum(0.0)
        .with_maximum(255.0)
        .with_value(128.0);

    let state_clone = color_state.clone();
    let label_clone = red_label.clone();
    red_slider.value_changed().subscribe(move |args| {
        let value = args.new_value as u8;
        state_clone.write().r = value;
        let _ = label_clone.set_text(&format!("Red: {}", value));
        println!("Red: {}", value);
    });

    // Green label
    let green_label = TextBlock::new()?
        .with_text("Green: 128")?;
    green_label.set_position(50, 150);
    green_label.set_size(100, 25);

    // Green slider
    let green_slider = Slider::new()?
        .with_minimum(0.0)
        .with_maximum(255.0)
        .with_value(128.0);

    let state_clone = color_state.clone();
    let label_clone = green_label.clone();
    green_slider.value_changed().subscribe(move |args| {
        let value = args.new_value as u8;
        state_clone.write().g = value;
        let _ = label_clone.set_text(&format!("Green: {}", value));
        println!("Green: {}", value);
    });

    // Blue label
    let blue_label = TextBlock::new()?
        .with_text("Blue: 128")?;
    blue_label.set_position(50, 220);
    blue_label.set_size(100, 25);

    // Blue slider
    let blue_slider = Slider::new()?
        .with_minimum(0.0)
        .with_maximum(255.0)
        .with_value(128.0);

    let state_clone = color_state.clone();
    let label_clone = blue_label.clone();
    blue_slider.value_changed().subscribe(move |args| {
        let value = args.new_value as u8;
        state_clone.write().b = value;
        let _ = label_clone.set_text(&format!("Blue: {}", value));
        println!("Blue: {}", value);
    });

    // Color display (text representation)
    let color_display = TextBlock::new()?
        .with_text("RGB(128, 128, 128)")?;
    color_display.set_position(50, 300);
    color_display.set_size(350, 40);

    // Copy button
    let copy_button = Button::new()?
        .with_content("Copy RGB")?;
    copy_button.set_position(50, 360);
    copy_button.set_size(120, 40);

    let state_clone = color_state.clone();
    let display_clone = color_display.clone();
    copy_button.click().subscribe(move |_| {
        let state = state_clone.read();
        let rgb_text = format!("RGB({}, {}, {})", state.r, state.g, state.b);
        let _ = display_clone.set_text(&rgb_text);
        println!("Color: {}", rgb_text);
    });

    window.set_content(copy_button)?;

    // Show the window
    window.show()?;

    println!("Starting application...");
    app.run()
}

struct ColorState {
    r: u8,
    g: u8,
    b: u8,
}
