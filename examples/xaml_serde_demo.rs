//! Serde-based XAML parsing demo.
//!
//! This example demonstrates using serde with quick-xml for type-safe,
//! struct-based XAML deserialization. This is 100% serde-compliant!
//!
//! Benefits:
//! - Type-safe deserialization into Rust structs
//! - Automatic validation of required fields
//! - Easy to work with in Rust code
//! - Standard serde derive macros
//!
//! Run with: cargo run --example xaml_serde_demo

use serde::{Deserialize, Serialize};
use quick_xml::de::from_str;

/// Button element with serde deserialization
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "Button")]
struct ButtonXaml {
    #[serde(rename = "@Content")]
    content: String,
    
    #[serde(rename = "@Width", default)]
    width: Option<f64>,
    
    #[serde(rename = "@Height", default)]
    height: Option<f64>,
    
    #[serde(rename = "@Background", default)]
    background: Option<String>,
    
    #[serde(rename = "@Foreground", default)]
    foreground: Option<String>,
    
    #[serde(rename = "@CornerRadius", default)]
    corner_radius: Option<f64>,
}

/// TextBlock element with serde deserialization
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "TextBlock")]
struct TextBlockXaml {
    #[serde(rename = "@Text")]
    text: String,
    
    #[serde(rename = "@FontSize", default)]
    font_size: Option<f64>,
    
    #[serde(rename = "@FontWeight", default)]
    font_weight: Option<i32>,
    
    #[serde(rename = "@Foreground", default)]
    foreground: Option<String>,
}

/// TextBox element with serde deserialization
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "TextBox")]
struct TextBoxXaml {
    #[serde(rename = "@PlaceholderText", default)]
    placeholder: Option<String>,
    
    #[serde(rename = "@Text", default)]
    text: Option<String>,
    
    #[serde(rename = "@Width", default)]
    width: Option<f64>,
    
    #[serde(rename = "@Height", default)]
    height: Option<f64>,
    
    #[serde(rename = "@Background", default)]
    background: Option<String>,
}

/// StackPanel with children
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "StackPanel")]
struct StackPanelXaml {
    #[serde(rename = "@Orientation", default)]
    orientation: Option<String>,
    
    #[serde(rename = "@Spacing", default)]
    spacing: Option<f64>,
    
    #[serde(rename = "@Background", default)]
    background: Option<String>,
    
    // Children would go here in a more complete implementation
    // #[serde(rename = "$value", default)]
    // children: Vec<XamlElement>,
}

