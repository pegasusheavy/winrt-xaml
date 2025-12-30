//! Timer and Stopwatch Example
//!
//! Demonstrates time-based UI updates with a countdown timer and stopwatch.
//!
//! Run with: cargo run --example timer_stopwatch

use winrt_xaml::prelude::*;
use std::sync::Arc;
use std::time::{Duration, Instant};
use parking_lot::RwLock;

/// Timer state
struct TimerState {
    duration_seconds: u32,
    remaining_seconds: u32,
    is_running: bool,
    start_time: Option<Instant>,
}

impl TimerState {
    fn new(duration: u32) -> Self {
        Self {
            duration_seconds: duration,
            remaining_seconds: duration,
            is_running: false,
            start_time: None,
        }
    }

    fn start(&mut self) {
        if !self.is_running {
            self.is_running = true;
            self.start_time = Some(Instant::now());
        }
    }

    fn pause(&mut self) {
        self.is_running = false;
    }

    fn reset(&mut self) {
        self.is_running = false;
        self.remaining_seconds = self.duration_seconds;
        self.start_time = None;
    }

    fn set_duration(&mut self, seconds: u32) {
        self.duration_seconds = seconds;
        if !self.is_running {
            self.remaining_seconds = seconds;
        }
    }

    fn tick(&mut self) {
        if self.is_running && self.remaining_seconds > 0 {
            self.remaining_seconds = self.remaining_seconds.saturating_sub(1);
            if self.remaining_seconds == 0 {
                self.is_running = false;
                println!("⏰ Timer finished!");
            }
        }
    }

    fn format_time(&self) -> String {
        let minutes = self.remaining_seconds / 60;
        let seconds = self.remaining_seconds % 60;
        format!("{:02}:{:02}", minutes, seconds)
    }

    fn progress_percent(&self) -> f64 {
        if self.duration_seconds == 0 {
            return 0.0;
        }
        let elapsed = self.duration_seconds - self.remaining_seconds;
        (elapsed as f64 / self.duration_seconds as f64) * 100.0
    }
}

/// Stopwatch state
struct StopwatchState {
    elapsed_millis: u64,
    is_running: bool,
    start_time: Option<Instant>,
    lap_times: Vec<u64>,
}

impl StopwatchState {
    fn new() -> Self {
        Self {
            elapsed_millis: 0,
            is_running: false,
            start_time: None,
            lap_times: Vec::new(),
        }
    }

    fn start(&mut self) {
        if !self.is_running {
            self.is_running = true;
            self.start_time = Some(Instant::now());
        }
    }

    fn pause(&mut self) {
        if self.is_running {
            if let Some(start) = self.start_time {
                self.elapsed_millis += start.elapsed().as_millis() as u64;
            }
            self.is_running = false;
            self.start_time = None;
        }
    }

    fn reset(&mut self) {
        self.elapsed_millis = 0;
        self.is_running = false;
        self.start_time = None;
        self.lap_times.clear();
    }

    fn lap(&mut self) {
        let current = self.current_time();
        self.lap_times.push(current);
        println!("Lap {}: {}", self.lap_times.len(), Self::format_time(current));
    }

    fn current_time(&self) -> u64 {
        let mut total = self.elapsed_millis;
        if let Some(start) = self.start_time {
            total += start.elapsed().as_millis() as u64;
        }
        total
    }

    fn format_time(millis: u64) -> String {
        let seconds = millis / 1000;
        let ms = millis % 1000;
        let mins = seconds / 60;
        let secs = seconds % 60;
        format!("{:02}:{:02}.{:03}", mins, secs, ms)
    }

    fn get_display_time(&self) -> String {
        Self::format_time(self.current_time())
    }
}

/// App mode
#[derive(Clone, Copy, PartialEq)]
enum AppMode {
    Timer,
    Stopwatch,
}

struct AppState {
    mode: AppMode,
    timer: TimerState,
    stopwatch: StopwatchState,
}

impl AppState {
    fn new() -> Self {
        Self {
            mode: AppMode::Timer,
            timer: TimerState::new(300), // 5 minutes default
            stopwatch: StopwatchState::new(),
        }
    }
}

fn main() -> Result<()> {
    env_logger::init();

    let app = Application::new()?;

    let window = Window::builder()
        .title("Timer & Stopwatch")
        .size(500, 600)
        .build()?;

    let state = Arc::new(RwLock::new(AppState::new()));

    let content = build_ui(state)?;

    window.set_content(content)?;
    window.center()?;
    window.show()?;

    app.run()
}

