//! Music Player Example
//!
//! A music player interface demonstrating progress bars, sliders,
//! and list-based navigation.
//!
//! Run with: cargo run --example music_player

use winrt_xaml::prelude::*;
use std::sync::Arc;
use parking_lot::RwLock;

/// Song metadata
#[derive(Clone)]
struct Song {
    id: usize,
    title: String,
    artist: String,
    album: String,
    duration: u32, // in seconds
}

impl Song {
    fn new(id: usize, title: &str, artist: &str, album: &str, duration: u32) -> Self {
        Self {
            id,
            title: title.to_string(),
            artist: artist.to_string(),
            album: album.to_string(),
            duration,
        }
    }

    fn format_duration(&self) -> String {
        let mins = self.duration / 60;
        let secs = self.duration % 60;
        format!("{:02}:{:02}", mins, secs)
    }
}

/// Player state
#[derive(Clone, Copy, PartialEq)]
enum PlayerState {
    Playing,
    Paused,
    Stopped,
}

/// Music player app state
struct PlayerAppState {
    playlist: Vec<Song>,
    current_song_index: Option<usize>,
    playback_position: u32, // seconds
    volume: f64,            // 0.0 to 1.0
    state: PlayerState,
    repeat_mode: RepeatMode,
    shuffle_enabled: bool,
}

#[derive(Clone, Copy, PartialEq)]
enum RepeatMode {
    Off,
    All,
    One,
}

impl RepeatMode {
    fn icon(&self) -> &str {
        match self {
            RepeatMode::Off => "🔁",
            RepeatMode::All => "🔁",
            RepeatMode::One => "🔂",
        }
    }

    fn next(&self) -> Self {
        match self {
            RepeatMode::Off => RepeatMode::All,
            RepeatMode::All => RepeatMode::One,
            RepeatMode::One => RepeatMode::Off,
        }
    }
}

impl PlayerAppState {
    fn new() -> Self {
        Self {
            playlist: vec![
                Song::new(1, "Midnight Drive", "The Wanderers", "Night Sessions", 245),
                Song::new(2, "Electric Dreams", "Neon Pulse", "Synthwave Vol. 1", 198),
                Song::new(3, "Ocean Breeze", "Acoustic Soul", "Summer Vibes", 210),
                Song::new(4, "Urban Jungle", "City Beats", "Metropolitan", 182),
                Song::new(5, "Starlight", "Luna Nova", "Cosmic Journey", 267),
                Song::new(6, "Mountain Echo", "Folk Revival", "Nature Sounds", 234),
                Song::new(7, "Digital Sunrise", "Tech House", "Morning Sessions", 189),
                Song::new(8, "Desert Mirage", "World Fusion", "Global Rhythms", 223),
            ],
            current_song_index: Some(0),
            playback_position: 0,
            volume: 0.7,
            state: PlayerState::Stopped,
            repeat_mode: RepeatMode::Off,
            shuffle_enabled: false,
        }
    }

    fn current_song(&self) -> Option<&Song> {
        self.current_song_index
            .and_then(|idx| self.playlist.get(idx))
    }

    fn play(&mut self) {
        if self.current_song_index.is_none() && !self.playlist.is_empty() {
            self.current_song_index = Some(0);
        }
        self.state = PlayerState::Playing;
    }

    fn pause(&mut self) {
        self.state = PlayerState::Paused;
    }

    fn next_song(&mut self) {
        if let Some(current) = self.current_song_index {
            let next = (current + 1) % self.playlist.len();
            self.current_song_index = Some(next);
            self.playback_position = 0;
            if self.state == PlayerState::Playing {
                println!("Playing next: {}", self.current_song().unwrap().title);
            }
        }
    }

    fn previous_song(&mut self) {
        if let Some(current) = self.current_song_index {
            let prev = if current == 0 {
                self.playlist.len() - 1
            } else {
                current - 1
            };
            self.current_song_index = Some(prev);
            self.playback_position = 0;
            if self.state == PlayerState::Playing {
                println!("Playing previous: {}", self.current_song().unwrap().title);
            }
        }
    }

    fn format_position(&self) -> String {
        let mins = self.playback_position / 60;
        let secs = self.playback_position % 60;
        format!("{:02}:{:02}", mins, secs)
    }

    fn progress_percent(&self) -> f64 {
        if let Some(song) = self.current_song() {
            (self.playback_position as f64 / song.duration as f64) * 100.0
        } else {
            0.0
        }
    }
}

fn main() -> Result<()> {
    env_logger::init();

    let app = Application::new()?;

    let window = Window::builder()
        .title("Music Player")
        .size(800, 600)
        .build()?;

    let state = Arc::new(RwLock::new(PlayerAppState::new()));

    let content = build_player_ui(state)?;

    window.set_content(content)?;
    window.center()?;
    window.show()?;

    app.run()
}

