//! Tests for Animation System functionality

#[cfg(feature = "xaml-islands")]
mod animation_tests {
    use winrt_xaml::xaml_native::{XamlDoubleAnimation, XamlColorAnimation, XamlStoryboard};

    #[test]
    fn test_storyboard_create() {
        let storyboard = XamlStoryboard::new();
        assert!(storyboard.is_ok(), "Should create Storyboard");
    }

    #[test]
    fn test_double_animation_create() {
        let animation = XamlDoubleAnimation::new();
        assert!(animation.is_ok(), "Should create DoubleAnimation");
    }

    #[test]
    fn test_double_animation_set_from() {
        let animation = XamlDoubleAnimation::new().unwrap();
        let result = animation.set_from(0.0);
        assert!(result.is_ok(), "Should set from value");
    }

    #[test]
    fn test_double_animation_set_to() {
        let animation = XamlDoubleAnimation::new().unwrap();
        let result = animation.set_to(100.0);
        assert!(result.is_ok(), "Should set to value");
    }

    #[test]
    fn test_double_animation_set_duration() {
        let animation = XamlDoubleAnimation::new().unwrap();
        let result = animation.set_duration_ms(500);
        assert!(result.is_ok(), "Should set duration");
    }

    #[test]
    fn test_double_animation_builder() {
        let animation = XamlDoubleAnimation::builder()
            .from(0.0)
            .to(100.0)
            .duration_ms(300)
            .build();
        
        assert!(animation.is_ok(), "Should build animation with builder pattern");
    }

    #[test]
    fn test_color_animation_create() {
        let animation = XamlColorAnimation::new();
        assert!(animation.is_ok(), "Should create ColorAnimation");
    }

    #[test]
    fn test_color_animation_set_from() {
        let animation = XamlColorAnimation::new().unwrap();
        let result = animation.set_from(0xFFFF0000); // Red
        assert!(result.is_ok(), "Should set from color");
    }

    #[test]
    fn test_color_animation_set_to() {
        let animation = XamlColorAnimation::new().unwrap();
        let result = animation.set_to(0xFF00FF00); // Green
        assert!(result.is_ok(), "Should set to color");
    }

    #[test]
    fn test_color_animation_set_duration() {
        let animation = XamlColorAnimation::new().unwrap();
        let result = animation.set_duration_ms(500);
        assert!(result.is_ok(), "Should set duration");
    }

    #[test]
    fn test_color_animation_builder() {
        let animation = XamlColorAnimation::builder()
            .from(0xFFFF0000)
            .to(0xFF00FF00)
            .duration_ms(400)
            .build();
        
        assert!(animation.is_ok(), "Should build color animation with builder pattern");
    }

    #[test]
    fn test_storyboard_add_animation() {
        let storyboard = XamlStoryboard::new().unwrap();
        let animation = XamlDoubleAnimation::new().unwrap();
        
        let result = storyboard.add_animation(&animation);
        assert!(result.is_ok(), "Should add animation to storyboard");
    }

    #[test]
    fn test_storyboard_add_color_animation() {
        let storyboard = XamlStoryboard::new().unwrap();
        let animation = XamlColorAnimation::new().unwrap();
        
        let result = storyboard.add_color_animation(&animation);
        assert!(result.is_ok(), "Should add color animation to storyboard");
    }

    #[test]
    fn test_default_storyboard() {
        let storyboard = XamlStoryboard::default();
        
        // Should be able to use the default storyboard
        let animation = XamlDoubleAnimation::new().unwrap();
        let result = storyboard.add_animation(&animation);
        assert!(result.is_ok(), "Default storyboard should be usable");
    }

    #[test]
    fn test_default_double_animation() {
        let animation = XamlDoubleAnimation::default();
        
        // Should be able to configure the default animation
        let result = animation.set_from(0.0);
        assert!(result.is_ok(), "Default animation should be usable");
    }

    #[test]
    fn test_default_color_animation() {
        let animation = XamlColorAnimation::default();
        
        // Should be able to configure the default animation
        let result = animation.set_from(0xFFFFFFFF);
        assert!(result.is_ok(), "Default color animation should be usable");
    }

    #[test]
    fn test_multiple_animations_in_storyboard() {
        let storyboard = XamlStoryboard::new().unwrap();
        
        let anim1 = XamlDoubleAnimation::builder()
            .from(0.0)
            .to(100.0)
            .duration_ms(300)
            .build()
            .unwrap();
        
        let anim2 = XamlDoubleAnimation::builder()
            .from(1.0)
            .to(0.0)
            .duration_ms(500)
            .build()
            .unwrap();
        
        assert!(storyboard.add_animation(&anim1).is_ok());
        assert!(storyboard.add_animation(&anim2).is_ok());
    }
}
