//! Settings Panel Example
//!
//! A comprehensive settings interface demonstrating various controls
//! working together in a practical application.
//!
//! Run with: cargo run --example settings_panel

use winrt_xaml::prelude::*;
use std::sync::Arc;
use parking_lot::RwLock;

/// Application settings
#[derive(Clone)]
struct AppSettings {
    // General
    theme: String,
    language: String,
    startup_enabled: bool,

    // Appearance
    font_size: f64,
    opacity: f64,
    animations_enabled: bool,

    // Notifications
    notifications_enabled: bool,
    sound_enabled: bool,
    notification_position: String,

    // Privacy
    analytics_enabled: bool,
    crash_reports: bool,
    auto_update: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            theme: "Light".to_string(),
            language: "English".to_string(),
            startup_enabled: false,
            font_size: 14.0,
            opacity: 1.0,
            animations_enabled: true,
            notifications_enabled: true,
            sound_enabled: true,
            notification_position: "Top Right".to_string(),
            analytics_enabled: false,
            crash_reports: true,
            auto_update: true,
        }
    }
}

fn main() -> Result<()> {
    env_logger::init();

    let app = Application::new()?;

    let window = Window::builder()
        .title("Application Settings")
        .size(700, 800)
        .build()?;

    let settings = Arc::new(RwLock::new(AppSettings::default()));

    let content = build_settings_ui(settings.clone())?;

    window.set_content(content)?;
    window.center()?;
    window.show()?;

    app.run()
}

fn build_settings_ui(settings: Arc<RwLock<AppSettings>>) -> Result<UIElement> {
    Ok(Border::new()
        .background(&Brush::from_color(Color::rgb(250, 250, 250)))
        .child(
            ScrollViewer::new()
                .vertical_scroll_bar_visibility(ScrollBarVisibility::Auto)
                .content(
                    StackPanel::new()
                        .orientation(Orientation::Vertical)
                        .padding_uniform(30.0)
                        .spacing(25.0)
                        // Header
                        .child(build_header())
                        // General Settings
                        .child(build_settings_section(
                            "General",
                            "Configure general application behavior",
                            build_general_settings(settings.clone()),
                        ))
                        // Appearance Settings
                        .child(build_settings_section(
                            "Appearance",
                            "Customize how the application looks",
                            build_appearance_settings(settings.clone()),
                        ))
                        // Notification Settings
                        .child(build_settings_section(
                            "Notifications",
                            "Manage notification preferences",
                            build_notification_settings(settings.clone()),
                        ))
                        // Privacy Settings
                        .child(build_settings_section(
                            "Privacy & Security",
                            "Control your data and privacy",
                            build_privacy_settings(settings.clone()),
                        ))
                        // Action Buttons
                        .child(build_action_buttons(settings)),
                ),
        )
        .into())
}

fn build_header() -> UIElement {
    StackPanel::new()
        .orientation(Orientation::Vertical)
        .spacing(8.0)
        .margin(Thickness::new(0.0, 0.0, 0.0, 10.0))
        .child(
            TextBlock::new()
                .text("Settings")
                .font_size(32.0)
                .font_weight(FontWeight::Bold)
                .foreground(&Brush::from_color(Color::rgb(32, 32, 32))),
        )
        .child(
            TextBlock::new()
                .text("Customize your application experience")
                .font_size(14.0)
                .foreground(&Brush::from_color(Color::rgb(100, 100, 100))),
        )
        .into()
}

