//! Image Gallery Example
//!
//! Demonstrates the Image control with a photo gallery layout.
//!
//! Run with: cargo run --example image_gallery

use winrt_xaml::prelude::*;
use std::sync::Arc;
use parking_lot::RwLock;

/// Photo metadata
#[derive(Clone)]
struct Photo {
    id: usize,
    title: String,
    description: String,
    file_path: String,
    category: String,
}

impl Photo {
    fn new(id: usize, title: &str, description: &str, file_path: &str, category: &str) -> Self {
        Self {
            id,
            title: title.to_string(),
            description: description.to_string(),
            file_path: file_path.to_string(),
            category: category.to_string(),
        }
    }
}

/// Gallery state
struct GalleryState {
    photos: Vec<Photo>,
    selected_photo: Option<usize>,
    filter_category: String,
}

impl GalleryState {
    fn new() -> Self {
        Self {
            photos: vec![
                Photo::new(1, "Mountain Sunset", "Beautiful sunset over the mountains", "assets/mountain.jpg", "Nature"),
                Photo::new(2, "Ocean Waves", "Peaceful ocean waves at dawn", "assets/ocean.jpg", "Nature"),
                Photo::new(3, "City Lights", "Downtown cityscape at night", "assets/city.jpg", "Urban"),
                Photo::new(4, "Forest Path", "Winding path through the forest", "assets/forest.jpg", "Nature"),
                Photo::new(5, "Desert Dunes", "Golden sand dunes at sunset", "assets/desert.jpg", "Nature"),
                Photo::new(6, "Skyscraper", "Modern architecture reaching the sky", "assets/building.jpg", "Urban"),
                Photo::new(7, "Autumn Leaves", "Colorful fall foliage", "assets/autumn.jpg", "Nature"),
                Photo::new(8, "Bridge at Night", "Illuminated bridge over water", "assets/bridge.jpg", "Urban"),
            ],
            selected_photo: None,
            filter_category: "All".to_string(),
        }
    }

    fn filtered_photos(&self) -> Vec<Photo> {
        if self.filter_category == "All" {
            self.photos.clone()
        } else {
            self.photos
                .iter()
                .filter(|p| p.category == self.filter_category)
                .cloned()
                .collect()
        }
    }

    fn get_photo(&self, id: usize) -> Option<&Photo> {
        self.photos.iter().find(|p| p.id == id)
    }

    fn categories(&self) -> Vec<String> {
        let mut cats: Vec<String> = self.photos
            .iter()
            .map(|p| p.category.clone())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();
        cats.sort();
        let mut result = vec!["All".to_string()];
        result.extend(cats);
        result
    }
}

fn main() -> Result<()> {
    env_logger::init();

    let app = Application::new()?;

    let window = Window::builder()
        .title("Image Gallery")
        .size(1000, 700)
        .build()?;

    let state = Arc::new(RwLock::new(GalleryState::new()));

    let content = build_gallery_ui(state)?;

    window.set_content(content)?;
    window.center()?;
    window.show()?;

    app.run()
}

fn build_gallery_ui(state: Arc<RwLock<GalleryState>>) -> Result<UIElement> {
    Ok(Border::new()
        .background(&Brush::from_color(Color::rgb(245, 245, 245)))
        .child(
            Grid::new()
                .rows(vec![
                    RowDefinition::auto(),    // Header
                    RowDefinition::auto(),    // Filter
                    RowDefinition::star(1.0), // Content
                    RowDefinition::auto(),    // Footer
                ])
                // Header
                .child_at(build_header(), 0, 0)
                // Filter bar
                .child_at(build_filter_bar(state.clone()), 1, 0)
                // Main content
                .child_at(build_main_content(state.clone()), 2, 0)
                // Footer
                .child_at(build_footer(state), 3, 0),
        )
        .into())
}

fn build_header() -> UIElement {
    Border::new()
        .background(&Brush::from_color(Color::rgb(0, 120, 215)))
        .padding(Thickness::new(30.0, 20.0, 30.0, 20.0))
        .child(
            StackPanel::new()
                .orientation(Orientation::Horizontal)
                .spacing(15.0)
                .child(
                    TextBlock::new()
                        .text("📷")
                        .font_size(32.0),
                )
                .child(
                    StackPanel::new()
                        .orientation(Orientation::Vertical)
                        .spacing(3.0)
                        .child(
                            TextBlock::new()
                                .text("Image Gallery")
                                .font_size(28.0)
                                .font_weight(FontWeight::Bold)
                                .foreground(&Brush::white()),
                        )
                        .child(
                            TextBlock::new()
                                .text("Browse your photo collection")
                                .font_size(14.0)
                                .foreground(&Brush::from_color(Color::rgb(220, 235, 255))),
                        ),
                ),
        )
        .into()
}

