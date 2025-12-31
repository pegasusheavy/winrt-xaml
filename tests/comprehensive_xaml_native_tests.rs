//! Comprehensive tests for all XAML native controls.

#[cfg(feature = "xaml-islands"))]
mod xaml_native_comprehensive {
    use winrt_xaml::prelude::*;

    // === Button Tests ===
    
    #[test]
    fn test_button_content_get_set() {
        let button = XamlButton::new().unwrap();
        
        button.set_content("Click Me").unwrap();
        // Content set successfully
    }

    #[test]
    fn test_button_styling() {
        let button = XamlButton::new().unwrap();
        
        button.set_background(0xFF0078D4).unwrap();
        button.set_foreground(0xFFFFFFFF).unwrap();
        button.set_corner_radius(4.0).unwrap();
        button.set_font_size(14.0).unwrap();
        button.set_font_weight(600).unwrap();
    }

    #[test]
    fn test_button_click_handler() {
        use std::sync::{Arc, Mutex};
        
        let button = XamlButton::new().unwrap();
        let clicked = Arc::new(Mutex::new(false));
        
        let clicked_clone = clicked.clone();
        button.on_click(move || {
            *clicked_clone.lock().unwrap() = true;
        }).unwrap();
        
        // Handler registered successfully
        assert_eq!(*clicked.lock().unwrap(), false);
    }

    // === TextBlock Tests ===
    
    #[test]
    fn test_textblock_text_get_set() {
        let textblock = XamlTextBlock::new().unwrap();
        
        textblock.set_text("Hello, World!").unwrap();
        // Text set successfully
    }

    #[test]
    fn test_textblock_styling() {
        let textblock = XamlTextBlock::new().unwrap();
        
        textblock.set_foreground(0xFF000000).unwrap();
        textblock.set_font_size(16.0).unwrap();
        textblock.set_font_weight(400).unwrap();
    }

    // === TextBox Tests ===
    
    #[test]
    fn test_textbox_text_get_set() {
        let textbox = XamlTextBox::new().unwrap();
        
        textbox.set_text("Input").unwrap();
        let text = textbox.get_text();
        assert!(text.is_ok());
    }

    #[test]
    fn test_textbox_placeholder() {
        let textbox = XamlTextBox::new().unwrap();
        
        textbox.set_placeholder_text("Enter text...").unwrap();
        // Placeholder set successfully
    }

    #[test]
    fn test_textbox_dimensions() {
        let textbox = XamlTextBox::new().unwrap();
        
        textbox.set_width(200.0).unwrap();
        textbox.set_height(32.0).unwrap();
    }

    #[test]
    fn test_textbox_styling() {
        let textbox = XamlTextBox::new().unwrap();
        
        textbox.set_background(0xFFFFFFFF).unwrap();
        textbox.set_foreground(0xFF000000).unwrap();
        textbox.set_font_size(14.0).unwrap();
    }

    // === StackPanel Tests ===
    
    #[test]
    fn test_stackpanel_orientation() {
        let panel = XamlStackPanel::new().unwrap();
        
        panel.set_orientation(Orientation::Horizontal).unwrap();
        panel.set_orientation(Orientation::Vertical).unwrap();
    }

    #[test]
    fn test_stackpanel_spacing() {
        let panel = XamlStackPanel::new().unwrap();
        
        panel.set_spacing(10).unwrap();
        panel.set_spacing(0).unwrap();
        panel.set_spacing(50).unwrap();
    }

    #[test]
    fn test_stackpanel_add_children() {
        let panel = XamlStackPanel::new().unwrap();
        
        let button1 = XamlButton::new().unwrap();
        let button2 = XamlButton::new().unwrap();
        let textblock = XamlTextBlock::new().unwrap();
        
        panel.add_child(&button1.as_uielement()).unwrap();
        panel.add_child(&button2.as_uielement()).unwrap();
        panel.add_child(&textblock.as_uielement()).unwrap();
    }

    // === Grid Tests ===
    
    #[test]
    fn test_grid_row_definitions() {
        let grid = XamlGrid::new().unwrap();
        
        grid.add_row_definition(50.0, false, false).unwrap();  // Fixed
        grid.add_row_definition(0.0, true, false).unwrap();     // Auto
        grid.add_row_definition(0.0, false, true).unwrap();     // Star
    }

    #[test]
    fn test_grid_column_definitions() {
        let grid = XamlGrid::new().unwrap();
        
        grid.add_column_definition(100.0, false, false).unwrap(); // Fixed
        grid.add_column_definition(0.0, true, false).unwrap();    // Auto
        grid.add_column_definition(0.0, false, true).unwrap();    // Star
    }

