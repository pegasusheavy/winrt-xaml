//! Todo List Application Example
//!
//! A complete example demonstrating a practical application using winrt-xaml.
//! This shows how to build a simple todo list with add, complete, and delete functionality.
//!
//! Run with: cargo run --example todo_app

use winrt_xaml::prelude::*;
use std::sync::Arc;
use parking_lot::RwLock;

/// Represents a single todo item
#[derive(Clone)]
struct TodoItem {
    id: u32,
    text: String,
    completed: bool,
}

/// Application state
struct AppState {
    todos: Vec<TodoItem>,
    next_id: u32,
}

impl AppState {
    fn new() -> Self {
        Self {
            todos: vec![
                TodoItem { id: 1, text: "Learn Rust".to_string(), completed: true },
                TodoItem { id: 2, text: "Build a WinRT app".to_string(), completed: false },
                TodoItem { id: 3, text: "Create amazing UIs".to_string(), completed: false },
            ],
            next_id: 4,
        }
    }

    fn add_todo(&mut self, text: String) -> TodoItem {
        let item = TodoItem {
            id: self.next_id,
            text,
            completed: false,
        };
        self.next_id += 1;
        self.todos.push(item.clone());
        item
    }

    fn toggle_todo(&mut self, id: u32) {
        if let Some(todo) = self.todos.iter_mut().find(|t| t.id == id) {
            todo.completed = !todo.completed;
        }
    }

    fn delete_todo(&mut self, id: u32) {
        self.todos.retain(|t| t.id != id);
    }

    fn pending_count(&self) -> usize {
        self.todos.iter().filter(|t| !t.completed).count()
    }

    fn completed_count(&self) -> usize {
        self.todos.iter().filter(|t| t.completed).count()
    }
}

fn main() -> Result<()> {
    env_logger::init();

    let app = Application::new()?;

    // Create shared application state
    let state = Arc::new(RwLock::new(AppState::new()));

    let window = Window::builder()
        .title("Todo List - WinRT-XAML Demo")
        .size(500, 600)
        .build()?;

    // Build the UI
    let content = build_ui(state.clone())?;

    window.set_content(content)?;
    window.center()?;
    window.show()?;

    app.run()
}

fn build_ui(state: Arc<RwLock<AppState>>) -> Result<UIElement> {
    let state_for_stats = state.clone();

    Ok(Border::new()
        .background(&Brush::from_color(Color::rgb(245, 245, 245)))
        .child(
            StackPanel::new()
                .orientation(Orientation::Vertical)
                .padding_uniform(20.0)
                // Header
                .child(build_header())
                // Add todo section
                .child(build_add_section(state.clone()))
                // Stats bar
                .child(build_stats_bar(state_for_stats))
                // Todo list
                .child(build_todo_list(state.clone()))
                // Footer
                .child(build_footer()),
        )
        .into())
}

fn build_header() -> UIElement {
    StackPanel::new()
        .orientation(Orientation::Vertical)
        .spacing(5.0)
        .margin(Thickness::new(0.0, 0.0, 0.0, 20.0))
        .child(
            TextBlock::new()
                .text("My Todo List")
                .font_size(32.0)
                .font_weight(FontWeight::Bold)
                .foreground(&Brush::from_color(Color::rgb(51, 51, 51)))
                .horizontal_alignment(HorizontalAlignment::Center),
        )
        .child(
            TextBlock::new()
                .text("Built with WinRT-XAML for Rust")
                .font_size(14.0)
                .foreground(&Brush::from_color(Color::rgb(128, 128, 128)))
                .horizontal_alignment(HorizontalAlignment::Center),
        )
        .into()
}

fn build_add_section(state: Arc<RwLock<AppState>>) -> UIElement {
    let input_text = Arc::new(RwLock::new(String::new()));
    let input_text_for_change = input_text.clone();
    let input_text_for_click = input_text.clone();

    Border::new()
        .background(&Brush::from_color(Color::WHITE))
        .border_thickness_uniform(1.0)
        .border_brush(&Brush::from_color(Color::rgb(200, 200, 200)))
        .corner_radius_uniform(8.0)
        .padding_uniform(15.0)
        .margin(Thickness::new(0.0, 0.0, 0.0, 15.0))
        .child(
            StackPanel::new()
                .orientation(Orientation::Horizontal)
                .spacing(10.0)
                .child(
                    TextBox::new()
                        .placeholder("What needs to be done?")
                        .width(320.0)
                        .font_size(14.0)
                        .on_text_changed(move |text| {
                            *input_text_for_change.write() = text;
                        }),
                )
                .child(
                    Button::new()
                        .content("Add Task")
                        .padding(Thickness::symmetric(20.0, 8.0))
                        .font_size(14.0)
                        .on_click(move |_| {
                            let text = input_text_for_click.read().clone();
                            if !text.trim().is_empty() {
                                let item = state.write().add_todo(text.clone());
                                println!("Added todo: {} (id: {})", item.text, item.id);
                                // In a real app, you'd update the UI here
                            }
                        }),
                ),
        )
        .into()
}