fn build_settings_section(title: &str, description: &str, content: UIElement) -> UIElement {
    Border::new()
        .background(&Brush::white())
        .border_thickness_uniform(1.0)
        .border_brush(&Brush::from_color(Color::rgb(220, 220, 220)))
        .corner_radius_uniform(10.0)
        .padding(Thickness::new(25.0, 20.0, 25.0, 20.0))
        .child(
            StackPanel::new()
                .orientation(Orientation::Vertical)
                .spacing(15.0)
                // Section Header
                .child(
                    StackPanel::new()
                        .orientation(Orientation::Vertical)
                        .spacing(4.0)
                        .child(
                            TextBlock::new()
                                .text(title)
                                .font_size(20.0)
                                .font_weight(FontWeight::SemiBold)
                                .foreground(&Brush::from_color(Color::rgb(32, 32, 32))),
                        )
                        .child(
                            TextBlock::new()
                                .text(description)
                                .font_size(12.0)
                                .foreground(&Brush::from_color(Color::rgb(128, 128, 128))),
                        ),
                )
                // Divider
                .child(
                    Border::new()
                        .height(1.0)
                        .background(&Brush::from_color(Color::rgb(230, 230, 230)))
                        .margin(Thickness::new(0.0, 5.0, 0.0, 5.0)),
                )
                // Content
                .child(content),
        )
        .into()
}

fn build_general_settings(settings: Arc<RwLock<AppSettings>>) -> UIElement {
    let settings_theme = settings.clone();
    let settings_lang = settings.clone();
    let settings_startup = settings.clone();

    StackPanel::new()
        .orientation(Orientation::Vertical)
        .spacing(18.0)
        // Theme
        .child(build_setting_row(
            "Theme",
            "Choose your preferred color theme",
            ComboBox::new()
                .width(200.0)
                .items(vec!["Light", "Dark", "System Default"])
                .selected_index(0)
                .on_selection_changed(move |index| {
                    let theme = match index {
                        Some(0) => "Light",
                        Some(1) => "Dark",
                        Some(2) => "System Default",
                        _ => "Light",
                    };
                    settings_theme.write().theme = theme.to_string();
                    println!("Theme changed to: {}", theme);
                })
                .into(),
        ))
        // Language
        .child(build_setting_row(
            "Language",
            "Select your preferred language",
            ComboBox::new()
                .width(200.0)
                .items(vec!["English", "Spanish", "French", "German", "Japanese", "Chinese"])
                .selected_index(0)
                .on_selection_changed(move |index| {
                    let lang = match index {
                        Some(0) => "English",
                        Some(1) => "Spanish",
                        Some(2) => "French",
                        Some(3) => "German",
                        Some(4) => "Japanese",
                        Some(5) => "Chinese",
                        _ => "English",
                    };
                    settings_lang.write().language = lang.to_string();
                    println!("Language changed to: {}", lang);
                })
                .into(),
        ))
        // Startup
        .child(build_setting_row(
            "Launch on Startup",
            "Automatically start when you log in",
            ToggleSwitch::new()
                .on_content("Enabled")
                .off_content("Disabled")
                .on_toggled(move |is_on| {
                    settings_startup.write().startup_enabled = is_on;
                    println!("Launch on startup: {}", is_on);
                })
                .into(),
        ))
        .into()
}

fn build_appearance_settings(settings: Arc<RwLock<AppSettings>>) -> UIElement {
    let settings_font = settings.clone();
    let settings_opacity = settings.clone();
    let settings_animations = settings.clone();

    let font_size = settings.read().font_size;
    let opacity = settings.read().opacity;

    StackPanel::new()
        .orientation(Orientation::Vertical)
        .spacing(18.0)
        // Font Size
        .child(build_slider_setting(
            "Font Size",
            &format!("{:.0} pt", font_size),
            10.0,
            24.0,
            font_size,
            move |value| {
                settings_font.write().font_size = value;
                println!("Font size: {:.0} pt", value);
            },
        ))
        // Window Opacity
        .child(build_slider_setting(
            "Window Opacity",
            &format!("{:.0}%", opacity * 100.0),
            0.3,
            1.0,
            opacity,
            move |value| {
                settings_opacity.write().opacity = value;
                println!("Opacity: {:.0}%", value * 100.0);
            },
        ))
        // Animations
        .child(build_setting_row(
            "Enable Animations",
            "Use smooth transitions and effects",
            ToggleSwitch::new()
                .is_on(true)
                .on_content("On")
                .off_content("Off")
                .on_toggled(move |is_on| {
                    settings_animations.write().animations_enabled = is_on;
                    println!("Animations: {}", is_on);
                })
                .into(),
        ))
        .into()
}

