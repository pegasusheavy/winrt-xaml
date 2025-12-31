//! WinRT Animation System - Storyboard and Animation types

use super::ffi::{self, XamlStoryboardHandle, XamlDoubleAnimationHandle, XamlColorAnimationHandle, XamlUIElementHandle};
use crate::error::{Error, Result};
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

/// A WinRT Storyboard for orchestrating animations
pub struct XamlStoryboard {
    handle: XamlStoryboardHandle,
}

impl XamlStoryboard {
    /// Create a new Storyboard
    pub fn new() -> Result<Self> {
        let handle = unsafe { ffi::xaml_storyboard_create() };
        if handle.0.is_null() {
            return Err(Error::control_creation("Failed to create Storyboard"));
        }
        Ok(Self { handle })
    }

    /// Add a DoubleAnimation to the storyboard
    pub fn add_animation(&self, animation: &XamlDoubleAnimation) -> Result<()> {
        let result = unsafe {
            ffi::xaml_storyboard_add_animation(self.handle, animation.handle())
        };

        if result != 0 {
            return Err(Error::invalid_operation("Failed to add animation"));
        }

        Ok(())
    }

    /// Add a ColorAnimation to the storyboard
    pub fn add_color_animation(&self, animation: &XamlColorAnimation) -> Result<()> {
        let result = unsafe {
            ffi::xaml_storyboard_add_color_animation(self.handle, animation.handle())
        };

        if result != 0 {
            return Err(Error::invalid_operation("Failed to add color animation"));
        }

        Ok(())
    }

    /// Set the target UI element for all animations in this storyboard
    pub(crate) fn set_target(&self, target: XamlUIElementHandle) -> Result<()> {
        let result = unsafe {
            ffi::xaml_storyboard_set_target(self.handle, target)
        };

        if result != 0 {
            return Err(Error::invalid_operation("Failed to set storyboard target"));
        }

        Ok(())
    }

    /// Begin the storyboard animation
    pub fn begin(&self) -> Result<()> {
        let result = unsafe {
            ffi::xaml_storyboard_begin(self.handle)
        };

        if result != 0 {
            return Err(Error::invalid_operation("Failed to begin storyboard"));
        }

        Ok(())
    }

    /// Stop the storyboard animation
    pub fn stop(&self) -> Result<()> {
        let result = unsafe {
            ffi::xaml_storyboard_stop(self.handle)
        };

        if result != 0 {
            return Err(Error::invalid_operation("Failed to stop storyboard"));
        }

        Ok(())
    }

    /// Pause the storyboard animation
    pub fn pause(&self) -> Result<()> {
        let result = unsafe {
            ffi::xaml_storyboard_pause(self.handle)
        };

        if result != 0 {
            return Err(Error::invalid_operation("Failed to pause storyboard"));
        }

        Ok(())
    }

    /// Resume a paused storyboard animation
    pub fn resume(&self) -> Result<()> {
        let result = unsafe {
            ffi::xaml_storyboard_resume(self.handle)
        };

        if result != 0 {
            return Err(Error::invalid_operation("Failed to resume storyboard"));
        }

        Ok(())
    }

    /// Get the raw handle
    pub(crate) fn handle(&self) -> XamlStoryboardHandle {
        self.handle
    }
}

impl Default for XamlStoryboard {
    fn default() -> Self {
        Self::new().expect("Failed to create default Storyboard")
    }
}

impl Drop for XamlStoryboard {
    fn drop(&mut self) {
        if !self.handle.0.is_null() {
            unsafe {
                ffi::xaml_storyboard_destroy(self.handle);
            }
        }
    }
}

unsafe impl Send for XamlStoryboard {}
unsafe impl Sync for XamlStoryboard {}

/// A WinRT DoubleAnimation for animating numeric properties
pub struct XamlDoubleAnimation {
    handle: XamlDoubleAnimationHandle,
}

impl XamlDoubleAnimation {
    /// Create a new DoubleAnimation
    pub fn new() -> Result<Self> {
        let handle = unsafe { ffi::xaml_double_animation_create() };
        if handle.0.is_null() {
            return Err(Error::control_creation("Failed to create DoubleAnimation"));
        }
        Ok(Self { handle })
    }

