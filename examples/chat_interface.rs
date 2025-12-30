//! Chat Interface Example
//!
//! A modern chat/messaging interface demonstrating dynamic lists,
//! user input, and message formatting.
//!
//! Run with: cargo run --example chat_interface

use winrt_xaml::prelude::*;
use std::sync::Arc;
use parking_lot::RwLock;
use std::time::{SystemTime, UNIX_EPOCH};

/// Message type
#[derive(Clone, Copy, PartialEq)]
enum MessageType {
    Sent,
    Received,
    System,
}

/// Chat message
#[derive(Clone)]
struct Message {
    id: u64,
    content: String,
    msg_type: MessageType,
    timestamp: String,
}

impl Message {
    fn new(content: String, msg_type: MessageType) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Simple time formatting
        let hours = ((now / 3600) % 24) as u32;
        let minutes = ((now / 60) % 60) as u32;
        let timestamp = format!("{:02}:{:02}", hours, minutes);

        Self {
            id: now,
            content,
            msg_type,
            timestamp,
        }
    }
}

/// Contact information
#[derive(Clone)]
struct Contact {
    name: String,
    status: String,
    last_seen: String,
    online: bool,
}

impl Contact {
    fn new(name: &str, status: &str, online: bool) -> Self {
        Self {
            name: name.to_string(),
            status: status.to_string(),
            last_seen: if online {
                "Online".to_string()
            } else {
                "Last seen recently".to_string()
            },
            online,
        }
    }
}

/// Chat app state
struct ChatState {
    messages: Vec<Message>,
    current_input: String,
    contact: Contact,
}

impl ChatState {
    fn new() -> Self {
        Self {
            messages: vec![
                Message::new("Hey! How are you?".to_string(), MessageType::Received),
                Message::new("I'm doing great, thanks for asking!".to_string(), MessageType::Sent),
                Message::new("That's awesome! Want to grab lunch?".to_string(), MessageType::Received),
                Message::new("Sure, what time works for you?".to_string(), MessageType::Sent),
                Message::new("Sarah joined the chat".to_string(), MessageType::System),
                Message::new("How about 12:30 PM?".to_string(), MessageType::Received),
            ],
            current_input: String::new(),
            contact: Contact::new("Alex Johnson", "Available", true),
        }
    }

    fn send_message(&mut self, content: String) {
        if !content.trim().is_empty() {
            self.messages.push(Message::new(content, MessageType::Sent));
            self.current_input.clear();
        }
    }

    fn simulate_received_message(&mut self) {
        let responses = vec![
            "That sounds great!",
            "Perfect! See you then.",
            "Thanks for letting me know!",
            "Looking forward to it!",
        ];

        let idx = (self.messages.len()) % responses.len();
        self.messages.push(Message::new(responses[idx].to_string(), MessageType::Received));
    }
}

fn main() -> Result<()> {
    env_logger::init();

    let app = Application::new()?;

    let window = Window::builder()
        .title("Chat Interface")
        .size(800, 650)
        .build()?;

    let state = Arc::new(RwLock::new(ChatState::new()));

    let content = build_chat_ui(state)?;

    window.set_content(content)?;
    window.center()?;
    window.show()?;

    app.run()
}

fn build_chat_ui(state: Arc<RwLock<ChatState>>) -> Result<UIElement> {
    Ok(Grid::new()
        .rows(vec![
            RowDefinition::auto(),      // Header
            RowDefinition::star(1.0),   // Messages
            RowDefinition::auto(),      // Input
        ])
        .child_at(build_chat_header(state.clone()), 0, 0)
        .child_at(build_messages_area(state.clone()), 1, 0)
        .child_at(build_input_area(state), 2, 0)
        .into())
}

