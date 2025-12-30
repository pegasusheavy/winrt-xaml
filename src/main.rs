//! Example application demonstrating WinRT-XAML.
//!
//! This binary requires the 'xaml-islands' or 'uwp' feature to be enabled.
//! Run with: cargo run --features xaml-islands

#[cfg(not(any(feature = "xaml-islands", feature = "uwp")))]
fn main() {
    eprintln!("ERROR: This binary requires the 'xaml-islands' or 'uwp' feature to be enabled.");
    eprintln!("Run with: cargo run --features xaml-islands");
    std::process::exit(1);
}

#[cfg(any(feature = "xaml-islands", feature = "uwp"))]
use winrt_xaml::prelude::*;

#[cfg(any(feature = "xaml-islands", feature = "uwp"))]
fn main() -> Result<()> {
    env_logger::init();

    let app = Application::new()?;

    let window = Window::builder()
        .title("WinRT-XAML Demo")
        .size(800, 600)
        .build()?;

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
                .text("This is a demonstration of the library.")
                .font_size(16.0)
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

    window.set_content(content)?;
    window.center()?;
    window.show()?;

    app.run()
}
