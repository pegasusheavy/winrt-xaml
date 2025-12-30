//! Data binding example (simplified for Win32 implementation).

use winrt_xaml::prelude::*;
use std::sync::Arc;
use parking_lot::RwLock;

fn main() -> Result<()> {
    env_logger::init();

    println!("Creating data binding demo...");

    let app = Application::new()?;

    let window = Window::builder()
        .title("Data Binding Demo")
        .size(500, 400)
        .build()?;

    // Shared data model
    let model = Arc::new(RwLock::new(DataModel {
        name: "John Doe".to_string(),
        age: 25,
        email: "john@example.com".to_string(),
    }));

    // Title
    let title = TextBlock::new()?
        .with_text("Data Binding Example")?;
    title.set_position(140, 20);
    title.set_size(220, 30);

    // Name label
    let name_label = TextBlock::new()?
        .with_text("Name:")?;
    name_label.set_position(50, 80);
    name_label.set_size(80, 25);

    // Name input
    let name_input = TextBox::new()?;
    name_input.set_position(140, 80);
    name_input.set_size(300, 30);
    name_input.set_text(&model.read().name)?;

    let model_clone = model.clone();
    name_input.text_changed().subscribe(move |args| {
        model_clone.write().name = args.text.clone();
        println!("Name updated: {}", args.text);
    });

    // Age label
    let age_label = TextBlock::new()?
        .with_text("Age:")?;
    age_label.set_position(50, 130);
    age_label.set_size(80, 25);

    // Age input
    let age_input = TextBox::new()?;
    age_input.set_position(140, 130);
    age_input.set_size(100, 30);
    age_input.set_text(&model.read().age.to_string())?;

    let model_clone = model.clone();
    age_input.text_changed().subscribe(move |args| {
        if let Ok(age) = args.text.parse::<i32>() {
            model_clone.write().age = age;
            println!("Age updated: {}", age);
        }
    });

    // Email label
    let email_label = TextBlock::new()?
        .with_text("Email:")?;
    email_label.set_position(50, 180);
    email_label.set_size(80, 25);

    // Email input
    let email_input = TextBox::new()?;
    email_input.set_position(140, 180);
    email_input.set_size(300, 30);
    email_input.set_text(&model.read().email)?;

    let model_clone = model.clone();
    email_input.text_changed().subscribe(move |args| {
        model_clone.write().email = args.text.clone();
        println!("Email updated: {}", args.text);
    });

    // Display button
    let display_button = Button::new()?
        .with_content("Show Data")?;
    display_button.set_position(140, 240);
    display_button.set_size(120, 40);

    let model_clone = model.clone();
    display_button.click().subscribe(move |_| {
        let data = model_clone.read();
        println!("\n=== Current Data ===");
        println!("Name: {}", data.name);
        println!("Age: {}", data.age);
        println!("Email: {}", data.email);
        println!("===================\n");
    });

    // Info text
    let info = TextBlock::new()?
        .with_text("Data is automatically synced as you type")?;
    info.set_position(50, 310);
    info.set_size(400, 25);

    window.set_content(display_button)?;

    // Show the window
    window.show()?;

    println!("Starting application...");
    app.run()
}

struct DataModel {
    name: String,
    age: i32,
    email: String,
}
