//! Chat interface example.

use winrt_xaml::prelude::*;
use std::sync::Arc;
use parking_lot::RwLock;

fn main() -> Result<()> {
    env_logger::init();

    let app = Application::new()?;

    let window = Window::builder()
        .title("Chat Interface")
        .size(500, 600)
        .build()?;

    let messages = Arc::new(RwLock::new(Vec::<String>::new()));

    // Title
    let title = TextBlock::new()?
        .with_text("Chat Room")?;
    title.set_position(190, 20);
    title.set_size(120, 30);

    // Messages display (simplified - would need scrollable list)
    let messages_display = TextBlock::new()?
        .with_text("No messages yet")?;
    messages_display.set_position(50, 70);
    messages_display.set_size(400, 350);

    // Message input
    let message_input = TextBox::new()?;
    message_input.set_position(50, 450);
    message_input.set_size(300, 35);
    message_input.set_placeholder("Type a message...");

    // Send button
    let send_button = Button::new()?
        .with_content("Send")?;
    send_button.set_position(370, 450);
    send_button.set_size(80, 35);

    let messages_clone = messages.clone();
    let input_clone = message_input.clone();
    let display_clone = messages_display.clone();
    send_button.click().subscribe(move |_| {
        let text = input_clone.text();
        if !text.is_empty() {
            let mut msgs = messages_clone.write();
            msgs.push(format!("You: {}", text));

            // Show last 5 messages
            let recent: Vec<_> = msgs.iter().rev().take(5).rev().cloned().collect();
            let display_text = recent.join("\n");
            let _ = display_clone.set_text(&display_text);

            let _ = input_clone.set_text("");
            println!("Sent: {}", text);
        }
    });

    // Clear button
    let clear_button = Button::new()?
        .with_content("Clear")?;
    clear_button.set_position(50, 510);
    clear_button.set_size(100, 35);

    let messages_clone = messages.clone();
    let display_clone = messages_display.clone();
    clear_button.click().subscribe(move |_| {
        messages_clone.write().clear();
        let _ = display_clone.set_text("No messages yet");
        println!("Chat cleared");
    });

    // Info
    let info = TextBlock::new()?
        .with_text("Simplified chat - no networking")?;
    info.set_position(150, 560);
    info.set_size(200, 25);

    window.set_content(send_button)?;

    // Show the window
    window.show()?;

    println!("Starting application...");
    app.run()
}