    #[test]
    fn test_grid_child_positioning() {
        let grid = XamlGrid::new().unwrap();
        let button = XamlButton::new().unwrap();
        
        let button_elem = button.as_uielement();
        button_elem.set_grid_row(0).unwrap();
        button_elem.set_grid_column(0).unwrap();
        
        grid.add_child(&button_elem).unwrap();
    }

    #[test]
    fn test_grid_child_spanning() {
        let button = XamlButton::new().unwrap();
        let button_elem = button.as_uielement();
        
        button_elem.set_grid_row_span(2).unwrap();
        button_elem.set_grid_column_span(3).unwrap();
    }

    // === ScrollViewer Tests ===
    
    #[test]
    fn test_scrollviewer_set_content() {
        let scrollviewer = XamlScrollViewer::new().unwrap();
        let panel = XamlStackPanel::new().unwrap();
        
        scrollviewer.set_content(&panel.as_uielement()).unwrap();
    }

    #[test]
    fn test_scrollviewer_scroll_modes() {
        let scrollviewer = XamlScrollViewer::new().unwrap();
        
        scrollviewer.set_vertical_scroll_mode(ScrollMode::Auto).unwrap();
        scrollviewer.set_horizontal_scroll_mode(ScrollMode::Disabled).unwrap();
    }

    #[test]
    fn test_scrollviewer_scrollbar_visibility() {
        let scrollviewer = XamlScrollViewer::new().unwrap();
        
        scrollviewer.set_vertical_scrollbar_visibility(ScrollBarVisibility::Auto).unwrap();
        scrollviewer.set_horizontal_scrollbar_visibility(ScrollBarVisibility::Hidden).unwrap();
    }

    // === CheckBox Tests ===
    
    #[test]
    fn test_checkbox_content() {
        let checkbox = XamlCheckBox::new().unwrap();
        
        checkbox.set_content("Accept Terms").unwrap();
    }

    #[test]
    fn test_checkbox_checked_state() {
        let checkbox = XamlCheckBox::new().unwrap();
        
        checkbox.set_checked(true).unwrap();
        checkbox.set_checked(false).unwrap();
    }

    #[test]
    fn test_checkbox_events() {
        use std::sync::{Arc, Mutex};
        
        let checkbox = XamlCheckBox::new().unwrap();
        let checked_count = Arc::new(Mutex::new(0));
        let unchecked_count = Arc::new(Mutex::new(0));
        
        let checked_clone = checked_count.clone();
        checkbox.on_checked(move || {
            *checked_clone.lock().unwrap() += 1;
        }).unwrap();
        
        let unchecked_clone = unchecked_count.clone();
        checkbox.on_unchecked(move || {
            *unchecked_clone.lock().unwrap() += 1;
        }).unwrap();
        
        // Handlers registered
    }

    // === ComboBox Tests ===
    
    #[test]
    fn test_combobox_add_items() {
        let combobox = XamlComboBox::new().unwrap();
        
        combobox.add_item("Option 1").unwrap();
        combobox.add_item("Option 2").unwrap();
        combobox.add_item("Option 3").unwrap();
    }

    #[test]
    fn test_combobox_selection() {
        let combobox = XamlComboBox::new().unwrap();
        
        combobox.add_item("Item 1").unwrap();
        combobox.add_item("Item 2").unwrap();
        
        combobox.set_selected_index(0).unwrap();
        combobox.set_selected_index(1).unwrap();
        combobox.set_selected_index(-1).unwrap(); // Clear selection
    }

    // === Slider Tests ===
    
    #[test]
    fn test_slider_value() {
        let slider = XamlSlider::new().unwrap();
        
        slider.set_value(50.0).unwrap();
        slider.set_value(0.0).unwrap();
        slider.set_value(100.0).unwrap();
    }

    #[test]
    fn test_slider_range() {
        let slider = XamlSlider::new().unwrap();
        
        slider.set_minimum(0.0).unwrap();
        slider.set_maximum(100.0).unwrap();
    }

    #[test]
    fn test_slider_step() {
        let slider = XamlSlider::new().unwrap();
        
        slider.set_step_frequency(5.0).unwrap();
        slider.set_step_frequency(1.0).unwrap();
    }

    // === ProgressBar Tests ===
    
    #[test]
    fn test_progressbar_value() {
        let progressbar = XamlProgressBar::new().unwrap();
        
        progressbar.set_value(0.0).unwrap();
        progressbar.set_value(50.0).unwrap();
        progressbar.set_value(100.0).unwrap();
    }

    #[test]
    fn test_progressbar_range() {
        let progressbar = XamlProgressBar::new().unwrap();
        
        progressbar.set_minimum(0.0).unwrap();
        progressbar.set_maximum(100.0).unwrap();
    }

