//! Media types (colors, brushes, etc.) using Win32 GDI.

/// Win32 COLORREF type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct COLORREF(pub u32);

/// A color value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    /// Red component (0-255).
    pub r: u8,
    /// Green component (0-255).
    pub g: u8,
    /// Blue component (0-255).
    pub b: u8,
    /// Alpha component (0-255).
    pub a: u8,
}

impl Color {
    /// Create a new color from RGBA components.
    pub fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color { r, g, b, a }
    }

    /// Create a new color from RGB components (full opacity).
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b, a: 255 }
    }

    /// Convert to Win32 COLORREF.
    pub fn to_colorref(&self) -> COLORREF {
        COLORREF((self.r as u32) | ((self.g as u32) << 8) | ((self.b as u32) << 16))
    }

    /// Create from Win32 COLORREF.
    pub fn from_colorref(colorref: COLORREF) -> Self {
        let value = colorref.0;
        Color {
            r: (value & 0xFF) as u8,
            g: ((value >> 8) & 0xFF) as u8,
            b: ((value >> 16) & 0xFF) as u8,
            a: 255,
        }
    }

    /// Black color.
    pub const BLACK: Color = Color { r: 0, g: 0, b: 0, a: 255 };
    /// White color.
    pub const WHITE: Color = Color { r: 255, g: 255, b: 255, a: 255 };
    /// Red color.
    pub const RED: Color = Color { r: 255, g: 0, b: 0, a: 255 };
    /// Green color.
    pub const GREEN: Color = Color { r: 0, g: 255, b: 0, a: 255 };
    /// Blue color.
    pub const BLUE: Color = Color { r: 0, g: 0, b: 255, a: 255 };
    /// Transparent color.
    pub const TRANSPARENT: Color = Color { r: 0, g: 0, b: 0, a: 0 };
}

/// A brush for painting.
#[derive(Clone)]
pub struct Brush {
    color: Color,
}

impl Brush {
    /// Create a new solid color brush.
    pub fn new(color: Color) -> Self {
        Brush { color }
    }

    /// Get the color of this brush.
    pub fn color(&self) -> Color {
        self.color
    }

    /// Set the color of this brush.
    pub fn set_color(&self, _color: Color) {
        // TODO: Implement
    }
}

/// A solid color brush.
pub type SolidColorBrush = Brush;