fn build_notification_settings(settings: Arc<RwLock<AppSettings>>) -> UIElement {
    let settings_notif = settings.clone();
    let settings_sound = settings.clone();
    let settings_position = settings.clone();

    StackPanel::new()
        .orientation(Orientation::Vertical)
        .spacing(18.0)
        // Enable Notifications
        .child(build_setting_row(
            "Enable Notifications",
            "Show desktop notifications",
            ToggleSwitch::new()
                .is_on(true)
                .on_content("Enabled")
                .off_content("Disabled")
                .on_toggled(move |is_on| {
                    settings_notif.write().notifications_enabled = is_on;
                    println!("Notifications: {}", is_on);
                })
                .into(),
        ))
        // Sound
        .child(build_setting_row(
            "Notification Sound",
            "Play sound with notifications",
            ToggleSwitch::new()
                .is_on(true)
                .on_content("On")
                .off_content("Off")
                .on_toggled(move |is_on| {
                    settings_sound.write().sound_enabled = is_on;
                    println!("Notification sound: {}", is_on);
                })
                .into(),
        ))
        // Position
        .child(build_setting_row(
            "Notification Position",
            "Where notifications appear on screen",
            ComboBox::new()
                .width(200.0)
                .items(vec!["Top Right", "Top Left", "Bottom Right", "Bottom Left"])
                .selected_index(0)
                .on_selection_changed(move |index| {
                    let position = match index {
                        Some(0) => "Top Right",
                        Some(1) => "Top Left",
                        Some(2) => "Bottom Right",
                        Some(3) => "Bottom Left",
                        _ => "Top Right",
                    };
                    settings_position.write().notification_position = position.to_string();
                    println!("Notification position: {}", position);
                })
                .into(),
        ))
        .into()
}

fn build_privacy_settings(settings: Arc<RwLock<AppSettings>>) -> UIElement {
    let settings_analytics = settings.clone();
    let settings_crash = settings.clone();
    let settings_update = settings.clone();

    StackPanel::new()
        .orientation(Orientation::Vertical)
        .spacing(18.0)
        // Analytics
        .child(build_setting_row(
            "Usage Analytics",
            "Help improve the app by sharing anonymous usage data",
            CheckBox::new()
                .on_checked_changed(move |checked| {
                    settings_analytics.write().analytics_enabled = checked.unwrap_or(false);
                    println!("Analytics: {}", checked.unwrap_or(false));
                })
                .into(),
        ))
        // Crash Reports
        .child(build_setting_row(
            "Crash Reports",
            "Automatically send crash reports to help us fix bugs",
            CheckBox::new()
                .checked(true)
                .on_checked_changed(move |checked| {
                    settings_crash.write().crash_reports = checked.unwrap_or(false);
                    println!("Crash reports: {}", checked.unwrap_or(false));
                })
                .into(),
        ))
        // Auto Update
        .child(build_setting_row(
            "Automatic Updates",
            "Keep the application up to date automatically",
            CheckBox::new()
                .checked(true)
                .on_checked_changed(move |checked| {
                    settings_update.write().auto_update = checked.unwrap_or(false);
                    println!("Auto update: {}", checked.unwrap_or(false));
                })
                .into(),
        ))
        .into()
}

