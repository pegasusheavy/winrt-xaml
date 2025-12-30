//! Weather Dashboard Example
//!
//! A weather dashboard demonstrating data visualization and layout composition.
//!
//! Run with: cargo run --example weather_dashboard

use winrt_xaml::prelude::*;
use std::sync::Arc;
use parking_lot::RwLock;

/// Weather condition
#[derive(Clone, Copy)]
enum WeatherCondition {
    Sunny,
    Cloudy,
    Rainy,
    Snowy,
    Stormy,
    Foggy,
}

impl WeatherCondition {
    fn icon(&self) -> &str {
        match self {
            WeatherCondition::Sunny => "☀️",
            WeatherCondition::Cloudy => "☁️",
            WeatherCondition::Rainy => "🌧️",
            WeatherCondition::Snowy => "❄️",
            WeatherCondition::Stormy => "⛈️",
            WeatherCondition::Foggy => "🌫️",
        }
    }

    fn name(&self) -> &str {
        match self {
            WeatherCondition::Sunny => "Sunny",
            WeatherCondition::Cloudy => "Cloudy",
            WeatherCondition::Rainy => "Rainy",
            WeatherCondition::Snowy => "Snowy",
            WeatherCondition::Stormy => "Stormy",
            WeatherCondition::Foggy => "Foggy",
        }
    }
}

/// Daily forecast
#[derive(Clone)]
struct DayForecast {
    day: String,
    condition: WeatherCondition,
    high: i32,
    low: i32,
}

/// Current weather data
struct WeatherData {
    location: String,
    temperature: i32,
    condition: WeatherCondition,
    humidity: u32,
    wind_speed: u32,
    pressure: u32,
    uv_index: u32,
    visibility: u32,
    feels_like: i32,
    forecast: Vec<DayForecast>,
}

impl WeatherData {
    fn sample() -> Self {
        Self {
            location: "San Francisco, CA".to_string(),
            temperature: 72,
            condition: WeatherCondition::Sunny,
            humidity: 65,
            wind_speed: 12,
            pressure: 1013,
            uv_index: 7,
            visibility: 10,
            feels_like: 70,
            forecast: vec![
                DayForecast {
                    day: "Mon".to_string(),
                    condition: WeatherCondition::Sunny,
                    high: 75,
                    low: 62,
                },
                DayForecast {
                    day: "Tue".to_string(),
                    condition: WeatherCondition::Cloudy,
                    high: 70,
                    low: 60,
                },
                DayForecast {
                    day: "Wed".to_string(),
                    condition: WeatherCondition::Rainy,
                    high: 68,
                    low: 58,
                },
                DayForecast {
                    day: "Thu".to_string(),
                    condition: WeatherCondition::Cloudy,
                    high: 71,
                    low: 59,
                },
                DayForecast {
                    day: "Fri".to_string(),
                    condition: WeatherCondition::Sunny,
                    high: 76,
                    low: 63,
                },
            ],
        }
    }
}

fn main() -> Result<()> {
    env_logger::init();

    let app = Application::new()?;

    let window = Window::builder()
        .title("Weather Dashboard")
        .size(900, 700)
        .build()?;

    let weather = Arc::new(RwLock::new(WeatherData::sample()));

    let content = build_dashboard_ui(weather)?;

    window.set_content(content)?;
    window.center()?;
    window.show()?;

    app.run()
}

fn build_dashboard_ui(weather: Arc<RwLock<WeatherData>>) -> Result<UIElement> {
    Ok(Border::new()
        .background(&Brush::from_color(Color::rgb(135, 206, 250))) // Sky blue
        .child(
            ScrollViewer::new()
                .vertical_scroll_bar_visibility(ScrollBarVisibility::Auto)
                .content(
                    StackPanel::new()
                        .orientation(Orientation::Vertical)
                        .padding_uniform(30.0)
                        .spacing(25.0)
                        // Header
                        .child(build_header(weather.clone()))
                        // Current weather
                        .child(build_current_weather(weather.clone()))
                        // Details grid
                        .child(build_details_grid(weather.clone()))
                        // 5-day forecast
                        .child(build_forecast(weather)),
                ),
        )
        .into())
}