fn build_chat_header(state: Arc<RwLock<ChatState>>) -> UIElement {
    let s = state.read();
    let status_color = if s.contact.online {
        Color::rgb(76, 175, 80) // Green
    } else {
        Color::GRAY
    };

    Border::new()
        .background(&Brush::from_color(Color::rgb(0, 120, 215)))
        .padding(Thickness::symmetric(25.0, 15.0))
        .border_thickness(Thickness::new(0.0, 0.0, 0.0, 1.0))
        .border_brush(&Brush::from_color(Color::rgb(0, 100, 200)))
        .child(
            Grid::new()
                .columns(vec![
                    ColumnDefinition::auto(),
                    ColumnDefinition::star(1.0),
                    ColumnDefinition::auto(),
                ])
                .column_spacing(15.0)
                // Profile picture
                .child_at(
                    Border::new()
                        .background(&Brush::from_color(Color::rgb(255, 255, 255)))
                        .corner_radius_uniform(25.0)
                        .width(50.0)
                        .height(50.0)
                        .child(
                            TextBlock::new()
                                .text("👤")
                                .font_size(28.0)
                                .horizontal_alignment(HorizontalAlignment::Center)
                                .vertical_alignment(VerticalAlignment::Center),
                        ),
                    0, 0
                )
                // Contact info
                .child_at(
                    StackPanel::new()
                        .orientation(Orientation::Vertical)
                        .spacing(4.0)
                        .vertical_alignment(VerticalAlignment::Center)
                        .child(
                            TextBlock::new()
                                .text(&s.contact.name)
                                .font_size(18.0)
                                .font_weight(FontWeight::SemiBold)
                                .foreground(&Brush::white()),
                        )
                        .child(
                            StackPanel::new()
                                .orientation(Orientation::Horizontal)
                                .spacing(6.0)
                                .child(
                                    Border::new()
                                        .background(&Brush::from_color(status_color))
                                        .corner_radius_uniform(4.0)
                                        .width(8.0)
                                        .height(8.0)
                                        .vertical_alignment(VerticalAlignment::Center),
                                )
                                .child(
                                    TextBlock::new()
                                        .text(&s.contact.last_seen)
                                        .font_size(12.0)
                                        .foreground(&Brush::from_color(Color::rgb(220, 235, 255))),
                                ),
                        ),
                    0, 1
                )
                // Action buttons
                .child_at(
                    StackPanel::new()
                        .orientation(Orientation::Horizontal)
                        .spacing(10.0)
                        .child(
                            Button::new()
                                .content("📞")
                                .font_size(20.0)
                                .padding(Thickness::symmetric(12.0, 8.0))
                                .on_click(|_| println!("Voice call")),
                        )
                        .child(
                            Button::new()
                                .content("📹")
                                .font_size(20.0)
                                .padding(Thickness::symmetric(12.0, 8.0))
                                .on_click(|_| println!("Video call")),
                        )
                        .child(
                            Button::new()
                                .content("ℹ️")
                                .font_size(20.0)
                                .padding(Thickness::symmetric(12.0, 8.0))
                                .on_click(|_| println!("Show info")),
                        ),
                    0, 2
                ),
        )
        .into()
}

fn build_messages_area(state: Arc<RwLock<ChatState>>) -> UIElement {
    let messages = state.read().messages.clone();

    Border::new()
        .background(&Brush::from_color(Color::rgb(240, 242, 245)))
        .child(
            ScrollViewer::new()
                .vertical_scroll_bar_visibility(ScrollBarVisibility::Auto)
                .padding(Thickness::symmetric(25.0, 20.0))
                .content({
                    let mut panel = StackPanel::new()
                        .orientation(Orientation::Vertical)
                        .spacing(12.0);

                    for message in messages {
                        panel = panel.child(build_message_bubble(&message));
                    }

                    panel
                }),
        )
        .into()
}

fn build_message_bubble(message: &Message) -> UIElement {
    match message.msg_type {
        MessageType::System => build_system_message(&message.content),
        MessageType::Sent => build_sent_message(message),
        MessageType::Received => build_received_message(message),
    }
}

fn build_system_message(content: &str) -> UIElement {
    Border::new()
        .padding(Thickness::symmetric(0.0, 10.0))
        .horizontal_alignment(HorizontalAlignment::Center)
        .child(
            Border::new()
                .background(&Brush::from_color(Color::rgb(200, 200, 200)))
                .corner_radius_uniform(12.0)
                .padding(Thickness::symmetric(12.0, 6.0))
                .child(
                    TextBlock::new()
                        .text(content)
                        .font_size(11.0)
                        .foreground(&Brush::white()),
                ),
        )
        .into()
}