fn build_filter_bar(state: Arc<RwLock<GalleryState>>) -> UIElement {
    let state_filter = state.clone();
    let categories = state.read().categories();
    let current_filter = state.read().filter_category.clone();

    Border::new()
        .background(&Brush::white())
        .border_thickness_uniform(1.0)
        .border_brush(&Brush::from_color(Color::rgb(220, 220, 220)))
        .padding(Thickness::new(30.0, 15.0, 30.0, 15.0))
        .child(
            StackPanel::new()
                .orientation(Orientation::Horizontal)
                .spacing(15.0)
                .child(
                    TextBlock::new()
                        .text("Filter by:")
                        .font_size(14.0)
                        .font_weight(FontWeight::Medium)
                        .vertical_alignment(VerticalAlignment::Center),
                )
                .child(
                    ComboBox::new()
                        .width(150.0)
                        .items(categories.clone())
                        .selected_index(categories.iter().position(|c| c == &current_filter).unwrap_or(0))
                        .on_selection_changed(move |index| {
                            if let Some(idx) = index {
                                if let Some(category) = categories.get(idx) {
                                    state_filter.write().filter_category = category.clone();
                                    println!("Filter changed to: {}", category);
                                }
                            }
                        }),
                )
                .child(
                    TextBlock::new()
                        .text(&format!("Showing {} photos", state.read().filtered_photos().len()))
                        .font_size(13.0)
                        .foreground(&Brush::from_color(Color::GRAY))
                        .vertical_alignment(VerticalAlignment::Center)
                        .margin(Thickness::new(20.0, 0.0, 0.0, 0.0)),
                ),
        )
        .into()
}

fn build_main_content(state: Arc<RwLock<GalleryState>>) -> UIElement {
    Grid::new()
        .columns(vec![
            ColumnDefinition::star(2.0), // Gallery grid
            ColumnDefinition::star(1.0), // Details panel
        ])
        .child_at(build_gallery_grid(state.clone()), 0, 0)
        .child_at(build_details_panel(state), 0, 1)
        .into()
}

fn build_gallery_grid(state: Arc<RwLock<GalleryState>>) -> UIElement {
    let photos = state.read().filtered_photos();

    let mut grid = Grid::new()
        .columns(vec![
            ColumnDefinition::star(1.0),
            ColumnDefinition::star(1.0),
            ColumnDefinition::star(1.0),
        ])
        .row_spacing(15.0)
        .column_spacing(15.0);

    // Calculate number of rows needed
    let rows_needed = (photos.len() + 2) / 3;
    let row_defs: Vec<RowDefinition> = (0..rows_needed)
        .map(|_| RowDefinition::pixel(200.0))
        .collect();

    grid = grid.rows(row_defs);

    for (idx, photo) in photos.iter().enumerate() {
        let row = idx / 3;
        let col = idx % 3;
        grid = grid.child_at(build_photo_card(photo, state.clone()), row, col);
    }

    ScrollViewer::new()
        .vertical_scroll_bar_visibility(ScrollBarVisibility::Auto)
        .padding_uniform(30.0)
        .content(grid)
        .into()
}

