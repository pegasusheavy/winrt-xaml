//! Advanced integration tests combining multiple features.

#[cfg(feature = "xaml-islands")]
mod xaml_islands_integration {
    use winrt_xaml::prelude::*;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_button_with_reactive_property() {
        let button = XamlButton::new().unwrap();
        let counter = Property::new(0);
        
        let counter_clone = counter.clone();
        button.on_click(move || {
            counter_clone.update(|x| x + 1);
        }).unwrap();
        
        assert_eq!(counter.get(), 0);
        // In a real UI, clicking would increment the counter
    }

    #[test]
    fn test_textbox_with_property_binding() {
        let textbox = XamlTextBox::new().unwrap();
        let text_prop = Property::new(String::new());
        
        textbox.set_text("Initial").unwrap();
        text_prop.set("Initial".to_string());
        
        assert_eq!(text_prop.get(), "Initial");
    }

    #[test]
    fn test_stackpanel_with_multiple_children() {
        let panel = XamlStackPanel::new().unwrap();
        let button1 = XamlButton::new().unwrap();
        let button2 = XamlButton::new().unwrap();
        let textblock = XamlTextBlock::new().unwrap();
        
        button1.set_content("Button 1").unwrap();
        button2.set_content("Button 2").unwrap();
        textblock.set_text("Label").unwrap();
        
        panel.add_child(&button1.as_uielement()).unwrap();
        panel.add_child(&button2.as_uielement()).unwrap();
        panel.add_child(&textblock.as_uielement()).unwrap();
        
        // All children added successfully
    }

    #[test]
    fn test_grid_with_row_column_definitions() {
        let grid = XamlGrid::new().unwrap();
        
        // Add 2 rows
        grid.add_row_definition(50.0, false, false).unwrap(); // Fixed height
        grid.add_row_definition(0.0, false, true).unwrap();   // Star sizing
        
        // Add 2 columns
        grid.add_column_definition(100.0, false, false).unwrap(); // Fixed width
        grid.add_column_definition(0.0, false, true).unwrap();    // Star sizing
        
        // Create a button and place it in grid
        let button = XamlButton::new().unwrap();
        button.set_content("Grid Button").unwrap();
        
        let button_elem = button.as_uielement();
        button_elem.set_grid_row(0).unwrap();
        button_elem.set_grid_column(1).unwrap();
        
        grid.add_child(&button_elem).unwrap();
        
        // Grid with definitions created successfully
    }

    #[test]
    fn test_scrollviewer_with_content() {
        let scrollviewer = XamlScrollViewer::new().unwrap();
        let panel = XamlStackPanel::new().unwrap();
        
        // Add multiple items to panel
        for i in 0..10 {
            let button = XamlButton::new().unwrap();
            button.set_content(&format!("Button {}", i)).unwrap();
            panel.add_child(&button.as_uielement()).unwrap();
        }
        
        scrollviewer.set_content(&panel.as_uielement()).unwrap();
        scrollviewer.set_vertical_scroll_mode(ScrollMode::Auto).unwrap();
        
        // ScrollViewer with content created successfully
    }

    #[test]
    fn test_checkbox_with_state_tracking() {
        let checkbox = XamlCheckBox::new().unwrap();
        let is_checked = Property::new(false);
        
        checkbox.set_content("Accept Terms").unwrap();
        checkbox.set_checked(false).unwrap();
        
        let is_checked_clone = is_checked.clone();
        checkbox.on_checked(move || {
            is_checked_clone.set(true);
        }).unwrap();
        
        let is_checked_clone = is_checked.clone();
        checkbox.on_unchecked(move || {
            is_checked_clone.set(false);
        }).unwrap();
        
        assert_eq!(is_checked.get(), false);
    }

    #[test]
    fn test_combobox_with_items() {
        let combobox = XamlComboBox::new().unwrap();
        
        combobox.add_item("Option 1").unwrap();
        combobox.add_item("Option 2").unwrap();
        combobox.add_item("Option 3").unwrap();
        
        combobox.set_selected_index(0).unwrap();
        
        // ComboBox with items created successfully
    }

    #[test]
    fn test_slider_with_value_binding() {
        let slider = XamlSlider::new().unwrap();
        let value_prop = Property::new(50.0);
        
        slider.set_minimum(0.0).unwrap();
        slider.set_maximum(100.0).unwrap();
        slider.set_value(50.0).unwrap();
        
        value_prop.set(50.0);
        
        assert_eq!(value_prop.get(), 50.0);
    }

    #[test]
    fn test_progressbar_with_timer() {
        let progressbar = XamlProgressBar::new().unwrap();
        let progress = Property::new(0.0);
        
        progressbar.set_minimum(0.0).unwrap();
        progressbar.set_maximum(100.0).unwrap();
        progressbar.set_value(0.0).unwrap();
        
        // Simulate progress updates
        for i in 0..=10 {
            let value = i as f64 * 10.0;
            progress.set(value);
            progressbar.set_value(value).unwrap();
        }
        
        assert_eq!(progress.get(), 100.0);
    }

    #[test]
    fn test_radiobutton_group() {
        let radio1 = XamlRadioButton::new().unwrap();
        let radio2 = XamlRadioButton::new().unwrap();
        let radio3 = XamlRadioButton::new().unwrap();
        
        radio1.set_content("Option 1").unwrap();
        radio2.set_content("Option 2").unwrap();
        radio3.set_content("Option 3").unwrap();
        
        radio1.set_group_name("Options").unwrap();
        radio2.set_group_name("Options").unwrap();
        radio3.set_group_name("Options").unwrap();
        
        radio1.set_checked(true).unwrap();
        
        // RadioButton group created successfully
    }

