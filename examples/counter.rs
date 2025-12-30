//! Simple Counter Example
//!
//! A minimal example showing state management with buttons.
//!
//! Run with: cargo run --example counter

use winrt_xaml::prelude::*;
use std::sync::Arc;
use parking_lot::RwLock;

fn main() -> Result<()> {
    env_logger::init();

    let app = Application::new()?;

    let window = Window::builder()
        .title("Counter - WinRT-XAML")
        .size(400, 300)
        .build()?;

    // Shared counter state
    let counter = Arc::new(RwLock::new(0i32));

    // Clone for each button
    let counter_inc = counter.clone();
    let counter_dec = counter.clone();
    let counter_reset = counter.clone();
    let counter_display = counter.clone();

    let content = StackPanel::new()
        .orientation(Orientation::Vertical)
        .spacing(20.0)
        .padding_uniform(40.0)
        .horizontal_alignment(HorizontalAlignment::Center)
        .vertical_alignment(VerticalAlignment::Center)
        // Title
        .child(
            TextBlock::new()
                .text("Simple Counter")
                .font_size(28.0)
                .font_weight(FontWeight::Bold)
                .horizontal_alignment(HorizontalAlignment::Center),
        )
        // Counter display
        .child(
            Border::new()
                .background(&Brush::from_color(Color::rgb(240, 240, 240)))
                .corner_radius_uniform(10.0)
                .padding(Thickness::symmetric(40.0, 20.0))
                .child(
                    TextBlock::new()
                        .text(&format!("{}", *counter_display.read()))
                        .font_size(48.0)
                        .font_weight(FontWeight::Light)
                        .horizontal_alignment(HorizontalAlignment::Center),
                ),
        )
        // Buttons row
        .child(
            StackPanel::new()
                .orientation(Orientation::Horizontal)
                .spacing(10.0)
                .horizontal_alignment(HorizontalAlignment::Center)
                // Decrement button
                .child(
                    Button::new()
                        .content("-")
                        .width(60.0)
                        .height(60.0)
                        .font_size(24.0)
                        .on_click(move |_| {
                            let mut count = counter_dec.write();
                            *count -= 1;
                            println!("Counter: {}", *count);
                        }),
                )
                // Reset button
                .child(
                    Button::new()
                        .content("Reset")
                        .width(80.0)
                        .height(60.0)
                        .font_size(14.0)
                        .on_click(move |_| {
                            let mut count = counter_reset.write();
                            *count = 0;
                            println!("Counter reset to 0");
                        }),
                )
                // Increment button
                .child(
                    Button::new()
                        .content("+")
                        .width(60.0)
                        .height(60.0)
                        .font_size(24.0)
                        .on_click(move |_| {
                            let mut count = counter_inc.write();
                            *count += 1;
                            println!("Counter: {}", *count);
                        }),
                ),
        )
        // Instructions
        .child(
            TextBlock::new()
                .text("Click + or - to change the counter")
                .font_size(12.0)
                .foreground(&Brush::from_color(Color::GRAY))
                .horizontal_alignment(HorizontalAlignment::Center),
        );

    window.set_content(content)?;
    window.center()?;
    window.show()?;

    app.run()
}
