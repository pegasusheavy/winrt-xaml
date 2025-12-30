//! Image gallery example (simplified - image loading not yet implemented).

use winrt_xaml::prelude::*;
use std::sync::Arc;
use parking_lot::RwLock;

fn main() -> Result<()> {
    env_logger::init();

    let app = Application::new()?;

    let window = Window::builder()
        .title("Image Gallery")
        .size(600, 500)
        .build()?;

    let current_index = Arc::new(RwLock::new(0usize));
    let images = vec![
        "image1.jpg",
        "image2.jpg",
        "image3.jpg",
        "image4.jpg",
        "image5.jpg",
    ];

    // Title
    let title = TextBlock::new()?
        .with_text("Image Gallery")?;
    title.set_position(220, 20);
    title.set_size(160, 30);

    // Image placeholder
    let image_display = TextBlock::new()?
        .with_text("Image: image1.jpg")?;
    image_display.set_position(180, 120);
    image_display.set_size(240, 40);

    // Previous button
    let prev_button = Button::new()?
        .with_content("← Previous")?;
    prev_button.set_position(100, 300);
    prev_button.set_size(120, 40);

    let index_clone = current_index.clone();
    let display_clone = image_display.clone();
    let images_clone = images.clone();
    prev_button.click().subscribe(move |_| {
        let mut index = index_clone.write();
        if *index > 0 {
            *index -= 1;
        } else {
            *index = images_clone.len() - 1;
        }
        let _ = display_clone.set_text(&format!("Image: {}", images_clone[*index]));
        println!("Showing: {}", images_clone[*index]);
    });

    // Next button
    let next_button = Button::new()?
        .with_content("Next →")?;
    next_button.set_position(380, 300);
    next_button.set_size(120, 40);

    let index_clone = current_index.clone();
    let display_clone = image_display.clone();
    let images_clone = images.clone();
    next_button.click().subscribe(move |_| {
        let mut index = index_clone.write();
        *index = (*index + 1) % images_clone.len();
        let _ = display_clone.set_text(&format!("Image: {}", images_clone[*index]));
        println!("Showing: {}", images_clone[*index]);
    });

    // Info
    let info = TextBlock::new()?
        .with_text("Note: Actual image loading not yet implemented")?;
    info.set_position(140, 400);
    info.set_size(320, 25);

    window.set_content(next_button)?;

    // Show the window
    window.show()?;

    println!("Starting application...");
    app.run()
}