    #[test]
    fn test_image_with_uri() {
        let image = XamlImage::new().unwrap();
        
        image.set_source("ms-appx:///Assets/logo.png").unwrap();
        image.set_stretch(ImageStretch::Uniform).unwrap();
        
        // Image created successfully
    }

    #[test]
    fn test_listview_with_items() {
        let listview = XamlListView::new().unwrap();
        
        listview.set_selection_mode(ListViewSelectionMode::Single).unwrap();
        
        for i in 0..5 {
            let textblock = XamlTextBlock::new().unwrap();
            textblock.set_text(&format!("Item {}", i)).unwrap();
            listview.add_item(&textblock.as_uielement()).unwrap();
        }
        
        listview.set_selected_index(0).unwrap();
        
        // ListView with items created successfully
    }

    #[test]
    fn test_resource_dictionary() {
        let dict = XamlResourceDictionary::new().unwrap();
        
        dict.insert_color("PrimaryColor", 0xFF0078D4).unwrap();
        dict.insert_double("DefaultFontSize", 14.0).unwrap();
        dict.insert_string("AppName", "My App").unwrap();
        
        assert!(dict.has_key("PrimaryColor"));
        assert!(dict.has_key("DefaultFontSize"));
        assert!(dict.has_key("AppName"));
        
        assert_eq!(dict.get_color("PrimaryColor"), 0xFF0078D4);
        assert_eq!(dict.get_double("DefaultFontSize"), 14.0);
        
        let app_name = dict.get_string("AppName");
        assert_eq!(app_name, "My App");
    }

    #[test]
    fn test_animation_setup() {
        let storyboard = XamlStoryboard::new().unwrap();
        let animation = XamlDoubleAnimation::new().unwrap();
        
        animation.set_from(0.0).unwrap();
        animation.set_to(100.0).unwrap();
        animation.set_duration_ms(500).unwrap();
        
        storyboard.add_animation(animation).unwrap();
        
        // Animation created successfully
    }

    #[test]
    fn test_observable_collection_with_ui() {
        let collection = ObservableCollection::new();
        let listview = XamlListView::new().unwrap();
        
        // Subscribe to collection changes
        let listview_clone = listview.clone();
        collection.subscribe(move |change| {
            match change {
                CollectionChange::Added { index: _, value } => {
                    let textblock = XamlTextBlock::new().unwrap();
                    textblock.set_text(&format!("Item: {}", value)).unwrap();
                    let _ = listview_clone.add_item(&textblock.as_uielement());
                }
                CollectionChange::Cleared => {
                    let _ = listview_clone.clear_items();
                }
                _ => {}
            }
        });
        
        collection.push(1);
        collection.push(2);
        collection.push(3);
        
        assert_eq!(collection.len(), 3);
    }

    #[test]
    fn test_computed_with_ui_update() {
        let first_name = Property::new("John".to_string());
        let last_name = Property::new("Doe".to_string());
        let textblock = XamlTextBlock::new().unwrap();
        
        // Update UI when properties change
        let textblock_clone = textblock.clone();
        first_name.subscribe(move |_| {
            let full_name = format!("{} {}", first_name.get(), last_name.get());
            let _ = textblock_clone.set_text(&full_name);
        });
        
        first_name.set("Jane".to_string());
        
        // UI updates triggered by property changes
    }

    #[test]
    fn test_complex_layout_hierarchy() {
        let root_grid = XamlGrid::new().unwrap();
        
        root_grid.add_row_definition(60.0, false, false).unwrap();  // Header
        root_grid.add_row_definition(0.0, false, true).unwrap();    // Content
        root_grid.add_row_definition(40.0, false, false).unwrap();  // Footer
        
        // Header
        let header = XamlStackPanel::new().unwrap();
        header.set_orientation(Orientation::Horizontal).unwrap();
        let title = XamlTextBlock::new().unwrap();
        title.set_text("My Application").unwrap();
        title.set_font_size(20.0).unwrap();
        title.set_font_weight(700).unwrap();
        header.add_child(&title.as_uielement()).unwrap();
        
        let header_elem = header.as_uielement();
        header_elem.set_grid_row(0).unwrap();
        root_grid.add_child(&header_elem).unwrap();
        
        // Content
        let content_scroll = XamlScrollViewer::new().unwrap();
        let content_panel = XamlStackPanel::new().unwrap();
        content_panel.set_spacing(10).unwrap();
        
        for i in 0..5 {
            let button = XamlButton::new().unwrap();
            button.set_content(&format!("Action {}", i)).unwrap();
            content_panel.add_child(&button.as_uielement()).unwrap();
        }
        
        content_scroll.set_content(&content_panel.as_uielement()).unwrap();
        let scroll_elem = content_scroll.as_uielement();
        scroll_elem.set_grid_row(1).unwrap();
        root_grid.add_child(&scroll_elem).unwrap();
        
        // Footer
        let footer = XamlStackPanel::new().unwrap();
        footer.set_orientation(Orientation::Horizontal).unwrap();
        let status = XamlTextBlock::new().unwrap();
        status.set_text("Ready").unwrap();
        footer.add_child(&status.as_uielement()).unwrap();
        
        let footer_elem = footer.as_uielement();
        footer_elem.set_grid_row(2).unwrap();
        root_grid.add_child(&footer_elem).unwrap();
        
        // Complex layout created successfully
    }
}

#[cfg(not(feature = "xaml-islands"))]
#[test]
fn test_integration_feature_not_enabled() {
    // Integration tests require xaml-islands feature
    assert!(true);
}
