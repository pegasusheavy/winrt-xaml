# WinRT-XAML Examples

This directory contains comprehensive examples demonstrating various features and use cases of the WinRT-XAML library.

## Running Examples

To run any example, use:

```bash
cargo run --example <example_name>
```

For example:
```bash
cargo run --example calculator
```

## Available Examples

### 1. Basic Window (`basic_window.rs`)
**Complexity:** Beginner
**Run:** `cargo run --example basic_window`

A simple window demonstrating the basics of the WinRT-XAML library.

**Features:**
- Basic window creation
- StackPanel layout
- TextBlock and Button controls
- Event handling
- Application lifecycle

### 2. Counter (`counter.rs`)
**Complexity:** Beginner
**Run:** `cargo run --example counter`

A minimal counter application showing state management.

**Features:**
- Shared state with `Arc<RwLock<T>>`
- Button click handlers
- Dynamic UI updates
- Simple layout with StackPanel

### 3. Controls Demo (`controls_demo.rs`)
**Complexity:** Beginner
**Run:** `cargo run --example controls_demo`

A comprehensive showcase of all available UI controls.

**Features:**
- TextBlock with various styles
- Button types and states
- TextBox (single-line, multi-line, read-only)
- CheckBox (2-state and 3-state)
- ComboBox with items
- ToggleSwitch
- Slider
- ProgressBar (determinate and indeterminate)
- Grid and StackPanel layouts
- ScrollViewer for scrollable content

### 4. Data Binding (`data_binding.rs`)
**Complexity:** Intermediate
**Run:** `cargo run --example data_binding`

Demonstrates XAML loading and resource dictionaries.

**Features:**
- Loading UI from XAML markup strings
- Resource dictionaries
- Application-level resources
- XAML parsing

### 5. Form Demo (`form_demo.rs`)
**Complexity:** Intermediate
**Run:** `cargo run --example form_demo`

A registration form with validation demonstrating complex form handling.

**Features:**
- Form field creation
- Input validation
- CheckBox for terms and conditions
- Password fields
- Form submission and clearing
- Error display
- ScrollViewer for long forms

### 6. Todo App (`todo_app.rs`)
**Complexity:** Intermediate
**Run:** `cargo run --example todo_app`

A complete todo list application with CRUD operations.

**Features:**
- Adding new tasks
- Toggling task completion
- Deleting tasks
- Task filtering
- Statistics display
- Complex state management
- Grid layout for list items

### 7. Calculator (`calculator.rs`)
**Complexity:** Intermediate
**Run:** `cargo run --example calculator`

A functional calculator with full arithmetic operations.

**Features:**
- Grid-based button layout
- Calculator logic implementation
- Number formatting
- Operation chaining
- Error handling (division by zero)
- Dark theme UI
- State machine pattern

### 8. Settings Panel (`settings_panel.rs`)
**Complexity:** Intermediate
**Run:** `cargo run --example settings_panel`

A comprehensive settings interface with various control types.

**Features:**
- ComboBox for dropdowns (theme, language)
- ToggleSwitch for on/off settings
- Slider for continuous values (font size, opacity)
- CheckBox for boolean options
- Organized sections
- Save and reset functionality
- Professional layout with descriptions

### 9. Color Picker (`color_picker.rs`)
**Complexity:** Intermediate
**Run:** `cargo run --example color_picker`

An interactive color picker with live preview.

**Features:**
- RGB sliders
- Live color preview
- Color format display (HEX, RGB, HSL, Decimal)
- Preset colors
- Random color generation
- Color samples on different backgrounds
- RGB to HSL conversion

### 10. Timer & Stopwatch (`timer_stopwatch.rs`)
**Complexity:** Intermediate
**Run:** `cargo run --example timer_stopwatch`

Time tracking with countdown timer and stopwatch functionality.

**Features:**
- Countdown timer with preset durations
- Stopwatch with lap times
- Mode switching
- Progress bar for timer
- Play/pause/reset controls
- Time formatting
- Lap time recording

### 11. Image Gallery (`image_gallery.rs`)
**Complexity:** Intermediate
**Run:** `cargo run --example image_gallery`