fn build_ui(state: Arc<RwLock<AppState>>) -> Result<UIElement> {
    let mode = state.read().mode;

    Ok(Border::new()
        .background(&Brush::from_color(Color::rgb(240, 240, 245)))
        .child(
            StackPanel::new()
                .orientation(Orientation::Vertical)
                .padding_uniform(30.0)
                .spacing(25.0)
                // Header
                .child(build_header())
                // Mode selector
                .child(build_mode_selector(state.clone()))
                // Content based on mode
                .child(if mode == AppMode::Timer {
                    build_timer_ui(state)
                } else {
                    build_stopwatch_ui(state)
                }),
        )
        .into())
}

fn build_header() -> UIElement {
    StackPanel::new()
        .orientation(Orientation::Vertical)
        .spacing(5.0)
        .child(
            TextBlock::new()
                .text("⏱️ Timer & Stopwatch")
                .font_size(32.0)
                .font_weight(FontWeight::Bold)
                .horizontal_alignment(HorizontalAlignment::Center),
        )
        .child(
            TextBlock::new()
                .text("Track your time efficiently")
                .font_size(14.0)
                .foreground(&Brush::from_color(Color::GRAY))
                .horizontal_alignment(HorizontalAlignment::Center),
        )
        .into()
}

fn build_mode_selector(state: Arc<RwLock<AppState>>) -> UIElement {
    let state_timer = state.clone();
    let state_stopwatch = state.clone();

    let current_mode = state.read().mode;

    Border::new()
        .background(&Brush::white())
        .border_thickness_uniform(1.0)
        .border_brush(&Brush::from_color(Color::rgb(200, 200, 200)))
        .corner_radius_uniform(10.0)
        .padding_uniform(8.0)
        .child(
            Grid::new()
                .columns(vec![
                    ColumnDefinition::star(1.0),
                    ColumnDefinition::star(1.0),
                ])
                .column_spacing(8.0)
                .child_at(
                    Button::new()
                        .content("⏲️ Timer")
                        .padding(Thickness::symmetric(20.0, 15.0))
                        .font_size(16.0)
                        .font_weight(if current_mode == AppMode::Timer {
                            FontWeight::Bold
                        } else {
                            FontWeight::Normal
                        })
                        .on_click(move |_| {
                            state_timer.write().mode = AppMode::Timer;
                            println!("Switched to Timer mode");
                        }),
                    0,
                    0,
                )
                .child_at(
                    Button::new()
                        .content("⏱️ Stopwatch")
                        .padding(Thickness::symmetric(20.0, 15.0))
                        .font_size(16.0)
                        .font_weight(if current_mode == AppMode::Stopwatch {
                            FontWeight::Bold
                        } else {
                            FontWeight::Normal
                        })
                        .on_click(move |_| {
                            state_stopwatch.write().mode = AppMode::Stopwatch;
                            println!("Switched to Stopwatch mode");
                        }),
                    0,
                    1,
                ),
        )
        .into()
}