fn build_sent_message(message: &Message) -> UIElement {
    StackPanel::new()
        .orientation(Orientation::Vertical)
        .spacing(4.0)
        .horizontal_alignment(HorizontalAlignment::Right)
        .child(
            Border::new()
                .background(&Brush::from_color(Color::rgb(0, 120, 215)))
                .corner_radius(CornerRadius::new(15.0, 15.0, 2.0, 15.0))
                .padding(Thickness::symmetric(15.0, 10.0))
                .max_width(400.0)
                .child(
                    TextBlock::new()
                        .text(&message.content)
                        .font_size(14.0)
                        .foreground(&Brush::white())
                        .text_wrapping(TextWrapping::Wrap),
                ),
        )
        .child(
            TextBlock::new()
                .text(&message.timestamp)
                .font_size(10.0)
                .foreground(&Brush::from_color(Color::rgb(150, 150, 150)))
                .margin(Thickness::new(0.0, 0.0, 8.0, 0.0))
                .horizontal_alignment(HorizontalAlignment::Right),
        )
        .into()
}

fn build_received_message(message: &Message) -> UIElement {
    StackPanel::new()
        .orientation(Orientation::Vertical)
        .spacing(4.0)
        .horizontal_alignment(HorizontalAlignment::Left)
        .child(
            Border::new()
                .background(&Brush::white())
                .corner_radius(CornerRadius::new(15.0, 15.0, 15.0, 2.0))
                .padding(Thickness::symmetric(15.0, 10.0))
                .max_width(400.0)
                .border_thickness_uniform(1.0)
                .border_brush(&Brush::from_color(Color::rgb(220, 220, 220)))
                .child(
                    TextBlock::new()
                        .text(&message.content)
                        .font_size(14.0)
                        .foreground(&Brush::from_color(Color::rgb(32, 32, 32)))
                        .text_wrapping(TextWrapping::Wrap),
                ),
        )
        .child(
            TextBlock::new()
                .text(&message.timestamp)
                .font_size(10.0)
                .foreground(&Brush::from_color(Color::rgb(150, 150, 150)))
                .margin(Thickness::new(8.0, 0.0, 0.0, 0.0))
                .horizontal_alignment(HorizontalAlignment::Left),
        )
        .into()
}

fn build_input_area(state: Arc<RwLock<ChatState>>) -> UIElement {
    let state_send = state.clone();
    let state_simulate = state.clone();
    let state_input = state.clone();

    Border::new()
        .background(&Brush::white())
        .border_thickness(Thickness::new(0.0, 1.0, 0.0, 0.0))
        .border_brush(&Brush::from_color(Color::rgb(220, 220, 220)))
        .padding_uniform(20.0)
        .child(
            Grid::new()
                .columns(vec![
                    ColumnDefinition::auto(),
                    ColumnDefinition::star(1.0),
                    ColumnDefinition::auto(),
                    ColumnDefinition::auto(),
                ])
                .column_spacing(12.0)
                // Attachment button
                .child_at(
                    Button::new()
                        .content("📎")
                        .font_size(20.0)
                        .padding(Thickness::symmetric(12.0, 10.0))
                        .on_click(|_| println!("Attach file")),
                    0, 0
                )
                // Text input
                .child_at(
                    TextBox::new()
                        .placeholder("Type a message...")
                        .font_size(14.0)
                        .padding(Thickness::symmetric(15.0, 12.0))
                        .on_text_changed(move |text| {
                            state_input.write().current_input = text;
                        }),
                    0, 1
                )
                // Emoji button
                .child_at(
                    Button::new()
                        .content("😊")
                        .font_size(20.0)
                        .padding(Thickness::symmetric(12.0, 10.0))
                        .on_click(|_| println!("Open emoji picker")),
                    0, 2
                )
                // Send button
                .child_at(
                    Button::new()
                        .content("Send ➤")
                        .padding(Thickness::symmetric(20.0, 12.0))
                        .font_size(14.0)
                        .on_click(move |_| {
                            let mut s = state_send.write();
                            let message = s.current_input.clone();
                            if !message.trim().is_empty() {
                                s.send_message(message.clone());
                                println!("Sent: {}", message);

                                // Simulate a response after a delay
                                state_simulate.write().simulate_received_message();
                            }
                        }),
                    0, 3
                ),
        )
        .into()
}

