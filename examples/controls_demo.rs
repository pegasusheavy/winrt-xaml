//! Controls demonstration showing various UI controls.

use winrt_xaml::prelude::*;

fn main() -> Result<()> {
    env_logger::init();

    println!("Creating controls demo...");

    let app = Application::new()?;

    let window = Window::builder()
        .title("Controls Demo")
        .size(600, 500)
        .build()?;

    // Title
    let title = TextBlock::new()?
        .with_text("UI Controls Demonstration")?;
    title.set_position(150, 20);
    title.set_size(300, 30);

    // Button
    let button = Button::new()?
        .with_content("Click Button")?;
    button.set_position(50, 70);
    button.set_size(150, 35);
    button.click().subscribe(|_| {
        println!("Button clicked!");
    });

    // TextBox
    let textbox = TextBox::new()?;
    textbox.set_position(50, 120);
    textbox.set_size(200, 30);
    textbox.set_placeholder("Enter text here...");

    // CheckBox
    let checkbox = CheckBox::new()?
        .with_content("Enable feature")?;
    checkbox.set_position(50, 170);
    checkbox.set_size(150, 30);
    checkbox.checked().subscribe(|args| {
        println!("Checkbox checked: {}", args.is_checked);
    });

    // Slider
    let slider = Slider::new()?
        .with_minimum(0.0)
        .with_maximum(100.0)
        .with_value(50.0);
    slider.value_changed().subscribe(|args| {
        println!("Slider value: {}", args.new_value);
    });

    // ProgressBar
    let progress = ProgressBar::new()?
        .with_value(65.0);

    // ComboBox
    let combo = ComboBox::new()?;
    combo.add_item("Option 1")?;
    combo.add_item("Option 2")?;
    combo.add_item("Option 3")?;
    combo.set_selected_index(0);

    // ToggleSwitch
    let toggle = ToggleSwitch::new()?
        .with_header("Toggle Setting");
    // Note: set_position and set_size not available for ToggleSwitch yet

    // Status label
    let status = TextBlock::new()?
        .with_text("All controls initialized!")?;
    status.set_position(50, 430);
    status.set_size(500, 30);

    window.set_content(button)?;

    // Show the window
    window.show()?;

    println!("Starting application...");
    app.run()
}
