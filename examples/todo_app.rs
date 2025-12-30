//! Todo application example.

use winrt_xaml::prelude::*;
use std::sync::Arc;
use parking_lot::RwLock;

fn main() -> Result<()> {
    env_logger::init();

    println!("Creating todo app...");

    let app = Application::new()?;

    let window = Window::builder()
        .title("Todo App")
        .size(500, 600)
        .build()?;

    // Todo list state
    let todos = Arc::new(RwLock::new(Vec::<TodoItem>::new()));

    // Title
    let title = TextBlock::new()?
        .with_text("My Todo List")?;
    title.set_position(150, 20);
    title.set_size(200, 30);

    // Input field
    let input = TextBox::new()?;
    input.set_position(50, 70);
    input.set_size(300, 35);
    input.set_placeholder("Enter a new todo...");

    // Add button
    let add_button = Button::new()?
        .with_content("Add")?;
    add_button.set_position(370, 70);
    add_button.set_size(80, 35);

    let todos_clone = todos.clone();
    let input_clone = input.clone();
    add_button.click().subscribe(move |_| {
        let text = input_clone.text();
        if !text.is_empty() {
            let mut list = todos_clone.write();
            list.push(TodoItem {
                text: text.clone(),
                completed: false,
            });
            let _ = input_clone.set_text("");
            println!("Added todo: {}", text);
            println!("Total todos: {}", list.len());
        }
    });

    // Status label
    let status = TextBlock::new()?
        .with_text("0 todos")?;
    status.set_position(50, 130);
    status.set_size(400, 25);

    // Clear completed button
    let clear_button = Button::new()?
        .with_content("Clear Completed")?;
    clear_button.set_position(50, 500);
    clear_button.set_size(150, 35);

    let todos_clone = todos.clone();
    let status_clone = status.clone();
    clear_button.click().subscribe(move |_| {
        let mut list = todos_clone.write();
        let before = list.len();
        list.retain(|todo| !todo.completed);
        let removed = before - list.len();
        let _ = status_clone.set_text(&format!("{} todos ({} removed)", list.len(), removed));
        println!("Cleared {} completed todos", removed);
    });

    // Show all button
    let show_all = Button::new()?
        .with_content("Show All")?;
    show_all.set_position(220, 500);
    show_all.set_size(100, 35);

    let todos_clone = todos.clone();
    let status_clone = status.clone();
    show_all.click().subscribe(move |_| {
        let list = todos_clone.read();
        let _ = status_clone.set_text(&format!("{} todos", list.len()));
        println!("\n=== All Todos ===");
        for (i, todo) in list.iter().enumerate() {
            println!("{}. [{}] {}", i + 1, if todo.completed { "✓" } else { " " }, todo.text);
        }
        println!("================\n");
    });

    // Instructions
    let instructions = TextBlock::new()?
        .with_text("Enter todos above and click Add")?;
    instructions.set_position(50, 550);
    instructions.set_size(400, 25);

    window.set_content(add_button)?;

    // Show the window
    window.show()?;

    println!("Starting application...");
    app.run()
}

#[derive(Clone)]
struct TodoItem {
    text: String,
    completed: bool,
}
