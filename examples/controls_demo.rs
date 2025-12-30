//! Controls demonstration example.
//!
//! This example shows all the available controls in the library.

use winrt_xaml::prelude::*;

fn main() -> Result<()> {
    env_logger::init();

    let app = Application::new()?;

    let window = Window::builder()
        .title("WinRT-XAML Controls Demo")
        .size(1000, 800)
        .build()?;

    // Create a scrollable content area
    let content = ScrollViewer::new()
        .horizontal_scroll_bar_visibility(ScrollBarVisibility::Disabled)
        .vertical_scroll_bar_visibility(ScrollBarVisibility::Auto)
        .content(
            StackPanel::new()
                .orientation(Orientation::Vertical)
                .spacing(20.0)
                .padding_uniform(30.0)
                // Title
                .child(
                    TextBlock::new()
                        .text("WinRT-XAML Controls Demo")
                        .font_size(28.0)
                        .font_weight(FontWeight::Bold),
                )
                // Text Controls Section
                .child(create_section("Text Controls", create_text_controls()))
                // Button Controls Section
                .child(create_section("Button Controls", create_button_controls()))
                // Input Controls Section
                .child(create_section("Input Controls", create_input_controls()))
                // Selection Controls Section
                .child(create_section("Selection Controls", create_selection_controls()))
                // Progress Controls Section
                .child(create_section("Progress Controls", create_progress_controls()))
                // Layout Section
                .child(create_section("Layout Panels", create_layout_demo())),
        );

    window.set_content(content)?;
    window.center()?;
    window.show()?;

    app.run()
}

fn create_section(title: &str, content: impl Into<UIElement>) -> UIElement {
    Border::new()
        .border_thickness_uniform(1.0)
        .border_brush(&Brush::from_color(Color::LIGHT_GRAY))
        .corner_radius_uniform(8.0)
        .padding_uniform(20.0)
        .margin(Thickness::new(0.0, 10.0, 0.0, 10.0))
        .child(
            StackPanel::new()
                .orientation(Orientation::Vertical)
                .spacing(15.0)
                .child(
                    TextBlock::new()
                        .text(title)
                        .font_size(20.0)
                        .font_weight(FontWeight::SemiBold),
                )
                .child(content),
        )
        .into()
}

fn create_text_controls() -> UIElement {
    StackPanel::new()
        .orientation(Orientation::Vertical)
        .spacing(10.0)
        .child(
            TextBlock::new()
                .text("Normal text")
                .font_size(14.0),
        )
        .child(
            TextBlock::new()
                .text("Bold text")
                .font_size(14.0)
                .font_weight(FontWeight::Bold),
        )
        .child(
            TextBlock::new()
                .text("Italic text")
                .font_size(14.0)
                .font_style(FontStyle::Italic),
        )
        .child(
            TextBlock::new()
                .text("Large heading")
                .font_size(24.0)
                .font_weight(FontWeight::Light),
        )
        .child(
            TextBlock::new()
                .text("This is a longer text that demonstrates text wrapping when the content exceeds the available width of the container.")
                .font_size(14.0)
                .text_wrapping(TextWrapping::Wrap)
                .max_lines(3),
        )
        .into()
}

fn create_button_controls() -> UIElement {
    StackPanel::new()
        .orientation(Orientation::Horizontal)
        .spacing(10.0)
        .child(
            Button::new()
                .content("Normal Button")
                .on_click(|_| println!("Normal button clicked")),
        )
        .child(
            Button::new()
                .content("Large Button")
                .font_size(18.0)
                .padding(Thickness::symmetric(30.0, 15.0))
                .on_click(|_| println!("Large button clicked")),
        )
        .child(
            Button::new()
                .content("Disabled")
                .enabled(false),
        )
        .into()
}

fn create_input_controls() -> UIElement {
    Grid::new()
        .columns(vec![
            ColumnDefinition::pixel(150.0),
            ColumnDefinition::star(1.0),
        ])
        .rows(vec![
            RowDefinition::auto(),
            RowDefinition::auto(),
            RowDefinition::auto(),
        ])
        .row_spacing(10.0)
        .column_spacing(10.0)
        // TextBox
        .child_at(
            TextBlock::new()
                .text("Text Input:")
                .vertical_alignment(VerticalAlignment::Center),
            0,
            0,
        )
        .child_at(
            TextBox::new()
                .placeholder("Enter text here...")
                .on_text_changed(|text| println!("Text changed: {}", text)),
            0,
            1,
        )
        // Password-style TextBox
        .child_at(
            TextBlock::new()
                .text("Read-only:")
                .vertical_alignment(VerticalAlignment::Center),
            1,
            0,
        )
        .child_at(
            TextBox::new()
                .text("This is read-only text")
                .read_only(true),
            1,
            1,
        )
        // Multi-line TextBox
        .child_at(
            TextBlock::new()
                .text("Multi-line:")
                .vertical_alignment(VerticalAlignment::Top),
            2,
            0,
        )
        .child_at(
            TextBox::new()
                .accepts_return(true)
                .height(80.0)
                .placeholder("Enter multiple lines..."),
            2,
            1,
        )
        .into()
}

