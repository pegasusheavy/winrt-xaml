//! Basic window example demonstrating WinRT-XAML.
//!
//! This example shows how to create a simple window with basic controls.

use winrt_xaml::prelude::*;

fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();

    // Create the application
    let app = Application::new()?;

    // Create a window
    let window = Window::builder()
        .title("Basic WinRT-XAML Window")
        .size(800, 600)
        .build()?;

    // Create the UI using the fluent builder API
    let content = StackPanel::new()
        .orientation(Orientation::Vertical)
        .spacing(20.0)
        .padding_uniform(30.0)
        .child(
            TextBlock::new()
                .text("Welcome to WinRT-XAML!")
                .font_size(32.0)
                .font_weight(FontWeight::Bold)
                .horizontal_alignment(HorizontalAlignment::Center),
        )
        .child(
            TextBlock::new()
                .text("This is a simple example demonstrating the library.")
                .font_size(16.0)
                .text_wrapping(TextWrapping::Wrap)
                .horizontal_alignment(HorizontalAlignment::Center),
        )
        .child(
            StackPanel::new()
                .orientation(Orientation::Horizontal)
                .spacing(10.0)
                .horizontal_alignment(HorizontalAlignment::Center)
                .child(
                    Button::new()
                        .content("Click Me!")
                        .padding_uniform(15.0)
                        .on_click(|_| {
                            println!("Button clicked!");
                        }),
                )
                .child(
                    Button::new()
                        .content("Exit")
                        .padding_uniform(15.0)
                        .on_click(|_| {
                            if let Some(app) = Application::current() {
                                app.exit();
                            }
                        }),
                ),
        );

    // Set the window content
    window.set_content(content)?;

    // Center and show the window
    window.center()?;
    window.show()?;

    // Run the application
    app.run()
}
