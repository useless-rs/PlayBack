//! Spring and ease-out animation tokens.

/// A duration expressed in milliseconds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Duration {
    /// Duration in milliseconds.
    pub milliseconds: u64,
}

impl Duration {
    /// Creates a duration token.
    pub const fn milliseconds(milliseconds: u64) -> Self {
        Self { milliseconds }
    }
}

/// The interpolation curve used by a transition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimationCurve {
    /// A spring-like curve for controls and sheets.
    Spring,
    /// A short ease-out curve for layout changes.
    EaseOut,
    /// A linear curve for progress indicators.
    Linear,
}

/// The standard animation timings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AnimationTokens {
    /// Play/pause icon crossfade duration.
    pub icon_crossfade: Duration,
    /// Controls reveal duration.
    pub controls_reveal: Duration,
    /// Timeline hover expansion duration.
    pub timeline_hover: Duration,
    /// Window reflow duration.
    pub window_reflow: Duration,
    /// Curve used for interactive controls.
    pub control_curve: AnimationCurve,
    /// Curve used for layout changes.
    pub layout_curve: AnimationCurve,
}

impl AnimationTokens {
    /// Returns the interaction timings specified by the player design.
    pub const fn macos_hig() -> Self {
        Self {
            icon_crossfade: Duration::milliseconds(200),
            controls_reveal: Duration::milliseconds(350),
            timeline_hover: Duration::milliseconds(120),
            window_reflow: Duration::milliseconds(150),
            control_curve: AnimationCurve::Spring,
            layout_curve: AnimationCurve::EaseOut,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{AnimationCurve, AnimationTokens};

    #[test]
    fn uses_short_spring_interactions() {
        let animation = AnimationTokens::macos_hig();
        assert_eq!(animation.icon_crossfade.milliseconds, 200);
        assert_eq!(animation.controls_reveal.milliseconds, 350);
        assert_eq!(animation.window_reflow.milliseconds, 150);
        assert_eq!(animation.control_curve, AnimationCurve::Spring);
    }
}