    /// Create a new DoubleAnimation with builder pattern
    ///
    /// # Example
    /// ```no_run
    /// use winrt_xaml::xaml_native::XamlDoubleAnimation;
    ///
    /// let animation = XamlDoubleAnimation::builder()
    ///     .from(0.0)
    ///     .to(100.0)
    ///     .duration_ms(300)
    ///     .build()?;
    /// # Ok::<(), winrt_xaml::Error>(())
    /// ```
    pub fn builder() -> DoubleAnimationBuilder {
        DoubleAnimationBuilder::default()
    }

    /// Set the starting value
    pub fn set_from(&self, from: f64) -> Result<()> {
        let result = unsafe {
            ffi::xaml_double_animation_set_from(self.handle, from)
        };

        if result != 0 {
            return Err(Error::invalid_operation("Failed to set from value"));
        }

        Ok(())
    }

    /// Set the ending value
    pub fn set_to(&self, to: f64) -> Result<()> {
        let result = unsafe {
            ffi::xaml_double_animation_set_to(self.handle, to)
        };

        if result != 0 {
            return Err(Error::invalid_operation("Failed to set to value"));
        }

        Ok(())
    }

    /// Set the animation duration in milliseconds
    pub fn set_duration_ms(&self, milliseconds: i32) -> Result<()> {
        let result = unsafe {
            ffi::xaml_double_animation_set_duration(self.handle, milliseconds)
        };

        if result != 0 {
            return Err(Error::invalid_operation("Failed to set duration"));
        }

        Ok(())
    }

    /// Set the target property path
    ///
    /// # Arguments
    /// * `target` - The UI element to animate
    /// * `property_path` - Property path (e.g., "Opacity", "Width", "Height")
    pub(crate) fn set_target_property(&self, target: XamlUIElementHandle, property_path: impl AsRef<str>) -> Result<()> {
        let path_wide: Vec<u16> = OsStr::new(property_path.as_ref())
            .encode_wide()
            .chain(Some(0))
            .collect();

        let result = unsafe {
            ffi::xaml_double_animation_set_target_property(
                self.handle,
                target,
                path_wide.as_ptr(),
            )
        };

        if result != 0 {
            return Err(Error::invalid_operation("Failed to set target property"));
        }

        Ok(())
    }

    /// Get the raw handle
    pub(crate) fn handle(&self) -> XamlDoubleAnimationHandle {
        self.handle
    }
}

impl Default for XamlDoubleAnimation {
    fn default() -> Self {
        Self::new().expect("Failed to create default DoubleAnimation")
    }
}

impl Drop for XamlDoubleAnimation {
    fn drop(&mut self) {
        if !self.handle.0.is_null() {
            unsafe {
                ffi::xaml_double_animation_destroy(self.handle);
            }
        }
    }
}

unsafe impl Send for XamlDoubleAnimation {}
unsafe impl Sync for XamlDoubleAnimation {}

/// Builder for DoubleAnimation
#[derive(Default)]
pub struct DoubleAnimationBuilder {
    from: Option<f64>,
    to: Option<f64>,
    duration_ms: Option<i32>,
}

impl DoubleAnimationBuilder {
    /// Set the starting value
    pub fn from(mut self, value: f64) -> Self {
        self.from = Some(value);
        self
    }

    /// Set the ending value
    pub fn to(mut self, value: f64) -> Self {
        self.to = Some(value);
        self
    }

    /// Set the duration in milliseconds
    pub fn duration_ms(mut self, milliseconds: i32) -> Self {
        self.duration_ms = Some(milliseconds);
        self
    }

    /// Build the animation
    pub fn build(self) -> Result<XamlDoubleAnimation> {
        let animation = XamlDoubleAnimation::new()?;

        if let Some(from) = self.from {
            animation.set_from(from)?;
        }

        if let Some(to) = self.to {
            animation.set_to(to)?;
        }

        if let Some(duration) = self.duration_ms {
            animation.set_duration_ms(duration)?;
        }

        Ok(animation)
    }
}

