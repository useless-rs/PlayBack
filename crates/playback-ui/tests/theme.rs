//! Headless UI contract tests.

use playback_ui::{Color, Theme};

#[test]
fn theme_matches_macos_hig_palette() {
    let theme = Theme::macos_hig();
    assert_eq!(theme.colors.accent, Color::rgb(10, 132, 255));
    assert_eq!(theme.colors.window, Color::rgb(30, 30, 30));
    assert!((theme.typography.body.points - 15.0).abs() < f32::EPSILON);
    assert!((theme.minimum_touch_target - 44.0).abs() < f32::EPSILON);
}
