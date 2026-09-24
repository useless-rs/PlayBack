//! macOS-inspired color tokens.

use serde::{Deserialize, Serialize};

/// An sRGB color with an alpha channel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Color {
    /// Red channel.
    pub red: u8,
    /// Green channel.
    pub green: u8,
    /// Blue channel.
    pub blue: u8,
    /// Alpha channel.
    pub alpha: u8,
}

impl Color {
    /// Creates an opaque color.
    pub const fn rgb(red: u8, green: u8, blue: u8) -> Self {
        Self {
            red,
            green,
            blue,
            alpha: 255,
        }
    }

    /// Creates a color with an explicit alpha channel.
    pub const fn rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self {
            red,
            green,
            blue,
            alpha,
        }
    }
}

/// The semantic colors used by the player chrome and controls.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColorTokens {
    /// System accent blue.
    pub accent: Color,
    /// Main window background.
    pub window: Color,
    /// Raised control background.
    pub surface: Color,
    /// Hovered control background.
    pub hover: Color,
    /// Primary text.
    pub text_primary: Color,
    /// Secondary text.
    pub text_secondary: Color,
    /// Buffered timeline range.
    pub buffered: Color,
    /// Played timeline range.
    pub played: Color,
    /// Timeline track.
    pub track: Color,
    /// Destructive or error state.
    pub danger: Color,
}

impl ColorTokens {
    /// Returns the Apple HIG-inspired dark palette.
    pub const fn macos_hig() -> Self {
        Self {
            accent: Color::rgb(10, 132, 255),
            window: Color::rgb(30, 30, 30),
            surface: Color::rgba(48, 48, 48, 230),
            hover: Color::rgba(255, 255, 255, 38),
            text_primary: Color::rgb(245, 245, 245),
            text_secondary: Color::rgb(174, 174, 178),
            buffered: Color::rgb(112, 112, 116),
            played: Color::rgb(255, 255, 255),
            track: Color::rgb(72, 72, 76),
            danger: Color::rgb(255, 69, 58),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Color, ColorTokens};

    #[test]
    fn macos_palette_uses_hig_accent() {
        let colors = ColorTokens::macos_hig();
        assert_eq!(colors.accent, Color::rgb(10, 132, 255));
        assert_eq!(colors.window, Color::rgb(30, 30, 30));
        assert_eq!(colors.text_primary, Color::rgb(245, 245, 245));
    }

    #[test]
    fn alpha_is_preserved() {
        let color = Color::rgba(30, 30, 30, 184);
        assert_eq!(color.alpha, 184);
    }
}