fn create_selection_controls() -> UIElement {
    StackPanel::new()
        .orientation(Orientation::Vertical)
        .spacing(15.0)
        // CheckBoxes
        .child(
            StackPanel::new()
                .orientation(Orientation::Horizontal)
                .spacing(20.0)
                .child(
                    CheckBox::new()
                        .content("Option 1")
                        .checked(true)
                        .on_checked_changed(|checked| println!("Option 1: {:?}", checked)),
                )
                .child(
                    CheckBox::new()
                        .content("Option 2")
                        .on_checked_changed(|checked| println!("Option 2: {:?}", checked)),
                )
                .child(
                    CheckBox::new()
                        .content("Three State")
                        .three_state(true)
                        .on_checked_changed(|checked| println!("Three state: {:?}", checked)),
                ),
        )
        // ComboBox
        .child(
            StackPanel::new()
                .orientation(Orientation::Horizontal)
                .spacing(10.0)
                .child(
                    TextBlock::new()
                        .text("ComboBox:")
                        .vertical_alignment(VerticalAlignment::Center),
                )
                .child(
                    ComboBox::new()
                        .width(200.0)
                        .items(vec!["First Option", "Second Option", "Third Option"])
                        .selected_index(0)
                        .on_selection_changed(|index| println!("Selected index: {:?}", index)),
                ),
        )
        // ToggleSwitch
        .child(
            StackPanel::new()
                .orientation(Orientation::Horizontal)
                .spacing(20.0)
                .child(
                    ToggleSwitch::new()
                        .header("Feature Toggle")
                        .on_content("Enabled")
                        .off_content("Disabled")
                        .on_toggled(|is_on| println!("Toggle: {}", is_on)),
                ),
        )
        // Slider
        .child(
            StackPanel::new()
                .orientation(Orientation::Horizontal)
                .spacing(10.0)
                .child(
                    TextBlock::new()
                        .text("Slider:")
                        .vertical_alignment(VerticalAlignment::Center),
                )
                .child(
                    Slider::new()
                        .minimum(0.0)
                        .maximum(100.0)
                        .value(50.0)
                        .width(200.0)
                        .on_value_changed(|value| println!("Slider value: {}", value)),
                ),
        )
        .into()
}

fn create_progress_controls() -> UIElement {
    StackPanel::new()
        .orientation(Orientation::Vertical)
        .spacing(15.0)
        .child(
            StackPanel::new()
                .orientation(Orientation::Horizontal)
                .spacing(10.0)
                .child(
                    TextBlock::new()
                        .text("Determinate:")
                        .width(100.0),
                )
                .child(
                    ProgressBar::new()
                        .value(75.0)
                        .maximum(100.0)
                        .width(300.0),
                ),
        )
        .child(
            StackPanel::new()
                .orientation(Orientation::Horizontal)
                .spacing(10.0)
                .child(
                    TextBlock::new()
                        .text("Indeterminate:")
                        .width(100.0),
                )
                .child(
                    ProgressBar::new()
                        .is_indeterminate(true)
                        .width(300.0),
                ),
        )
        .into()
}

fn create_layout_demo() -> UIElement {
    StackPanel::new()
        .orientation(Orientation::Vertical)
        .spacing(20.0)
        // StackPanel demo
        .child(
            StackPanel::new()
                .orientation(Orientation::Vertical)
                .spacing(5.0)
                .child(TextBlock::new().text("StackPanel (Vertical):").font_weight(FontWeight::Medium))
                .child(
                    Border::new()
                        .background(&Brush::from_color(Color::LIGHT_GRAY))
                        .padding_uniform(10.0)
                        .child(
                            StackPanel::new()
                                .orientation(Orientation::Vertical)
                                .spacing(5.0)
                                .child(Button::new().content("Item 1"))
                                .child(Button::new().content("Item 2"))
                                .child(Button::new().content("Item 3")),
                        ),
                ),
        )
        // Grid demo
        .child(
            StackPanel::new()
                .orientation(Orientation::Vertical)
                .spacing(5.0)
                .child(TextBlock::new().text("Grid (2x2):").font_weight(FontWeight::Medium))
                .child(
                    Border::new()
                        .background(&Brush::from_color(Color::LIGHT_GRAY))
                        .padding_uniform(10.0)
                        .child(
                            Grid::new()
                                .rows(vec![RowDefinition::pixel(40.0), RowDefinition::pixel(40.0)])
                                .columns(vec![ColumnDefinition::star(1.0), ColumnDefinition::star(1.0)])
                                .row_spacing(5.0)
                                .column_spacing(5.0)
                                .child_at(Button::new().content("(0,0)"), 0, 0)
                                .child_at(Button::new().content("(0,1)"), 0, 1)
                                .child_at(Button::new().content("(1,0)"), 1, 0)
                                .child_at(Button::new().content("(1,1)"), 1, 1),
                        ),
                ),
        )
        .into()
}
