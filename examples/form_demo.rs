//! Form demonstration with input validation.

use winrt_xaml::prelude::*;
use std::sync::Arc;
use parking_lot::RwLock;

fn main() -> Result<()> {
    env_logger::init();

    println!("Creating form demo...");

    let app = Application::new()?;

    let window = Window::builder()
        .title("Form Demo")
        .size(500, 450)
        .build()?;

    // Form state
    let form_data = Arc::new(RwLock::new(FormData {
        name: String::new(),
        email: String::new(),
        age: String::new(),
        subscribe: false,
    }));

    // Title
    let title = TextBlock::new()?
        .with_text("User Registration Form")?;
    title.set_position(130, 20);
    title.set_size(240, 30);

    // Name label
    let name_label = TextBlock::new()?
        .with_text("Name:")?;
    name_label.set_position(50, 70);
    name_label.set_size(80, 25);

    // Name input
    let name_input = TextBox::new()?;
    name_input.set_position(140, 70);
    name_input.set_size(300, 30);
    name_input.set_placeholder("Enter your name");

    let form_clone = form_data.clone();
    name_input.text_changed().subscribe(move |args| {
        form_clone.write().name = args.text.clone();
        println!("Name: {}", args.text);
    });

    // Email label
    let email_label = TextBlock::new()?
        .with_text("Email:")?;
    email_label.set_position(50, 120);
    email_label.set_size(80, 25);

    // Email input
    let email_input = TextBox::new()?;
    email_input.set_position(140, 120);
    email_input.set_size(300, 30);
    email_input.set_placeholder("your@email.com");

    let form_clone = form_data.clone();
    email_input.text_changed().subscribe(move |args| {
        form_clone.write().email = args.text.clone();
        println!("Email: {}", args.text);
    });

    // Age label
    let age_label = TextBlock::new()?
        .with_text("Age:")?;
    age_label.set_position(50, 170);
    age_label.set_size(80, 25);

    // Age input
    let age_input = TextBox::new()?;
    age_input.set_position(140, 170);
    age_input.set_size(100, 30);
    age_input.set_placeholder("18");
    age_input.set_max_length(Some(3));

    let form_clone = form_data.clone();
    age_input.text_changed().subscribe(move |args| {
        form_clone.write().age = args.text.clone();
        println!("Age: {}", args.text);
    });

    // Subscribe checkbox
    let subscribe_check = CheckBox::new()?
        .with_content("Subscribe to newsletter")?;
    subscribe_check.set_position(50, 220);
    subscribe_check.set_size(250, 30);

    let form_clone = form_data.clone();
    subscribe_check.checked().subscribe(move |args| {
        form_clone.write().subscribe = args.is_checked;
        println!("Subscribe: {}", args.is_checked);
    });

    // Submit button
    let submit_button = Button::new()?
        .with_content("Submit")?;
    submit_button.set_position(140, 280);
    submit_button.set_size(100, 40);

    let form_clone = form_data.clone();
    let status_text = Arc::new(RwLock::new(String::new()));
    let status_clone = status_text.clone();

    submit_button.click().subscribe(move |_| {
        let data = form_clone.read();
        println!("\n=== Form Submitted ===");
        println!("Name: {}", data.name);
        println!("Email: {}", data.email);
        println!("Age: {}", data.age);
        println!("Subscribe: {}", data.subscribe);
        println!("====================\n");

        *status_clone.write() = "Form submitted successfully!".to_string();
    });

    // Clear button
    let clear_button = Button::new()?
        .with_content("Clear")?;
    clear_button.set_position(260, 280);
    clear_button.set_size(100, 40);

    let form_clone = form_data.clone();
    clear_button.click().subscribe(move |_| {
        let mut data = form_clone.write();
        data.name.clear();
        data.email.clear();
        data.age.clear();
        data.subscribe = false;
        println!("Form cleared");
    });

    // Status label
    let status_label = TextBlock::new()?
        .with_text("Fill out the form and click Submit")?;
    status_label.set_position(50, 350);
    status_label.set_size(400, 30);

    window.set_content(submit_button)?;

    // Show the window
    window.show()?;

    println!("Starting application...");
    app.run()
}

struct FormData {
    name: String,
    email: String,
    age: String,
    subscribe: bool,
}