/// A WinRT ColorAnimation for animating color properties
pub struct XamlColorAnimation {
    handle: XamlColorAnimationHandle,
}

impl XamlColorAnimation {
    /// Create a new ColorAnimation
    pub fn new() -> Result<Self> {
        let handle = unsafe { ffi::xaml_color_animation_create() };
        if handle.0.is_null() {
            return Err(Error::control_creation("Failed to create ColorAnimation"));
        }
        Ok(Self { handle })
    }

    /// Create a new ColorAnimation with builder pattern
    pub fn builder() -> ColorAnimationBuilder {
        ColorAnimationBuilder::default()
    }

    /// Set the starting color (ARGB format)
    pub fn set_from(&self, from: u32) -> Result<()> {
        let result = unsafe {
            ffi::xaml_color_animation_set_from(self.handle, from)
        };

        if result != 0 {
            return Err(Error::invalid_operation("Failed to set from color"));
        }

        Ok(())
    }

    /// Set the ending color (ARGB format)
    pub fn set_to(&self, to: u32) -> Result<()> {
        let result = unsafe {
            ffi::xaml_color_animation_set_to(self.handle, to)
        };

        if result != 0 {
            return Err(Error::invalid_operation("Failed to set to color"));
        }

        Ok(())
    }

    /// Set the animation duration in milliseconds
    pub fn set_duration_ms(&self, milliseconds: i32) -> Result<()> {
        let result = unsafe {
            ffi::xaml_color_animation_set_duration(self.handle, milliseconds)
        };

        if result != 0 {
            return Err(Error::invalid_operation("Failed to set duration"));
        }

        Ok(())
    }

    /// Set the target property path
    ///
    /// # Arguments
    /// * `target` - The UI element to animate
    /// * `property_path` - Property path (e.g., "(Button.Background).(SolidColorBrush.Color)")
    pub(crate) fn set_target_property(&self, target: XamlUIElementHandle, property_path: impl AsRef<str>) -> Result<()> {
        let path_wide: Vec<u16> = OsStr::new(property_path.as_ref())
            .encode_wide()
            .chain(Some(0))
            .collect();

        let result = unsafe {
            ffi::xaml_color_animation_set_target_property(
                self.handle,
                target,
                path_wide.as_ptr(),
            )
        };

        if result != 0 {
            return Err(Error::invalid_operation("Failed to set target property"));
        }

        Ok(())
    }

    /// Get the raw handle
    pub(crate) fn handle(&self) -> XamlColorAnimationHandle {
        self.handle
    }
}

impl Default for XamlColorAnimation {
    fn default() -> Self {
        Self::new().expect("Failed to create default ColorAnimation")
    }
}

impl Drop for XamlColorAnimation {
    fn drop(&mut self) {
        if !self.handle.0.is_null() {
            unsafe {
                ffi::xaml_color_animation_destroy(self.handle);
            }
        }
    }
}

unsafe impl Send for XamlColorAnimation {}
unsafe impl Sync for XamlColorAnimation {}

/// Builder for ColorAnimation
#[derive(Default)]
pub struct ColorAnimationBuilder {
    from: Option<u32>,
    to: Option<u32>,
    duration_ms: Option<i32>,
}

impl ColorAnimationBuilder {
    /// Set the starting color (ARGB format, e.g., 0xFFFF0000 for red)
    pub fn from(mut self, color: u32) -> Self {
        self.from = Some(color);
        self
    }

    /// Set the ending color (ARGB format)
    pub fn to(mut self, color: u32) -> Self {
        self.to = Some(color);
        self
    }

    /// Set the duration in milliseconds
    pub fn duration_ms(mut self, milliseconds: i32) -> Self {
        self.duration_ms = Some(milliseconds);
        self
    }

    /// Build the animation
    pub fn build(self) -> Result<XamlColorAnimation> {
        let animation = XamlColorAnimation::new()?;

        if let Some(from) = self.from {
            animation.set_from(from)?;
        }

        if let Some(to) = self.to {
            animation.set_to(to)?;
        }

        if let Some(duration) = self.duration_ms {
            animation.set_duration_ms(duration)?;
        }

        Ok(animation)
    }
}
