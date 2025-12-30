//! Music player example (UI only - no actual playback).

use winrt_xaml::prelude::*;
use std::sync::Arc;
use parking_lot::RwLock;

fn main() -> Result<()> {
    env_logger::init();

    let app = Application::new()?;

    let window = Window::builder()
        .title("Music Player")
        .size(500, 450)
        .build()?;

    let is_playing = Arc::new(RwLock::new(false));
    let current_track = Arc::new(RwLock::new(0usize));

    let tracks = vec![
        "Track 1 - Song Name",
        "Track 2 - Another Song",
        "Track 3 - Third Song",
        "Track 4 - Fourth Song",
    ];

    // Title
    let title = TextBlock::new()?
        .with_text("Music Player")?;
    title.set_position(170, 20);
    title.set_size(160, 30);

    // Now playing
    let now_playing = TextBlock::new()?
        .with_text("Track 1 - Song Name")?;
    now_playing.set_position(100, 80);
    now_playing.set_size(300, 30);

    // Progress bar
    let progress = ProgressBar::new()?
        .with_value(0.0);

    // Previous button
    let prev_button = Button::new()?
        .with_content("⏮")?;
    prev_button.set_position(100, 200);
    prev_button.set_size(60, 50);

    let track_clone = current_track.clone();
    let display_clone = now_playing.clone();
    let tracks_clone = tracks.clone();
    prev_button.click().subscribe(move |_| {
        let mut track = track_clone.write();
        if *track > 0 {
            *track -= 1;
        } else {
            *track = tracks_clone.len() - 1;
        }
        let _ = display_clone.set_text(tracks_clone[*track]);
        println!("Previous: {}", tracks_clone[*track]);
    });

    // Play/Pause button
    let play_button = Button::new()?
        .with_content("▶")?;
    play_button.set_position(180, 200);
    play_button.set_size(60, 50);

    let playing_clone = is_playing.clone();
    let button_clone = play_button.clone();
    play_button.click().subscribe(move |_| {
        let mut playing = playing_clone.write();
        *playing = !*playing;
        let symbol = if *playing { "⏸" } else { "▶" };
        let _ = button_clone.set_content(symbol);
        println!("{}", if *playing { "Playing" } else { "Paused" });
    });

    // Stop button
    let stop_button = Button::new()?
        .with_content("⏹")?;
    stop_button.set_position(260, 200);
    stop_button.set_size(60, 50);

    let playing_clone = is_playing.clone();
    let button_clone = play_button.clone();
    stop_button.click().subscribe(move |_| {
        *playing_clone.write() = false;
        let _ = button_clone.set_content("▶");
        println!("Stopped");
    });

    // Next button
    let next_button = Button::new()?
        .with_content("⏭")?;
    next_button.set_position(340, 200);
    next_button.set_size(60, 50);

    let track_clone = current_track.clone();
    let display_clone = now_playing.clone();
    let tracks_clone = tracks.clone();
    next_button.click().subscribe(move |_| {
        let mut track = track_clone.write();
        *track = (*track + 1) % tracks_clone.len();
        let _ = display_clone.set_text(tracks_clone[*track]);
        println!("Next: {}", tracks_clone[*track]);
    });

    // Volume slider
    let volume_slider = Slider::new()?
        .with_minimum(0.0)
        .with_maximum(100.0)
        .with_value(75.0);

    volume_slider.value_changed().subscribe(|args| {
        println!("Volume: {}%", args.new_value as i32);
    });

    // Info
    let info = TextBlock::new()?
        .with_text("UI only - audio playback not implemented")?;
    info.set_position(110, 380);
    info.set_size(280, 25);

    window.set_content(play_button)?;

    // Show the window
    window.show()?;

    println!("Starting application...");
    app.run()
}
