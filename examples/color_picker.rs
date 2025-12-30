//! Color Picker Example
//!
//! An interactive color picker demonstrating real-time UI updates
//! with sliders and live color preview.
//!
//! Run with: cargo run --example color_picker

use winrt_xaml::prelude::*;
use std::sync::Arc;
use parking_lot::RwLock;

/// RGB color values
#[derive(Clone, Copy)]
struct ColorRGB {
    r: u8,
    g: u8,
    b: u8,
}

impl ColorRGB {
    fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }

    fn to_rgb_string(&self) -> String {
        format!("rgb({}, {}, {})", self.r, self.g, self.b)
    }

    fn to_hsl(&self) -> (f64, f64, f64) {
        let r = self.r as f64 / 255.0;
        let g = self.g as f64 / 255.0;
        let b = self.b as f64 / 255.0;

        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let delta = max - min;

        // Luminance
        let l = (max + min) / 2.0;

        if delta == 0.0 {
            return (0.0, 0.0, l);
        }

        // Saturation
        let s = if l < 0.5 {
            delta / (max + min)
        } else {
            delta / (2.0 - max - min)
        };

        // Hue
        let h = if max == r {
            ((g - b) / delta + if g < b { 6.0 } else { 0.0 }) / 6.0
        } else if max == g {
            ((b - r) / delta + 2.0) / 6.0
        } else {
            ((r - g) / delta + 4.0) / 6.0
        };

        (h * 360.0, s * 100.0, l * 100.0)
    }
}

impl Default for ColorRGB {
    fn default() -> Self {
        Self::new(0, 120, 215) // Windows blue
    }
}

fn main() -> Result<()> {
    env_logger::init();

    let app = Application::new()?;

    let window = Window::builder()
        .title("Color Picker")
        .size(650, 700)
        .build()?;

    let color = Arc::new(RwLock::new(ColorRGB::default()));

    let content = build_color_picker_ui(color)?;

    window.set_content(content)?;
    window.center()?;
    window.show()?;

    app.run()
}

fn build_color_picker_ui(color: Arc<RwLock<ColorRGB>>) -> Result<UIElement> {
    Ok(Border::new()
        .background(&Brush::from_color(Color::rgb(245, 245, 245)))
        .child(
            StackPanel::new()
                .orientation(Orientation::Vertical)
                .padding_uniform(30.0)
                .spacing(25.0)
                // Header
                .child(build_header())
                // Color preview
                .child(build_color_preview(color.clone()))
                // RGB Sliders
                .child(build_rgb_sliders(color.clone()))
                // Color info panel
                .child(build_color_info(color.clone()))
                // Preset colors
                .child(build_preset_colors(color.clone()))
                // Actions
                .child(build_actions(color)),
        )
        .into())
}

fn build_header() -> UIElement {
    StackPanel::new()
        .orientation(Orientation::Vertical)
        .spacing(5.0)
        .child(
            TextBlock::new()
                .text("Color Picker")
                .font_size(32.0)
                .font_weight(FontWeight::Bold)
                .horizontal_alignment(HorizontalAlignment::Center),
        )
        .child(
            TextBlock::new()
                .text("Adjust the sliders to create your perfect color")
                .font_size(14.0)
                .foreground(&Brush::from_color(Color::GRAY))
                .horizontal_alignment(HorizontalAlignment::Center),
        )
        .into()
}

fn build_color_preview(color: Arc<RwLock<ColorRGB>>) -> UIElement {
    let c = color.read();

    Border::new()
        .background(&Brush::white())
        .border_thickness_uniform(1.0)
        .border_brush(&Brush::from_color(Color::rgb(200, 200, 200)))
        .corner_radius_uniform(12.0)
        .padding_uniform(20.0)
        .child(
            StackPanel::new()
                .orientation(Orientation::Vertical)
                .spacing(15.0)
                .child(
                    TextBlock::new()
                        .text("Preview")
                        .font_size(16.0)
                        .font_weight(FontWeight::SemiBold),
                )
                // Large color preview box
                .child(
                    Border::new()
                        .background(&Brush::from_color(Color::rgb(c.r, c.g, c.b)))
                        .border_thickness_uniform(2.0)
                        .border_brush(&Brush::from_color(Color::rgb(180, 180, 180)))
                        .corner_radius_uniform(8.0)
                        .height(150.0)
                        .horizontal_alignment(HorizontalAlignment::Stretch),
                )
                // Preview text samples
                .child(
                    Grid::new()
                        .columns(vec![
                            ColumnDefinition::star(1.0),
                            ColumnDefinition::star(1.0),
                        ])
                        .column_spacing(10.0)
                        // Text on color
                        .child_at(
                            Border::new()
                                .background(&Brush::from_color(Color::rgb(c.r, c.g, c.b)))
                                .corner_radius_uniform(6.0)
                                .padding_uniform(15.0)
                                .child(
                                    TextBlock::new()
                                        .text("Sample Text")
                                        .font_size(14.0)
                                        .foreground(&Brush::white())
                                        .horizontal_alignment(HorizontalAlignment::Center),
                                ),
                            0,
                            0,
                        )
                        // Color on white
                        .child_at(
                            Border::new()
                                .background(&Brush::white())
                                .border_thickness_uniform(1.0)
                                .border_brush(&Brush::from_color(Color::LIGHT_GRAY))
                                .corner_radius_uniform(6.0)
                                .padding_uniform(15.0)
                                .child(
                                    TextBlock::new()
                                        .text("Sample Text")
                                        .font_size(14.0)
                                        .foreground(&Brush::from_color(Color::rgb(c.r, c.g, c.b)))
                                        .horizontal_alignment(HorizontalAlignment::Center),
                                ),
                            0,
                            1,
                        ),
                ),
        )
        .into()
}

