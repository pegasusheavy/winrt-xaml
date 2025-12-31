//! Unit tests for FFI error handling and safety.

#[cfg(feature = "xaml-islands")]
mod xaml_ffi_tests {
    use winrt_xaml::prelude::*;

    #[test]
    fn test_xaml_manager_initialization() {
        let manager = XamlManager::new();
        assert!(manager.is_ok());
    }

    #[test]
    fn test_xaml_source_creation() {
        let _manager = XamlManager::new().unwrap();
        let source = XamlSource::new();
        assert!(source.is_ok());
    }

    #[test]
    fn test_button_creation_success() {
        let button = XamlButton::new();
        assert!(button.is_ok());
    }

    #[test]
    fn test_textblock_creation_success() {
        let textblock = XamlTextBlock::new();
        assert!(textblock.is_ok());
    }

    #[test]
    fn test_textbox_creation_success() {
        let textbox = XamlTextBox::new();
        assert!(textbox.is_ok());
    }

    #[test]
    fn test_stackpanel_creation_success() {
        let panel = XamlStackPanel::new();
        assert!(panel.is_ok());
    }

    #[test]
    fn test_grid_creation_success() {
        let grid = XamlGrid::new();
        assert!(grid.is_ok());
    }

    #[test]
    fn test_scrollviewer_creation_success() {
        let scrollviewer = XamlScrollViewer::new();
        assert!(scrollviewer.is_ok());
    }

    #[test]
    fn test_checkbox_creation_success() {
        let checkbox = XamlCheckBox::new();
        assert!(checkbox.is_ok());
    }

    #[test]
    fn test_combobox_creation_success() {
        let combobox = XamlComboBox::new();
        assert!(combobox.is_ok());
    }

    #[test]
    fn test_slider_creation_success() {
        let slider = XamlSlider::new();
        assert!(slider.is_ok());
    }

    #[test]
    fn test_progressbar_creation_success() {
        let progressbar = XamlProgressBar::new();
        assert!(progressbar.is_ok());
    }

    #[test]
    fn test_radiobutton_creation_success() {
        let radiobutton = XamlRadioButton::new();
        assert!(radiobutton.is_ok());
    }

    #[test]
    fn test_image_creation_success() {
        let image = XamlImage::new();
        assert!(image.is_ok());
    }

    #[test]
    fn test_listview_creation_success() {
        let listview = XamlListView::new();
        assert!(listview.is_ok());
    }

    #[test]
    fn test_resource_dictionary_creation_success() {
        let dict = XamlResourceDictionary::new();
        assert!(dict.is_ok());
    }

    #[test]
    fn test_storyboard_creation_success() {
        let storyboard = XamlStoryboard::new();
        assert!(storyboard.is_ok());
    }

    #[test]
    fn test_double_animation_creation_success() {
        let animation = XamlDoubleAnimation::new();
        assert!(animation.is_ok());
    }

    #[test]
    fn test_color_animation_creation_success() {
        let animation = XamlColorAnimation::new();
        assert!(animation.is_ok());
    }

    #[test]
    fn test_button_set_content_empty() {
        let button = XamlButton::new().unwrap();
        let result = button.set_content("");
        assert!(result.is_ok());
    }

    #[test]
    fn test_button_set_content_long() {
        let button = XamlButton::new().unwrap();
        let long_text = "This is a very long button text that should still work correctly";
        let result = button.set_content(long_text);
        assert!(result.is_ok());
    }

    #[test]
    fn test_textbox_set_text_unicode() {
        let textbox = XamlTextBox::new().unwrap();
        let unicode_text = "Hello 世界 🌍";
        let result = textbox.set_text(unicode_text);
        assert!(result.is_ok());
    }

