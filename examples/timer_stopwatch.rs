//! Timer and stopwatch example.

use winrt_xaml::prelude::*;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

fn main() -> Result<()> {
    env_logger::init();

    let app = Application::new()?;

    let window = Window::builder()
        .title("Timer & Stopwatch")
        .size(400, 350)
        .build()?;

    let elapsed_ms = Arc::new(AtomicU64::new(0));
    let is_running = Arc::new(AtomicBool::new(false));

    // Title
    let title = TextBlock::new()?
        .with_text("Stopwatch")?;
    title.set_position(140, 20);
    title.set_size(120, 30);

    // Time display
    let time_display = TextBlock::new()?
        .with_text("00:00:00.000")?;
    time_display.set_position(80, 80);
    time_display.set_size(240, 50);

    // Start/Stop button
    let start_stop = Button::new()?
        .with_content("Start")?;
    start_stop.set_position(80, 160);
    start_stop.set_size(100, 40);

    let running_clone = is_running.clone();
    let button_clone = start_stop.clone();
    start_stop.click().subscribe(move |_| {
        let was_running = running_clone.fetch_xor(true, Ordering::SeqCst);
        let new_text = if was_running { "Start" } else { "Stop" };
        let _ = button_clone.set_content(new_text);
        println!("{}", if was_running { "Stopped" } else { "Started" });
    });

    // Reset button
    let reset_button = Button::new()?
        .with_content("Reset")?;
    reset_button.set_position(200, 160);
    reset_button.set_size(100, 40);

    let elapsed_clone = elapsed_ms.clone();
    let display_clone = time_display.clone();
    reset_button.click().subscribe(move |_| {
        elapsed_clone.store(0, Ordering::SeqCst);
        let _ = display_clone.set_text("00:00:00.000");
        println!("Reset");
    });

    // Lap button
    let lap_button = Button::new()?
        .with_content("Lap")?;
    lap_button.set_position(140, 220);
    lap_button.set_size(100, 40);

    let elapsed_clone = elapsed_ms.clone();
    lap_button.click().subscribe(move |_| {
        let ms = elapsed_clone.load(Ordering::SeqCst);
        let hours = ms / 3600000;
        let minutes = (ms % 3600000) / 60000;
        let seconds = (ms % 60000) / 1000;
        let millis = ms % 1000;
        println!("Lap: {:02}:{:02}:{:02}.{:03}", hours, minutes, seconds, millis);
    });

    // Instructions
    let instructions = TextBlock::new()?
        .with_text("Note: Timer updates not yet implemented")?;
    instructions.set_position(50, 290);
    instructions.set_size(300, 25);

    window.set_content(start_stop)?;

    // Show the window
    window.show()?;

    println!("Starting application...");
    app.run()
}