fn build_rgb_sliders(color: Arc<RwLock<ColorRGB>>) -> UIElement {
    let color_r = color.clone();
    let color_g = color.clone();
    let color_b = color.clone();

    let c = color.read();

    Border::new()
        .background(&Brush::white())
        .border_thickness_uniform(1.0)
        .border_brush(&Brush::from_color(Color::rgb(200, 200, 200)))
        .corner_radius_uniform(12.0)
        .padding_uniform(20.0)
        .child(
            StackPanel::new()
                .orientation(Orientation::Vertical)
                .spacing(20.0)
                .child(
                    TextBlock::new()
                        .text("RGB Sliders")
                        .font_size(16.0)
                        .font_weight(FontWeight::SemiBold),
                )
                // Red slider
                .child(build_color_slider(
                    "Red",
                    c.r,
                    Color::rgb(255, 0, 0),
                    move |value| {
                        color_r.write().r = value as u8;
                        println!("Red: {}", value);
                    },
                ))
                // Green slider
                .child(build_color_slider(
                    "Green",
                    c.g,
                    Color::rgb(0, 255, 0),
                    move |value| {
                        color_g.write().g = value as u8;
                        println!("Green: {}", value);
                    },
                ))
                // Blue slider
                .child(build_color_slider(
                    "Blue",
                    c.b,
                    Color::rgb(0, 0, 255),
                    move |value| {
                        color_b.write().b = value as u8;
                        println!("Blue: {}", value);
                    },
                )),
        )
        .into()
}

fn build_color_slider<F>(label: &str, value: u8, color: Color, on_change: F) -> UIElement
where
    F: Fn(f64) + Send + Sync + 'static,
{
    Grid::new()
        .columns(vec![
            ColumnDefinition::pixel(60.0),
            ColumnDefinition::star(1.0),
            ColumnDefinition::pixel(50.0),
        ])
        .column_spacing(15.0)
        // Label
        .child_at(
            TextBlock::new()
                .text(label)
                .font_size(14.0)
                .font_weight(FontWeight::Medium)
                .foreground(&Brush::from_color(color))
                .vertical_alignment(VerticalAlignment::Center),
            0,
            0,
        )
        // Slider
        .child_at(
            Slider::new()
                .minimum(0.0)
                .maximum(255.0)
                .value(value as f64)
                .on_value_changed(move |val| {
                    on_change(val);
                }),
            0,
            1,
        )
        // Value display
        .child_at(
            Border::new()
                .background(&Brush::from_color(Color::rgb(240, 240, 240)))
                .corner_radius_uniform(4.0)
                .padding(Thickness::symmetric(8.0, 4.0))
                .child(
                    TextBlock::new()
                        .text(&format!("{}", value))
                        .font_size(13.0)
                        .horizontal_alignment(HorizontalAlignment::Center)
                        .vertical_alignment(VerticalAlignment::Center),
                ),
            0,
            2,
        )
        .into()
}

