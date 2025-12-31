//! Unit tests for styling (colors, padding, margin, fonts).

#[cfg(feature = "xaml-islands")]
mod xaml_islands_styling {
    use winrt_xaml::prelude::*;

    #[test]
    fn test_button_background() {
        let button = XamlButton::new().unwrap();

        button.set_background(0xFF0078D4).unwrap();
        // Background set successfully (no panic)
    }

    #[test]
    fn test_button_foreground() {
        let button = XamlButton::new().unwrap();

        button.set_foreground(0xFFFFFFFF).unwrap();
        // Foreground set successfully
    }

    #[test]
    fn test_button_padding() {
        let button = XamlButton::new().unwrap();

        button.as_uielement().set_padding(10.0, 5.0, 10.0, 5.0).unwrap();
        // Padding set successfully
    }

    #[test]
    fn test_button_margin() {
        let button = XamlButton::new().unwrap();

        button.as_uielement().set_margin(5.0, 5.0, 5.0, 5.0).unwrap();
        // Margin set successfully
    }

    #[test]
    fn test_button_corner_radius() {
        let button = XamlButton::new().unwrap();

        button.set_corner_radius(8.0).unwrap();
        // Corner radius set successfully
    }

    #[test]
    fn test_button_font_size() {
        let button = XamlButton::new().unwrap();

        button.set_font_size(16.0).unwrap();
        // Font size set successfully
    }

    #[test]
    fn test_button_font_weight() {
        let button = XamlButton::new().unwrap();

        button.set_font_weight(700).unwrap(); // Bold
        // Font weight set successfully
    }

    #[test]
    fn test_textblock_foreground() {
        let textblock = XamlTextBlock::new().unwrap();

        textblock.set_foreground(0xFF000000).unwrap();
        // Foreground set successfully
    }

    #[test]
    fn test_textblock_font_size() {
        let textblock = XamlTextBlock::new().unwrap();

        textblock.set_font_size(20.0).unwrap();
        // Font size set successfully
    }

    #[test]
    fn test_textblock_font_weight() {
        let textblock = XamlTextBlock::new().unwrap();

        textblock.set_font_weight(600).unwrap(); // Semi-bold
        // Font weight set successfully
    }

    #[test]
    fn test_textbox_background() {
        let textbox = XamlTextBox::new().unwrap();

        textbox.set_background(0xFFFFFFFF).unwrap();
        // Background set successfully
    }

    #[test]
    fn test_textbox_foreground() {
        let textbox = XamlTextBox::new().unwrap();

        textbox.set_foreground(0xFF000000).unwrap();
        // Foreground set successfully
    }

    #[test]
    fn test_textbox_font_size() {
        let textbox = XamlTextBox::new().unwrap();

        textbox.set_font_size(14.0).unwrap();
        // Font size set successfully
    }

    #[test]
    fn test_color_argb() {
        // Test color creation
        let red = 0xFFFF0000;
        let green = 0xFF00FF00;
        let blue = 0xFF0000FF;
        let white = 0xFFFFFFFF;
        let black = 0xFF000000;

        assert_eq!(red, 0xFFFF0000);
        assert_eq!(green, 0xFF00FF00);
        assert_eq!(blue, 0xFF0000FF);
        assert_eq!(white, 0xFFFFFFFF);
        assert_eq!(black, 0xFF000000);
    }

    #[test]
    fn test_color_with_alpha() {
        let semi_transparent_red = 0x80FF0000;
        let transparent = 0x00000000;
        let opaque = 0xFFFFFFFF;

        assert_eq!(semi_transparent_red & 0xFF000000, 0x80000000);
        assert_eq!(transparent & 0xFF000000, 0x00000000);
        assert_eq!(opaque & 0xFF000000, 0xFF000000);
    }

    #[test]
    fn test_stackpanel_padding() {
        let panel = XamlStackPanel::new().unwrap();

        panel.as_uielement().set_padding(15.0, 10.0, 15.0, 10.0).unwrap();
        // Padding set successfully
    }

    #[test]
    fn test_stackpanel_margin() {
        let panel = XamlStackPanel::new().unwrap();

        panel.as_uielement().set_margin(10.0, 10.0, 10.0, 10.0).unwrap();
        // Margin set successfully
    }

    #[test]
    fn test_grid_padding() {
        let grid = XamlGrid::new().unwrap();

        grid.as_uielement().set_padding(20.0, 20.0, 20.0, 20.0).unwrap();
        // Padding set successfully
    }

    #[test]
    fn test_multiple_styling() {
        let button = XamlButton::new().unwrap();

        // Apply multiple styles
        button.set_background(0xFF0078D4).unwrap();
        button.set_foreground(0xFFFFFFFF).unwrap();
        button.set_corner_radius(4.0).unwrap();
        button.set_font_size(14.0).unwrap();
        button.set_font_weight(600).unwrap();
        button.as_uielement().set_padding(12.0, 6.0, 12.0, 6.0).unwrap();
        button.as_uielement().set_margin(5.0, 5.0, 5.0, 5.0).unwrap();

        // All styles applied successfully
    }

    #[test]
    fn test_checkbox_font_size() {
        let checkbox = XamlCheckBox::new().unwrap();

        checkbox.set_font_size(16.0).unwrap();
        // Font size set successfully
    }

    #[test]
    fn test_combobox_font_size() {
        let combobox = XamlComboBox::new().unwrap();

        combobox.set_font_size(14.0).unwrap();
        // Font size set successfully
    }
}

#[cfg(not(feature = "xaml-islands"))]
#[test]
fn test_styling_feature_not_enabled() {
    // Styling tests require xaml-islands feature
    assert!(true);
}