fn build_player_ui(state: Arc<RwLock<PlayerAppState>>) -> Result<UIElement> {
    Ok(Border::new()
        .background(&Brush::from_color(Color::rgb(18, 18, 18)))
        .child(
            Grid::new()
                .rows(vec![
                    RowDefinition::star(1.0),   // Main content
                    RowDefinition::auto(),       // Player controls
                ])
                .child_at(build_main_content(state.clone()), 0, 0)
                .child_at(build_player_controls(state), 1, 0),
        )
        .into())
}

fn build_main_content(state: Arc<RwLock<AppState>>) -> UIElement {
    Grid::new()
        .columns(vec![
            ColumnDefinition::star(2.0), // Now playing
            ColumnDefinition::star(1.0), // Playlist
        ])
        .child_at(build_now_playing(state.clone()), 0, 0)
        .child_at(build_playlist(state), 0, 1)
        .into()
}

fn build_now_playing(state: Arc<RwLock<PlayerAppState>>) -> UIElement {
    let s = state.read();

    let (title, artist, album) = if let Some(song) = s.current_song() {
        (song.title.clone(), song.artist.clone(), song.album.clone())
    } else {
        ("No song selected".to_string(), "".to_string(), "".to_string())
    };

    Border::new()
        .background(&Brush::from_color(Color::rgb(28, 28, 28)))
        .padding_uniform(40.0)
        .child(
            StackPanel::new()
                .orientation(Orientation::Vertical)
                .spacing(30.0)
                .vertical_alignment(VerticalAlignment::Center)
                // Album art
                .child(
                    Border::new()
                        .background(&Brush::from_color(Color::rgb(50, 50, 50)))
                        .corner_radius_uniform(15.0)
                        .width(300.0)
                        .height(300.0)
                        .horizontal_alignment(HorizontalAlignment::Center)
                        .child(
                            StackPanel::new()
                                .orientation(Orientation::Vertical)
                                .vertical_alignment(VerticalAlignment::Center)
                                .horizontal_alignment(HorizontalAlignment::Center)
                                .spacing(10.0)
                                .child(
                                    TextBlock::new()
                                        .text("🎵")
                                        .font_size(100.0),
                                )
                                .child(
                                    TextBlock::new()
                                        .text("Album Art")
                                        .font_size(14.0)
                                        .foreground(&Brush::from_color(Color::rgb(150, 150, 150))),
                                ),
                        ),
                )
                // Song info
                .child(
                    StackPanel::new()
                        .orientation(Orientation::Vertical)
                        .spacing(8.0)
                        .horizontal_alignment(HorizontalAlignment::Center)
                        .child(
                            TextBlock::new()
                                .text(&title)
                                .font_size(28.0)
                                .font_weight(FontWeight::Bold)
                                .foreground(&Brush::white())
                                .horizontal_alignment(HorizontalAlignment::Center),
                        )
                        .child(
                            TextBlock::new()
                                .text(&artist)
                                .font_size(18.0)
                                .foreground(&Brush::from_color(Color::rgb(180, 180, 180)))
                                .horizontal_alignment(HorizontalAlignment::Center),
                        )
                        .child(
                            TextBlock::new()
                                .text(&album)
                                .font_size(14.0)
                                .foreground(&Brush::from_color(Color::rgb(120, 120, 120)))
                                .horizontal_alignment(HorizontalAlignment::Center),
                        ),
                ),
        )
        .into()
}

fn build_playlist(state: Arc<RwLock<PlayerAppState>>) -> UIElement {
    let s = state.read();
    let current_index = s.current_song_index;

    let mut panel = StackPanel::new()
        .orientation(Orientation::Vertical)
        .spacing(0.0);

    for (idx, song) in s.playlist.iter().enumerate() {
        let is_current = Some(idx) == current_index;
        panel = panel.child(build_playlist_item(song, is_current, state.clone()));
    }

    Border::new()
        .background(&Brush::from_color(Color::rgb(24, 24, 24)))
        .border_thickness(Thickness::new(1.0, 0.0, 0.0, 0.0))
        .border_brush(&Brush::from_color(Color::rgb(40, 40, 40)))
        .child(
            StackPanel::new()
                .orientation(Orientation::Vertical)
                .child(
                    Border::new()
                        .background(&Brush::from_color(Color::rgb(32, 32, 32)))
                        .padding(Thickness::symmetric(20.0, 15.0))
                        .child(
                            StackPanel::new()
                                .orientation(Orientation::Horizontal)
                                .spacing(10.0)
                                .child(
                                    TextBlock::new()
                                        .text("📝")
                                        .font_size(18.0),
                                )
                                .child(
                                    TextBlock::new()
                                        .text("Playlist")
                                        .font_size(16.0)
                                        .font_weight(FontWeight::SemiBold)
                                        .foreground(&Brush::white()),
                                )
                                .child(
                                    TextBlock::new()
                                        .text(&format!("({} songs)", s.playlist.len()))
                                        .font_size(13.0)
                                        .foreground(&Brush::from_color(Color::rgb(140, 140, 140)))
                                        .vertical_alignment(VerticalAlignment::Center),
                                ),
                        ),
                )
                .child(
                    ScrollViewer::new()
                        .vertical_scroll_bar_visibility(ScrollBarVisibility::Auto)
                        .content(panel),
                ),
        )
        .into()
}