fn build_header(weather: Arc<RwLock<WeatherData>>) -> UIElement {
    let w = weather.read();

    StackPanel::new()
        .orientation(Orientation::Vertical)
        .spacing(8.0)
        .child(
            StackPanel::new()
                .orientation(Orientation::Horizontal)
                .spacing(10.0)
                .child(
                    TextBlock::new()
                        .text("🌍")
                        .font_size(32.0),
                )
                .child(
                    TextBlock::new()
                        .text(&w.location)
                        .font_size(32.0)
                        .font_weight(FontWeight::Bold)
                        .foreground(&Brush::white()),
                ),
        )
        .child(
            TextBlock::new()
                .text("Last updated: Just now")
                .font_size(14.0)
                .foreground(&Brush::from_color(Color::rgb(240, 248, 255))),
        )
        .into()
}

fn build_current_weather(weather: Arc<RwLock<WeatherData>>) -> UIElement {
    let w = weather.read();

    Border::new()
        .background(&Brush::white())
        .corner_radius_uniform(20.0)
        .padding_uniform(40.0)
        .child(
            Grid::new()
                .columns(vec![
                    ColumnDefinition::auto(),
                    ColumnDefinition::star(1.0),
                ])
                .column_spacing(40.0)
                // Weather icon and temperature
                .child_at(
                    StackPanel::new()
                        .orientation(Orientation::Vertical)
                        .spacing(10.0)
                        .horizontal_alignment(HorizontalAlignment::Center)
                        .child(
                            TextBlock::new()
                                .text(w.condition.icon())
                                .font_size(120.0)
                                .horizontal_alignment(HorizontalAlignment::Center),
                        )
                        .child(
                            TextBlock::new()
                                .text(w.condition.name())
                                .font_size(24.0)
                                .font_weight(FontWeight::Medium)
                                .foreground(&Brush::from_color(Color::rgb(100, 100, 100)))
                                .horizontal_alignment(HorizontalAlignment::Center),
                        ),
                    0, 0
                )
                // Temperature and details
                .child_at(
                    StackPanel::new()
                        .orientation(Orientation::Vertical)
                        .spacing(20.0)
                        .vertical_alignment(VerticalAlignment::Center)
                        .child(
                            StackPanel::new()
                                .orientation(Orientation::Horizontal)
                                .spacing(10.0)
                                .child(
                                    TextBlock::new()
                                        .text(&format!("{}°", w.temperature))
                                        .font_size(96.0)
                                        .font_weight(FontWeight::Light)
                                        .foreground(&Brush::from_color(Color::rgb(0, 120, 215))),
                                )
                                .child(
                                    StackPanel::new()
                                        .orientation(Orientation::Vertical)
                                        .spacing(5.0)
                                        .vertical_alignment(VerticalAlignment::Top)
                                        .margin(Thickness::new(0.0, 10.0, 0.0, 0.0))
                                        .child(
                                            TextBlock::new()
                                                .text("F")
                                                .font_size(32.0)
                                                .foreground(&Brush::from_color(Color::rgb(100, 100, 100))),
                                        ),
                                ),
                        )
                        .child(
                            TextBlock::new()
                                .text(&format!("Feels like {}°F", w.feels_like))
                                .font_size(18.0)
                                .foreground(&Brush::from_color(Color::rgb(120, 120, 120))),
                        ),
                    0, 1
                ),
        )
        .into()
}

fn build_details_grid(weather: Arc<RwLock<WeatherData>>) -> UIElement {
    let w = weather.read();

    Grid::new()
        .columns(vec![
            ColumnDefinition::star(1.0),
            ColumnDefinition::star(1.0),
            ColumnDefinition::star(1.0),
        ])
        .rows(vec![
            RowDefinition::auto(),
            RowDefinition::auto(),
        ])
        .row_spacing(15.0)
        .column_spacing(15.0)
        // Row 1
        .child_at(build_weather_card("💧", "Humidity", &format!("{}%", w.humidity)), 0, 0)
        .child_at(build_weather_card("💨", "Wind", &format!("{} mph", w.wind_speed)), 0, 1)
        .child_at(build_weather_card("🌡️", "Pressure", &format!("{} mb", w.pressure)), 0, 2)
        // Row 2
        .child_at(build_weather_card("☀️", "UV Index", &w.uv_index.to_string()), 1, 0)
        .child_at(build_weather_card("👁️", "Visibility", &format!("{} mi", w.visibility)), 1, 1)
        .child_at(build_weather_card("🌅", "Sunrise", "6:42 AM"), 1, 2)
        .into()
}