fn build_timer_ui(state: Arc<RwLock<AppState>>) -> UIElement {
    let state_start = state.clone();
    let state_pause = state.clone();
    let state_reset = state.clone();
    let state_duration = state.clone();

    let timer = &state.read().timer;
    let display_time = timer.format_time();
    let progress = timer.progress_percent();
    let is_running = timer.is_running;

    Border::new()
        .background(&Brush::white())
        .border_thickness_uniform(1.0)
        .border_brush(&Brush::from_color(Color::rgb(200, 200, 200)))
        .corner_radius_uniform(12.0)
        .padding_uniform(30.0)
        .child(
            StackPanel::new()
                .orientation(Orientation::Vertical)
                .spacing(25.0)
                // Timer display
                .child(
                    Border::new()
                        .background(&Brush::from_color(Color::rgb(245, 247, 250)))
                        .corner_radius_uniform(15.0)
                        .padding_uniform(30.0)
                        .child(
                            TextBlock::new()
                                .text(&display_time)
                                .font_size(72.0)
                                .font_weight(FontWeight::Light)
                                .horizontal_alignment(HorizontalAlignment::Center)
                                .foreground(&Brush::from_color(Color::rgb(0, 120, 215))),
                        ),
                )
                // Progress bar
                .child(
                    StackPanel::new()
                        .orientation(Orientation::Vertical)
                        .spacing(8.0)
                        .child(
                            TextBlock::new()
                                .text(&format!("Progress: {:.0}%", progress))
                                .font_size(12.0)
                                .foreground(&Brush::from_color(Color::GRAY)),
                        )
                        .child(
                            ProgressBar::new()
                                .value(progress)
                                .maximum(100.0)
                                .height(10.0),
                        ),
                )
                // Duration selector
                .child(
                    StackPanel::new()
                        .orientation(Orientation::Vertical)
                        .spacing(10.0)
                        .child(
                            TextBlock::new()
                                .text("Set Duration (minutes):")
                                .font_size(14.0)
                                .font_weight(FontWeight::Medium),
                        )
                        .child(
                            Grid::new()
                                .columns(vec![
                                    ColumnDefinition::star(1.0),
                                    ColumnDefinition::star(1.0),
                                    ColumnDefinition::star(1.0),
                                    ColumnDefinition::star(1.0),
                                ])
                                .column_spacing(8.0)
                                .child_at(
                                    Button::new()
                                        .content("1 min")
                                        .padding(Thickness::symmetric(15.0, 10.0))
                                        .on_click(move |_| {
                                            state_duration.write().timer.set_duration(60);
                                            println!("Set timer to 1 minute");
                                        }),
                                    0, 0
                                )
                                .child_at(
                                    Button::new()
                                        .content("5 min")
                                        .padding(Thickness::symmetric(15.0, 10.0))
                                        .on_click(move |_| {
                                            let state_5 = state_duration.clone();
                                            state_5.write().timer.set_duration(300);
                                            println!("Set timer to 5 minutes");
                                        }),
                                    0, 1
                                )
                                .child_at(
                                    Button::new()
                                        .content("10 min")
                                        .padding(Thickness::symmetric(15.0, 10.0))
                                        .on_click(move |_| {
                                            let state_10 = state_duration.clone();
                                            state_10.write().timer.set_duration(600);
                                            println!("Set timer to 10 minutes");
                                        }),
                                    0, 2
                                )
                                .child_at(
                                    Button::new()
                                        .content("30 min")
                                        .padding(Thickness::symmetric(15.0, 10.0))
                                        .on_click(move |_| {
                                            let state_30 = state_duration.clone();
                                            state_30.write().timer.set_duration(1800);
                                            println!("Set timer to 30 minutes");
                                        }),
                                    0, 3
                                ),
                        ),
                )
                // Control buttons
                .child(
                    Grid::new()
                        .columns(vec![
                            ColumnDefinition::star(1.0),
                            ColumnDefinition::star(1.0),
                            ColumnDefinition::star(1.0),
                        ])
                        .column_spacing(10.0)
                        .child_at(
                            Button::new()
                                .content(if is_running { "⏸️ Pause" } else { "▶️ Start" })
                                .padding(Thickness::symmetric(20.0, 15.0))
                                .font_size(14.0)
                                .on_click(move |_| {
                                    let mut s = state_start.write();
                                    if s.timer.is_running {
                                        s.timer.pause();
                                        println!("Timer paused");
                                    } else {
                                        s.timer.start();
                                        println!("Timer started");
                                    }
                                }),
                            0, 0
                        )
                        .child_at(
                            Button::new()
                                .content("⏹️ Reset")
                                .padding(Thickness::symmetric(20.0, 15.0))
                                .font_size(14.0)
                                .on_click(move |_| {
                                    state_reset.write().timer.reset();
                                    println!("Timer reset");
                                }),
                            0, 1
                        )
                        .child_at(
                            Button::new()
                                .content("➖ -1 min")
                                .padding(Thickness::symmetric(20.0, 15.0))
                                .font_size(14.0)
                                .enabled(!is_running)
                                .on_click(move |_| {
                                    let mut s = state_pause.write();
                                    let current = s.timer.duration_seconds;
                                    if current > 60 {
                                        s.timer.set_duration(current - 60);
                                        println!("Decreased timer by 1 minute");
                                    }
                                }),
                            0, 2
                        ),
                ),
        )
        .into()
}

