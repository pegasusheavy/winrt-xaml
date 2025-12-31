//! Tests for Resource Dictionary functionality

#[cfg(feature = "xaml-islands")]
mod resource_dictionary_tests {
    use winrt_xaml::xaml_native::XamlResourceDictionary;

    #[test]
    fn test_resource_dictionary_create() {
        let dict = XamlResourceDictionary::new();
        assert!(dict.is_ok(), "Should create ResourceDictionary");
    }

    #[test]
    fn test_insert_and_get_color() {
        let dict = XamlResourceDictionary::new().unwrap();
        
        let result = dict.insert_color("PrimaryColor", 0xFF0078D4);
        assert!(result.is_ok(), "Should insert color");

        let color = dict.get_color("PrimaryColor");
        assert_eq!(color, Some(0xFF0078D4), "Should retrieve correct color");
    }

    #[test]
    fn test_insert_and_get_double() {
        let dict = XamlResourceDictionary::new().unwrap();
        
        let result = dict.insert_double("Spacing", 20.0);
        assert!(result.is_ok(), "Should insert double");

        let value = dict.get_double("Spacing");
        assert_eq!(value, Some(20.0), "Should retrieve correct double");
    }

    #[test]
    fn test_insert_and_get_string() {
        let dict = XamlResourceDictionary::new().unwrap();
        
        let result = dict.insert_string("AppName", "Test App");
        assert!(result.is_ok(), "Should insert string");
    }

    #[test]
    fn test_has_key() {
        let dict = XamlResourceDictionary::new().unwrap();
        
        dict.insert_color("TestColor", 0xFFFF0000).unwrap();
        
        assert!(dict.has_key("TestColor"), "Should find existing key");
        assert!(!dict.has_key("NonExistent"), "Should not find non-existent key");
    }

    #[test]
    fn test_remove_resource() {
        let dict = XamlResourceDictionary::new().unwrap();
        
        dict.insert_color("TempColor", 0xFF00FF00).unwrap();
        assert!(dict.has_key("TempColor"), "Key should exist before removal");
        
        let result = dict.remove("TempColor");
        assert!(result.is_ok(), "Should remove resource");
        
        assert!(!dict.has_key("TempColor"), "Key should not exist after removal");
    }

    #[test]
    fn test_clear_resources() {
        let dict = XamlResourceDictionary::new().unwrap();
        
        dict.insert_color("Color1", 0xFFFF0000).unwrap();
        dict.insert_color("Color2", 0xFF00FF00).unwrap();
        dict.insert_double("Value1", 10.0).unwrap();
        
        dict.clear();
        
        assert!(!dict.has_key("Color1"), "Dictionary should be empty after clear");
        assert!(!dict.has_key("Color2"), "Dictionary should be empty after clear");
        assert!(!dict.has_key("Value1"), "Dictionary should be empty after clear");
    }

    #[test]
    fn test_multiple_resources() {
        let dict = XamlResourceDictionary::new().unwrap();
        
        dict.insert_color("Primary", 0xFF0078D4).unwrap();
        dict.insert_color("Secondary", 0xFF107C10).unwrap();
        dict.insert_double("Spacing", 20.0).unwrap();
        dict.insert_double("FontSize", 14.0).unwrap();
        dict.insert_string("Title", "App Title").unwrap();
        
        assert_eq!(dict.get_color("Primary"), Some(0xFF0078D4));
        assert_eq!(dict.get_color("Secondary"), Some(0xFF107C10));
        assert_eq!(dict.get_double("Spacing"), Some(20.0));
        assert_eq!(dict.get_double("FontSize"), Some(14.0));
    }

    #[test]
    fn test_get_nonexistent_resource() {
        let dict = XamlResourceDictionary::new().unwrap();
        
        assert_eq!(dict.get_color("NonExistent"), None);
        assert_eq!(dict.get_double("NonExistent"), None);
    }

    #[test]
    fn test_default_resource_dictionary() {
        let dict = XamlResourceDictionary::default();
        
        // Should be able to use the default dictionary
        let result = dict.insert_color("TestColor", 0xFFFFFFFF);
        assert!(result.is_ok(), "Default dictionary should be usable");
    }
}