A photo gallery with filtering and detail view.

**Features:**
- Grid-based gallery layout
- Photo metadata display
- Category filtering
- Selection highlighting
- Detail panel
- Action buttons
- Scrollable content
- Responsive layout

### 12. Music Player (`music_player.rs`)
**Complexity:** Advanced
**Run:** `cargo run --example music_player`

A music player interface with playlist and playback controls.

**Features:**
- Playlist management
- Now playing display
- Playback controls (play, pause, next, previous)
- Progress bar for playback
- Volume slider
- Shuffle and repeat modes
- Dark theme
- Album art placeholder
- Track information display

### 13. Weather Dashboard (`weather_dashboard.rs`)
**Complexity:** Advanced
**Run:** `cargo run --example weather_dashboard`

A weather dashboard with current conditions and forecast.

**Features:**
- Current weather display with large icons
- Weather details grid (humidity, wind, pressure, UV, visibility)
- 5-day forecast
- Icon-based weather conditions
- Responsive card layout
- Gradient background
- Professional data visualization

### 14. Shopping Cart (`shopping_cart.rs`)
**Complexity:** Advanced
**Run:** `cargo run --example shopping_cart`

An e-commerce shopping cart interface.

**Features:**
- Product catalog with grid layout
- Add to cart functionality
- Quantity management
- Cart summary with calculations
- Discount code system
- Tax calculation
- Product categories and ratings
- Responsive product cards
- Checkout flow

### 15. Chat Interface (`chat_interface.rs`)
**Complexity:** Advanced
**Run:** `cargo run --example chat_interface`

A modern messaging interface.

**Features:**
- Message bubbles (sent/received/system)
- Contact header with status
- Input area with attachment button
- Timestamp display
- Online status indicator
- Auto-scroll to latest messages
- Message composition
- Simulated responses

## Learning Path

### Beginners
Start with these examples to learn the basics:
1. `basic_window` - Learn basic structure
2. `counter` - Understand state management
3. `controls_demo` - Explore available controls

### Intermediate
Progress to these for more complex patterns:
4. `form_demo` - Learn form handling
5. `calculator` - Master layout with Grid
6. `todo_app` - Build a complete app
7. `settings_panel` - Use various controls together
8. `color_picker` - Implement real-time updates

### Advanced
Tackle these for production-ready patterns:
9. `music_player` - Complex state and playback controls
10. `shopping_cart` - E-commerce patterns
11. `weather_dashboard` - Data visualization
12. `chat_interface` - Real-time messaging UI

## Common Patterns

### State Management
Most examples use `Arc<RwLock<T>>` for shared mutable state:

```rust
let state = Arc::new(RwLock::new(AppState::new()));
```

### Event Handling
Events are handled with closures:

```rust
Button::new()
    .content("Click Me")
    .on_click(move |_| {
        println!("Button clicked!");
    })
```

### Layouts
Examples demonstrate three main layout types:
- **StackPanel**: Linear layouts (vertical/horizontal)
- **Grid**: Complex row/column layouts
- **Border**: Single-child container with styling

### UI Updates
To update UI, modify the shared state and trigger re-renders:

```rust
let state_clone = state.clone();
Button::new()
    .on_click(move |_| {
        state_clone.write().counter += 1;
    })
```

## Building Your Own App

1. Start with `basic_window.rs` as a template
2. Choose the appropriate layout (StackPanel, Grid, or combination)
3. Add controls and configure their properties using the builder pattern
4. Set up shared state with `Arc<RwLock<T>>`
5. Connect event handlers
6. Add styling with borders, colors, and spacing

## Additional Resources

- [Main README](../README.md) - Library documentation
- [API Documentation](https://docs.rs/winrt-xaml) - Full API reference
- [WinRT Documentation](https://docs.microsoft.com/en-us/windows/uwp/xaml-platform/) - XAML concepts

## Contributing

Found a bug or want to add a new example? Contributions are welcome!

1. Fork the repository
2. Create your example
3. Follow the existing naming and documentation patterns
4. Add your example to this README
5. Submit a pull request

## License

These examples are provided under the same license as the main library (MIT OR Apache-2.0).