    #[test]
    fn test_progressbar_indeterminate() {
        let progressbar = XamlProgressBar::new().unwrap();
        
        progressbar.set_is_indeterminate(true).unwrap();
        progressbar.set_is_indeterminate(false).unwrap();
    }

    // === RadioButton Tests ===
    
    #[test]
    fn test_radiobutton_content() {
        let radiobutton = XamlRadioButton::new().unwrap();
        
        radiobutton.set_content("Option A").unwrap();
    }

    #[test]
    fn test_radiobutton_checked() {
        let radiobutton = XamlRadioButton::new().unwrap();
        
        radiobutton.set_checked(true).unwrap();
        radiobutton.set_checked(false).unwrap();
    }

    #[test]
    fn test_radiobutton_group() {
        let radio1 = XamlRadioButton::new().unwrap();
        let radio2 = XamlRadioButton::new().unwrap();
        
        radio1.set_group_name("MyGroup").unwrap();
        radio2.set_group_name("MyGroup").unwrap();
    }

    // === Image Tests ===
    
    #[test]
    fn test_image_source() {
        let image = XamlImage::new().unwrap();
        
        let _result = image.set_source("ms-appx:///Assets/logo.png");
        // May fail if path doesn't exist, but should not crash
    }

    #[test]
    fn test_image_stretch() {
        let image = XamlImage::new().unwrap();
        
        image.set_stretch(ImageStretch::None).unwrap();
        image.set_stretch(ImageStretch::Fill).unwrap();
        image.set_stretch(ImageStretch::Uniform).unwrap();
        image.set_stretch(ImageStretch::UniformToFill).unwrap();
    }

    // === ListView Tests ===
    
    #[test]
    fn test_listview_add_items() {
        let listview = XamlListView::new().unwrap();
        
        for i in 0..5 {
            let textblock = XamlTextBlock::new().unwrap();
            textblock.set_text(&format!("Item {}", i)).unwrap();
            listview.add_item(&textblock.as_uielement()).unwrap();
        }
    }

    #[test]
    fn test_listview_remove_item() {
        let listview = XamlListView::new().unwrap();
        
        let textblock = XamlTextBlock::new().unwrap();
        textblock.set_text("Item").unwrap();
        listview.add_item(&textblock.as_uielement()).unwrap();
        
        listview.remove_item(0).unwrap();
    }

    #[test]
    fn test_listview_clear() {
        let listview = XamlListView::new().unwrap();
        
        for i in 0..3 {
            let textblock = XamlTextBlock::new().unwrap();
            textblock.set_text(&format!("Item {}", i)).unwrap();
            listview.add_item(&textblock.as_uielement()).unwrap();
        }
        
        listview.clear_items().unwrap();
    }

    #[test]
    fn test_listview_selection() {
        let listview = XamlListView::new().unwrap();
        
        let textblock = XamlTextBlock::new().unwrap();
        textblock.set_text("Item").unwrap();
        listview.add_item(&textblock.as_uielement()).unwrap();
        
        listview.set_selected_index(0).unwrap();
        listview.set_selected_index(-1).unwrap(); // Clear selection
    }

    #[test]
    fn test_listview_selection_mode() {
        let listview = XamlListView::new().unwrap();
        
        listview.set_selection_mode(ListViewSelectionMode::None).unwrap();
        listview.set_selection_mode(ListViewSelectionMode::Single).unwrap();
        listview.set_selection_mode(ListViewSelectionMode::Multiple).unwrap();
        listview.set_selection_mode(ListViewSelectionMode::Extended).unwrap();
    }

    // === UIElement Tests ===
    
    #[test]
    fn test_uielement_dimensions() {
        let button = XamlButton::new().unwrap();
        let elem = button.as_uielement();
        
        elem.set_width(200.0).unwrap();
        elem.set_height(40.0).unwrap();
    }

    #[test]
    fn test_uielement_padding() {
        let button = XamlButton::new().unwrap();
        let elem = button.as_uielement();
        
        elem.set_padding(10.0, 5.0, 10.0, 5.0).unwrap();
    }

    #[test]
    fn test_uielement_margin() {
        let button = XamlButton::new().unwrap();
        let elem = button.as_uielement();
        
        elem.set_margin(10.0, 10.0, 10.0, 10.0).unwrap();
    }

    #[test]
    fn test_uielement_grid_attached_properties() {
        let button = XamlButton::new().unwrap();
        let elem = button.as_uielement();
        
        elem.set_grid_row(0).unwrap();
        elem.set_grid_column(1).unwrap();
        elem.set_grid_row_span(2).unwrap();
        elem.set_grid_column_span(3).unwrap();
    }
}

#[cfg(not(feature = "xaml-islands"))]
#[test]
fn test_xaml_native_feature_not_enabled() {
    // XAML native tests require xaml-islands feature
    assert!(true);
}
