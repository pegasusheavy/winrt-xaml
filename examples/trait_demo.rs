//! Trait system demonstration - automatic control creation.
//!
//! This example shows the new trait-based system where controls
//! are automatically created when added to a window.

use winrt_xaml::prelude::*;

fn main() -> Result<()> {
    env_logger::init();

    println!("Trait System Demo - Automatic Control Creation");

    // Create the application
    let app = Application::new()?;

    // Create the main window
    let window = Window::builder()
        .title("Trait System Demo")
        .size(500, 400)
        .build()?;

    // Create a title label
    let title = TextBlock::new()?
        .with_text("Trait-Based Control System")?;
    title.element().set_x(100);
    title.element().set_y(50);
    title.element().set_width(300);
    title.element().set_height(30);

    // Create a button with event handler
    let button1 = Button::new()?
        .with_content("Button 1")?
        .with_width(120)
        .with_height(40)
        .with_x(100)
        .with_y(100);
    button1.click().subscribe(|_| {
        println!("Button 1 clicked!");
    });

    // Create another button
    let button2 = Button::new()?
        .with_content("Button 2")?
        .with_width(120)
        .with_height(40)
        .with_x(240)
        .with_y(100);
    button2.click().subscribe(|_| {
        println!("Button 2 clicked!");
    });

    // Create a text input
    let textbox = TextBox::new()?
        .with_text("Type here...")?;
    textbox.element().set_x(100);
    textbox.element().set_y(160);
    textbox.element().set_width(260);
    textbox.element().set_height(30);

    // Create a checkbox
    let checkbox = CheckBox::new()?
        .with_content("Enable feature")?;
    checkbox.element().set_x(100);
    checkbox.element().set_y(210);
    checkbox.element().set_width(150);
    checkbox.element().set_height(30);

    // Add all controls to the window
    // They will be automatically created when window.show() is called
    window.add_control(title)?;
    window.add_control(button1)?;
    window.add_control(button2)?;
    window.add_control(textbox)?;
    window.add_control(checkbox)?;

    println!("Added {} controls to window", window.controls().len());

    // Show the window - all controls will be created automatically!
    window.show()?;

    println!("Window shown with automatic control creation");
    println!("Starting message loop...");

    app.run()
}

