//! Unit tests for XAML parser.

use winrt_xaml::xaml::XamlReader;
use winrt_xaml::error::Error;

#[test]
fn test_xaml_reader_load_not_implemented() {
    let result = XamlReader::load("<Button Content='Click Me' />");
    assert!(result.is_err());
    
    match result {
        Err(e) => {
            let msg = e.to_string();
            assert!(msg.contains("not implemented") || msg.contains("not yet implemented"));
        }
        Ok(_) => panic!("Expected not implemented error"),
    }
}

#[test]
fn test_xaml_reader_parse_not_implemented() {
    let result = XamlReader::parse("<StackPanel><Button /></StackPanel>");
    assert!(result.is_err());
    
    match result {
        Err(e) => {
            let msg = e.to_string();
            assert!(msg.contains("not implemented") || msg.contains("not yet implemented"));
        }
        Ok(_) => panic!("Expected not implemented error"),
    }
}

#[test]
fn test_xaml_reader_load_empty_string() {
    let result = XamlReader::load("");
    assert!(result.is_err());
}

#[test]
fn test_xaml_reader_parse_empty_string() {
    let result = XamlReader::parse("");
    assert!(result.is_err());
}

#[test]
fn test_xaml_reader_load_invalid_xaml() {
    let result = XamlReader::load("not valid xaml");
    assert!(result.is_err());
}

#[test]
fn test_xaml_reader_parse_invalid_xaml() {
    let result = XamlReader::parse("<<invalid>>");
    assert!(result.is_err());
}

#[test]
fn test_xaml_reader_api_exists() {
    // Compile-time checks that the API exists
    
    fn _check_load() {
        use winrt_xaml::controls::UIElement;
        fn _needs_method(_: fn(&str) -> Result<UIElement, Error>) {}
        _needs_method(XamlReader::load);
    }
    
    fn _check_parse() {
        use winrt_xaml::controls::UIElement;
        fn _needs_method(_: fn(&str) -> Result<UIElement, Error>) {}
        _needs_method(XamlReader::parse);
    }
}

#[test]
fn test_xaml_parse_error_type() {
    let err = Error::xaml_parse("Invalid XAML syntax");
    let msg = err.to_string();
    assert!(msg.contains("XAML parse error") || msg.contains("Invalid XAML syntax"));
}

#[test]
fn test_not_implemented_error_type() {
    let err = Error::not_implemented("XAML parsing");
    let msg = err.to_string();
    assert!(msg.contains("not implemented") || msg.contains("XAML parsing"));
}

// Future tests for when XAML parsing is implemented:

#[test]
#[ignore]
fn test_xaml_reader_load_button() {
    let xaml = r#"<Button Content="Click Me" Width="200" Height="50" />"#;
    let result = XamlReader::load(xaml);
    // When implemented, should succeed
    assert!(result.is_ok());
}

#[test]
#[ignore]
fn test_xaml_reader_parse_stackpanel() {
    let xaml = r#"
        <StackPanel Orientation="Vertical" Spacing="10">
            <Button Content="Button 1" />
            <Button Content="Button 2" />
            <TextBlock Text="Hello" />
        </StackPanel>
    "#;
    let result = XamlReader::parse(xaml);
    // When implemented, should succeed
    assert!(result.is_ok());
}

#[test]
#[ignore]
fn test_xaml_reader_parse_grid() {
    let xaml = r#"
        <Grid>
            <Grid.RowDefinitions>
                <RowDefinition Height="Auto" />
                <RowDefinition Height="*" />
            </Grid.RowDefinitions>
            <TextBlock Grid.Row="0" Text="Header" />
            <Button Grid.Row="1" Content="Content" />
        </Grid>
    "#;
    let result = XamlReader::parse(xaml);
    // When implemented, should succeed
    assert!(result.is_ok());
}

#[test]
#[ignore]
fn test_xaml_reader_parse_nested() {
    let xaml = r#"
        <StackPanel>
            <StackPanel Orientation="Horizontal">
                <Button Content="1" />
                <Button Content="2" />
            </StackPanel>
            <Grid>
                <TextBlock Text="Nested" />
            </Grid>
        </StackPanel>
    "#;
    let result = XamlReader::parse(xaml);
    // When implemented, should succeed
    assert!(result.is_ok());
}

#[test]
#[ignore]
fn test_xaml_reader_parse_with_properties() {
    let xaml = r##"
        <Button 
            Content="Click Me"
            Width="200"
            Height="50"
            Background="#FF0078D4"
            Foreground="#FFFFFFFF"
            CornerRadius="10"
        />
    "##;
    let result = XamlReader::parse(xaml);
    // When implemented, should succeed
    assert!(result.is_ok());
}
