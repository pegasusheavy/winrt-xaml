//! Basic window example demonstrating WinRT-XAML.
//!
//! This example shows how to create a simple window with basic controls.

use winrt_xaml::prelude::*;

fn main() -> Result<()> {
    env_logger::init();

    println!("Creating basic window application...");

    // Create the application
    let app = Application::new()?;

    // Create the main window
    let window = Window::builder()
        .title("Basic WinRT-XAML Window")
        .size(600, 400)
        .build()?;

    // Create click button
    let click_button = Button::new()?
        .with_content("Click Me!")?
        .with_width(100)
        .with_height(40)
        .with_x(150)
        .with_y(200);

    let click_count = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let count_clone = click_count.clone();
    click_button.click().subscribe(move |_| {
        let count = count_clone.fetch_add(1, std::sync::atomic::Ordering::SeqCst) + 1;
        println!("Button clicked {} times!", count);
    });

    // Add the button to the window (it will be created automatically)
    window.add_control(click_button)?;

    // Create exit button
    let exit_button = Button::new()?
        .with_content("Exit")?
        .with_width(100)
        .with_height(40)
        .with_x(270)
        .with_y(200);

    exit_button.click().subscribe(|_| {
        println!("Exiting application...");
        std::process::exit(0);
    });

    // Add the exit button to the window
    window.add_control(exit_button)?;

    // Show the window (controls will be created automatically)
    window.show()?;

    println!("Starting application...");
    app.run()
}