fn build_stopwatch_ui(state: Arc<RwLock<AppState>>) -> UIElement {
    let state_start = state.clone();
    let state_reset = state.clone();
    let state_lap = state.clone();

    let stopwatch = &state.read().stopwatch;
    let display_time = stopwatch.get_display_time();
    let is_running = stopwatch.is_running;
    let lap_times = stopwatch.lap_times.clone();

    Border::new()
        .background(&Brush::white())
        .border_thickness_uniform(1.0)
        .border_brush(&Brush::from_color(Color::rgb(200, 200, 200)))
        .corner_radius_uniform(12.0)
        .padding_uniform(30.0)
        .child(
            StackPanel::new()
                .orientation(Orientation::Vertical)
                .spacing(25.0)
                // Stopwatch display
                .child(
                    Border::new()
                        .background(&Brush::from_color(Color::rgb(245, 247, 250)))
                        .corner_radius_uniform(15.0)
                        .padding_uniform(30.0)
                        .child(
                            TextBlock::new()
                                .text(&display_time)
                                .font_size(64.0)
                                .font_weight(FontWeight::Light)
                                .horizontal_alignment(HorizontalAlignment::Center)
                                .foreground(&Brush::from_color(Color::rgb(76, 175, 80))),
                        ),
                )
                // Control buttons
                .child(
                    Grid::new()
                        .columns(vec![
                            ColumnDefinition::star(1.0),
                            ColumnDefinition::star(1.0),
                            ColumnDefinition::star(1.0),
                        ])
                        .column_spacing(10.0)
                        .child_at(
                            Button::new()
                                .content(if is_running { "⏸️ Pause" } else { "▶️ Start" })
                                .padding(Thickness::symmetric(20.0, 15.0))
                                .font_size(14.0)
                                .on_click(move |_| {
                                    let mut s = state_start.write();
                                    if s.stopwatch.is_running {
                                        s.stopwatch.pause();
                                        println!("Stopwatch paused");
                                    } else {
                                        s.stopwatch.start();
                                        println!("Stopwatch started");
                                    }
                                }),
                            0, 0
                        )
                        .child_at(
                            Button::new()
                                .content("🔄 Reset")
                                .padding(Thickness::symmetric(20.0, 15.0))
                                .font_size(14.0)
                                .on_click(move |_| {
                                    state_reset.write().stopwatch.reset();
                                    println!("Stopwatch reset");
                                }),
                            0, 1
                        )
                        .child_at(
                            Button::new()
                                .content("🏁 Lap")
                                .padding(Thickness::symmetric(20.0, 15.0))
                                .font_size(14.0)
                                .enabled(is_running)
                                .on_click(move |_| {
                                    state_lap.write().stopwatch.lap();
                                }),
                            0, 2
                        ),
                )
                // Lap times
                .child(build_lap_times_panel(lap_times)),
        )
        .into()
}

fn build_lap_times_panel(lap_times: Vec<u64>) -> UIElement {
    let mut panel = StackPanel::new()
        .orientation(Orientation::Vertical)
        .spacing(8.0);

    if lap_times.is_empty() {
        panel = panel.child(
            TextBlock::new()
                .text("No lap times recorded")
                .font_size(14.0)
                .foreground(&Brush::from_color(Color::GRAY))
                .horizontal_alignment(HorizontalAlignment::Center),
        );
    } else {
        panel = panel.child(
            TextBlock::new()
                .text("Lap Times")
                .font_size(16.0)
                .font_weight(FontWeight::SemiBold)
                .margin(Thickness::new(0.0, 0.0, 0.0, 10.0)),
        );

        for (i, lap_time) in lap_times.iter().enumerate() {
            panel = panel.child(
                Border::new()
                    .background(&Brush::from_color(Color::rgb(248, 248, 248)))
                    .corner_radius_uniform(6.0)
                    .padding(Thickness::symmetric(15.0, 10.0))
                    .child(
                        Grid::new()
                            .columns(vec![
                                ColumnDefinition::auto(),
                                ColumnDefinition::star(1.0),
                            ])
                            .child_at(
                                TextBlock::new()
                                    .text(&format!("Lap {}", i + 1))
                                    .font_size(14.0)
                                    .font_weight(FontWeight::Medium),
                                0, 0
                            )
                            .child_at(
                                TextBlock::new()
                                    .text(&StopwatchState::format_time(*lap_time))
                                    .font_size(14.0)
                                    .horizontal_alignment(HorizontalAlignment::Right)
                                    .foreground(&Brush::from_color(Color::rgb(76, 175, 80))),
                                0, 1
                            ),
                    ),
            );
        }
    }

    ScrollViewer::new()
        .vertical_scroll_bar_visibility(ScrollBarVisibility::Auto)
        .height(150.0)
        .content(panel)
        .into()
}

