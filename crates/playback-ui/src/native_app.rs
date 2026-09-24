//! Minimal native GPUI shell for the macOS build.
//!
//! The shell owns the window and control surface. Media-engine work remains in
//! `playback-core`, so a renderer can replace the placeholder video region with
//! a texture-backed mpv surface without changing parsing or configuration.

#![cfg(all(feature = "native", target_os = "macos"))]

use std::path::PathBuf;

use gpui::{
    App, Application, Bounds, Context, Render, SharedString, Window, WindowBounds, WindowOptions,
    div, px, rgb, size,
};
use gpui_design::DesignSystem;
use gpui_ui_kit::{Button, ButtonVariant};
use lucide_icons::Icon;
use playback_config::PlaybackConfig;

struct NativePlayer {
    title: SharedString,
    design: DesignSystem,
    paused: bool,
    volume: f64,
}

impl Render for NativePlayer {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl gpui::IntoElement {
        let design = self.design.clone();
        let play_label = if self.paused { "Play" } else { "Pause" };
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0x1e1e1e))
            .child(
                div().flex_1().flex().items_center().justify_center().child(
                    div()
                        .text_color(rgb(0xf5f5f5))
                        .text_size(px(design.typography.base_size))
                        .child(format!("{} — {}", self.title, Icon::Film)),
                ),
            )
            .child(
                div()
                    .flex()
                    .gap(px(design.spacing.control_gap))
                    .p(px(design.spacing.card_padding))
                    .rounded(px(design.corners.md))
                    .bg(rgb(0x303030))
                    .child(
                        Button::new("playback-toggle", play_label)
                            .variant(ButtonVariant::Secondary)
                            .icon_left(Icon::Play.to_string()),
                    )
                    .child(format!("Volume {:.0}%", self.volume)),
            )
    }
}

/// Starts the native macOS player window.
///
/// # Errors
///
/// Returns an error only when the host cannot initialize the native windowing
/// system. The current GPUI runtime reports window creation through its event
/// loop, so normal startup completes after the first window is scheduled.
pub fn run_native(media: &[PathBuf], config: &PlaybackConfig) -> anyhow::Result<()> {
    let title = media
        .first()
        .and_then(|path| path.file_name())
        .map(|name| SharedString::from(name.to_string_lossy().into_owned()))
        .unwrap_or_else(|| SharedString::from("PlayBack"));
    let design = DesignSystem::apple_hig();
    let volume = config.playback().volume();
    let paused = !config.playback().autoplay;
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(1280.), px(720.)), cx);
        let window = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            ..Default::default()
        };
        if let Err(error) = cx.open_window(window, |_, cx| {
            cx.new(|_| NativePlayer {
                title: title.clone(),
                design: design.clone(),
                paused,
                volume,
            })
        }) {
            eprintln!("PlayBack could not open its native window: {error}");
        }
        cx.activate(true);
    });
    Ok(())
}