fn build_color_info(color: Arc<RwLock<ColorRGB>>) -> UIElement {
    let c = color.read();
    let (h, s, l) = c.to_hsl();

    Border::new()
        .background(&Brush::white())
        .border_thickness_uniform(1.0)
        .border_brush(&Brush::from_color(Color::rgb(200, 200, 200)))
        .corner_radius_uniform(12.0)
        .padding_uniform(20.0)
        .child(
            StackPanel::new()
                .orientation(Orientation::Vertical)
                .spacing(15.0)
                .child(
                    TextBlock::new()
                        .text("Color Information")
                        .font_size(16.0)
                        .font_weight(FontWeight::SemiBold),
                )
                .child(
                    Grid::new()
                        .columns(vec![
                            ColumnDefinition::pixel(100.0),
                            ColumnDefinition::star(1.0),
                        ])
                        .rows(vec![
                            RowDefinition::auto(),
                            RowDefinition::auto(),
                            RowDefinition::auto(),
                            RowDefinition::auto(),
                        ])
                        .row_spacing(10.0)
                        .column_spacing(10.0)
                        // HEX
                        .child_at(
                            TextBlock::new()
                                .text("HEX:")
                                .font_weight(FontWeight::Medium),
                            0, 0
                        )
                        .child_at(
                            TextBlock::new()
                                .text(&c.to_hex())
                                .foreground(&Brush::from_color(Color::rgb(80, 80, 80))),
                            0, 1
                        )
                        // RGB
                        .child_at(
                            TextBlock::new()
                                .text("RGB:")
                                .font_weight(FontWeight::Medium),
                            1, 0
                        )
                        .child_at(
                            TextBlock::new()
                                .text(&c.to_rgb_string())
                                .foreground(&Brush::from_color(Color::rgb(80, 80, 80))),
                            1, 1
                        )
                        // HSL
                        .child_at(
                            TextBlock::new()
                                .text("HSL:")
                                .font_weight(FontWeight::Medium),
                            2, 0
                        )
                        .child_at(
                            TextBlock::new()
                                .text(&format!("hsl({:.0}°, {:.0}%, {:.0}%)", h, s, l))
                                .foreground(&Brush::from_color(Color::rgb(80, 80, 80))),
                            2, 1
                        )
                        // Decimal
                        .child_at(
                            TextBlock::new()
                                .text("Decimal:")
                                .font_weight(FontWeight::Medium),
                            3, 0
                        )
                        .child_at(
                            TextBlock::new()
                                .text(&format!("{}, {}, {}", c.r, c.g, c.b))
                                .foreground(&Brush::from_color(Color::rgb(80, 80, 80))),
                            3, 1
                        ),
                ),
        )
        .into()
}

fn build_preset_colors(color: Arc<RwLock<ColorRGB>>) -> UIElement {
    let presets = vec![
        ("Red", ColorRGB::new(255, 0, 0)),
        ("Green", ColorRGB::new(0, 255, 0)),
        ("Blue", ColorRGB::new(0, 0, 255)),
        ("Yellow", ColorRGB::new(255, 255, 0)),
        ("Cyan", ColorRGB::new(0, 255, 255)),
        ("Magenta", ColorRGB::new(255, 0, 255)),
        ("Orange", ColorRGB::new(255, 165, 0)),
        ("Purple", ColorRGB::new(128, 0, 128)),
        ("Pink", ColorRGB::new(255, 192, 203)),
        ("Teal", ColorRGB::new(0, 128, 128)),
    ];

    let mut grid = Grid::new()
        .columns(vec![
            ColumnDefinition::star(1.0),
            ColumnDefinition::star(1.0),
            ColumnDefinition::star(1.0),
            ColumnDefinition::star(1.0),
            ColumnDefinition::star(1.0),
        ])
        .rows(vec![
            RowDefinition::pixel(60.0),
            RowDefinition::pixel(60.0),
        ])
        .row_spacing(8.0)
        .column_spacing(8.0);

    for (idx, (name, preset_color)) in presets.iter().enumerate() {
        let row = idx / 5;
        let col = idx % 5;
        let color_clone = color.clone();
        let preset_clone = *preset_color;

        grid = grid.child_at(
            Button::new()
                .content(name)
                .padding_uniform(10.0)
                .on_click(move |_| {
                    *color_clone.write() = preset_clone;
                    println!("Selected preset: {} - {}", name, preset_clone.to_hex());
                }),
            row,
            col,
        );
    }

    Border::new()
        .background(&Brush::white())
        .border_thickness_uniform(1.0)
        .border_brush(&Brush::from_color(Color::rgb(200, 200, 200)))
        .corner_radius_uniform(12.0)
        .padding_uniform(20.0)
        .child(
            StackPanel::new()
                .orientation(Orientation::Vertical)
                .spacing(15.0)
                .child(
                    TextBlock::new()
                        .text("Preset Colors")
                        .font_size(16.0)
                        .font_weight(FontWeight::SemiBold),
                )
                .child(grid),
        )
        .into()
}

fn build_actions(color: Arc<RwLock<ColorRGB>>) -> UIElement {
    let color_copy = color.clone();
    let color_random = color.clone();

    StackPanel::new()
        .orientation(Orientation::Horizontal)
        .spacing(10.0)
        .horizontal_alignment(HorizontalAlignment::Center)
        .child(
            Button::new()
                .content("Copy HEX")
                .padding(Thickness::symmetric(25.0, 12.0))
                .on_click(move |_| {
                    let hex = color_copy.read().to_hex();
                    println!("Copied to clipboard: {}", hex);
                    // In a real app, this would copy to clipboard
                }),
        )
        .child(
            Button::new()
                .content("Random Color")
                .padding(Thickness::symmetric(25.0, 12.0))
                .on_click(move |_| {
                    use std::time::{SystemTime, UNIX_EPOCH};
                    let seed = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_millis();

                    let r = ((seed * 9301 + 49297) % 233280) as u8;
                    let g = ((seed * 5237 + 37429) % 233280) as u8;
                    let b = ((seed * 7919 + 29473) % 233280) as u8;

                    let new_color = ColorRGB::new(r, g, b);
                    *color_random.write() = new_color;
                    println!("Random color: {}", new_color.to_hex());
                }),
        )
        .into()
}