fn build_playlist_item(song: &Song, is_current: bool, state: Arc<RwLock<PlayerAppState>>) -> UIElement {
    let song_id = song.id;

    let bg_color = if is_current {
        Color::rgb(40, 40, 40)
    } else {
        Color::rgb(24, 24, 24)
    };

    let text_color = if is_current {
        Color::rgb(100, 200, 255)
    } else {
        Color::rgb(200, 200, 200)
    };

    Border::new()
        .background(&Brush::from_color(bg_color))
        .border_thickness(Thickness::new(0.0, 0.0, 0.0, 1.0))
        .border_brush(&Brush::from_color(Color::rgb(35, 35, 35)))
        .child(
            Button::new()
                .padding_uniform(0.0)
                .on_click(move |_| {
                    let mut s = state.write();
                    s.current_song_index = s.playlist.iter().position(|s| s.id == song_id);
                    s.playback_position = 0;
                    s.play();
                    println!("Playing: {}", s.current_song().unwrap().title);
                })
                .content(
                    Grid::new()
                        .columns(vec![
                            ColumnDefinition::auto(),
                            ColumnDefinition::star(1.0),
                            ColumnDefinition::auto(),
                        ])
                        .padding(Thickness::symmetric(15.0, 12.0))
                        .column_spacing(12.0)
                        // Play indicator
                        .child_at(
                            TextBlock::new()
                                .text(if is_current { "▶" } else { "" })
                                .font_size(12.0)
                                .foreground(&Brush::from_color(Color::rgb(100, 200, 255)))
                                .width(20.0),
                            0, 0
                        )
                        // Song info
                        .child_at(
                            StackPanel::new()
                                .orientation(Orientation::Vertical)
                                .spacing(4.0)
                                .child(
                                    TextBlock::new()
                                        .text(&song.title)
                                        .font_size(13.0)
                                        .font_weight(if is_current { FontWeight::SemiBold } else { FontWeight::Normal })
                                        .foreground(&Brush::from_color(text_color))
                                        .text_wrapping(TextWrapping::NoWrap),
                                )
                                .child(
                                    TextBlock::new()
                                        .text(&format!("{} • {}", song.artist, song.album))
                                        .font_size(11.0)
                                        .foreground(&Brush::from_color(Color::rgb(120, 120, 120)))
                                        .text_wrapping(TextWrapping::NoWrap),
                                ),
                            0, 1
                        )
                        // Duration
                        .child_at(
                            TextBlock::new()
                                .text(&song.format_duration())
                                .font_size(12.0)
                                .foreground(&Brush::from_color(Color::rgb(140, 140, 140))),
                            0, 2
                        ),
                ),
        )
        .into()
}

