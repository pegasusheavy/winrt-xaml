//! Weather dashboard example (UI only - no actual API calls).

use winrt_xaml::prelude::*;

fn main() -> Result<()> {
    env_logger::init();

    let app = Application::new()?;

    let window = Window::builder()
        .title("Weather Dashboard")
        .size(550, 500)
        .build()?;

    // Title
    let title = TextBlock::new()?
        .with_text("Weather Dashboard")?;
    title.set_position(170, 20);
    title.set_size(210, 30);

    // Location
    let location = TextBlock::new()?
        .with_text("Seattle, WA")?;
    location.set_position(200, 70);
    location.set_size(150, 25);

    // Temperature
    let temp = TextBlock::new()?
        .with_text("72°F")?;
    temp.set_position(220, 120);
    temp.set_size(110, 50);

    // Condition
    let condition = TextBlock::new()?
        .with_text("Partly Cloudy")?;
    condition.set_position(200, 180);
    condition.set_size(150, 25);

    // Humidity label
    let humidity_label = TextBlock::new()?
        .with_text("Humidity: 65%")?;
    humidity_label.set_position(50, 240);
    humidity_label.set_size(150, 25);

    // Wind label
    let wind_label = TextBlock::new()?
        .with_text("Wind: 10 mph NW")?;
    wind_label.set_position(50, 280);
    wind_label.set_size(150, 25);

    // Pressure label
    let pressure_label = TextBlock::new()?
        .with_text("Pressure: 30.12 in")?;
    pressure_label.set_position(50, 320);
    pressure_label.set_size(200, 25);

    // Refresh button
    let refresh_button = Button::new()?
        .with_content("Refresh")?;
    refresh_button.set_position(200, 380);
    refresh_button.set_size(120, 40);

    refresh_button.click().subscribe(|_| {
        println!("Refreshing weather data...");
        println!("Note: API integration not implemented");
    });

    // Info
    let info = TextBlock::new()?
        .with_text("Mock data - API integration not implemented")?;
    info.set_position(130, 450);
    info.set_size(290, 25);

    window.set_content(refresh_button)?;

    // Show the window
    window.show()?;

    println!("Starting application...");
    app.run()
}
