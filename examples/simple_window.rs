//! Simple window example - minimal demonstration.
//!
//! This example shows a basic window with minimal functionality.

use winrt_xaml::prelude::*;

fn main() -> Result<()> {
    println!("Creating simple window...");

    // Create the application
    let app = Application::new()?;
    println!("Application created");

    // Create a window
    let window = Window::builder()
        .title("Simple WinRT-XAML Window")
        .size(800, 600)
        .build()?;

    println!("Window created");

    // Center and show the window
    window.center()?;
    window.show()?;

    println!("Window shown - starting message loop");

    // Run the application
    app.run()?;

    Ok(())
}

