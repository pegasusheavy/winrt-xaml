//! Settings panel example.

use winrt_xaml::prelude::*;

fn main() -> Result<()> {
    env_logger::init();

    let app = Application::new()?;

    let window = Window::builder()
        .title("Settings Panel")
        .size(500, 550)
        .build()?;

    // Title
    let title = TextBlock::new()?
        .with_text("Application Settings")?;
    title.set_position(140, 20);
    title.set_size(220, 30);

    // Notifications
    let notif_check = CheckBox::new()?
        .with_content("Enable Notifications")?;
    notif_check.set_position(50, 80);
    notif_check.set_size(200, 30);
    notif_check.set_checked(true);

    // Auto-save
    let autosave_check = CheckBox::new()?
        .with_content("Auto-save")?;
    autosave_check.set_position(50, 130);
    autosave_check.set_size(200, 30);

    // Theme label
    let theme_label = TextBlock::new()?
        .with_text("Theme:")?;
    theme_label.set_position(50, 180);
    theme_label.set_size(80, 25);

    // Theme selector
    let theme_combo = ComboBox::new()?;
    theme_combo.set_position(140, 180);
    theme_combo.set_size(200, 30);
    theme_combo.add_item("Light")?;
    theme_combo.add_item("Dark")?;
    theme_combo.add_item("System")?;
    theme_combo.set_selected_index(0);

    // Volume label
    let volume_label = TextBlock::new()?
        .with_text("Volume:")?;
    volume_label.set_position(50, 240);
    volume_label.set_size(80, 25);

    // Volume slider
    let volume_slider = Slider::new()?
        .with_minimum(0.0)
        .with_maximum(100.0)
        .with_value(75.0);

    // Save button
    let save_button = Button::new()?
        .with_content("Save Settings")?;
    save_button.set_position(140, 320);
    save_button.set_size(140, 40);

    save_button.click().subscribe(|_| {
        println!("Settings saved!");
    });

    // Reset button
    let reset_button = Button::new()?
        .with_content("Reset to Defaults")?;
    reset_button.set_position(140, 380);
    reset_button.set_size(140, 40);

    reset_button.click().subscribe(|_| {
        println!("Settings reset to defaults");
    });

    window.set_content(save_button)?;

    // Show the window
    window.show()?;

    println!("Starting application...");
    app.run()
}