fn build_player_controls(state: Arc<RwLock<PlayerAppState>>) -> UIElement {
    let state_play = state.clone();
    let state_prev = state.clone();
    let state_next = state.clone();
    let state_seek = state.clone();
    let state_volume = state.clone();
    let state_shuffle = state.clone();
    let state_repeat = state.clone();

    let s = state.read();
    let is_playing = s.state == PlayerState::Playing;
    let position = s.format_position();
    let duration = s.current_song().map(|s| s.format_duration()).unwrap_or_else(|| "00:00".to_string());
    let progress = s.progress_percent();
    let volume = s.volume;
    let shuffle = s.shuffle_enabled;
    let repeat = s.repeat_mode;

    Border::new()
        .background(&Brush::from_color(Color::rgb(32, 32, 32)))
        .border_thickness(Thickness::new(0.0, 1.0, 0.0, 0.0))
        .border_brush(&Brush::from_color(Color::rgb(50, 50, 50)))
        .padding(Thickness::symmetric(30.0, 20.0))
        .child(
            StackPanel::new()
                .orientation(Orientation::Vertical)
                .spacing(15.0)
                // Progress bar
                .child(
                    Grid::new()
                        .columns(vec![
                            ColumnDefinition::auto(),
                            ColumnDefinition::star(1.0),
                            ColumnDefinition::auto(),
                        ])
                        .column_spacing(15.0)
                        // Current time
                        .child_at(
                            TextBlock::new()
                                .text(&position)
                                .font_size(12.0)
                                .foreground(&Brush::from_color(Color::rgb(180, 180, 180))),
                            0, 0
                        )
                        // Progress bar
                        .child_at(
                            Slider::new()
                                .minimum(0.0)
                                .maximum(100.0)
                                .value(progress)
                                .on_value_changed(move |value| {
                                    println!("Seek to: {:.0}%", value);
                                    // In real app, update playback position
                                }),
                            0, 1
                        )
                        // Duration
                        .child_at(
                            TextBlock::new()
                                .text(&duration)
                                .font_size(12.0)
                                .foreground(&Brush::from_color(Color::rgb(180, 180, 180))),
                            0, 2
                        ),
                )
                // Main controls
                .child(
                    Grid::new()
                        .columns(vec![
                            ColumnDefinition::star(1.0),
                            ColumnDefinition::auto(),
                            ColumnDefinition::star(1.0),
                        ])
                        // Left controls (shuffle, repeat)
                        .child_at(
                            StackPanel::new()
                                .orientation(Orientation::Horizontal)
                                .spacing(15.0)
                                .child(
                                    Button::new()
                                        .content("🔀")
                                        .font_size(18.0)
                                        .padding(Thickness::symmetric(12.0, 8.0))
                                        .on_click(move |_| {
                                            let mut s = state_shuffle.write();
                                            s.shuffle_enabled = !s.shuffle_enabled;
                                            println!("Shuffle: {}", s.shuffle_enabled);
                                        }),
                                )
                                .child(
                                    Button::new()
                                        .content(repeat.icon())
                                        .font_size(18.0)
                                        .padding(Thickness::symmetric(12.0, 8.0))
                                        .on_click(move |_| {
                                            let mut s = state_repeat.write();
                                            s.repeat_mode = s.repeat_mode.next();
                                            println!("Repeat: {:?}", s.repeat_mode);
                                        }),
                                ),
                            0, 0
                        )
                        // Center controls (playback)
                        .child_at(
                            StackPanel::new()
                                .orientation(Orientation::Horizontal)
                                .spacing(15.0)
                                .horizontal_alignment(HorizontalAlignment::Center)
                                .child(
                                    Button::new()
                                        .content("⏮️")
                                        .font_size(24.0)
                                        .padding(Thickness::symmetric(15.0, 10.0))
                                        .on_click(move |_| {
                                            state_prev.write().previous_song();
                                        }),
                                )
                                .child(
                                    Button::new()
                                        .content(if is_playing { "⏸️" } else { "▶️" })
                                        .font_size(32.0)
                                        .padding(Thickness::symmetric(20.0, 15.0))
                                        .on_click(move |_| {
                                            let mut s = state_play.write();
                                            if s.state == PlayerState::Playing {
                                                s.pause();
                                                println!("Paused");
                                            } else {
                                                s.play();
                                                println!("Playing");
                                            }
                                        }),
                                )
                                .child(
                                    Button::new()
                                        .content("⏭️")
                                        .font_size(24.0)
                                        .padding(Thickness::symmetric(15.0, 10.0))
                                        .on_click(move |_| {
                                            state_next.write().next_song();
                                        }),
                                ),
                            0, 1
                        )
                        // Right controls (volume)
                        .child_at(
                            StackPanel::new()
                                .orientation(Orientation::Horizontal)
                                .spacing(10.0)
                                .horizontal_alignment(HorizontalAlignment::Right)
                                .child(
                                    TextBlock::new()
                                        .text(if volume > 0.0 { "🔊" } else { "🔇" })
                                        .font_size(18.0)
                                        .vertical_alignment(VerticalAlignment::Center),
                                )
                                .child(
                                    Slider::new()
                                        .minimum(0.0)
                                        .maximum(1.0)
                                        .value(volume)
                                        .width(120.0)
                                        .on_value_changed(move |val| {
                                            state_volume.write().volume = val;
                                            println!("Volume: {:.0}%", val * 100.0);
                                        }),
                                )
                                .child(
                                    TextBlock::new()
                                        .text(&format!("{:.0}%", volume * 100.0))
                                        .font_size(12.0)
                                        .foreground(&Brush::from_color(Color::rgb(180, 180, 180)))
                                        .vertical_alignment(VerticalAlignment::Center)
                                        .width(40.0),
                                ),
                            0, 2
                        ),
                ),
        )
        .into()
}

