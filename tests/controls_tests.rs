//! Unit tests for UI controls.

#[cfg(feature = "library-enabled")]
use winrt_xaml::controls::*;

#[cfg(feature = "library-enabled")]
#[test]
fn test_button_creation() {
    let button = Button::new();
    assert!(button.is_ok());
}

#[cfg(feature = "library-enabled")]
#[test]
fn test_button_content() {
    let button = Button::new().unwrap();
    assert_eq!(button.content(), "");

    button.set_content("Click me").unwrap();
    assert_eq!(button.content(), "Click me");
}

#[cfg(feature = "library-enabled")]
#[test]
fn test_button_fluent_api() {
    let button = Button::new()
        .unwrap()
        .with_content("Test")
        .unwrap();

    assert_eq!(button.content(), "Test");
}

#[cfg(feature = "library-enabled")]
#[test]
fn test_button_click_event() {
    let button = Button::new().unwrap();
    let click_handler = button.click();

    assert_eq!(click_handler.subscriber_count(), 0);

    click_handler.subscribe(|_| {
        println!("Button clicked!");
    });

    assert_eq!(click_handler.subscriber_count(), 1);
}

#[cfg(feature = "library-enabled")]
#[test]
fn test_textblock_creation() {
    let textblock = TextBlock::new();
    assert!(textblock.is_ok());
}

#[cfg(feature = "library-enabled")]
#[test]
fn test_textblock_text() {
    let textblock = TextBlock::new().unwrap();
    assert_eq!(textblock.text(), "");

    textblock.set_text("Hello, World!").unwrap();
    assert_eq!(textblock.text(), "Hello, World!");
}

#[cfg(feature = "library-enabled")]
#[test]
fn test_textblock_font_size() {
    let textblock = TextBlock::new().unwrap();
    assert_eq!(textblock.font_size(), 14.0);

    textblock.set_font_size(24.0);
    assert_eq!(textblock.font_size(), 24.0);
}

#[cfg(feature = "library-enabled")]
#[test]
fn test_textblock_alignment() {
    let textblock = TextBlock::new().unwrap();
    assert_eq!(textblock.alignment(), TextAlignment::Left);

    textblock.set_alignment(TextAlignment::Center);
    assert_eq!(textblock.alignment(), TextAlignment::Center);
}

#[cfg(feature = "library-enabled")]
#[test]
fn test_textbox_creation() {
    let textbox = TextBox::new();
    assert!(textbox.is_ok());
}

#[cfg(feature = "library-enabled")]
#[test]
fn test_textbox_text() {
    let textbox = TextBox::new().unwrap();
    assert_eq!(textbox.text(), "");

    textbox.set_text("Input text").unwrap();
    assert_eq!(textbox.text(), "Input text");
}

#[cfg(feature = "library-enabled")]
#[test]
fn test_textbox_placeholder() {
    let textbox = TextBox::new().unwrap();
    assert_eq!(textbox.placeholder(), "");

    textbox.set_placeholder("Enter text here");
    assert_eq!(textbox.placeholder(), "Enter text here");
}

#[cfg(feature = "library-enabled")]
#[test]
fn test_textbox_max_length() {
    let textbox = TextBox::new().unwrap();
    assert!(textbox.max_length().is_none());

    textbox.set_max_length(Some(100));
    assert_eq!(textbox.max_length(), Some(100));
}

#[cfg(feature = "library-enabled")]
#[test]
fn test_checkbox_creation() {
    let checkbox = CheckBox::new();
    assert!(checkbox.is_ok());
}

#[cfg(feature = "library-enabled")]
#[test]
fn test_checkbox_state() {
    let checkbox = CheckBox::new().unwrap();
    assert!(!checkbox.is_checked());

    checkbox.set_checked(true);
    assert!(checkbox.is_checked());
}

#[cfg(feature = "library-enabled")]
#[test]
fn test_combobox_creation() {
    let combobox = ComboBox::new();
    assert!(combobox.is_ok());
}

#[cfg(feature = "library-enabled")]
#[test]
fn test_combobox_items() {
    let combobox = ComboBox::new().unwrap();
    assert_eq!(combobox.items().len(), 0);

    combobox.add_item("Item 1").unwrap();
    combobox.add_item("Item 2").unwrap();
    combobox.add_item("Item 3").unwrap();

    assert_eq!(combobox.items().len(), 3);
}

#[cfg(feature = "library-enabled")]
#[test]
fn test_combobox_selection() {
    let combobox = ComboBox::new().unwrap();
    assert_eq!(combobox.selected_index(), -1);

    combobox.add_item("Item 1").unwrap();
    combobox.set_selected_index(0);

    assert_eq!(combobox.selected_index(), 0);
}

#[cfg(feature = "library-enabled")]
#[test]
fn test_slider_creation() {
    let slider = Slider::new();
    assert!(slider.is_ok());
}

#[cfg(feature = "library-enabled")]
#[test]
fn test_slider_value() {
    let slider = Slider::new().unwrap();
    assert_eq!(slider.value(), 0.0);

    slider.set_value(50.0);
    assert_eq!(slider.value(), 50.0);
}

#[cfg(feature = "library-enabled")]
#[test]
fn test_slider_range() {
    let slider = Slider::new().unwrap();
    assert_eq!(slider.minimum(), 0.0);
    assert_eq!(slider.maximum(), 100.0);

    slider.set_minimum(10.0);
    slider.set_maximum(200.0);

    assert_eq!(slider.minimum(), 10.0);
    assert_eq!(slider.maximum(), 200.0);
}

#[cfg(feature = "library-enabled")]
#[test]
fn test_progressbar_creation() {
    let progressbar = ProgressBar::new();
    assert!(progressbar.is_ok());
}

#[cfg(feature = "library-enabled")]
#[test]
fn test_progressbar_value() {
    let progressbar = ProgressBar::new().unwrap();
    assert_eq!(progressbar.value(), 0.0);

    progressbar.set_value(75.0);
    assert_eq!(progressbar.value(), 75.0);
}

#[cfg(feature = "library-enabled")]
#[test]
fn test_image_creation() {
    let image = Image::new();
    assert!(image.is_ok());
}

#[cfg(feature = "library-enabled")]
#[test]
fn test_image_stretch() {
    let image = Image::new().unwrap();
    assert_eq!(image.stretch(), Stretch::Uniform);

    image.set_stretch(Stretch::Fill);
    assert_eq!(image.stretch(), Stretch::Fill);
}

#[cfg(feature = "library-enabled")]
#[test]
fn test_listview_creation() {
    let listview = ListView::new();
    assert!(listview.is_ok());
}

#[cfg(feature = "library-enabled")]
#[test]
fn test_toggle_switch_creation() {
    let toggle = ToggleSwitch::new();
    assert!(toggle.is_ok());
}

#[cfg(feature = "library-enabled")]
#[test]
fn test_toggle_switch_state() {
    let toggle = ToggleSwitch::new().unwrap();
    assert!(!toggle.is_on());

    toggle.set_is_on(true);
    assert!(toggle.is_on());
}

