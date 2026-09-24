//! Design tokens shared by every PlayBack view.

pub mod animation;
pub mod colors;
pub mod typography;

pub use animation::{AnimationCurve, AnimationTokens, Duration};
pub use colors::{Color, ColorTokens};
pub use typography::{FontFamily, FontSize, TypographyTokens};

/// The complete visual theme used by the player.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Theme {
    /// Semantic color tokens.
    pub colors: ColorTokens,
    /// Typography tokens.
    pub typography: TypographyTokens,
    /// Animation tokens.
    pub animation: AnimationTokens,
    /// Minimum interactive target size in points.
    pub minimum_touch_target: f32,
    /// Standard control corner radius in points.
    pub control_corner_radius: f32,
    /// Playlist sidebar width in points.
    pub playlist_width: f32,
}

impl Theme {
    /// Returns the macOS HIG-inspired theme.
    pub const fn macos_hig() -> Self {
        Self {
            colors: ColorTokens::macos_hig(),
            typography: TypographyTokens::macos_hig(),
            animation: AnimationTokens::macos_hig(),
            minimum_touch_target: 44.0,
            control_corner_radius: 8.0,
            playlist_width: 280.0,
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::macos_hig()
    }
}

#[cfg(test)]
mod tests {
    use super::{Color, Theme};

    #[test]
    fn theme_contains_macos_hig_tokens() {
        let theme = Theme::macos_hig();
        assert_eq!(theme.colors.accent, Color::rgb(10, 132, 255));
        assert!((theme.typography.body.points - 15.0).abs() < f32::EPSILON);
        assert!((theme.minimum_touch_target - 44.0).abs() < f32::EPSILON);
        assert!((theme.playlist_width - 280.0).abs() < f32::EPSILON);
    }
}