fn build_weather_card(icon: &str, label: &str, value: &str) -> UIElement {
    Border::new()
        .background(&Brush::white())
        .corner_radius_uniform(12.0)
        .padding(Thickness::symmetric(20.0, 25.0))
        .child(
            StackPanel::new()
                .orientation(Orientation::Vertical)
                .spacing(12.0)
                .child(
                    TextBlock::new()
                        .text(icon)
                        .font_size(40.0)
                        .horizontal_alignment(HorizontalAlignment::Center),
                )
                .child(
                    TextBlock::new()
                        .text(label)
                        .font_size(13.0)
                        .foreground(&Brush::from_color(Color::rgb(120, 120, 120)))
                        .horizontal_alignment(HorizontalAlignment::Center),
                )
                .child(
                    TextBlock::new()
                        .text(value)
                        .font_size(22.0)
                        .font_weight(FontWeight::SemiBold)
                        .foreground(&Brush::from_color(Color::rgb(0, 120, 215)))
                        .horizontal_alignment(HorizontalAlignment::Center),
                ),
        )
        .into()
}

fn build_forecast(weather: Arc<RwLock<WeatherData>>) -> UIElement {
    let w = weather.read();

    Border::new()
        .background(&Brush::white())
        .corner_radius_uniform(15.0)
        .padding_uniform(25.0)
        .child(
            StackPanel::new()
                .orientation(Orientation::Vertical)
                .spacing(20.0)
                // Header
                .child(
                    TextBlock::new()
                        .text("5-Day Forecast")
                        .font_size(22.0)
                        .font_weight(FontWeight::SemiBold)
                        .foreground(&Brush::from_color(Color::rgb(32, 32, 32))),
                )
                // Forecast cards
                .child({
                    let mut grid = Grid::new()
                        .columns(vec![
                            ColumnDefinition::star(1.0),
                            ColumnDefinition::star(1.0),
                            ColumnDefinition::star(1.0),
                            ColumnDefinition::star(1.0),
                            ColumnDefinition::star(1.0),
                        ])
                        .column_spacing(12.0);

                    for (idx, day) in w.forecast.iter().enumerate() {
                        grid = grid.child_at(build_forecast_card(day), 0, idx);
                    }

                    grid
                }),
        )
        .into()
}

fn build_forecast_card(day: &DayForecast) -> UIElement {
    Border::new()
        .background(&Brush::from_color(Color::rgb(248, 250, 252)))
        .corner_radius_uniform(10.0)
        .padding(Thickness::symmetric(15.0, 20.0))
        .child(
            StackPanel::new()
                .orientation(Orientation::Vertical)
                .spacing(15.0)
                // Day
                .child(
                    TextBlock::new()
                        .text(&day.day)
                        .font_size(16.0)
                        .font_weight(FontWeight::SemiBold)
                        .horizontal_alignment(HorizontalAlignment::Center),
                )
                // Icon
                .child(
                    TextBlock::new()
                        .text(day.condition.icon())
                        .font_size(48.0)
                        .horizontal_alignment(HorizontalAlignment::Center),
                )
                // Condition
                .child(
                    TextBlock::new()
                        .text(day.condition.name())
                        .font_size(12.0)
                        .foreground(&Brush::from_color(Color::rgb(100, 100, 100)))
                        .horizontal_alignment(HorizontalAlignment::Center),
                )
                // Temperature range
                .child(
                    StackPanel::new()
                        .orientation(Orientation::Horizontal)
                        .spacing(8.0)
                        .horizontal_alignment(HorizontalAlignment::Center)
                        .child(
                            TextBlock::new()
                                .text(&format!("{}°", day.high))
                                .font_size(18.0)
                                .font_weight(FontWeight::SemiBold)
                                .foreground(&Brush::from_color(Color::rgb(0, 120, 215))),
                        )
                        .child(
                            TextBlock::new()
                                .text("/")
                                .font_size(14.0)
                                .foreground(&Brush::from_color(Color::LIGHT_GRAY)),
                        )
                        .child(
                            TextBlock::new()
                                .text(&format!("{}°", day.low))
                                .font_size(16.0)
                                .foreground(&Brush::from_color(Color::rgb(120, 120, 120))),
                        ),
                ),
        )
        .into()
}