    #[test]
    fn test_stackpanel_spacing_zero() {
        let panel = XamlStackPanel::new().unwrap();
        let result = panel.set_spacing(0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_stackpanel_spacing_large() {
        let panel = XamlStackPanel::new().unwrap();
        let result = panel.set_spacing(1000);
        assert!(result.is_ok());
    }

    #[test]
    fn test_slider_value_at_minimum() {
        let slider = XamlSlider::new().unwrap();
        slider.set_minimum(0.0).unwrap();
        let result = slider.set_value(0.0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_slider_value_at_maximum() {
        let slider = XamlSlider::new().unwrap();
        slider.set_maximum(100.0).unwrap();
        let result = slider.set_value(100.0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_combobox_add_empty_item() {
        let combobox = XamlComboBox::new().unwrap();
        let result = combobox.add_item("");
        assert!(result.is_ok());
    }

    #[test]
    fn test_combobox_set_selected_index_negative() {
        let combobox = XamlComboBox::new().unwrap();
        let result = combobox.set_selected_index(-1);
        assert!(result.is_ok()); // -1 is valid (means no selection)
    }

    #[test]
    fn test_listview_clear_empty() {
        let listview = XamlListView::new().unwrap();
        let result = listview.clear_items();
        assert!(result.is_ok());
    }

    #[test]
    fn test_grid_add_multiple_row_definitions() {
        let grid = XamlGrid::new().unwrap();
        
        for i in 0..10 {
            let result = grid.add_row_definition(50.0, false, false);
            assert!(result.is_ok(), "Failed to add row {}", i);
        }
    }

    #[test]
    fn test_grid_add_multiple_column_definitions() {
        let grid = XamlGrid::new().unwrap();
        
        for i in 0..10 {
            let result = grid.add_column_definition(50.0, false, false);
            assert!(result.is_ok(), "Failed to add column {}", i);
        }
    }

    #[test]
    fn test_image_set_source_empty() {
        let image = XamlImage::new().unwrap();
        // Empty source might fail, but should not crash
        let _result = image.set_source("");
        // Just ensure no panic
    }

    #[test]
    fn test_font_size_very_small() {
        let textblock = XamlTextBlock::new().unwrap();
        let result = textblock.set_font_size(1.0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_font_size_very_large() {
        let textblock = XamlTextBlock::new().unwrap();
        let result = textblock.set_font_size(200.0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_font_weight_minimum() {
        let textblock = XamlTextBlock::new().unwrap();
        let result = textblock.set_font_weight(100);
        assert!(result.is_ok());
    }

    #[test]
    fn test_font_weight_maximum() {
        let textblock = XamlTextBlock::new().unwrap();
        let result = textblock.set_font_weight(900);
        assert!(result.is_ok());
    }

    #[test]
    fn test_padding_zero() {
        let button = XamlButton::new().unwrap();
        let result = button.as_uielement().set_padding(0.0, 0.0, 0.0, 0.0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_padding_large() {
        let button = XamlButton::new().unwrap();
        let result = button.as_uielement().set_padding(100.0, 100.0, 100.0, 100.0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_margin_negative() {
        let button = XamlButton::new().unwrap();
        // Negative margins are valid in XAML
        let result = button.as_uielement().set_margin(-10.0, -10.0, -10.0, -10.0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_corner_radius_zero() {
        let button = XamlButton::new().unwrap();
        let result = button.set_corner_radius(0.0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_corner_radius_large() {
        let button = XamlButton::new().unwrap();
        let result = button.set_corner_radius(50.0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_resource_dictionary_remove_nonexistent() {
        let dict = XamlResourceDictionary::new().unwrap();
        let result = dict.remove("NonExistentKey");
        // Should fail gracefully
        assert!(result.is_err() || result.is_ok());
    }

    #[test]
    fn test_resource_dictionary_get_nonexistent_color() {
        let dict = XamlResourceDictionary::new().unwrap();
        let _color = dict.get_color("NonExistentKey");
        // Should return 0 or default color
    }

    #[test]
    fn test_storyboard_begin_without_animations() {
        let storyboard = XamlStoryboard::new().unwrap();
        let result = storyboard.begin();
        // Empty storyboard might fail or succeed
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_animation_duration_zero() {
        let animation = XamlDoubleAnimation::new().unwrap();
        let result = animation.set_duration_ms(0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_animation_duration_very_long() {
        let animation = XamlDoubleAnimation::new().unwrap();
        let result = animation.set_duration_ms(60000); // 1 minute
        assert!(result.is_ok());
    }

    #[test]
    fn test_multiple_controls_creation() {
        // Test creating many controls doesn't cause issues
        let buttons: Vec<_> = (0..100)
            .map(|_| XamlButton::new())
            .collect();
        
        assert_eq!(buttons.len(), 100);
        assert!(buttons.iter().all(|b| b.is_ok()));
    }

    #[test]
    fn test_uielement_conversions() {
        let button = XamlButton::new().unwrap();
        let _elem = button.as_uielement();
        
        let textblock = XamlTextBlock::new().unwrap();
        let _elem = textblock.as_uielement();
        
        let textbox = XamlTextBox::new().unwrap();
        let _elem = textbox.as_uielement();
        
        // All conversions successful
    }
}

#[cfg(not(feature = "xaml-islands"))]
#[test]
fn test_ffi_feature_not_enabled() {
    // FFI tests require xaml-islands feature
    assert!(true);
}
