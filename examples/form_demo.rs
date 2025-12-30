//! Form Demo Example
//!
//! Demonstrates a registration form with validation and user feedback.
//!
//! Run with: cargo run --example form_demo

use winrt_xaml::prelude::*;
use std::sync::Arc;
use parking_lot::RwLock;

/// Form data
#[derive(Default, Clone)]
struct FormData {
    name: String,
    email: String,
    password: String,
    confirm_password: String,
    agree_terms: bool,
    newsletter: bool,
}

impl FormData {
    fn validate(&self) -> Vec<String> {
        let mut errors = Vec::new();

        if self.name.trim().is_empty() {
            errors.push("Name is required".to_string());
        } else if self.name.len() < 2 {
            errors.push("Name must be at least 2 characters".to_string());
        }

        if self.email.trim().is_empty() {
            errors.push("Email is required".to_string());
        } else if !self.email.contains('@') || !self.email.contains('.') {
            errors.push("Please enter a valid email address".to_string());
        }

        if self.password.is_empty() {
            errors.push("Password is required".to_string());
        } else if self.password.len() < 8 {
            errors.push("Password must be at least 8 characters".to_string());
        }

        if self.password != self.confirm_password {
            errors.push("Passwords do not match".to_string());
        }

        if !self.agree_terms {
            errors.push("You must agree to the terms".to_string());
        }

        errors
    }

    fn is_valid(&self) -> bool {
        self.validate().is_empty()
    }
}

fn main() -> Result<()> {
    env_logger::init();

    let app = Application::new()?;

    let window = Window::builder()
        .title("Registration Form Demo")
        .size(500, 650)
        .build()?;

    let form_data = Arc::new(RwLock::new(FormData::default()));

    let content = build_form_ui(form_data)?;

    window.set_content(content)?;
    window.center()?;
    window.show()?;

    app.run()
}

fn build_form_ui(form_data: Arc<RwLock<FormData>>) -> Result<UIElement> {
    let form_data_name = form_data.clone();
    let form_data_email = form_data.clone();
    let form_data_password = form_data.clone();
    let form_data_confirm = form_data.clone();
    let form_data_terms = form_data.clone();
    let form_data_newsletter = form_data.clone();
    let form_data_submit = form_data.clone();
    let form_data_clear = form_data.clone();

    Ok(ScrollViewer::new()
        .vertical_scroll_bar_visibility(ScrollBarVisibility::Auto)
        .content(
            StackPanel::new()
                .orientation(Orientation::Vertical)
                .padding_uniform(30.0)
                .spacing(20.0)
                // Header
                .child(
                    StackPanel::new()
                        .orientation(Orientation::Vertical)
                        .spacing(5.0)
                        .margin(Thickness::new(0.0, 0.0, 0.0, 10.0))
                        .child(
                            TextBlock::new()
                                .text("Create Account")
                                .font_size(28.0)
                                .font_weight(FontWeight::Bold),
                        )
                        .child(
                            TextBlock::new()
                                .text("Fill out the form below to create your account")
                                .font_size(14.0)
                                .foreground(&Brush::from_color(Color::GRAY)),
                        ),
                )
                // Name field
                .child(build_form_field(
                    "Full Name",
                    "Enter your full name",
                    false,
                    move |text| {
                        form_data_name.write().name = text;
                    },
                ))
                // Email field
                .child(build_form_field(
                    "Email Address",
                    "Enter your email address",
                    false,
                    move |text| {
                        form_data_email.write().email = text;
                    },
                ))
                // Password field
                .child(build_form_field(
                    "Password",
                    "Enter a password (min 8 characters)",
                    true,
                    move |text| {
                        form_data_password.write().password = text;
                    },
                ))
                // Confirm password field
                .child(build_form_field(
                    "Confirm Password",
                    "Re-enter your password",
                    true,
                    move |text| {
                        form_data_confirm.write().confirm_password = text;
                    },
                ))
                // Terms checkbox
                .child(
                    CheckBox::new()
                        .content("I agree to the Terms of Service and Privacy Policy")
                        .on_checked_changed(move |checked| {
                            form_data_terms.write().agree_terms = checked.unwrap_or(false);
                        }),
                )
                // Newsletter checkbox
                .child(
                    CheckBox::new()
                        .content("Subscribe to our newsletter for updates")
                        .on_checked_changed(move |checked| {
                            form_data_newsletter.write().newsletter = checked.unwrap_or(false);
                        }),
                )
                // Buttons
                .child(
                    StackPanel::new()
                        .orientation(Orientation::Horizontal)
                        .spacing(10.0)
                        .margin(Thickness::new(0.0, 10.0, 0.0, 0.0))
                        .child(
                            Button::new()
                                .content("Create Account")
                                .padding(Thickness::symmetric(30.0, 12.0))
                                .font_size(14.0)
                                .on_click(move |_| {
                                    let data = form_data_submit.read().clone();
                                    let errors = data.validate();

                                    if errors.is_empty() {
                                        println!("=== Form Submitted Successfully ===");
                                        println!("Name: {}", data.name);
                                        println!("Email: {}", data.email);
                                        println!("Newsletter: {}", data.newsletter);
                                        println!("===================================");
                                    } else {
                                        println!("=== Validation Errors ===");
                                        for error in &errors {
                                            println!("  - {}", error);
                                        }
                                        println!("=========================");
                                    }
                                }),
                        )
                        .child(
                            Button::new()
                                .content("Clear Form")
                                .padding(Thickness::symmetric(20.0, 12.0))
                                .font_size(14.0)
                                .on_click(move |_| {
                                    *form_data_clear.write() = FormData::default();
                                    println!("Form cleared");
                                }),
                        ),
                )
                // Password requirements info
                .child(
                    Border::new()
                        .background(&Brush::from_color(Color::rgb(240, 248, 255)))
                        .border_thickness_uniform(1.0)
                        .border_brush(&Brush::from_color(Color::rgb(173, 216, 230)))
                        .corner_radius_uniform(5.0)
                        .padding_uniform(15.0)
                        .margin(Thickness::new(0.0, 10.0, 0.0, 0.0))
                        .child(
                            StackPanel::new()
                                .orientation(Orientation::Vertical)
                                .spacing(5.0)
                                .child(
                                    TextBlock::new()
                                        .text("Password Requirements:")
                                        .font_size(12.0)
                                        .font_weight(FontWeight::SemiBold),
                                )
                                .child(
                                    TextBlock::new()
                                        .text("- At least 8 characters long")
                                        .font_size(12.0),
                                )
                                .child(
                                    TextBlock::new()
                                        .text("- Contains letters and numbers (recommended)")
                                        .font_size(12.0),
                                ),
                        ),
                ),
        )
        .into())
}

fn build_form_field<F>(
    label: &str,
    placeholder: &str,
    _is_password: bool,
    on_change: F,
) -> UIElement
where
    F: Fn(String) + Send + Sync + 'static,
{
    StackPanel::new()
        .orientation(Orientation::Vertical)
        .spacing(5.0)
        .child(
            TextBlock::new()
                .text(label)
                .font_size(14.0)
                .font_weight(FontWeight::Medium),
        )
        .child(
            TextBox::new()
                .placeholder(placeholder)
                .font_size(14.0)
                .horizontal_alignment(HorizontalAlignment::Stretch)
                .on_text_changed(move |text| {
                    on_change(text);
                }),
        )
        .into()
}
