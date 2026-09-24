//! Typography tokens for the player interface.

/// The preferred font family for UI text.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontFamily {
    /// Apple's system display face.
    SystemDisplay,
    /// Apple's system text face.
    SystemText,
    /// A monospaced face for timestamps.
    SystemMonospace,
}

impl FontFamily {
    /// Returns the platform font-family name.
    pub const fn name(self) -> &'static str {
        match self {
            Self::SystemDisplay => "SF Pro Display",
            Self::SystemText => "SF Pro Text",
            Self::SystemMonospace => "SF Mono",
        }
    }
}

/// A font size in points.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FontSize {
    /// Point size.
    pub points: f32,
}

impl FontSize {
    /// Creates a font-size token.
    pub const fn points(points: f32) -> Self {
        Self { points }
    }
}

/// The type scale used by the player.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TypographyTokens {
    /// Base UI text size.
    pub body: FontSize,
    /// Secondary UI text size.
    pub caption: FontSize,
    /// Large title text size.
    pub title: FontSize,
    /// Monospace timestamp size.
    pub timestamp: FontSize,
    /// Preferred UI family.
    pub family: FontFamily,
    /// Preferred monospace family.
    pub monospace_family: FontFamily,
}

impl TypographyTokens {
    /// Returns the macOS-native type scale.
    pub const fn macos_hig() -> Self {
        Self {
            body: FontSize::points(15.0),
            caption: FontSize::points(12.0),
            title: FontSize::points(20.0),
            timestamp: FontSize::points(12.0),
            family: FontFamily::SystemDisplay,
            monospace_family: FontFamily::SystemMonospace,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{FontFamily, TypographyTokens};

    #[test]
    fn base_type_is_fifteen_points() {
        let typography = TypographyTokens::macos_hig();
        assert!((typography.body.points - 15.0).abs() < f32::EPSILON);
        assert_eq!(typography.family, FontFamily::SystemDisplay);
        assert_eq!(typography.family.name(), "SF Pro Display");
    }

    #[test]
    fn timestamps_use_monospace() {
        let typography = TypographyTokens::macos_hig();
        assert_eq!(typography.monospace_family, FontFamily::SystemMonospace);
    }
}