fn main() {
    println!("\n╔═══════════════════════════════════════════════════════════════════════════╗");
    println!("║              🔧 SERDE-BASED XAML PARSING DEMO 🔧                       ║");
    println!("╚═══════════════════════════════════════════════════════════════════════════╝\n");

    println!("This demo shows 100% serde-compliant XAML parsing!");
    println!("Using quick-xml with serde derive macros.\n");

    // Example 1: Parse a Button
    println!("📝 Example 1: Deserialize Button from XAML");
    let button_xaml = r##"
        <Button Content="Click Me"
                Width="200"
                Height="50"
                Background="#FF0078D4"
                Foreground="#FFFFFFFF"
                CornerRadius="8" />
    "##;

    match from_str::<ButtonXaml>(button_xaml) {
        Ok(button) => {
            println!("   ✅ Successfully deserialized Button!");
            println!("      Content: {}", button.content);
            println!("      Width: {:?}", button.width);
            println!("      Height: {:?}", button.height);
            println!("      Background: {:?}", button.background);
            println!("      Foreground: {:?}", button.foreground);
            println!("      CornerRadius: {:?}\n", button.corner_radius);
        }
        Err(e) => println!("   ❌ Error: {}\n", e),
    }

    // Example 2: Parse a TextBlock
    println!("📝 Example 2: Deserialize TextBlock from XAML");
    let textblock_xaml = r##"
        <TextBlock Text="Hello Serde!"
                   FontSize="24"
                   FontWeight="700"
                   Foreground="#FF00D7FF" />
    "##;

    match from_str::<TextBlockXaml>(textblock_xaml) {
        Ok(textblock) => {
            println!("   ✅ Successfully deserialized TextBlock!");
            println!("      Text: {}", textblock.text);
            println!("      FontSize: {:?}", textblock.font_size);
            println!("      FontWeight: {:?}", textblock.font_weight);
            println!("      Foreground: {:?}\n", textblock.foreground);
        }
        Err(e) => println!("   ❌ Error: {}\n", e),
    }

    // Example 3: Parse a TextBox
    println!("📝 Example 3: Deserialize TextBox from XAML");
    let textbox_xaml = r##"
        <TextBox PlaceholderText="Enter text..."
                 Width="300"
                 Height="40"
                 Background="#FF2D2D2D" />
    "##;

    match from_str::<TextBoxXaml>(textbox_xaml) {
        Ok(textbox) => {
            println!("   ✅ Successfully deserialized TextBox!");
            println!("      PlaceholderText: {:?}", textbox.placeholder);
            println!("      Width: {:?}", textbox.width);
            println!("      Height: {:?}", textbox.height);
            println!("      Background: {:?}\n", textbox.background);
        }
        Err(e) => println!("   ❌ Error: {}\n", e),
    }

    // Example 4: Parse a StackPanel
    println!("📝 Example 4: Deserialize StackPanel from XAML");
    let panel_xaml = r##"
        <StackPanel Orientation="Vertical"
                    Spacing="10"
                    Background="#FF1A1A1A" />
    "##;

    match from_str::<StackPanelXaml>(panel_xaml) {
        Ok(panel) => {
            println!("   ✅ Successfully deserialized StackPanel!");
            println!("      Orientation: {:?}", panel.orientation);
            println!("      Spacing: {:?}", panel.spacing);
            println!("      Background: {:?}\n", panel.background);
        }
        Err(e) => println!("   ❌ Error: {}\n", e),
    }

    // Example 5: Validation - missing required field
    println!("📝 Example 5: Validation (missing required field)");
    let invalid_xaml = r##"
        <Button Width="200" Height="50" />
    "##;

    match from_str::<ButtonXaml>(invalid_xaml) {
        Ok(_) => println!("   ⚠️  Unexpectedly succeeded\n"),
        Err(e) => {
            println!("   ✅ Validation worked! Missing 'Content' field caught:");
            println!("      Error: {}\n", e);
        }
    }

    // Example 6: Serialize back to XAML
    println!("📝 Example 6: Serialize Rust struct to XAML");
    let button = ButtonXaml {
        content: "Serialized Button".to_string(),
        width: Some(250.0),
        height: Some(60.0),
        background: Some(String::from("#FF28A745")),
        foreground: Some(String::from("#FFFFFFFF")),
        corner_radius: Some(10.0),
    };

    match quick_xml::se::to_string(&button) {
        Ok(xml) => {
            println!("   ✅ Successfully serialized to XAML!");
            println!("      {}\n", xml);
        }
        Err(e) => println!("   ❌ Error: {}\n", e),
    }

    println!("╔═══════════════════════════════════════════════════════════════════════════╗");
    println!("║                         ✨ DEMO COMPLETE! ✨                           ║");
    println!("╚═══════════════════════════════════════════════════════════════════════════╝\n");

    println!("🎯 KEY BENEFITS OF SERDE-BASED PARSING:");
    println!("   ✅ Type-safe: Rust structs with strong typing");
    println!("   ✅ Validated: Required fields enforced by serde");
    println!("   ✅ Standard: Uses standard serde derive macros");
    println!("   ✅ Bidirectional: Serialize and deserialize");
    println!("   ✅ Flexible: Optional fields with defaults");
    println!("   ✅ Composable: Easy to nest structures\n");

    println!("📚 ALTERNATIVES:");
    println!("   • quick-xml (with serde) - Fast, zero-copy, serde-compliant ✅");
    println!("   • serde-xml-rs - Pure serde implementation");
    println!("   • roxmltree - Read-only, fast XML tree");
    println!("   • xml-rs - Low-level SAX-style parser\n");
}