fn build_photo_card(photo: &Photo, state: Arc<RwLock<GalleryState>>) -> UIElement {
    let photo_id = photo.id;
    let is_selected = state.read().selected_photo == Some(photo_id);

    let border_color = if is_selected {
        Color::rgb(0, 120, 215)
    } else {
        Color::rgb(220, 220, 220)
    };

    let border_thickness = if is_selected { 3.0 } else { 1.0 };

    Border::new()
        .background(&Brush::white())
        .border_thickness_uniform(border_thickness)
        .border_brush(&Brush::from_color(border_color))
        .corner_radius_uniform(8.0)
        .child(
            Button::new()
                .padding_uniform(0.0)
                .on_click(move |_| {
                    state.write().selected_photo = Some(photo_id);
                    println!("Selected photo ID: {}", photo_id);
                })
                .content(
                    StackPanel::new()
                        .orientation(Orientation::Vertical)
                        .spacing(0.0)
                        // Image placeholder
                        .child(
                            Border::new()
                                .background(&Brush::from_color(Color::rgb(230, 230, 230)))
                                .height(140.0)
                                .corner_radius(CornerRadius::new(8.0, 8.0, 0.0, 0.0))
                                .child(
                                    StackPanel::new()
                                        .orientation(Orientation::Vertical)
                                        .vertical_alignment(VerticalAlignment::Center)
                                        .horizontal_alignment(HorizontalAlignment::Center)
                                        .child(
                                            TextBlock::new()
                                                .text("🖼️")
                                                .font_size(48.0),
                                        )
                                        .child(
                                            TextBlock::new()
                                                .text(&photo.file_path)
                                                .font_size(10.0)
                                                .foreground(&Brush::from_color(Color::GRAY)),
                                        ),
                                ),
                        )
                        // Info section
                        .child(
                            Border::new()
                                .padding(Thickness::symmetric(12.0, 10.0))
                                .child(
                                    StackPanel::new()
                                        .orientation(Orientation::Vertical)
                                        .spacing(4.0)
                                        .child(
                                            TextBlock::new()
                                                .text(&photo.title)
                                                .font_size(13.0)
                                                .font_weight(FontWeight::SemiBold)
                                                .text_wrapping(TextWrapping::NoWrap),
                                        )
                                        .child(
                                            Border::new()
                                                .background(&Brush::from_color(Color::rgb(230, 240, 255)))
                                                .corner_radius_uniform(3.0)
                                                .padding(Thickness::symmetric(6.0, 2.0))
                                                .horizontal_alignment(HorizontalAlignment::Left)
                                                .child(
                                                    TextBlock::new()
                                                        .text(&photo.category)
                                                        .font_size(10.0)
                                                        .foreground(&Brush::from_color(Color::rgb(0, 100, 180))),
                                                ),
                                        ),
                                ),
                        ),
                ),
        )
        .into()
}

fn build_details_panel(state: Arc<RwLock<GalleryState>>) -> UIElement {
    let s = state.read();

    let content = if let Some(photo_id) = s.selected_photo {
        if let Some(photo) = s.get_photo(photo_id) {
            build_photo_details(photo)
        } else {
            build_no_selection()
        }
    } else {
        build_no_selection()
    };

    Border::new()
        .background(&Brush::white())
        .border_thickness(Thickness::new(1.0, 0.0, 0.0, 0.0))
        .border_brush(&Brush::from_color(Color::rgb(220, 220, 220)))
        .child(
            ScrollViewer::new()
                .vertical_scroll_bar_visibility(ScrollBarVisibility::Auto)
                .padding_uniform(25.0)
                .content(content),
        )
        .into()
}

fn build_no_selection() -> UIElement {
    StackPanel::new()
        .orientation(Orientation::Vertical)
        .spacing(15.0)
        .vertical_alignment(VerticalAlignment::Center)
        .horizontal_alignment(HorizontalAlignment::Center)
        .margin(Thickness::new(0.0, 100.0, 0.0, 0.0))
        .child(
            TextBlock::new()
                .text("📷")
                .font_size(64.0)
                .horizontal_alignment(HorizontalAlignment::Center),
        )
        .child(
            TextBlock::new()
                .text("No photo selected")
                .font_size(16.0)
                .foreground(&Brush::from_color(Color::GRAY))
                .horizontal_alignment(HorizontalAlignment::Center),
        )
        .child(
            TextBlock::new()
                .text("Click on a photo to view details")
                .font_size(12.0)
                .foreground(&Brush::from_color(Color::LIGHT_GRAY))
                .horizontal_alignment(HorizontalAlignment::Center)
                .text_wrapping(TextWrapping::Wrap),
        )
        .into()
}