fn build_setting_row(label: &str, description: &str, control: UIElement) -> UIElement {
    Grid::new()
        .columns(vec![
            ColumnDefinition::star(1.0),
            ColumnDefinition::auto(),
        ])
        .column_spacing(20.0)
        // Label and description
        .child_at(
            StackPanel::new()
                .orientation(Orientation::Vertical)
                .spacing(4.0)
                .child(
                    TextBlock::new()
                        .text(label)
                        .font_size(14.0)
                        .font_weight(FontWeight::Medium),
                )
                .child(
                    TextBlock::new()
                        .text(description)
                        .font_size(12.0)
                        .foreground(&Brush::from_color(Color::rgb(128, 128, 128)))
                        .text_wrapping(TextWrapping::Wrap),
                ),
            0,
            0,
        )
        // Control
        .child_at(
            Border::new()
                .vertical_alignment(VerticalAlignment::Center)
                .child(control),
            0,
            1,
        )
        .into()
}

fn build_slider_setting<F>(
    label: &str,
    value_display: &str,
    min: f64,
    max: f64,
    value: f64,
    on_change: F,
) -> UIElement
where
    F: Fn(f64) + Send + Sync + 'static,
{
    Grid::new()
        .columns(vec![
            ColumnDefinition::star(1.0),
            ColumnDefinition::pixel(250.0),
        ])
        .column_spacing(20.0)
        // Label
        .child_at(
            TextBlock::new()
                .text(label)
                .font_size(14.0)
                .font_weight(FontWeight::Medium)
                .vertical_alignment(VerticalAlignment::Center),
            0,
            0,
        )
        // Slider and value
        .child_at(
            StackPanel::new()
                .orientation(Orientation::Horizontal)
                .spacing(15.0)
                .child(
                    Slider::new()
                        .minimum(min)
                        .maximum(max)
                        .value(value)
                        .width(180.0)
                        .on_value_changed(move |val| {
                            on_change(val);
                        }),
                )
                .child(
                    TextBlock::new()
                        .text(value_display)
                        .font_size(13.0)
                        .width(50.0)
                        .vertical_alignment(VerticalAlignment::Center)
                        .foreground(&Brush::from_color(Color::rgb(80, 80, 80))),
                ),
            0,
            1,
        )
        .into()
}

fn build_action_buttons(settings: Arc<RwLock<AppSettings>>) -> UIElement {
    let settings_save = settings.clone();
    let settings_reset = settings.clone();

    Border::new()
        .padding(Thickness::new(0.0, 15.0, 0.0, 0.0))
        .child(
            StackPanel::new()
                .orientation(Orientation::Horizontal)
                .spacing(10.0)
                .horizontal_alignment(HorizontalAlignment::Right)
                .child(
                    Button::new()
                        .content("Reset to Defaults")
                        .padding(Thickness::symmetric(20.0, 12.0))
                        .on_click(move |_| {
                            *settings_reset.write() = AppSettings::default();
                            println!("Settings reset to defaults");
                            println!("=== Default Settings ===");
                            let s = settings_reset.read();
                            println!("Theme: {}", s.theme);
                            println!("Language: {}", s.language);
                            println!("Font Size: {}", s.font_size);
                            println!("========================");
                        }),
                )
                .child(
                    Button::new()
                        .content("Save Settings")
                        .padding(Thickness::symmetric(30.0, 12.0))
                        .on_click(move |_| {
                            let s = settings_save.read().clone();
                            println!("=== Saving Settings ===");
                            println!("Theme: {}", s.theme);
                            println!("Language: {}", s.language);
                            println!("Startup: {}", s.startup_enabled);
                            println!("Font Size: {:.0} pt", s.font_size);
                            println!("Opacity: {:.0}%", s.opacity * 100.0);
                            println!("Animations: {}", s.animations_enabled);
                            println!("Notifications: {}", s.notifications_enabled);
                            println!("Sound: {}", s.sound_enabled);
                            println!("Position: {}", s.notification_position);
                            println!("Analytics: {}", s.analytics_enabled);
                            println!("Crash Reports: {}", s.crash_reports);
                            println!("Auto Update: {}", s.auto_update);
                            println!("======================");
                        }),
                ),
        )
        .into()
}