fn build_stats_bar(state: Arc<RwLock<AppState>>) -> UIElement {
    let state_read = state.read();
    let pending = state_read.pending_count();
    let completed = state_read.completed_count();
    let total = state_read.todos.len();

    StackPanel::new()
        .orientation(Orientation::Horizontal)
        .spacing(20.0)
        .margin(Thickness::new(0.0, 0.0, 0.0, 15.0))
        .horizontal_alignment(HorizontalAlignment::Center)
        .child(build_stat_badge("Total", total, Color::rgb(100, 100, 100)))
        .child(build_stat_badge("Pending", pending, Color::rgb(255, 152, 0)))
        .child(build_stat_badge("Completed", completed, Color::rgb(76, 175, 80)))
        .into()
}

fn build_stat_badge(label: &str, count: usize, color: Color) -> UIElement {
    Border::new()
        .background(&Brush::from_color(color))
        .corner_radius_uniform(15.0)
        .padding(Thickness::symmetric(15.0, 8.0))
        .child(
            StackPanel::new()
                .orientation(Orientation::Horizontal)
                .spacing(8.0)
                .child(
                    TextBlock::new()
                        .text(label)
                        .font_size(12.0)
                        .foreground(&Brush::white()),
                )
                .child(
                    TextBlock::new()
                        .text(&count.to_string())
                        .font_size(12.0)
                        .font_weight(FontWeight::Bold)
                        .foreground(&Brush::white()),
                ),
        )
        .into()
}

fn build_todo_list(state: Arc<RwLock<AppState>>) -> UIElement {
    let todos = state.read().todos.clone();

    let mut list_panel = StackPanel::new()
        .orientation(Orientation::Vertical)
        .spacing(8.0);

    if todos.is_empty() {
        list_panel = list_panel.child(
            Border::new()
                .padding_uniform(30.0)
                .child(
                    TextBlock::new()
                        .text("No tasks yet. Add one above!")
                        .font_size(16.0)
                        .foreground(&Brush::from_color(Color::GRAY))
                        .horizontal_alignment(HorizontalAlignment::Center),
                ),
        );
    } else {
        for todo in todos {
            list_panel = list_panel.child(build_todo_item(&todo, state.clone()));
        }
    }

    ScrollViewer::new()
        .vertical_scroll_bar_visibility(ScrollBarVisibility::Auto)
        .horizontal_scroll_bar_visibility(ScrollBarVisibility::Disabled)
        .height(300.0)
        .content(list_panel)
        .into()
}

fn build_todo_item(todo: &TodoItem, state: Arc<RwLock<AppState>>) -> UIElement {
    let todo_id = todo.id;
    let is_completed = todo.completed;
    let state_for_toggle = state.clone();
    let state_for_delete = state.clone();

    let background_color = if is_completed {
        Color::rgb(232, 245, 233)
    } else {
        Color::WHITE
    };

    let text_color = if is_completed {
        Color::rgb(128, 128, 128)
    } else {
        Color::rgb(51, 51, 51)
    };

    Border::new()
        .background(&Brush::from_color(background_color))
        .border_thickness_uniform(1.0)
        .border_brush(&Brush::from_color(Color::rgb(224, 224, 224)))
        .corner_radius_uniform(6.0)
        .padding(Thickness::symmetric(15.0, 12.0))
        .child(
            Grid::new()
                .columns(vec![
                    ColumnDefinition::auto(),
                    ColumnDefinition::star(1.0),
                    ColumnDefinition::auto(),
                ])
                .column_spacing(12.0)
                // Checkbox
                .child_at(
                    CheckBox::new()
                        .checked(is_completed)
                        .on_checked_changed(move |_| {
                            state_for_toggle.write().toggle_todo(todo_id);
                            println!("Toggled todo {}", todo_id);
                        }),
                    0,
                    0,
                )
                // Text
                .child_at(
                    TextBlock::new()
                        .text(&todo.text)
                        .font_size(14.0)
                        .foreground(&Brush::from_color(text_color))
                        .text_wrapping(TextWrapping::Wrap)
                        .vertical_alignment(VerticalAlignment::Center),
                    0,
                    1,
                )
                // Delete button
                .child_at(
                    Button::new()
                        .content("Delete")
                        .padding(Thickness::symmetric(10.0, 5.0))
                        .font_size(12.0)
                        .on_click(move |_| {
                            state_for_delete.write().delete_todo(todo_id);
                            println!("Deleted todo {}", todo_id);
                        }),
                    0,
                    2,
                ),
        )
        .into()
}

fn build_footer() -> UIElement {
    Border::new()
        .margin(Thickness::new(0.0, 20.0, 0.0, 0.0))
        .padding_uniform(15.0)
        .child(
            StackPanel::new()
                .orientation(Orientation::Vertical)
                .spacing(10.0)
                .child(
                    TextBlock::new()
                        .text("Keyboard Shortcuts")
                        .font_size(14.0)
                        .font_weight(FontWeight::SemiBold),
                )
                .child(
                    StackPanel::new()
                        .orientation(Orientation::Horizontal)
                        .spacing(20.0)
                        .child(
                            TextBlock::new()
                                .text("Enter - Add task")
                                .font_size(12.0)
                                .foreground(&Brush::from_color(Color::GRAY)),
                        )
                        .child(
                            TextBlock::new()
                                .text("Click checkbox - Toggle complete")
                                .font_size(12.0)
                                .foreground(&Brush::from_color(Color::GRAY)),
                        ),
                ),
        )
        .into()
}