fn build_photo_details(photo: &Photo) -> UIElement {
    StackPanel::new()
        .orientation(Orientation::Vertical)
        .spacing(20.0)
        // Large preview
        .child(
            Border::new()
                .background(&Brush::from_color(Color::rgb(240, 240, 240)))
                .corner_radius_uniform(10.0)
                .height(250.0)
                .child(
                    StackPanel::new()
                        .orientation(Orientation::Vertical)
                        .vertical_alignment(VerticalAlignment::Center)
                        .horizontal_alignment(HorizontalAlignment::Center)
                        .child(
                            TextBlock::new()
                                .text("🖼️")
                                .font_size(80.0),
                        )
                        .child(
                            TextBlock::new()
                                .text(&photo.file_path)
                                .font_size(11.0)
                                .foreground(&Brush::from_color(Color::GRAY)),
                        ),
                ),
        )
        // Title
        .child(
            TextBlock::new()
                .text(&photo.title)
                .font_size(22.0)
                .font_weight(FontWeight::Bold)
                .text_wrapping(TextWrapping::Wrap),
        )
        // Category badge
        .child(
            Border::new()
                .background(&Brush::from_color(Color::rgb(230, 240, 255)))
                .corner_radius_uniform(5.0)
                .padding(Thickness::symmetric(12.0, 6.0))
                .horizontal_alignment(HorizontalAlignment::Left)
                .child(
                    TextBlock::new()
                        .text(&photo.category)
                        .font_size(12.0)
                        .font_weight(FontWeight::Medium)
                        .foreground(&Brush::from_color(Color::rgb(0, 100, 180))),
                ),
        )
        // Description
        .child(
            StackPanel::new()
                .orientation(Orientation::Vertical)
                .spacing(8.0)
                .child(
                    TextBlock::new()
                        .text("Description")
                        .font_size(14.0)
                        .font_weight(FontWeight::SemiBold),
                )
                .child(
                    TextBlock::new()
                        .text(&photo.description)
                        .font_size(13.0)
                        .foreground(&Brush::from_color(Color::rgb(80, 80, 80)))
                        .text_wrapping(TextWrapping::Wrap),
                ),
        )
        // Metadata
        .child(
            Border::new()
                .background(&Brush::from_color(Color::rgb(250, 250, 250)))
                .corner_radius_uniform(8.0)
                .padding_uniform(15.0)
                .child(
                    StackPanel::new()
                        .orientation(Orientation::Vertical)
                        .spacing(10.0)
                        .child(
                            TextBlock::new()
                                .text("Details")
                                .font_size(14.0)
                                .font_weight(FontWeight::SemiBold),
                        )
                        .child(build_detail_row("ID", &photo.id.to_string()))
                        .child(build_detail_row("File", &photo.file_path))
                        .child(build_detail_row("Category", &photo.category)),
                ),
        )
        // Action buttons
        .child(
            StackPanel::new()
                .orientation(Orientation::Vertical)
                .spacing(8.0)
                .margin(Thickness::new(0.0, 10.0, 0.0, 0.0))
                .child(
                    Button::new()
                        .content("📤 Share")
                        .horizontal_alignment(HorizontalAlignment::Stretch)
                        .padding(Thickness::symmetric(15.0, 10.0))
                        .on_click(|_| println!("Share photo")),
                )
                .child(
                    Button::new()
                        .content("⬇️ Download")
                        .horizontal_alignment(HorizontalAlignment::Stretch)
                        .padding(Thickness::symmetric(15.0, 10.0))
                        .on_click(|_| println!("Download photo")),
                )
                .child(
                    Button::new()
                        .content("🗑️ Delete")
                        .horizontal_alignment(HorizontalAlignment::Stretch)
                        .padding(Thickness::symmetric(15.0, 10.0))
                        .on_click(|_| println!("Delete photo")),
                ),
        )
        .into()
}

fn build_detail_row(label: &str, value: &str) -> UIElement {
    Grid::new()
        .columns(vec![
            ColumnDefinition::pixel(80.0),
            ColumnDefinition::star(1.0),
        ])
        .child_at(
            TextBlock::new()
                .text(label)
                .font_size(12.0)
                .foreground(&Brush::from_color(Color::GRAY)),
            0, 0
        )
        .child_at(
            TextBlock::new()
                .text(value)
                .font_size(12.0)
                .text_wrapping(TextWrapping::Wrap),
            0, 1
        )
        .into()
}

fn build_footer(state: Arc<RwLock<GalleryState>>) -> UIElement {
    let total = state.read().photos.len();
    let filtered = state.read().filtered_photos().len();

    Border::new()
        .background(&Brush::from_color(Color::rgb(250, 250, 250)))
        .border_thickness(Thickness::new(0.0, 1.0, 0.0, 0.0))
        .border_brush(&Brush::from_color(Color::rgb(220, 220, 220)))
        .padding(Thickness::symmetric(30.0, 12.0))
        .child(
            StackPanel::new()
                .orientation(Orientation::Horizontal)
                .spacing(20.0)
                .child(
                    TextBlock::new()
                        .text(&format!("Total: {} photos", total))
                        .font_size(12.0)
                        .foreground(&Brush::from_color(Color::GRAY)),
                )
                .child(
                    TextBlock::new()
                        .text(&format!("Filtered: {} photos", filtered))
                        .font_size(12.0)
                        .foreground(&Brush::from_color(Color::GRAY)),
                )
                .child(
                    TextBlock::new()
                        .text("© 2025 Image Gallery")
                        .font_size(12.0)
                        .foreground(&Brush::from_color(Color::LIGHT_GRAY))
                        .horizontal_alignment(HorizontalAlignment::Right),
                ),
        )
        .into()
}

