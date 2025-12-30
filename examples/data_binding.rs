//! Data binding example demonstrating XAML loading and resources.
//!
//! This example shows how to use XAML markup and resource dictionaries.

use winrt_xaml::prelude::*;

fn main() -> Result<()> {
    env_logger::init();

    let app = Application::new()?;

    // Set up application resources
    let resources = ResourceDictionary::new();
    resources.insert("PrimaryBrush", Brush::from_color(Color::rgb(0, 120, 215)));
    resources.insert("SecondaryBrush", Brush::from_color(Color::rgb(100, 100, 100)));
    resources.insert("HeaderFontSize", 24.0f64);
    resources.insert("BodyFontSize", 14.0f64);
    app.set_resources(resources);

    let window = Window::builder()
        .title("XAML and Resources Demo")
        .size(800, 600)
        .build()?;

    // Create UI from XAML string
    let xaml = r#"
        <StackPanel Orientation="Vertical" Spacing="20" Padding="30">
            <TextBlock Text="XAML Loading Demo" FontSize="28" FontWeight="Bold" HorizontalAlignment="Center"/>

            <Border BorderThickness="1" BorderBrush="Gray" CornerRadius="8" Padding="20">
                <StackPanel Orientation="Vertical" Spacing="10">
                    <TextBlock Text="This UI was loaded from XAML markup!" FontSize="16"/>
                    <TextBlock Text="The library supports parsing XAML strings and files." TextWrapping="Wrap"/>
                </StackPanel>
            </Border>

            <StackPanel Orientation="Horizontal" Spacing="10" HorizontalAlignment="Center">
                <Button Content="Button 1" Padding="20,10"/>
                <Button Content="Button 2" Padding="20,10"/>
                <Button Content="Button 3" Padding="20,10"/>
            </StackPanel>

            <Grid RowSpacing="10" ColumnSpacing="10">
                <TextBlock Text="This is a Grid layout loaded from XAML"/>
            </Grid>

            <StackPanel Orientation="Vertical" Spacing="10">
                <TextBlock Text="Input Controls:" FontWeight="SemiBold"/>
                <TextBox PlaceholderText="Enter text here..." Width="300"/>
                <CheckBox Content="I agree to the terms" />
                <ToggleSwitch Header="Enable notifications" OnContent="On" OffContent="Off"/>
            </StackPanel>

            <ProgressBar Value="65" Maximum="100" Width="400"/>
            <Slider Minimum="0" Maximum="100" Value="50" Width="400"/>
        </StackPanel>
    "#;

    // Load the XAML
    let content = load_xaml(xaml)?;

    window.set_content(content)?;
    window.center()?;
    window.show()?;

    app.run()
}
